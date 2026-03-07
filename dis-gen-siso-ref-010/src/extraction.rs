use super::{
    BasicEnumItem, Bitfield, BitfieldItem, CrossRefEnumItem, Enum, EnumItem, GenerationItem,
    RangeEnumItem, BITFIELD_UIDS, ENUM_UIDS, SKIP_XREF_UIDS,
};
use quick_xml::events::{BytesStart, Event};
use quick_xml::name::QName;
use quick_xml::Reader;
use std::fs::File;
use std::io::BufReader;
use std::ops::RangeInclusive;
use std::str::FromStr;

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

#[allow(clippy::too_many_lines)]
pub fn extract(reader: &mut Reader<BufReader<File>>) -> Vec<GenerationItem> {
    let mut buf = Vec::new();
    let mut items = Vec::new();
    let mut current_item = None;

    // find all enumerations that we want to generate
    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(ref element)) => match element.name() {
                ENUM_ELEMENT => {
                    current_item = if let Ok(extracted) = extract_enum(element, reader) {
                        Some(GenerationItem::Enum(extracted))
                    } else {
                        None
                    }
                }
                ENUM_ROW_ELEMENT => {
                    current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) =
                        (current_item, extract_enum_item(element, reader))
                    {
                        current.items.push(item);
                        Some(GenerationItem::Enum(current))
                    } else {
                        None
                    };
                }
                ENUM_ROW_RANGE_ELEMENT => {
                    current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) =
                        (current_item, extract_enum_range_item(element, reader))
                    {
                        current.items.push(item);
                        Some(GenerationItem::Enum(current))
                    } else {
                        None
                    };
                }
                BITFIELD_ELEMENT => {
                    current_item = if let Ok(extracted) = extract_bitfield(element, reader) {
                        Some(GenerationItem::Bitfield(extracted))
                    } else {
                        None
                    }
                }
                BITFIELD_ROW_ELEMENT => {
                    current_item = if let (Some(GenerationItem::Bitfield(mut current)), Ok(item)) =
                        (current_item, extract_bitfield_item(element, reader))
                    {
                        current.fields.push(item);
                        Some(GenerationItem::Bitfield(current))
                    } else {
                        None
                    }
                }
                _ => (),
            },
            Ok(Event::End(ref element)) => {
                match element.name() {
                    ENUM_ELEMENT | BITFIELD_ELEMENT => {
                        // finish up the current enum element
                        if let Some(current) = current_item {
                            items.push(current.clone());
                        }
                        current_item = None;
                    }
                    _ => (),
                }
            }
            Ok(Event::Empty(ref element)) => match element.name() {
                ENUM_ROW_ELEMENT => {
                    current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) =
                        (current_item, extract_enum_item(element, reader))
                    {
                        current.items.push(item);
                        Some(GenerationItem::Enum(current))
                    } else {
                        None
                    };
                }
                ENUM_ROW_RANGE_ELEMENT => {
                    current_item = if let (Some(GenerationItem::Enum(mut current)), Ok(item)) =
                        (current_item, extract_enum_range_item(element, reader))
                    {
                        current.items.push(item);
                        Some(GenerationItem::Enum(current))
                    } else {
                        None
                    };
                }
                BITFIELD_ROW_ELEMENT => {
                    current_item = if let (Some(GenerationItem::Bitfield(mut current)), Ok(item)) =
                        (current_item, extract_bitfield_item(element, reader))
                    {
                        current.fields.push(item);
                        Some(GenerationItem::Bitfield(current))
                    } else {
                        None
                    }
                }
                _ => (),
            },
            Ok(Event::Eof) => break, // exits the loop when reaching end of file
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (), // There are several other `Event`s we do not consider here
        }
    }
    items
}

fn extract_enum(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Result<Enum, ()> {
    let uid = if let Ok(Some(attr_uid)) = element.try_get_attribute(ELEMENT_ATTR_UID) {
        Some(usize::from_str(&reader.decoder().decode(&attr_uid.value).unwrap()).unwrap())
    } else {
        None
    };
    let should_generate = ENUM_UIDS.iter().find(|&&tuple| tuple.0 == uid.unwrap());

    if let Some(should_generate) = should_generate {
        let name_override = should_generate.1;
        let size_override = should_generate.2;
        let postfix_items = should_generate.3;

        let name = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            if let Some(name) = name_override {
                Some(name.to_string())
            } else {
                Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
            }
        } else {
            None
        };

        let size = if let Ok(Some(attr_size)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            if let Some(size) = size_override {
                Some(size)
            } else {
                Some(usize::from_str(&reader.decoder().decode(&attr_size.value).unwrap()).unwrap())
            }
        } else {
            None
        };

        if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
            Ok(Enum {
                uid,
                name,
                size,
                items: vec![],
                postfix_items,
            })
        } else {
            // something is wrong with the attributes of the element, skip it.
            Err(())
        }
    } else {
        Err(())
    }
}

