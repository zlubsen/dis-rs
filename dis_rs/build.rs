use std::{env, fs};
use std::fs::File;
use std::io::{BufReader};
use std::ops::RangeInclusive;
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
const ENUM_ROW_RANGE_ELEMENT : &[u8] = b"enumrow_range";
const ENUM_ROW_ATTR_VALUE : &[u8] = b"value";
const ENUM_ROW_ATTR_VALUE_MIN : &[u8] = b"value_min";
const ENUM_ROW_ATTR_VALUE_MAX : &[u8] = b"value_max";
const ENUM_ROW_ATTR_DESC : &[u8] = b"description";

/// Array containing all the uids of enumerations that should be generated.
/// Each entry is a tuple containing the uid, an Optional string
/// literal to override the name of the resulting enum, and an Optional data size (in bits).
/// For example, the 'DISPDUType' enum (having uid 4) has an override
/// to 'PduType', which is nicer in code. The entry thus is (4, Some("PduType"))
/// Also, the 'Articulated Parts-Type Metric' enum has a defined size of 5, but needs to be aligned with a 32-bit field.
// FIXME replace with crate pfh::map
const ENUM_UIDS: [(usize, Option<&str>, Option<usize>); 15] = [
    // 3,                          // protocol version
    (4, Some("PduType"), None),           // pdu type
    (5, Some("ProtocolFamily"), None),    // pdu family
    (6, Some("ForceId"), None), // Force Id
    (7, None, None), // Entity Kind
    (8, None, None), // Domain
    // 9-28 // (Sub-)Categories
    (29, None, None), // Country
    (44, None, None), // Dead Reckoning Algorithm
    (45, None, None), // entity marking char set
    // 46-54 do not exist
    // 55, // Entity Capabilities (together with bitfields 450-462)
    (56, None, None), // Variable Parameter Record Type
    (57, None, None), // Attached Parts
    (58, None, Some(32)), // Articulated Parts-Type Metric
    (59, None, None), // Articulated Parts-Type Class
    (60, None, None), // Munition Descriptor-Warhead
    (61, None, None), // Munition Descriptor-Fuse
    (62, None, None), // Detonation result
    // 63-75, // All kinds of stuff for lesser priority PDUs
    // 76-79, // Emitter stuff
    // 80-81, // Designator stuff
    // 82-84, 87, 96-98 // IFF stuff
    // 100-106, // Subcategories
];

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Enum {
    uid: usize,
    name: String,
    size: usize,
    items: Vec<EnumItem>,
}

#[derive(Debug, Clone)]
enum EnumItem {
    Basic(BasicEnumItem),
    Range(RangeEnumItem),
    CrossRef(CrossRefEnumItem),
}

#[derive(Debug, Clone)]
struct BasicEnumItem {
    description: String,
    value: usize,
}

#[derive(Debug, Clone)]
struct RangeEnumItem {
    description: String,
    range: RangeInclusive<usize>
}

#[derive(Debug, Clone)]
struct CrossRefEnumItem {
    description: String,
    value: usize,
    xref: usize,
}

struct Bitfield {
    name: String,
    bit_position: usize,
    length: usize,
}

