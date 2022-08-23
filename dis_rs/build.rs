use std::{env, fs};
use std::fs::File;
use std::io::{BufReader};
use std::path::Path;
use std::str::FromStr;

use quick_xml::Reader;
use quick_xml::events::{BytesStart, Event};
use quote::__private::TokenStream;
use quote::quote;

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

const UIDS : [usize; 2] = [
    //3, 4, 5, // protocol version, pdu type, pdu family
    // 6, // Force Id
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
    let out_dir = env::var_os("OUT_DIR").unwrap();
    // let enums_input_file = env::var("SISO_REF_010_FILE").unwrap();
    // println!("{}", enums_input_file);
    // read the xml file from an env var
    // let enums_file_path = format!("./enumerations/{}", enums_input_file.as_str());
    // println!("{}", enums_file_path);
    // let xml = std::fs::read_to_string(Path::new(enums_file_path.as_str())).unwrap();
    // let mut reader = Reader::from_str(&xml);
    // let mut reader = Reader::from_file(Path::new(enums_file_path.as_str())).unwrap();
    let mut reader = Reader::from_file(Path::new("./enumerations/SISO-REF-010.xml")).unwrap();
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
    for e in enums {
        // generate enum declarations
        let decl = quote_decl(&e);
        // generate From impls (2x)
        let from_impl = quote_from_impl(&e);
        let into_impl = quote_into_impl(&e);
        // generate Display impl
        let display_impl = quote_display_impl(&e);
    }

    // save to file
    let dest_path = env::current_dir().unwrap();
    let dest_path = dest_path.join("gen_src").join("enumerations.rs");

    fs::write(
        &dest_path,
        "pub mod enumerations {

            pub fn message() -> &'static str {
                \"Hello, World!\"
            }

            pub enum Test {
                One,
                Two,
                Three,
            }
        }"
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

        // current_enum = if let Some(mut current) = current_enum {
        //     current.items.push(item);
        //     Some(current)
        // } else { None };
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
}

fn quote_decl_arms(enums: &Vec<EnumItem>) -> Vec<TokenStream> {
    enums.items.iter().map(|item| {
        let item_name = item.description.clone();
        let value = item.value;
        quote!(
                #item_name = #value,
            )
    }).collect()
}

fn quote_decl(e: &Enum) -> TokenStream {
    let name = e.name.as_str();
    let items = quote_decl_arms(&e.items);
    quote!(
        #[derive(Copy, Clone, Debug, PartialEq)]
        pub enum #name {
            #(#items),*
        }
    )
}
