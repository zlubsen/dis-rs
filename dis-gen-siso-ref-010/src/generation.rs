use super::{
    escape_description, format_field_name, format_name, format_name_postfix, Bitfield,
    BitfieldItem, Enum, EnumItem, GenerationItem, Overrides,
};
use proc_macro2::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

macro_rules! override_postfix {
    ($map:ident, $uid:expr) => {
        $map.get(&$uid).is_some_and(|entry| entry.postfix_value)
    };
}
macro_rules! override_embed_xref {
    ($map:ident, $uid:expr) => {
        $map.get(&$uid).is_some_and(|entry| entry.embed_xref)
    };
}
macro_rules! override_size {
    ($map:ident, $uid:expr, $actual:expr) => {
        $map.get(&$uid)
            .and_then(|entry| entry.size)
            .unwrap_or($actual)
    };
}
macro_rules! override_name {
    ($map:ident, $uid:expr, $actual:expr) => {
        $map.get(&$uid)
            .and_then(|ov| ov.name.clone())
            .unwrap_or($actual.clone())
            .as_str()
    };
}

pub fn generate(items: &[GenerationItem], overrides: &Overrides) -> TokenStream {
    let mut generated_items = vec![];

    let lookup_xref = |xref: usize| items.iter().find(|&it| it.uid() == xref);

    for item in items
        .iter()
        .filter(|item| overrides.get(&item.uid()).is_none_or(|entry| !entry.skip))
    {
        match item {
            GenerationItem::Enum(e) => {
                let tokens = generate_enum(e, lookup_xref, overrides);
                // println!("{tokens}");
                // let _ast = syn::parse_file(&tokens.to_string()).unwrap();
                generated_items.push(tokens);
            }
            GenerationItem::Bitfield(b) => {
                let tokens = generate_bitfield(b, lookup_xref);
                // println!("{tokens}");
                // let _ast = syn::parse_file(&tokens.to_string()).unwrap();
                generated_items.push(tokens);
            }
        }
    }

    println!("generated item: {}", generated_items.len());

    // FIXME remove after implementing all stuff for v8
    let type_placeholder = generate_type_placeholder();

    quote!(
        #[allow(clippy::default_trait_access)]
        #[allow(clippy::identity_op)]
        #[allow(clippy::match_same_arms)]
        #[allow(clippy::match_single_binding)]
        #[allow(clippy::struct_excessive_bools)]
        #[allow(clippy::too_many_lines)]
        #[allow(clippy::uninlined_format_args)]
        #[allow(clippy::unreadable_literal)]
        #[allow(clippy::write_literal)]
        #[expect(arithmetic_overflow, reason = "Shifting for bitfields")]
        pub mod enumerations {
            use std::fmt::{Display, Formatter};
            #[cfg(feature = "serde")]
            use serde::{Deserialize, Serialize};

            #(#generated_items)*

            #type_placeholder
        }
    )
}

fn generate_enum<'a, F>(item: &Enum, lookup_xref: F, overrides: &Overrides) -> TokenStream
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    // let formatted_name = format_name(item.name.as_str(), item.uid);
    let formatted_name = format_name(override_name!(overrides, item.uid, item.name), item.uid);
    let name_ident = format_ident!("{}", formatted_name);
    // generate enum declarations
    let decl = quote_enum_decl(item, lookup_xref, overrides);
    // generate From impls (2x)
    let from_impl = quote_enum_from_impl(item, &name_ident, overrides);
    let into_impl = quote_enum_into_impl(item, &name_ident, overrides);
    // generate Display impl
    let display_impl = quote_enum_display_impl(item, &name_ident, overrides);
    // generate Default impl
    let default_impl = quote_enum_default_impl(&name_ident);
    quote!(
        #decl

        #from_impl

        #into_impl

        #display_impl

        #default_impl

    )
}

fn quote_enum_decl<'a, F>(e: &Enum, lookup_xref: F, overrides: &Overrides) -> TokenStream
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let name = format_name(override_name!(overrides, e.uid, e.name), e.uid);
    let name_ident = format_ident!("{}", name);
    let needs_postfix = override_postfix!(overrides, e.uid);
    let data_size = override_size!(overrides, e.uid, e.size);
    let embed_xref = override_embed_xref!(overrides, e.uid);
    let arms = quote_enum_decl_arms(&e.items, data_size, needs_postfix, embed_xref, lookup_xref);
    let uid_doc_comment = format!(" UID {}", e.uid);
    quote!(
        #[doc = #uid_doc_comment]
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[allow(non_camel_case_types)]
        pub enum #name_ident {
            #(#arms),*
        }
    )
}

