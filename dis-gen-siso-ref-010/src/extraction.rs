use super::{
    BasicEnumItem, Bitfield, BitfieldItem, CrossRefEnumItem, Enum, EnumItem, GenerationItem,
    RangeEnumItem, BITFIELD_UIDS, SKIP_XREF_UIDS,
};
use dis_gen_utils::{extract_attr_as_string, extract_attr_as_usize};
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
use std::ops::RangeInclusive;

const ENUM_ELEMENT: QName = QName(b"enum");
const ELEMENT_ATTR_UID: QName = QName(b"uid");
const ELEMENT_ATTR_NAME: QName = QName(b"name");
const ELEMENT_ATTR_SIZE: QName = QName(b"size");
const ENUM_ROW_ELEMENT: QName = QName(b"enumrow");
const ENUM_ROW_RANGE_ELEMENT: QName = QName(b"enumrow_range");
const ENUM_ROW_ATTR_VALUE: QName = QName(b"value");
const ENUM_ROW_ATTR_VALUE_MIN: QName = QName(b"value_min");
const ENUM_ROW_ATTR_VALUE_MAX: QName = QName(b"value_max");
const ENUM_ROW_ATTR_DESC: QName = QName(b"description");
const ENUM_ROW_ATTR_XREF: QName = QName(b"xref");
const ENUM_ROW_ATTR_DEPR: QName = QName(b"deprecated");
const BITFIELD_ELEMENT: QName = QName(b"bitfield");
const BITFIELD_ROW_ELEMENT: QName = QName(b"bitfieldrow");
const BITFIELD_ROW_ATTR_NAME: QName = QName(b"name");
const BITFIELD_ROW_ATTR_BIT_POSITION: QName = QName(b"bit_position");
const BITFIELD_ROW_ATTR_LENGTH: QName = QName(b"length");
const BITFIELD_ROW_ATTR_XREF: QName = QName(b"xref");
const BITFIELD_ROW_ATTR_DESCRIPTION: QName = QName(b"description");
const CR_ELEMENT: QName = QName(b"cr");

#[allow(clippy::too_many_lines)]
pub fn extract(reader: &mut Reader<BufReader<File>>) -> Vec<GenerationItem> {
    let mut buf = Vec::new();
    let mut items = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref element)) => match element.name() {
                ENUM_ELEMENT => {
                    if let Some(item) = extract_enum(element, reader) {
                        items.push(GenerationItem::Enum(item));
                    }
                }
                BITFIELD_ELEMENT => {
                    if let Some(item) = extract_bitfield(element, reader) {
                        items.push(GenerationItem::Bitfield(item));
                    }
                }
                _ => (), // REVISIONS, REVISION, DICT, EBV, CET and COT elements, among others
            },
            Ok(Event::End(ref _element)) => {
                // REVISIONS, REVISION, DICT, EBV, etc
            }
            Ok(Event::Empty(ref _element)) => {
                // CR, CR_RANGE
            }
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    items
}

fn extract_enum(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> Option<Enum> {
    let uid = extract_attr_as_usize(element, ELEMENT_ATTR_UID, reader);

    let name = extract_attr_as_string(element, ELEMENT_ATTR_NAME);
    let size = extract_attr_as_usize(element, ELEMENT_ATTR_SIZE, reader);
    let items = extract_enum_rows(reader);

    if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
        Some(Enum {
            uid,
            name,
            size,
            items,
        })
    } else {
        panic!("Encountered an error extracting an '{ENUM_ELEMENT:?}'.");
    }
}

fn extract_enum_rows(reader: &mut Reader<BufReader<File>>) -> Vec<EnumItem> {
    let mut buf = Vec::new();
    let mut rows = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            #[allow(clippy::match_same_arms)]
            Ok(Event::Start(ref element)) => match element.name() {
                ENUM_ROW_ELEMENT => rows.push(extract_enum_row(element, reader)),
                ENUM_ROW_RANGE_ELEMENT => rows.push(extract_enum_range_row(element, reader)),
                _ => (), // HEADER element
            },
            Ok(Event::End(ref element)) => {
                if element.name() == ENUM_ELEMENT {
                    break;
                }
            }
            Ok(Event::Empty(ref element)) => match element.name() {
                ENUM_ROW_ELEMENT => rows.push(extract_enum_row(element, reader)),
                ENUM_ROW_RANGE_ELEMENT => rows.push(extract_enum_range_row(element, reader)),
                _ => (), // CR, COL elements
                         // CR_ELEMENT => (),
                         // element => panic!("Unexpected empty element '{element:?}' in {ENUM_ELEMENT:?}"),
            },
            Ok(Event::Eof) => panic!("Unexpected end of file at element {ENUM_ELEMENT:?}."), // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    rows
}