fn extract_enum_item(
    element: &BytesStart,
    reader: &Reader<BufReader<File>>,
) -> Result<EnumItem, ()> {
    let value = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE) {
        Some(usize::from_str(&reader.decoder().decode(&attr_value.value).unwrap()).unwrap())
    } else {
        None
    };
    let description = if let Ok(Some(attr_desc)) = element.try_get_attribute(ENUM_ROW_ATTR_DESC) {
        Some(String::from_utf8(attr_desc.value.to_vec()).unwrap())
    } else {
        None
    };
    let xref = if let Ok(Some(attr_xref)) = element.try_get_attribute(ENUM_ROW_ATTR_XREF) {
        let xref_value =
            usize::from_str(&reader.decoder().decode(&attr_xref.value).unwrap()).unwrap();
        if SKIP_XREF_UIDS.contains(&xref_value) {
            None
        } else {
            Some(xref_value)
        }
    } else {
        None
    };
    let deprecated = matches!(
        element.try_get_attribute(ENUM_ROW_ATTR_DEPR),
        Ok(Some(_attr_depr))
    );

    match (value, description, xref) {
        (Some(value), Some(description), Some(xref)) => Ok(EnumItem::CrossRef(CrossRefEnumItem {
            description,
            value,
            xref,
            deprecated,
        })),
        (Some(value), Some(description), None) => Ok(EnumItem::Basic(BasicEnumItem {
            description,
            value,
            deprecated,
        })),
        _ => {
            // something is wrong with the attributes of the element, skip it.
            Err(())
        }
    }
}

fn extract_enum_range_item(
    element: &BytesStart,
    reader: &Reader<BufReader<File>>,
) -> Result<EnumItem, ()> {
    let value_min = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE_MIN)
    {
        Some(usize::from_str(&reader.decoder().decode(&attr_value.value).unwrap()).unwrap())
    } else {
        None
    };
    let value_max = if let Ok(Some(attr_value)) = element.try_get_attribute(ENUM_ROW_ATTR_VALUE_MAX)
    {
        Some(usize::from_str(&reader.decoder().decode(&attr_value.value).unwrap()).unwrap())
    } else {
        None
    };
    let description = if let Ok(Some(attr_desc)) = element.try_get_attribute(ENUM_ROW_ATTR_DESC) {
        Some(String::from_utf8(attr_desc.value.to_vec()).unwrap())
    } else {
        None
    };
    let deprecated = matches!(
        element.try_get_attribute(ENUM_ROW_ATTR_DEPR),
        Ok(Some(_attr_depr))
    );

    if let (Some(value_min), Some(value_max), Some(description)) =
        (value_min, value_max, description)
    {
        Ok(EnumItem::Range(RangeEnumItem {
            description,
            range: RangeInclusive::new(value_min, value_max),
            deprecated,
        }))
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
}

fn extract_bitfield(
    element: &BytesStart,
    reader: &Reader<BufReader<File>>,
) -> Result<Bitfield, ()> {
    let uid = if let Ok(Some(attr_uid)) = element.try_get_attribute(ELEMENT_ATTR_UID) {
        Some(usize::from_str(&reader.decoder().decode(&attr_uid.value).unwrap()).unwrap())
    } else {
        None
    };
    if let Some(uid) = uid {
        if !BITFIELD_UIDS.iter().any(|range| range.contains(&uid)) {
            // uid is not in the list, skip this bitfield, not to be generated
            return Err(());
        }
    }

    let name = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
        Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
    } else {
        None
    };
    let size = if let Ok(Some(attr_size)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
        Some(usize::from_str(&reader.decoder().decode(&attr_size.value).unwrap()).unwrap())
    } else {
        None
    };

    if let (Some(uid), Some(name), Some(size)) = (uid, name, size) {
        Ok(Bitfield {
            uid,
            name,
            size,
            fields: vec![],
        })
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
}

fn extract_bitfield_item(
    element: &BytesStart,
    reader: &Reader<BufReader<File>>,
) -> Result<BitfieldItem, ()> {
    let name = if let Ok(Some(attr_name)) = element.try_get_attribute(BITFIELD_ROW_ATTR_NAME) {
        Some(String::from_utf8(attr_name.value.to_vec()).unwrap())
    } else {
        None
    };
    let position = if let Ok(Some(attr_position)) =
        element.try_get_attribute(BITFIELD_ROW_ATTR_BIT_POSITION)
    {
        Some(usize::from_str(&reader.decoder().decode(&attr_position.value).unwrap()).unwrap())
    } else {
        None
    };
    let length = if let Ok(Some(attr_length)) = element.try_get_attribute(BITFIELD_ROW_ATTR_LENGTH)
    {
        usize::from_str(&reader.decoder().decode(&attr_length.value).unwrap()).unwrap()
    } else {
        1
    };
    let xref = if let Ok(Some(attr_xref)) = element.try_get_attribute(BITFIELD_ROW_ATTR_XREF) {
        Some(usize::from_str(&reader.decoder().decode(&attr_xref.value).unwrap()).unwrap())
    } else {
        None
    };

    if let (Some(name), Some(bit_position)) = (name, position) {
        Ok(BitfieldItem {
            name,
            bit_position,
            length,
            xref,
        })
    } else {
        // something is wrong with the attributes of the element, skip it.
        Err(())
    }
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