fn quote_enum_decl_arms<'a, F>(
    items: &[EnumItem],
    data_size: usize,
    postfix_items: bool,
    embed_xref: bool,
    lookup_xref: F,
) -> Vec<TokenStream>
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let size_type = size_to_type(data_size);
    let size_ident = format_ident!("{}", size_type);

    let mut arms: Vec<TokenStream> = items
        .iter()
        .map(|item| match item {
            EnumItem::Basic(item) => {
                let item_name =
                    format_name_postfix(item.description.as_str(), item.value, postfix_items);
                let item_ident = format_ident!("{}", item_name);
                quote!(
                    #item_ident
                )
            }
            EnumItem::Range(item) => {
                let item_name = format_name(item.description.as_str(), *item.range.start());
                let item_ident = format_ident!("{}", item_name);
                quote!(
                    #item_ident(#size_ident)
                )
            }
            EnumItem::CrossRef(item) => {
                let item_name = format_name(item.description.as_str(), item.value);
                let item_ident = format_ident!("{}", item_name);
                match (embed_xref, lookup_xref(item.xref)) {
                    (false, _) | (true, None) => {
                        quote!(
                            #item_ident
                        )
                    }
                    (true, Some(xref_item)) => {
                        let xref_name = format_name(xref_item.name(), xref_item.size());
                        let xref_ident = format_ident!("{}", xref_name);
                        quote!(
                            #item_ident(#xref_ident)
                        )
                    }
                }
            }
        })
        .collect();

    arms.push(quote!(
        Unspecified(#size_ident)
    ));
    arms
}

fn quote_enum_from_impl(e: &Enum, name_ident: &Ident, overrides: &Overrides) -> TokenStream {
    let needs_postfix = override_postfix!(overrides, e.uid);
    let data_size = override_size!(overrides, e.uid, e.size);
    let embed_xref = override_embed_xref!(overrides, e.uid);
    let arms = quote_enum_from_arms(name_ident, &e.items, data_size, needs_postfix, embed_xref);
    let discriminant_type = size_to_type(data_size);
    let discriminant_ident = format_ident!("{}", discriminant_type);

    quote!(
        impl From<#discriminant_ident> for #name_ident {
            fn from(value: #discriminant_ident) -> Self {
                match value {
                    #(#arms),*
                }
            }
        }
    )
}

fn quote_enum_from_arms(
    name_ident: &Ident,
    items: &[EnumItem],
    data_size: usize,
    postfix_items: bool,
    embed_xref: bool,
) -> Vec<TokenStream> {
    #[allow(clippy::unnecessary_filter_map)]
    let mut arms: Vec<TokenStream> = items.iter().filter_map(|item| {
        match item {
            EnumItem::Basic(item) => {
                let item_name = format_name_postfix(item.description.as_str(), item.value, postfix_items);
                let item_ident = format_ident!("{}", item_name);
                let discriminant_literal = discriminant_literal(item.value, data_size);
                Some(quote!(
                        #discriminant_literal => #name_ident::#item_ident
                    ))
            }
            EnumItem::Range(item) => {
                let item_name = format_name(item.description.as_str(), *item.range.start());
                let item_ident = format_ident!("{}", item_name);
                let discriminant_literal_min = discriminant_literal(*item.range.start(), data_size);
                let discriminant_literal_max = discriminant_literal(*item.range.end(), data_size);
                Some(quote!(
                        #discriminant_literal_min..=#discriminant_literal_max => #name_ident::#item_ident(value)
                    ))
            }
            EnumItem::CrossRef(item) => {
                // Aside from this code a manual impl is required for crossref'ed bitfields, cannot be determined based on discriminant value alone (e.g., need domain enum for capabilities and appearance)
                // Here we set a default value for the contained CrossRef item
                let item_name = format_name(item.description.as_str(), item.value);
                let item_ident = format_ident!("{}", item_name);
                let discriminant_literal = discriminant_literal(item.value, data_size);
                if embed_xref {
                    Some(quote!(
                        #discriminant_literal => #name_ident::#item_ident(Default::default())
                    ))
                } else {
                    Some(quote! {
                        #discriminant_literal => #name_ident::#item_ident
                    })
                }
            }
        }
    }).collect();
    // For conversion from bytes to enum, add exhaustive arm resulting in the Unspecified variant of the enum
    let unspecified_ident = format_ident!("{}", "unspecified_value");
    arms.push(quote!(
        #unspecified_ident => #name_ident::Unspecified(#unspecified_ident)
    ));
    arms
}