// TODO refactor a bit and make testable (include some unit tests in the regular code)
// test generated impls for enums/enumrows (from, into, display)
// test enumrow_range fields and unspecified values
// TODO read and generate bitfield enums
    // appearances: 31-43, plus enums 378-411
    // capabilities: 450-462 with enum 55

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
                    ENUM_ROW_RANGE_ELEMENT => {
                        current_enum = if let (Some(mut current), Ok(item)) = (current_enum, extract_enum_range_item(element, &reader)) {
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
                    ENUM_ROW_RANGE_ELEMENT => {
                        current_enum = if let (Some(mut current), Ok(item)) = (current_enum, extract_enum_range_item(element, &reader)) {
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
        let formatted_name = format_name(e.name.as_str(), e.uid);
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
}

fn extract_enum(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<Enum, ()> {
    let uid = if let Ok(Some(attr_uid)) = element.try_get_attribute(ENUM_ATTR_UID) {
        Some(usize::from_str(reader.decoder().decode(&*attr_uid.value).unwrap()).unwrap())
    } else { None };
    let should_generate = ENUM_UIDS.iter().find(|&&tuple| tuple.0 == uid.unwrap());
    if should_generate.is_none() {
        // skip this enum, not to be generated
        return Err(());
    }
    let name_override = should_generate.unwrap().1;
    let size_override = should_generate.unwrap().2;

    let name = if let Ok(Some(attr_name)) = element.try_get_attribute(ENUM_ATTR_NAME) {
        if name_override.is_some() {
            Some(name_override.unwrap().to_string())
        } else {
            Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
        }
    } else { None };

    let size = if let Ok(Some(attr_size)) = element.try_get_attribute(ENUM_ATTR_SIZE) {
        if size_override.is_some() {
            Some(size_override.unwrap())
        } else {
            Some(usize::from_str(reader.decoder().decode(&*attr_size.value).unwrap()).unwrap())
        }
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
    let value = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE) {
        Some(usize::from_str(reader.decoder().decode(&*attr_value.value).unwrap()).unwrap())
    } else { None };
    let description = if let Ok(Some(attr_desc)) = element.try_get_attribute(ENUM_ROW_ATTR_DESC) {
        Some(String::from_utf8(attr_desc.value.to_vec()).unwrap())
    } else { None };

    if let (Some(value), Some(description)) = (value, description) {
        Ok(EnumItem::Basic(BasicEnumItem {
            description,
            value,
        }))
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
}

fn extract_enum_range_item(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<EnumItem, ()> {
    let value_min = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE_MIN) {
        Some(usize::from_str(reader.decoder().decode(&*attr_value.value).unwrap()).unwrap())
    } else { None };
    let value_max = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE_MAX) {
        Some(usize::from_str(reader.decoder().decode(&*attr_value.value).unwrap()).unwrap())
    } else { None };
    let description = if let Ok(Some(attr_desc)) = element.try_get_attribute(ENUM_ROW_ATTR_DESC) {
        Some(String::from_utf8(attr_desc.value.to_vec()).unwrap())
    } else { None };

    if let (Some(value_min), Some(value_max), Some(description)) = (value_min, value_max, description) {
        Ok(EnumItem::Range(RangeEnumItem {
            description,
            range : RangeInclusive::new(value_min, value_max),
        }))
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
}

fn quote_decl(e: &Enum) -> TokenStream {
    let name = format_name(e.name.as_str(), e.uid);
    let name_ident = format_ident!("{}", name);
    let arms = quote_decl_arms(&e.items, e.size);
    quote!(
        #[derive(Copy, Clone, Debug, PartialEq)]
        #[allow(non_camel_case_types)]
        pub enum #name_ident {
            #(#arms),*
        }
    )
}

fn quote_decl_arms(items: &Vec<EnumItem>, data_size: usize) -> Vec<TokenStream> {
    let size_type = size_to_type(data_size);
    let size_ident = format_ident!("{}", size_type);

    let mut arms : Vec<TokenStream> = items.iter().map(|item| {
        match item {
            EnumItem::Basic(item) => {
                let item_name = format_name(item.description.as_str(), item.value);
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
            EnumItem::CrossRef(item) => { todo!() }
        }
    }).collect();

    arms.push(quote!(
        Unspecified(#size_ident)
    ));
    arms
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
        match item {
            EnumItem::Basic(item) => {
                let item_name = format_name(item.description.as_str(), item.value);
                let item_ident = format_ident!("{}", item_name);
                let discriminant_literal = discriminant_literal(item.value, data_size);
                quote!(
                    #discriminant_literal => #name_ident::#item_ident
                )
            }
            EnumItem::Range(item) => {
                let item_name = format_name(item.description.as_str(), *item.range.start());
                let item_ident = format_ident!("{}", item_name);
                let discriminant_literal_min = discriminant_literal(*item.range.start(), data_size);
                let discriminant_literal_max = discriminant_literal(*item.range.end(), data_size);
                quote!(
                    #discriminant_literal_min..=#discriminant_literal_max => #name_ident::#item_ident(value)
                )
            }
            EnumItem::CrossRef(item) => {
                todo!()
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
    let mut arms: Vec<TokenStream> = items.iter().map(|item| {
        match item {
            EnumItem::Basic(item) => {
                let item_name = format_name(item.description.as_str(), item.value);
                let item_ident = format_ident!("{}", item_name);
                let discriminant_literal = discriminant_literal(item.value, data_size);
                quote!(
                    #name_ident::#item_ident => #discriminant_literal
                )
            }
            EnumItem::Range(item) => {
                let item_name = format_name(item.description.as_str(), *item.range.start());
                let item_ident = format_ident!("{}", item_name);
                let value_ident = format_ident!("{}", "specific_value");
                quote!(
                    #name_ident::#item_ident(#value_ident) => #value_ident
                )
            }
            EnumItem::CrossRef(item) => {
                todo!()
            }
        }
    }).collect();
    let unspecified_ident = format_ident!("{}", "unspecified_value");
    arms.push(quote!(
        #name_ident::Unspecified(#unspecified_ident) => #unspecified_ident
    ));
    arms
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
    let mut arms: Vec<TokenStream> = items.iter().map(|item| {
        match item {
            EnumItem::Basic(item) => {
                let item_description = item.description.as_str();
                let item_name = format_name(item_description, item.value);
                let item_ident = format_ident!("{}", item_name);

                quote!(
                    #name_ident::#item_ident => write!(f, "{}", #item_description)
                )
            }
            EnumItem::Range(item) => {
                let item_description = item.description.as_str();
                let item_name = format_name(item_description, *item.range.start());
                let item_ident = format_ident!("{}", item_name);

                let value_ident = format_ident!("{}", "specific_value");
                quote!(
                    #name_ident::#item_ident(#value_ident) => write!(f, "{} ({})", #item_description, #value_ident)
                )
            }
            EnumItem::CrossRef(item) => {
                todo!()
            }
        }
    }).collect();
    let unspecified_ident = format_ident!("{}", "unspecified_value");
    arms.push(quote!(
        #name_ident::Unspecified(#unspecified_ident) => write!(f, "Unspecified ({})", #unspecified_ident)
    ));
    arms
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

fn format_name(value: &str, uid: usize) -> String {
    // Remove / replace the following characters
    let intermediate = value
        .replace(" ", "")
        .replace("-", "")
        .replace("/","")
        .replace(".", "_")
        .replace(",", "_")
        .replace("'", "")
        .replace("#", "");

    // Prefix values starting with a digit with '_'
    let intermediate = if intermediate.chars().next().unwrap().is_digit(10) {
        format!("_{}_{}", intermediate, uid)
    } else { intermediate };

    // // Remove text sections between parenthesis
    // let open_parenthesis = intermediate.find("(");
    // let close_parenthesis = intermediate.find(")");
    // let intermediate = if let (Some(open_idx), Some(close_idx)) = (open_parenthesis, close_parenthesis) {
    //     intermediate
    //         .chars()
    //         .take(open_idx)
    //         .chain(intermediate.chars().skip(close_idx+1))
    //         .collect()
    // } else {
    //     intermediate
    // };

    // When there are multiple parenthesis sections, replace them with '_' (such as Countries)
    intermediate
        .replace("(","_")
        .replace(")","_")
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
