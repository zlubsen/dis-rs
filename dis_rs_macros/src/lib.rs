extern crate proc_macro;

use std::str::FromStr;
use proc_macro2::{Span, TokenStream};

use syn::{Data, parse_macro_input, DeriveInput, Type, TypeParen};
use quote::quote;

/// Derive macro for reading and writing DIS PDU fields from byte format into dis-rs data structures (enums) and vice versa.
/// The derive macro will generate the correct From<T> trail implementations for enum data structures,
/// provided that it is specified what representation (data type) is used in the wire specification (e.g., u8, u16, f32, ...).
///
/// Example:
/// '''
/// #[derive(PduField)]
/// #[repr(u8)]
/// pub enum ForceId {
///     Other = 0,
///     Friendly = 1,
///     Opposing = 2,
///     Neutral = 3,
/// }
/// '''
///
/// Will generate:
/// '''rust
/// impl From<u8> for ForceId {
///     fn from(value: u8) -> Self {
///         match value {
///             0 => ForceId::Other,
///             1 => ForceId::Friendly,
///             2 => ForceId::Opposing,
///             3 => ForceId::Neutral,
///             _ => ForceId::Other,
///         }
///     }
/// }
///
/// impl From<ForceId> for u8 {
///     fn from(value: ForceId) -> Self {
///         match value {
///             ForceId::Other => { 0u8 }
///             ForceId::Friendly => { 1u8 }
///             ForceId::Opposing => { 2u8 }
///             ForceId::Neutral => { 3u8 }
///         }
///     }
/// }
/// '''
#[proc_macro_derive(PduConversion)]
pub fn derive_pdu_field(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let tokens =
        derive_pdu_field_impl(&input).unwrap_or_else(|err| err.to_compile_error());
    tokens.into()
}

fn derive_pdu_field_impl(input: &DeriveInput) -> syn::Result<TokenStream> {
    let name = &input.ident;
    let attrs = &input.attrs;

    let mut discriminant_type: Type = syn::parse("u8".parse().unwrap()).unwrap();
    for attr in attrs {
        let path = &attr.path;
        let tokens = &attr.tokens;

        if path.leading_colon.is_some() {
            continue;
        }
        if path.segments.len() != 1 {
            continue;
        }
        let segment = path.segments.first().unwrap();
        if segment.ident != "repr" {
            continue;
        }
        let typ_paren = match syn::parse2::<Type>(tokens.clone()) {
            Ok(Type::Paren(TypeParen { elem, .. })) => *elem,
            _ => continue,
        };
        let inner_path = match &typ_paren {
            Type::Path(t) => t,
            _ => continue,
        };
        if let Some(seg) = inner_path.path.segments.last() {
            for t in &[
                "u8", "u16", "u32", "u64", "usize", "i8", "i16", "i32", "i64", "isize",
            ] {
                if seg.ident == t {
                    // discriminant_ident = seg.ident.clone();
                    discriminant_type = typ_paren;
                    break;
                }
            }
        }
    }

    let mut from_arms = Vec::new();
    let mut into_arms = Vec::new();

    let variants = match &input.data {
        Data::Enum(v) => &v.variants,
        _ => return Err(syn::Error::new(Span::call_site(), "This macro only supports enums.")),
    };
    for variant in variants {
        let ident = &variant.ident;
        let discriminant = if let Some((_, expr)) = &variant.discriminant {
            quote! { #expr }
        } else {
            return Err(syn::Error::new(Span::call_site(), "Discriminant value must be set."))
        };

        from_arms.push(quote! {#discriminant => #name::#ident});
        let into_value_formatted = TokenStream::from_str(format!("{}{}", quote! { #discriminant }, quote! { #discriminant_type }).as_str()).unwrap();
        into_arms.push(quote! {#name::#ident => { #into_value_formatted }});
    }

    // For conversion from bytes to enum, add exhaustive arm resulting in the default variant of the enum
    from_arms.push(quote! { _unspecified_value => #name::default() });

    Ok(quote! {
        impl From<#discriminant_type> for #name {
            fn from(value: #discriminant_type) -> Self {
                match value {
                    #(#from_arms),*
                }
            }
        }

        impl From<#name> for #discriminant_type {
            fn from(value: #name) -> Self {
                match value {
                    #(#into_arms),*
                }
            }
        }
    })
}