fn quote_enum_into_impl(e: &Enum, name_ident: &Ident, overrides: &Overrides) -> TokenStream {
    let needs_postfix = override_postfix!(overrides, e.uid);
    let data_size = override_size!(overrides, e.uid, e.size);
    let embed_xref = override_embed_xref!(overrides, e.uid);
    let arms = quote_enum_into_arms(name_ident, &e.items, data_size, needs_postfix, embed_xref);
    let discriminant_type = size_to_type(data_size);
    let discriminant_ident = format_ident!("{}", discriminant_type);
    quote!(
        impl From<#name_ident> for #discriminant_ident {
            fn from(value: #name_ident) -> Self {
                match value {
                    #(#arms),*
                }
            }
        }
    )
}

#[allow(clippy::unnecessary_filter_map)]
fn quote_enum_into_arms(
    name_ident: &Ident,
    items: &[EnumItem],
    data_size: usize,
    postfix_items: bool,
    embed_xref: bool,
) -> Vec<TokenStream> {
    let mut arms: Vec<TokenStream> = items
        .iter()
        .filter_map(|item| match item {
            EnumItem::Basic(item) => {
                let item_name =
                    format_name_postfix(item.description.as_str(), item.value, postfix_items);
                let item_ident = format_ident!("{}", item_name);
                let discriminant_literal = discriminant_literal(item.value, data_size);
                Some(quote!(
                    #name_ident::#item_ident => #discriminant_literal
                ))
            }
            EnumItem::Range(item) => {
                let item_name = format_name(item.description.as_str(), *item.range.start());
                let item_ident = format_ident!("{}", item_name);
                let value_ident = format_ident!("{}", "specific_value");
                Some(quote!(
                    #name_ident::#item_ident(#value_ident) => #value_ident
                ))
            }
            EnumItem::CrossRef(item) => {
                let item_name = format_name(item.description.as_str(), item.value);
                let item_ident = format_ident!("{}", item_name);
                let value_ident = format_ident!("{}", "contained");
                if embed_xref {
                    Some(quote!(
                        #name_ident::#item_ident(#value_ident) => #value_ident.into()
                    ))
                } else {
                    let discriminant_literal = discriminant_literal(item.value, data_size);
                    Some(quote! {
                        #name_ident::#item_ident => #discriminant_literal
                    })
                }
            }
        })
        .collect();
    let unspecified_ident = format_ident!("{}", "unspecified_value");
    arms.push(quote!(
        #name_ident::Unspecified(#unspecified_ident) => #unspecified_ident
    ));
    arms
}

fn quote_enum_display_impl(e: &Enum, name_ident: &Ident, overrides: &Overrides) -> TokenStream {
    let needs_postfix = override_postfix!(overrides, e.uid);
    let embed_xref = override_embed_xref!(overrides, e.uid);
    let arms = quote_enum_display_arms(&e.items, name_ident, needs_postfix, embed_xref);
    quote!(
        impl Display for #name_ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    #(#arms),*
                }
            }
        }
    )
}

#[allow(clippy::unnecessary_filter_map)]
fn quote_enum_display_arms(
    items: &[EnumItem],
    name_ident: &Ident,
    postfix_items: bool,
    embed_xref: bool,
) -> Vec<TokenStream> {
    let mut arms: Vec<TokenStream> = items.iter().filter_map(|item| {
        match item {
            EnumItem::Basic(item) => {
                let item_description = escape_description(item.description.as_str());
                let item_name = format_name_postfix(&item_description, item.value, postfix_items);
                let item_ident = format_ident!("{}", item_name);

                Some(quote!(
                        #name_ident::#item_ident => write!(f, #item_description)
                    ))
            }
            EnumItem::Range(item) => {
                let item_description = escape_description(item.description.as_str());
                let item_name = format_name(&item_description, *item.range.start());
                let item_ident = format_ident!("{}", item_name);
                let value_ident = format_ident!("{}", "specific_value");

                Some(quote!(
                        #name_ident::#item_ident(#value_ident) => write!(f, "{} ({})", #item_description, #value_ident)
                    ))
            }
            EnumItem::CrossRef(item) => {
                let item_description = escape_description(item.description.as_str());
                let item_name = format_name(&item_description, item.value);
                let item_ident = format_ident!("{}", item_name);
                // let _value_ident = format_ident!("{}", "contained");

                if embed_xref {
                    Some(quote! {
                        #name_ident::#item_ident(_) => write!(f, #item_description)
                    })
                } else {
                    Some(quote! {
                        #name_ident::#item_ident => write!(f, #item_description)
                    })
                }

            }
        }
    }).collect();
    let unspecified_ident = format_ident!("{}", "unspecified_value");
    arms.push(quote!(
            #name_ident::Unspecified(#unspecified_ident) => write!(f, "Unspecified ({})", #unspecified_ident)
        ));
    arms
}

