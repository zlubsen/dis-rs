use std::{env, fs};
use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use std::str::FromStr;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quote::__private::{Ident, Literal, TokenStream};
use quote::{format_ident, quote};

const ENUM_ELEMENT : &[u8] = b"enum";
const ENUM_ATTR_UID : &[u8] = b"uid";
const ENUM_ATTR_NAME : &[u8] = b"name";
const ENUM_ATTR_SIZE : &[u8] = b"size";
const ENUM_ROW_ELEMENT : &[u8] = b"enumrow";
const ENUM_ROW_ATTR_VALUE : &[u8] = b"value";
const ENUM_ROW_ATTR_DESC : &[u8] = b"description";


#[derive(Debug, Clone)]
struct Enum {
    uid: usize,
    name: String,
    size: usize,
    items: Vec<EnumItem>,
}

#[derive(Debug, Clone)]
struct EnumItem {
    description: String,
    value: usize,
}

const UIDS : [usize; 3] = [
    //3, 4, 5, // protocol version, pdu type, pdu family
    6, // Force Id
    7, // Entity Kind
    //19, 20, 21
    // 29, // Country
    // 44, // DR algorithms
    45, // entity marking char set
    // 56, 57, 58, 59, // attached and articulated parts
    // 60, 61 // Munition Descriptor-Warhead, Fuse
    // 62, // Detonation result
];

