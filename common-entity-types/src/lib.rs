use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use dis_gen_utils::extract_attr_as_usize;

pub struct CommonEntityTypes {
    root: Lookup,
}

impl CommonEntityTypes {
    pub fn new_from_reference(reference: &str) -> Self {
        let mut reader = reader_from_path(PathBuf::from(reference).as_path());
        let root = extract_siso_ref_010_cet(&mut reader);
        Self {
            root,
        }
    }
}

struct Lookup {
    descriptions: Vec<String>,
    index_root: Vec<Kind>,
}

struct Kind {
    id: u8,
    description: String,
    domains: Vec<Domain>,
}

struct Domain {
    id: u8,
    description: String,
    countries: Vec<Country>,
}

struct Country {
    id: u16,
    description: String,
    categories: Vec<Category>,
}

struct Category {
    id: u8,
    description: String,
    categories: Vec<SubCategory>,
}

struct SubCategory {
    id: u8,
    description: String,
    specifics: Vec<Specific>,
}

struct Specific {
    id: u8,
    description: String,
    extras: Vec<Extra>,
}

struct Extra {
    id: u8,
    description: String,
}

fn reader_from_path(path: &Path) -> Reader<BufReader<File>> {
    let mut reader = Reader::from_file(path).unwrap();
    reader.config_mut().trim_text(true);
    reader
}

const CET_UID: usize = 30;

const CET_ELEMENT: QName = QName(b"cet");
const ENTITY_ELEMENT: QName = QName(b"entity");
const UID_ATTR: QName = QName(b"uid");
const KIND_ATTR: QName = QName(b"kind");
const DOMAIN_ATTR: QName = QName(b"domain");
const COUNTRY_ATTR: QName = QName(b"country");
const CATEGORY_ELEMENT: QName = QName(b"category");
const SUBCATEGORY_ELEMENT: QName = QName(b"subcategory");
const SPECIFIC_ELEMENT: QName = QName(b"specific");
const EXTRA_ELEMENT: QName = QName(b"extra");
const DESCRIPTION_ATTR: QName = QName(b"description");

fn extract_siso_ref_010_cet(reader: &mut Reader<BufReader<File>>) -> Lookup {
    let mut index_root = vec![];
    let mut descriptions = vec![];

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref element)) => match element.name() {
                CET_ELEMENT => {
                    let uid = extract_attr_as_usize(element, UID_ATTR, reader).expect("Element `cet` is expected to have an attribute `uid`");
                    if uid == CET_UID {
                        index_root = extract_cet_element(element, reader, &mut descriptions);
                    }
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }

    let lookup = Lookup {
        descriptions,
        index_root,
    };

    lookup
}

fn extract_cet_element(element: &BytesStart, reader: &mut Reader<BufReader<File>>, descriptions: &mut Vec<String>) -> Vec<Kind> {
    let mut buf = Vec::new();
    let mut kinds = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref element)) => if element.name() == CET_ELEMENT {
                    kinds.push(extract_entity_element(element, reader));
            }
            Ok(Event::End(ref element)) => if element.name() == CET_ELEMENT {
                return kinds
            },
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
}

fn extract_entity_element(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> Kind {
    let kind = dis_gen_utils::extract_attr_as_usize(element, KIND_ATTR, reader).expect("Expected `kind` attribute on entity element");
    let domain = dis_gen_utils::extract_attr_as_usize(element, DOMAIN_ATTR, reader).expect("Expected `domain` attribute on entity element");
    let country = dis_gen_utils::extract_attr_as_usize(element, COUNTRY_ATTR, reader).expect("Expected `country` attribute on entity element");

    let category = extract_category_element(reader);
    let domains

    Kind {
        domains
        id: kind
    }
}

fn extract_category_element(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> Kind {

}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