fn quote_enum_default_impl(name_ident: &Ident) -> TokenStream {
    quote!(
        impl Default for #name_ident {
            fn default() -> Self {
                #name_ident::from(0)
            }
        }
    )
}

fn generate_bitfield<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let decl = quote_bitfield_decl(item, &lookup_xref);
    let from = quote_bitfield_from_impl(item, &lookup_xref); // struct from u32
    let into = quote_bitfield_into_impl(item, &lookup_xref); // struct into u32
    let display = quote_bitfield_display_impl(item);

    quote!(
        #decl

        #from

        #into

        #display
    )
}

fn quote_bitfield_decl<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let formatted_name = format_name(item.name.as_str(), item.uid);
    let name_ident = format_ident!("{}", formatted_name);
    let fields = quote_bitfield_decl_fields(&item.fields, lookup_xref);
    let uid_doc_comment = format!("UID {}", item.uid);
    quote!(
        #[doc = #uid_doc_comment]
        #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct #name_ident {
            #(#fields),*
        }
    )
}

fn quote_bitfield_decl_fields<'a, F>(fields: &[BitfieldItem], lookup_xref: F) -> Vec<TokenStream>
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let generated_fields: Vec<TokenStream> = fields
        .iter()
        .map(|field| {
            let field_name = format_field_name(field.name.as_str());
            let field_ident = format_ident!("{}", field_name);
            let type_literal = if let Some(xref_uid) = field.xref {
                let xref = lookup_xref(xref_uid).unwrap_or_else(|| panic!("{}", xref_uid));
                format_ident!("{}", format_name(xref.name(), xref.uid()))
            } else {
                format_ident!("bool")
            };
            quote!(
                pub #field_ident : #type_literal
            )
        })
        .collect();
    generated_fields
}

fn quote_bitfield_from_impl<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let formatted_name = format_name(item.name.as_str(), item.uid);
    let name_ident = format_ident!("{}", formatted_name);
    let size_type = size_to_type(item.size);
    let size_ident = format_ident!("{}", size_type);
    let field_assignments = quote_bitfield_from_fields(&item.fields, lookup_xref);
    let field_names: Vec<TokenStream> = item
        .fields
        .iter()
        .map(|field| {
            let ident = format_ident!("{}", format_field_name(field.name.as_str()));
            quote!(#ident)
        })
        .collect();
    let value_ident = if field_assignments.is_empty() {
        format_ident!("_value")
    } else {
        format_ident!("value")
    };
    quote!(
        impl From<#size_ident> for #name_ident {
            fn from(#value_ident: #size_ident) -> Self {
                #(#field_assignments)*

                Self {
                    #(#field_names),*
                }
            }
        }
    )
}

fn quote_bitfield_from_fields<'a, F>(fields: &[BitfieldItem], lookup_xref: F) -> Vec<TokenStream>
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    fields.iter().map(|field| {
        let field_name = format_field_name(&field.name);
        let field_ident = format_ident!("{}", field_name);
        let shift_literal = Literal::usize_unsuffixed(field.bit_position);
        #[allow(clippy::cast_possible_truncation)]
        let bitmask_literal = Literal::usize_unsuffixed(2usize.pow(field.length as u32) - 1);
        if let Some(xref) = field.xref {
            let xref = lookup_xref(xref).unwrap();
            let xref_name = format_name(xref.name(), xref.uid());
            let xref_ident = format_ident!("{}", xref_name);
            let xref_data_size = size_to_type(xref.size());
            let xref_size_ident = format_ident!("{}", xref_data_size);
            quote!(
                    let #field_ident = #xref_ident::from(((value >> #shift_literal) & #bitmask_literal) as #xref_size_ident);
                )
        } else {
            quote!(
                    let #field_ident = ((value >> #shift_literal) & #bitmask_literal) != 0;
                )
        }
    }).collect()
}