fn main() {
    let mut reader = Reader::from_file(
        Path::new("./enumerations/SISO-REF-010.xml")
    ).unwrap();
    reader.trim_text(true);

    let mut buf = Vec::new();

    let mut enums = Vec::new();
    let mut current_enum = None;

    // find all enumerations that we want to generate
    loop {
        match reader.read_event(&mut buf) {
            Ok(Event::Start(ref element)) => {
                match element.name() {
                    ENUM_ELEMENT => {
                        current_enum = if let Ok(extracted) = extract_enum(element, &reader) {
                            Some(extracted)
                        } else { None }
                    },
                    ENUM_ROW_ELEMENT => {
                        current_enum = if let (Some(mut current), Ok(item)) = (current_enum, extract_enum_item(element, &reader)) {
                            current.items.push(item);
                            Some(current)
                        } else { None };
                    },
                    _ => (),
                }
            }
            Ok(Event::End(ref element)) => {
                match element.name() {
                    ENUM_ELEMENT => {
                        // finish up the current enum element
                        if let Some(current) = current_enum {
                            enums.push(current.clone());
                        }
                        current_enum = None
                    },
                    _ => (),
                }
            }
            Ok(Event::Empty(ref element)) => {
                match element.name() {
                    ENUM_ROW_ELEMENT => {
                        current_enum = if let (Some(mut current), Ok(item)) = (current_enum, extract_enum_item(element, &reader)) {
                            current.items.push(item);
                            Some(current)
                        } else { None };
                    },
                    _ => (),
                }
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    // generate all code for enums
    let mut generated_enums = vec![];
    for e in enums {
        let formatted_name = format_name(e.name.as_str());
        let name_ident = format_ident!("{}", formatted_name);
        // generate enum declarations
        let decl = quote_decl(&e);
        // generate From impls (2x)
        let from_impl = quote_from_impl(&e, &name_ident);
        let into_impl = quote_into_impl(&e, &name_ident);
        // generate Display impl
        let display_impl = quote_display_impl(&e, &name_ident);
        // generate Default impl
        let default_impl = quote_default_impl(&name_ident);
        generated_enums.push(quote!(
            #decl
            #from_impl
            #into_impl
            #display_impl
            #default_impl
        ));
    }

    // save to file
    let dest_path = env::current_dir().unwrap();
    let dest_path = dest_path.join("gen_src").join("enumerations.rs");

    fs::write(
        &dest_path,
        quote!(
            pub mod enumerations {
                use std::fmt::{Display, Formatter};

                #(#generated_enums)*
            }
        ).to_string()
    ).unwrap();

    // println!("cargo:rerun-if-changed={enums_file_path}");
}

fn extract_enum(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<Enum, ()> {
    // extract 'uid' 'name' 'size' attributes; create a struct for this with a vec for enumrow elements
    let uid = if let Ok(Some(attr_uid)) = element.try_get_attribute(ENUM_ATTR_UID) {
        Some(usize::from_str(reader.decoder().decode(&*attr_uid.value).unwrap()).unwrap())
    } else { None };
    if !UIDS.contains(&uid.unwrap()) {
        // skip this enum, not to be generated
        return Err(());
    }

    let name = if let Ok(Some(attr_name)) = element.try_get_attribute(ENUM_ATTR_NAME) {
        Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
    } else { None };

    let size = if let Ok(Some(attr_size)) = element.try_get_attribute(ENUM_ATTR_SIZE) {
        Some(usize::from_str(reader.decoder().decode(&*attr_size.value).unwrap()).unwrap())
    } else { None };

    if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
        Ok(Enum {
            uid,
            name,
            size,
            items: vec![]
        })
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
}

fn extract_enum_item(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<EnumItem, ()> {
    // extract 'value' and 'description' attributes into an EnumItem struct, add to parent's list of items;
    let value = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE) {
        Some(usize::from_str(reader.decoder().decode(&*attr_value.value).unwrap()).unwrap())
    } else { None };
    let description = if let Ok(Some(attr_desc)) = element.try_get_attribute(ENUM_ROW_ATTR_DESC) {
        Some(String::from_utf8(attr_desc.value.to_vec()).unwrap())
    } else { None };

    if let (Some(value), Some(description)) = (value, description) {
        Ok(EnumItem {
            description,
            value
        })
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
}

fn quote_decl(e: &Enum) -> TokenStream {
    let name = format_name(e.name.as_str());
    let name_ident = format_ident!("{}", name);
    let arms = quote_decl_arms(&e.items);
    quote!(
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum #name_ident {
            #(#arms),*
        }
    )
}

fn quote_decl_arms(items: &Vec<EnumItem>) -> Vec<TokenStream> {
    items.iter().map(|item| {
        let item_name = format_name(item.description.as_str());
        let item_ident = format_ident!("{}", item_name);
        let discriminant_literal = Literal::isize_unsuffixed(item.value as isize);
        quote!(
            #item_ident = #discriminant_literal
        )
    }).collect()
}

fn quote_from_impl(e: &Enum, name_ident: &Ident) -> TokenStream {
    let arms = quote_from_arms(name_ident, &e.items, e.size);
    let discriminant_type = size_to_type(e.size);
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

fn quote_from_arms(name_ident: &Ident, items: &Vec<EnumItem>, data_size: usize) -> Vec<TokenStream> {
    let mut arms: Vec<TokenStream> = items.iter().map(|item| {
        let item_name = format_name(item.description.as_str());
        let item_ident = format_ident!("{}", item_name);
        let discriminant_literal = discriminant_literal(item.value, data_size);
        quote!(
            #discriminant_literal => #name_ident::#item_ident
        )
    }).collect();
    // For conversion from bytes to enum, add exhaustive arm resulting in the default variant of the enum
    arms.push(quote!(
        _unspecified_value => #name_ident::default()
    ));
    arms
}

fn quote_into_impl(e: &Enum, name_ident: &Ident) -> TokenStream {
    let arms = quote_into_arms(name_ident, &e.items, e.size);
    let discriminant_type = size_to_type(e.size);
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

fn quote_into_arms(name_ident: &Ident, items: &Vec<EnumItem>, data_size: usize) -> Vec<TokenStream> {
    items.iter().map(|item| {
        let item_name = format_name(item.description.as_str());
        let item_ident = format_ident!("{}", item_name);
        let discriminant_literal = discriminant_literal(item.value, data_size);
        quote!(
            #name_ident::#item_ident => #discriminant_literal
        )
    }).collect()
}

fn quote_display_impl(e: &Enum, name_ident: &Ident) -> TokenStream {
    let arms = quote_display_arms(&e.items, name_ident);
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

fn quote_display_arms(items: &Vec<EnumItem>, name_ident: &Ident) -> Vec<TokenStream> {
    items.iter().map(|item| {
        let item_description = item.description.as_str();
        let item_name = format_name(item_description);
        let item_ident = format_ident!("{}", item_name);
        quote!(
            #name_ident::#item_ident => write!(f, "{}", #item_description)
        )
    }).collect()
}

fn quote_default_impl(name_ident: &Ident) -> TokenStream {
    quote!(
        impl Default for #name_ident {
            fn default() -> Self {
                #name_ident::from(0)
            }
        }
    )
}

fn format_name(value: &str) -> String {
    let intermediate = value
        .replace(" ", "")
        .replace("-", "")// TODO decide on remove or replace with '_'
        .replace("/","")
        .replace(".", "_");
    let open_parenthesis = intermediate.find("(");
    let close_parenthesis = intermediate.find(")");
    if let (Some(open_idx), Some(close_idx)) = (open_parenthesis, close_parenthesis) {
        intermediate
            .chars()
            .take(open_idx)
            .chain(intermediate.chars().skip(close_idx))
            .collect()
    } else {
        intermediate
    }
}

fn size_to_type(data_size: usize) -> &'static str {
    match data_size {
        64      => "u64",
        32      => "u32",
        16      => "u16",
        8 | _   => "u8",
    }
}

fn discriminant_literal(value: usize, data_size: usize) -> Literal {
    match data_size {
        64      => Literal::u64_suffixed(value as u64),
        32      => Literal::u32_suffixed(value as u32),
        16      => Literal::u16_suffixed(value as u16),
        8 | _   => Literal::u8_suffixed(value as u8),
    }
}