fn extract_enum_row(element: &BytesStart, reader: &Reader<BufReader<File>>) -> EnumItem {
    let value = extract_attr_as_usize(element, ENUM_ROW_ATTR_VALUE, reader);
    let description = extract_attr_as_string(element, ENUM_ROW_ATTR_DESC);
    // TODO simplify - generate that single xref correctly
    let xref = extract_attr_as_usize(element, ENUM_ROW_ATTR_XREF, reader)
        .filter(|&xref_value| !SKIP_XREF_UIDS.contains(&xref_value));
    let deprecated = deprecated_attribute_present(element);

    match (value, description, xref) {
        (Some(value), Some(description), Some(xref)) => EnumItem::CrossRef(CrossRefEnumItem {
            description,
            value,
            xref,
            deprecated,
        }),
        (Some(value), Some(description), None) => EnumItem::Basic(BasicEnumItem {
            description,
            value,
            deprecated,
        }),
        _ => {
            panic!("Encountered an error extracting an '{ENUM_ROW_ELEMENT:?}'.");
        }
    }
}

fn extract_enum_range_row(element: &BytesStart, reader: &Reader<BufReader<File>>) -> EnumItem {
    let value_min = extract_attr_as_usize(element, ENUM_ROW_ATTR_VALUE_MIN, reader);
    let value_max = extract_attr_as_usize(element, ENUM_ROW_ATTR_VALUE_MAX, reader);
    let description = extract_attr_as_string(element, ENUM_ROW_ATTR_DESC);
    let deprecated = deprecated_attribute_present(element);

    if let (Some(value_min), Some(value_max), Some(description)) =
        (value_min, value_max, description)
    {
        EnumItem::Range(RangeEnumItem {
            description,
            range: RangeInclusive::new(value_min, value_max),
            deprecated,
        })
    } else {
        panic!("Encountered an error extracting an '{ENUM_ROW_RANGE_ELEMENT:?}'.");
    }
}

fn extract_bitfield(
    element: &BytesStart,
    reader: &mut Reader<BufReader<File>>,
) -> Option<Bitfield> {
    let uid = extract_attr_as_usize(element, ELEMENT_ATTR_UID, reader);
    if let Some(uid) = uid {
        if !BITFIELD_UIDS.iter().any(|range| range.contains(&uid)) {
            // uid is not in the list, skip this bitfield, not to be generated
            return None;
        }
    }

    let name = extract_attr_as_string(element, ELEMENT_ATTR_NAME);
    let size = extract_attr_as_usize(element, ELEMENT_ATTR_SIZE, reader);

    let fields = extract_bitfield_rows(reader);

    if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
        Some(Bitfield {
            uid,
            name,
            size,
            fields,
        })
    } else {
        panic!("Encountered an error extracting an '{BITFIELD_ELEMENT:?}'.");
    }
}

fn extract_bitfield_rows(reader: &mut Reader<BufReader<File>>) -> Vec<BitfieldItem> {
    let mut buf = Vec::new();
    let mut rows = Vec::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref element)) => match element.name() {
                BITFIELD_ROW_ELEMENT => rows.push(extract_bitfield_row(element, reader)),
                _ => (),
            },
            Ok(Event::End(ref element)) => match element.name() {
                BITFIELD_ELEMENT => break,
                _ => (),
            },
            Ok(Event::Empty(ref element)) => match element.name() {
                BITFIELD_ROW_ELEMENT => rows.push(extract_bitfield_row(element, reader)),
                CR_ELEMENT => (),
                element => panic!("Unexpected empty element '{element:?}' in {BITFIELD_ELEMENT:?}"),
            },
            Ok(Event::Eof) => panic!("Unexpected end of file at element {BITFIELD_ELEMENT:?}."), // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    rows
}