fn quote_bitfield_into_impl<'a, F>(item: &Bitfield, lookup_xref: F) -> TokenStream
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let formatted_name = format_name(item.name.as_str(), item.uid);
    let name_ident = format_ident!("{}", formatted_name);
    let size_type = size_to_type(item.size);
    let size_ident = format_ident!("{}", size_type);
    let field_assignments = quote_bitfield_into_fields(&item.fields, item.size, lookup_xref);
    let field_names: Vec<TokenStream> = item
        .fields
        .iter()
        .map(|field| {
            let ident = format_ident!("{}", format_field_name(field.name.as_str()));
            quote!(#ident)
        })
        .collect();
    let base_size_literal = discriminant_literal(0, item.size);
    let value_ident = if field_assignments.is_empty() {
        format_ident!("_value")
    } else {
        format_ident!("value")
    };
    quote!(
        impl From<#name_ident> for #size_ident {
            fn from(#value_ident: #name_ident) -> Self {
                #(#field_assignments)*

                #base_size_literal #( | #field_names)*
            }
        }
    )
}

fn quote_bitfield_into_fields<'a, F>(
    fields: &[BitfieldItem],
    data_size: usize,
    lookup_xref: F,
) -> Vec<TokenStream>
where
    F: Fn(usize) -> Option<&'a GenerationItem>,
{
    let field_size_type = size_to_type(data_size);
    let field_size_ident = format_ident!("{}", field_size_type);

    fields.iter().map(|field| {
        let field_name = format_field_name(&field.name);
        let field_ident = format_ident!("{}", field_name);
        let shift_literal = Literal::usize_unsuffixed(field.bit_position);
        #[allow(clippy::cast_possible_truncation)]
        let bitmask_literal = Literal::usize_unsuffixed(2usize.pow(field.length as u32) - 1);
        if let Some(xref) = field.xref {
            let xref_size_type = size_to_type(lookup_xref(xref).unwrap().size());
            let xref_size_ident = format_ident!("{}", xref_size_type);
            quote!(
                    let #field_ident = (#field_size_ident::from(#xref_size_ident::from(value.#field_ident)) & #bitmask_literal) << #shift_literal;
                )
        } else {
            quote!(
                    let #field_ident = (#field_size_ident::from(value.#field_ident) & #bitmask_literal) << #shift_literal;
                )
        }
    }).collect()
}

fn quote_bitfield_display_impl(item: &Bitfield) -> TokenStream {
    let formatted_name = format_name(item.name.as_str(), item.uid);
    let name_ident = format_ident!("{}", formatted_name);

    quote!(
        impl Display for #name_ident {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, #formatted_name)
            }
        }
    )
}

fn size_to_type(data_size: usize) -> &'static str {
    #[allow(clippy::match_same_arms)]
    match data_size {
        64 => "u64",
        32 => "u32",
        16 => "u16",
        8 => "u8",
        _ => "u8",
    }
}

fn discriminant_literal(value: usize, data_size: usize) -> Literal {
    #[allow(clippy::match_same_arms)]
    #[allow(clippy::cast_possible_truncation)]
    match data_size {
        64 => Literal::u64_suffixed(value as u64),
        32 => Literal::u32_suffixed(value as u32),
        16 => Literal::u16_suffixed(value as u16),
        8 => Literal::u8_suffixed(value as u8),
        _ => Literal::u8_suffixed(value as u8),
    }
}

// FIXME: placeholder values for currently unsupported UIDs
/// Generates wrapper types for `u8`/`u16` enumerations or bitfields
fn generate_type_placeholder() -> TokenStream {
    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct Enumeration<T: Display>(T);

        impl<T: Display> From<T> for Enumeration<T> {
            fn from(value: T) -> Self {
                Self(value)
            }
        }

        impl<T: Default + Display> Default for Enumeration<T> {
            fn default() -> Self {
                Self(T::default())
            }
        }

        impl<T: Display> Display for Enumeration<T> {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    }
}
