use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;

pub struct CommonEntityTypes {
    root: Root,
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

struct Root {
    kinds: Vec<Kind>,
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

fn extract_siso_ref_010_cet(reader: &mut Reader<BufReader<File>>) -> Root {
    let mut buf = Vec::new();
    let mut kinds = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref element)) => match element.name() {
                ENTITY_ELEMENT => {
                    kinds.push(extract_entity(element, reader));
                }
            }
        }
    }

    let root = Root {
        kinds
    };

    root
}

fn extract_entity_element(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> Kind {
    let kind = dis_gen_utils::extract_attr_as_usize(element, KIND_ATTR, reader).expect("Expected `kind` attribute on entity element");
    let domain = dis_gen_utils::extract_attr_as_usize(element, DOMAIN_ATTR, reader).expect("Expected `domain` attribute on entity element");
    let country = dis_gen_utils::extract_attr_as_usize(element, COUNTRY_ATTR, reader).expect("Expected `country` attribute on entity element");

    let categories = extract_category_element(reader);
    let domains

    Kind {
        domains
        id: kind
    }
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