fn extract_bitfield_row(element: &BytesStart, reader: &Reader<BufReader<File>>) -> BitfieldItem {
    const DEFAULT_BIT_SIZE: usize = 1;
    let name = extract_attr_as_string(element, BITFIELD_ROW_ATTR_NAME);
    let position = extract_attr_as_usize(element, BITFIELD_ROW_ATTR_BIT_POSITION, reader);
    let length = extract_attr_as_usize(element, BITFIELD_ROW_ATTR_LENGTH, reader)
        .unwrap_or(DEFAULT_BIT_SIZE);
    let xref = extract_attr_as_usize(element, BITFIELD_ROW_ATTR_XREF, reader);
    let description = extract_attr_as_string(element, BITFIELD_ROW_ATTR_DESCRIPTION)
        .unwrap_or_else(|| panic!("Expected attribute '{BITFIELD_ROW_ATTR_DESCRIPTION:?}' on element '{BITFIELD_ROW_ELEMENT:?}'"));

    if let (Some(name), Some(bit_position)) = (name, position) {
        BitfieldItem {
            name,
            bit_position,
            length,
            xref,
            description,
        }
    } else {
        panic!("Encountered an error extracting an '{BITFIELD_ROW_ELEMENT:?}'.");
    }
}

fn deprecated_attribute_present(element: &BytesStart) -> bool {
    matches!(
        element.try_get_attribute(ENUM_ROW_ATTR_DEPR),
        Ok(Some(_attr_depr))
    )
}

#[cfg(test)]
mod tests {
    use crate::extraction::extract;
    use crate::GenerationItem;
    use quick_xml::Reader;
    use std::fs::File;
    use std::io::{BufReader, Write};
    use tempfile::NamedTempFile;

    fn xml_reader_from_input(input: &str) -> Reader<BufReader<File>> {
        let mut file = NamedTempFile::new().expect("Could not create temp file for testing");
        file.write_all(input.as_bytes()).unwrap();
        let mut reader = Reader::from_file(file.path()).unwrap();
        reader.config_mut().trim_text(true);
        reader
    }

    #[test]
    fn test_extract_simple_enumeration() {
        let xml = r#"
            <enum uid="3" name="DIS-Protocol Version" size="8">
                <cr value="3093" />
                <enumrow value="0" description="Other" uuid="c1ed527a-5269-11df-8509-080069138b88" />
                <enumrow value="1" description="DIS PDU version 1.0 (May 92)" uuid="c1ee60a2-5269-11df-8b65-080069138b88" />
                <enumrow value="2" description="IEEE 1278-1993" uuid="c1ef7258-5269-11df-84cc-080069138b88" />
                <enumrow value="3" description="DIS Applications Version 2.0 - Third Draft (28 May 1993)" uuid="c1f08314-5269-11df-b3f9-080069138b88" footnote="IST-CR-93-15">
                  <cr value="4164" />
                </enumrow>
                <enumrow value="4" description="DIS Application Protocols Version 2.0 - Fourth Draft (Revised) (16 March 1994)" uuid="c1f19394-5269-11df-b5e1-080069138b88" footnote="IST-CR-94-50">
                  <cr value="4164" />
                </enumrow>
                <enumrow value="5" description="IEEE 1278.1-1995" uuid="c1f2a5ea-5269-11df-b53c-080069138b88" />
                <enumrow value="6" description="IEEE 1278.1A-1998" uuid="c1f3b732-5269-11df-b47c-080069138b88" />
                <enumrow value="7" description="IEEE 1278.1-2012" uuid="f1c9e470-51eb-4e9d-88ab-9ab706e68116">
                  <cr value="2711" />
                </enumrow>
                <enumrow value="8" description="IEEE 1278.1-202X" status="new" uuid="0aa6ba27-140a-4929-a336-063131ed338e">
                  <cr value="5049" />
                </enumrow>
              </enum>"#;

        let mut reader = xml_reader_from_input(xml);

        let extracted = extract(&mut reader);
        let item = extracted.first().expect("At least one extracted item");
        assert_eq!(item.uid(), 3);
        assert_eq!(item.name(), "ProtocolVersion");
        if let GenerationItem::Enum(e) = item {
            assert_eq!(e.size, 8);
            assert_eq!(e.items.len(), 9);
        }
    }
}
