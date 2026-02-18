use std::collections::HashMap;

const XML_FILE_EXTENSION: &str = "xml";
const SISO_SCHEMA_DIR: &str = "./definitions/v8-schemas";

pub fn execute(uid_index: &HashMap<usize, String>) {
    if std::path::Path::new(SISO_SCHEMA_DIR).is_dir() {
        let mut file_paths = std::fs::read_dir(SISO_SCHEMA_DIR)
            .unwrap()
            .map(|a| a.unwrap())
            .filter(|a| a.path().is_file())
            .filter(|a| a.path().extension() == Some(XML_FILE_EXTENSION.as_ref()))
            .map(|a| a.path())
            .collect::<Vec<std::path::PathBuf>>();
        file_paths.sort();

        let generation_items = file_paths
            .iter()
            .flat_map(extraction::extract_from_file)
            .collect::<Vec<GenerationItem>>();

        todo!("Generate the code");
    }
}

#[derive(Debug, Clone)]
enum GenerationItem {
    Pdu(Pdu),
    FixedRecord(FixedRecord),
    BitRecord(BitRecord),
    AdaptiveRecord(AdaptiveRecord),
    ExtensionRecord(ExtensionRecord),
}

#[derive(Debug, Clone)]
struct NumericField {
    pub name: String,
    pub primitive_type: String,
    pub units: Option<String>,
}

#[derive(Debug, Clone)]
struct CountField {
    pub name: String,
    pub primitive_type: String,
}

#[derive(Debug, Clone)]
struct EnumField {
    pub name: String,
    pub field_type: String,
    pub enum_uid: Option<Vec<usize>>,
    pub hierarchy_dependency: Option<String>,
    pub is_discriminant: Option<bool>,
}

#[derive(Debug, Clone)]
struct FixedStringField {
    pub name: String,
    pub length: usize,
}

#[derive(Debug, Clone)]
struct IntBitField {
    pub name: String,
    pub bit_position: usize,
    pub size: Option<usize>,
    pub units: Option<String>,
}

#[derive(Debug, Clone)]
struct EnumBitField {
    pub name: String,
    pub bit_position: usize,
    pub size: Option<usize>,
    pub enum_uid: Option<Vec<usize>>,
    pub is_discriminant: Option<bool>,
}

#[derive(Debug, Clone)]
struct BoolBitField {
    pub name: String,
    pub bit_position: usize,
}

#[derive(Debug, Clone)]
struct FixedRecordField {
    pub name: String,
    pub length: usize,
    pub field_type: String,
}

#[derive(Debug, Clone)]
struct BitRecordField {
    pub name: String,
    pub size: usize,
    pub field_type: Option<String>,
    pub enum_uid: Option<Vec<usize>>,
}

#[derive(Debug, Clone)]
struct AdaptiveRecordField {
    pub name: String,
    pub length: usize,
    pub field_type: Option<String>,
    pub enum_uid: Option<Vec<usize>>,
    pub discriminant: String,
}

#[derive(Debug, Clone)]
struct VariableString {
    pub count_field: CountField,
    pub string_field: VariableStringField,
}

#[derive(Debug, Clone)]
struct VariableStringField {
    pub name: String,
    pub fixed_number_of_strings: Option<usize>,
}

#[derive(Debug, Clone)]
struct OpaqueDataField {
    pub name: String,
}

#[derive(Debug, Clone)]
enum FixedRecordFieldsEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
    AdaptiveRecord(AdaptiveRecordField),
}

#[derive(Debug, Clone)]
enum ArrayFieldEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
}

#[derive(Debug, Clone)]
struct Array {
    pub count_field: CountField,
    pub type_field: ArrayFieldEnum,
}

#[derive(Debug, Clone)]
struct OpaqueData {
    pub count_field: CountField,
    pub opaque_data_field: OpaqueDataField,
}

#[derive(Debug, Clone)]
struct FixedRecord {
    pub fields: Vec<FixedRecordFieldsEnum>,
    pub record_type: String,
    pub length: usize,
}

#[derive(Debug, Clone)]
enum BitRecordFieldEnum {
    Enum(EnumBitField),
    Int(IntBitField),
    Bool(BoolBitField),
}

#[derive(Debug, Clone)]
struct BitRecord {
    pub fields: Vec<BitRecordFieldEnum>,
    pub record_type: String,
    pub size: usize,
}

#[derive(Debug, Clone)]
struct AdaptiveRecord {
    pub fields: Vec<AdaptiveFormatEnum>,
    pub record_type: String,
    pub length: usize,
    pub discriminant_start_value: usize,
}

#[derive(Debug, Clone)]
enum AdaptiveFormatEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
}

#[derive(Debug, Clone)]
struct ExtensionRecordSet {
    pub count_field: CountField,
}

#[derive(Debug, Clone)]
enum ExtensionRecordFieldEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    VariableString(VariableString),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
    Array(Array),
    AdaptiveRecord(AdaptiveRecordField),
    Opaque(OpaqueData),
    PaddingTo16,
    PaddingTo32,
}

#[derive(Debug, Clone)]
struct PaddingTo16;

#[derive(Debug, Clone)]
struct PaddingTo32;

#[derive(Debug, Clone)]
struct PaddingTo64;

#[derive(Debug, Clone)]
struct ExtensionRecord {
    pub name_attr: String,
    pub record_type_attr: usize,
    pub base_length_attr: usize,
    pub is_variable_attr: bool,
    pub record_type_field: EnumField,
    pub record_length_field: NumericField,
    pub fields: Vec<ExtensionRecordFieldEnum>,
    pub padding_to_64_field: Option<PaddingTo64>,
}

#[derive(Debug, Clone)]
struct Pdu {
    pub name_attr: String,
    pub type_attr: usize,
    pub protocol_family_attr: usize,
    pub base_length_attr: usize,
    pub header_field: FixedRecordField,
    pub fields: Vec<PduFieldsEnum>,
    pub extension_record_set: ExtensionRecordSet,
}

#[derive(Debug, Clone)]
enum PduFieldsEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
    AdaptiveRecord(AdaptiveRecordField),
}

mod extraction {
    use crate::pdus::{
        AdaptiveFormatEnum, AdaptiveRecord, AdaptiveRecordField, Array, ArrayFieldEnum, BitRecord,
        BitRecordField, BitRecordFieldEnum, BoolBitField, CountField, EnumBitField, EnumField,
        ExtensionRecord, ExtensionRecordFieldEnum, ExtensionRecordSet, FixedRecord,
        FixedRecordField, FixedRecordFieldsEnum, FixedStringField, GenerationItem, IntBitField,
        NumericField, OpaqueData, OpaqueDataField, PaddingTo16, PaddingTo32, PaddingTo64, Pdu,
        PduFieldsEnum, VariableString, VariableStringField,
    };
    use quick_xml::events::attributes::Attribute;
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::name::QName;
    use quick_xml::Reader;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::str::FromStr;

    // Top-level elements
    const DIS_SYNTAX_ELEMENT: QName = QName(b"DISsyntax");
    const COPYRIGHT_ELEMENT: QName = QName(b"copyright");
    const PDU_ELEMENT: QName = QName(b"PDU");
    const FIXED_RECORD_ELEMENT: QName = QName(b"FixedRecord");
    const BIT_RECORD_ELEMENT: QName = QName(b"BitRecord");
    const ADAPTIVE_RECORD_ELEMENT: QName = QName(b"AdaptiveRecord");
    const EXTENSION_RECORD_ELEMENT: QName = QName(b"ExtensionRecord");
    // Record Field elements
    const FIXED_RECORD_FIELD_ELEMENT: QName = QName(b"FixedRecordField");
    const NUMERIC_FIELD_ELEMENT: QName = QName(b"NumericField");
    const ENUM_FIELD_ELEMENT: QName = QName(b"EnumField");
    const FIXED_STRING_FIELD_ELEMENT: QName = QName(b"FixedStringField");
    const VARIABLE_STRING_ELEMENT: QName = QName(b"VariableString");
    const VARIABLE_STRING_FIELD_ELEMENT: QName = QName(b"VariableStringField");
    const BIT_RECORD_FIELD_ELEMENT: QName = QName(b"BitRecordField");
    const ADAPTIVE_RECORD_FIELD_ELEMENT: QName = QName(b"AdaptiveRecordField");
    const ADAPTIVE_FORMAT_ELEMENT: QName = QName(b"AdaptiveFormat");
    const EXTENSION_RECORD_SET_ELEMENT: QName = QName(b"ExtensionRecordSet");
    const EXTENSION_RECORD_FIELDS_ELEMENT: QName = QName(b"ExtensionRecordFields");
    const ARRAY_ELEMENT: QName = QName(b"Array");
    const COUNT_FIELD_ELEMENT: QName = QName(b"CountField");
    const OPAQUE_DATA_ELEMENT: QName = QName(b"OpaqueData");
    const OPAQUE_DATA_FIELD_ELEMENT: QName = QName(b"OpaqueDataField");
    const PADDING_TO_16_ELEMENT: QName = QName(b"PaddingTo16");
    const PADDING_TO_32_ELEMENT: QName = QName(b"PaddingTo32");
    const PADDING_TO_64_ELEMENT: QName = QName(b"PaddingTo64");
    // Bit Records
    const ENUM_BIT_FIELD_ELEMENT: QName = QName(b"EnumBitField");
    const INT_BIT_FIELD_ELEMENT: QName = QName(b"IntBitField");
    const BOOL_BIT_FIELD_ELEMENT: QName = QName(b"BooleanBitField");

    const ELEMENT_ATTR_NAME: QName = QName(b"name");
    const ELEMENT_ATTR_TYPE: QName = QName(b"type");
    const ELEMENT_ATTR_UNITS: QName = QName(b"units");
    const ELEMENT_ATTR_LENGTH: QName = QName(b"length");
    const ELEMENT_ATTR_SIZE: QName = QName(b"size");
    const ELEMENT_ATTR_ENUM_UID: QName = QName(b"enumTableUID");
    const ELEMENT_ATTR_HIERARCHY_DEPENDENCY: QName = QName(b"hierarchyDependency");
    const ELEMENT_ATTR_IS_DISCRIMINANT: QName = QName(b"isDiscriminant");
    const ELEMENT_ATTR_DISCRIMINANT: QName = QName(b"discriminant");
    const ELEMENT_ATTR_DISCRIMINANT_START_VALUE: QName = QName(b"discriminantStartValue");
    const ELEMENT_ATTR_BIT_POSITION: QName = QName(b"bitPosition");
    const ELEMENT_ATTR_FIXED_NO_OF_STRINGS: QName = QName(b"fixedNumberOfStrings");
    const ELEMENT_ATTR_RECORD_TYPE_ENUM: QName = QName(b"recordTypeEnum");
    const ELEMENT_ATTR_BASE_LENGTH: QName = QName(b"baseLength");
    const ELEMENT_ATTR_IS_VARIABLE: QName = QName(b"isVariable");
    const ELEMENT_ATTR_PDU_TYPE: QName = QName(b"PDUType");
    const ELEMENT_ATTR_PROTOCOL_FAMILY: QName = QName(b"protocolFamily");

    pub(crate) fn extract_from_file(path: &PathBuf) -> Vec<GenerationItem> {
        let mut reader = Reader::from_file(path).unwrap();
        reader.config_mut().trim_text(true);

        extract_from_root(&mut reader)
    }

    fn extract_from_root(reader: &mut Reader<BufReader<File>>) -> Vec<GenerationItem> {
        let mut buf = Vec::new();
        let mut items: Vec<GenerationItem> = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref element)) => match element.name() {
                    DIS_SYNTAX_ELEMENT => items.append(&mut extract_protocol_items(reader)),
                    element => {
                        panic!("Encountered unexpected start element {element:?} at top-level.")
                    }
                },
                Ok(Event::End(ref element)) => {
                    let element = element.name();
                    panic!("Encountered unexpected end element {element:?} at top-level.")
                }
                Ok(Event::Empty(ref element)) => {
                    let name = element.name();
                    panic!("Unexpected closed element '{name:?}' encountered at top-level")
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        items
    }

    fn extract_protocol_items(reader: &mut Reader<BufReader<File>>) -> Vec<GenerationItem> {
        let mut buf = Vec::new();
        let mut items = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref element)) => match element.name() {
                    PDU_ELEMENT => {
                        items.push(GenerationItem::Pdu(extract_pdu(element, reader)));
                    }
                    FIXED_RECORD_ELEMENT => {
                        items.push(GenerationItem::FixedRecord(extract_fixed_record(
                            element, reader,
                        )));
                    }
                    BIT_RECORD_ELEMENT => {
                        items.push(GenerationItem::BitRecord(extract_bit_record(
                            element, reader,
                        )));
                    }
                    ADAPTIVE_RECORD_ELEMENT => {
                        items.push(GenerationItem::AdaptiveRecord(extract_adaptive_record(
                            element, reader,
                        )));
                    }
                    EXTENSION_RECORD_ELEMENT => {
                        items.push(GenerationItem::ExtensionRecord(extract_extension_record(
                            element, reader,
                        )));
                    }
                    COPYRIGHT_ELEMENT => (), // skip
                    element => {
                        panic!("Encountered unexpected start element {element:?} at top-level.")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    // Pass over the main structure elements that are not interesting
                    COPYRIGHT_ELEMENT => (), // skip
                    DIS_SYNTAX_ELEMENT => break,
                    element => {
                        panic!("Encountered unexpected end element {element:?} at top-level.")
                    }
                },
                Ok(Event::Empty(ref element)) => {
                    let name = element.name();
                    panic!("Unexpected closed element '{name:?}' encountered at top-level")
                }
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {DIS_SYNTAX_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        items
    }

    fn extract_pdu(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> Pdu {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_pdu_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_PDU_TYPE)
        {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };
        let attr_protocol_family =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_PROTOCOL_FAMILY) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };
        let attr_base_length =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_BASE_LENGTH) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };

        let mut buf = Vec::new();
        let mut header_field = None;
        let mut fields = vec![];
        let mut extension_record_set_field = None;

        loop {
            let evt = reader.read_event_into(&mut buf);
            match evt {
                Ok(Event::Empty(ref element)) => match element.name() {
                    NUMERIC_FIELD_ELEMENT => {
                        fields.push(PduFieldsEnum::Numeric(extract_numeric_field(element)));
                    }
                    ENUM_FIELD_ELEMENT => {
                        fields.push(PduFieldsEnum::Enum(extract_enum_field(element, reader)));
                    }
                    FIXED_STRING_FIELD_ELEMENT => {
                        fields.push(PduFieldsEnum::FixedString(extract_fixed_string_field(
                            element, reader,
                        )));
                    }
                    FIXED_RECORD_FIELD_ELEMENT => {
                        if header_field.is_none() {
                            header_field = Some(extract_fixed_record_field(element, reader));
                        } else {
                            fields.push(PduFieldsEnum::FixedRecord(extract_fixed_record_field(
                                element, reader,
                            )));
                        }
                    }
                    BIT_RECORD_FIELD_ELEMENT => {
                        fields.push(PduFieldsEnum::BitRecord(extract_bit_record_field(
                            element, reader,
                        )));
                    }
                    ADAPTIVE_RECORD_FIELD_ELEMENT => {
                        fields.push(PduFieldsEnum::AdaptiveRecord(
                            extract_adaptive_record_field(element, reader),
                        ));
                    }
                    element => {
                        panic!("Unexpected element '{element:?}' in {PDU_ELEMENT:?}")
                    }
                },
                Ok(Event::Start(ref element)) => match element.name() {
                    EXTENSION_RECORD_SET_ELEMENT => {
                        extension_record_set_field = Some(extract_extension_record_set(reader));
                    }
                    element => {
                        panic!("Unexpected element '{element:?}' in {PDU_ELEMENT:?}")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    PDU_ELEMENT => break, // PDU finished
                    element => {
                        panic!("Encountered unexpected end element {element:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {PDU_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }

        Pdu {
            name_attr: attr_name.expect("Expected PDU attribute 'name' to be present."),
            type_attr: attr_pdu_type.expect("Expected PDU attribute 'PDUType' to be present."),
            protocol_family_attr: attr_protocol_family
                .expect("Expected PDU attribute 'protocolFamily' to be present."),
            base_length_attr: attr_base_length
                .expect("Expected PDU attribute 'baseLength' to be present."),
            header_field: header_field.expect("Expected PDU header record to be present."),
            fields,
            extension_record_set: extension_record_set_field
                .expect("Expected PDU extension record set to be present"),
        }
    }

    fn extract_fixed_record(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> FixedRecord {
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };
        let fields = extract_fixed_record_fields(reader);

        FixedRecord {
            record_type: attr_type.expect("Expected record attribute 'type' to be present."),
            length: attr_length.expect("Expected record attribute 'length' to be present."),
            fields,
        }
    }

    fn extract_fixed_record_fields(
        reader: &mut Reader<BufReader<File>>,
    ) -> Vec<FixedRecordFieldsEnum> {
        let mut buf = Vec::new();
        let mut fields = vec![];
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    NUMERIC_FIELD_ELEMENT => {
                        fields.push(FixedRecordFieldsEnum::Numeric(extract_numeric_field(
                            element,
                        )));
                    }
                    ENUM_FIELD_ELEMENT => {
                        fields.push(FixedRecordFieldsEnum::Enum(extract_enum_field(
                            element, reader,
                        )));
                    }
                    FIXED_STRING_FIELD_ELEMENT => {
                        fields.push(FixedRecordFieldsEnum::FixedString(
                            extract_fixed_string_field(element, reader),
                        ));
                    }
                    FIXED_RECORD_FIELD_ELEMENT => {
                        fields.push(FixedRecordFieldsEnum::FixedRecord(
                            extract_fixed_record_field(element, reader),
                        ));
                    }
                    BIT_RECORD_FIELD_ELEMENT => {
                        fields.push(FixedRecordFieldsEnum::BitRecord(extract_bit_record_field(
                            element, reader,
                        )));
                    }
                    ADAPTIVE_RECORD_FIELD_ELEMENT => {
                        fields.push(FixedRecordFieldsEnum::AdaptiveRecord(
                            extract_adaptive_record_field(element, reader),
                        ));
                    }
                    element => {
                        panic!("Unexpected element '{element:?}' in {FIXED_RECORD_ELEMENT:?}")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    FIXED_RECORD_ELEMENT => break, // record finished
                    element => {
                        panic!("Encountered unexpected end element {element:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {FIXED_RECORD_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        fields
    }

    fn extract_bit_record(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> BitRecord {
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_size = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };
        let fields = extract_bit_record_fields(reader);

        BitRecord {
            record_type: attr_type.expect("Expected BitRecord attribute 'type' to be present."),
            size: attr_size.expect("Expected BitRecord attribute 'size' to be present."),
            fields,
        }
    }

    fn extract_bit_record_fields(reader: &mut Reader<BufReader<File>>) -> Vec<BitRecordFieldEnum> {
        let mut buf = Vec::new();
        let mut fields = vec![];
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    ENUM_BIT_FIELD_ELEMENT => {
                        fields.push(BitRecordFieldEnum::Enum(extract_enum_bit_field(
                            element, reader,
                        )));
                    }
                    INT_BIT_FIELD_ELEMENT => {
                        fields.push(BitRecordFieldEnum::Int(extract_int_bit_field(
                            element, reader,
                        )));
                    }
                    BOOL_BIT_FIELD_ELEMENT => {
                        fields.push(BitRecordFieldEnum::Bool(extract_bool_bit_field(
                            element, reader,
                        )));
                    }
                    element => {
                        panic!("Unexpected element '{element:?}' in {BIT_RECORD_ELEMENT:?}")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    BIT_RECORD_ELEMENT => break, // record finished
                    element => {
                        panic!("Encountered unexpected end element {element:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {BIT_RECORD_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        fields
    }

    fn extract_enum_bit_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> EnumBitField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_bit_position =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_BIT_POSITION) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };
        let attr_size = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };
        let attr_enum_uid = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_ENUM_UID)
        {
            expand_uid_string(&extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_is_discriminant =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_IS_DISCRIMINANT) {
                Some(extract_attr_to_bool(reader, &attr))
            } else {
                None
            };

        EnumBitField {
            name: attr_name.expect("Expected EnumBitField attribute 'name' to be present."),
            bit_position: attr_bit_position
                .expect("Expected EnumBitField attribute 'bitPosition' to be present."),
            size: attr_size,
            enum_uid: attr_enum_uid,
            is_discriminant: attr_is_discriminant,
        }
    }

    fn extract_int_bit_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> IntBitField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_bit_position =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_BIT_POSITION) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };
        let attr_size = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };
        let attr_units = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_UNITS) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };

        IntBitField {
            name: attr_name.expect("Expected IntBitField attribute 'name' to be present."),
            bit_position: attr_bit_position
                .expect("Expected IntBitField attribute 'bitPosition' to be present."),
            size: attr_size,
            units: attr_units,
        }
    }

    fn extract_bool_bit_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> BoolBitField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_bit_position =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_BIT_POSITION) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };

        BoolBitField {
            name: attr_name.expect("Expected BooleanBitField attribute 'name' to be present."),
            bit_position: attr_bit_position
                .expect("Expected BooleanBitField attribute 'bitPosition' to be present."),
        }
    }

    fn extract_adaptive_record(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> AdaptiveRecord {
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };

        let (fields, discriminant_start_value) = extract_adaptive_record_formats(reader);

        AdaptiveRecord {
            record_type: attr_type
                .expect("Expected AdaptiveRecord attribute 'type' to be present."),
            length: attr_length.expect("Expected AdaptiveRecord attribute 'length' to be present."),
            fields,
            discriminant_start_value: discriminant_start_value.expect(
                "Expected AdaptiveFormat attribute 'discriminantStartValue' to be present.",
            ),
        }
    }

    fn extract_adaptive_record_formats(
        reader: &mut Reader<BufReader<File>>,
    ) -> (Vec<AdaptiveFormatEnum>, Option<usize>) {
        let mut buf = Vec::new();
        let mut fields = vec![];
        let mut attr_discriminant_start_value = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    NUMERIC_FIELD_ELEMENT => {
                        fields.push(AdaptiveFormatEnum::Numeric(extract_numeric_field(element)));
                    }
                    ENUM_FIELD_ELEMENT => {
                        fields.push(AdaptiveFormatEnum::Enum(extract_enum_field(
                            element, reader,
                        )));
                    }
                    FIXED_STRING_FIELD_ELEMENT => {
                        fields.push(AdaptiveFormatEnum::FixedString(extract_fixed_string_field(
                            element, reader,
                        )));
                    }
                    FIXED_RECORD_FIELD_ELEMENT => {
                        fields.push(AdaptiveFormatEnum::FixedRecord(extract_fixed_record_field(
                            element, reader,
                        )));
                    }
                    BIT_RECORD_FIELD_ELEMENT => fields.push(AdaptiveFormatEnum::BitRecord(
                        extract_bit_record_field(element, reader),
                    )),
                    element => {
                        panic!("Unexpected element '{element:?}' in {ADAPTIVE_FORMAT_ELEMENT:?}")
                    }
                },
                Ok(Event::Start(ref element)) => match element.name() {
                    ADAPTIVE_FORMAT_ELEMENT => {
                        let attr = if let Ok(Some(attr)) =
                            element.try_get_attribute(ELEMENT_ATTR_DISCRIMINANT_START_VALUE)
                        {
                            Some(extract_attr_to_usize(reader, &attr))
                        } else {
                            None
                        };
                        attr_discriminant_start_value = Some(attr.expect("Expected element AdaptiveFormat attribute 'discriminantStartValue' to be present."));
                    } // pass, enter the AdaptiveFormat element
                    element => {
                        panic!("Encountered unexpected start element {element:?}.")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    ADAPTIVE_FORMAT_ELEMENT => (), // pass, exit the AdaptiveFormat element
                    ADAPTIVE_RECORD_ELEMENT => break, // record finished
                    element => {
                        panic!("Encountered unexpected end element {element:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {ADAPTIVE_RECORD_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        (fields, attr_discriminant_start_value)
    }

    fn extract_extension_record_set(reader: &mut Reader<BufReader<File>>) -> ExtensionRecordSet {
        let mut buf = Vec::new();
        let mut count_field = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => {
                    match element.name() {
                        COUNT_FIELD_ELEMENT => {
                            count_field = Some(extract_count_field(element));
                        }
                        EXTENSION_RECORD_FIELDS_ELEMENT => (),
                        element => {
                            panic!("Unexpected element '{element:?}' in {EXTENSION_RECORD_SET_ELEMENT:?}")
                        }
                    }
                }
                Ok(Event::End(ref element)) => match element.name() {
                    EXTENSION_RECORD_SET_ELEMENT => break, // record finished
                    element => {
                        panic!("Encountered unexpected end element {element:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {EXTENSION_RECORD_SET_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }

        ExtensionRecordSet {
            count_field: count_field
                .expect("Expected CountField element to be present in ExtensionRecordSet"),
        }
    }

    fn extract_extension_record(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> ExtensionRecord {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_record_type =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_RECORD_TYPE_ENUM) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };
        let attr_base_length =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_BASE_LENGTH) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };
        let attr_is_variable =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_IS_VARIABLE) {
                Some(extract_attr_to_bool(reader, &attr))
            } else {
                None
            };

        let mut buf = Vec::new();
        let record_type_field =
            if let Ok(Event::Empty(ref element)) = reader.read_event_into(&mut buf) {
                Some(extract_enum_field(element, reader))
            } else {
                None
            };
        let record_length_field =
            if let Ok(Event::Empty(ref element)) = reader.read_event_into(&mut buf) {
                Some(extract_numeric_field(element))
            } else {
                None
            };
        let (fields, padding_to_64_field) = extract_extension_record_fields(reader);

        ExtensionRecord {
            name_attr: attr_name.expect("Expected ExtensionRecord attribute 'name' to be present."),
            record_type_attr: attr_record_type
                .expect("Expected ExtensionRecord attribute 'recordTypeEnum' to be present."),
            base_length_attr: attr_base_length
                .expect("Expected ExtensionRecord attribute 'baseLength' to be present."),
            is_variable_attr: attr_is_variable
                .expect("Expected ExtensionRecord attribute 'isVariable' to be present."),
            record_type_field: record_type_field
                .expect("Expected child element EnumField 'Record Type' for ExtensionRecord."),
            record_length_field: record_length_field
                .expect("Expected child element NumericField 'Record Length' for ExtensionRecord."),
            fields,
            padding_to_64_field,
        }
    }

    fn extract_extension_record_fields(
        reader: &mut Reader<BufReader<File>>,
    ) -> (Vec<ExtensionRecordFieldEnum>, Option<PaddingTo64>) {
        let mut buf = Vec::new();
        let mut fields = vec![];
        let mut padding_to_64_field = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    NUMERIC_FIELD_ELEMENT => {
                        fields.push(ExtensionRecordFieldEnum::Numeric(extract_numeric_field(
                            element,
                        )));
                    }
                    ENUM_FIELD_ELEMENT => {
                        fields.push(ExtensionRecordFieldEnum::Enum(extract_enum_field(
                            element, reader,
                        )));
                    }
                    FIXED_STRING_FIELD_ELEMENT => {
                        fields.push(ExtensionRecordFieldEnum::FixedString(
                            extract_fixed_string_field(element, reader),
                        ));
                    }
                    FIXED_RECORD_FIELD_ELEMENT => {
                        fields.push(ExtensionRecordFieldEnum::FixedRecord(
                            extract_fixed_record_field(element, reader),
                        ));
                    }
                    BIT_RECORD_FIELD_ELEMENT => fields.push(ExtensionRecordFieldEnum::BitRecord(
                        extract_bit_record_field(element, reader),
                    )),
                    ADAPTIVE_RECORD_FIELD_ELEMENT => {
                        fields.push(ExtensionRecordFieldEnum::AdaptiveRecord(
                            extract_adaptive_record_field(element, reader),
                        ));
                    }
                    PADDING_TO_64_ELEMENT => padding_to_64_field = Some(extract_padding_64_field()),
                    element => {
                        panic!("Unexpected element '{element:?}' in {EXTENSION_RECORD_ELEMENT:?}")
                    }
                },
                Ok(Event::Start(ref element)) => match element.name() {
                    VARIABLE_STRING_ELEMENT => {
                        fields.push(ExtensionRecordFieldEnum::VariableString(
                            extract_variable_string_element(reader),
                        ));
                    }
                    ARRAY_ELEMENT => fields.push(ExtensionRecordFieldEnum::Array(
                        extract_array_element(reader),
                    )),
                    OPAQUE_DATA_ELEMENT => fields.push(ExtensionRecordFieldEnum::Opaque(
                        extract_opaque_data_element(reader),
                    )),
                    element => {
                        panic!("Encountered unexpected start element {element:?} in {EXTENSION_RECORD_ELEMENT:?}.")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    EXTENSION_RECORD_ELEMENT => break, // record finished
                    element => {
                        panic!("Encountered unexpected end element {element:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {EXTENSION_RECORD_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        (fields, padding_to_64_field)
    }

    fn extract_numeric_field(element: &BytesStart) -> NumericField {
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };

        NumericField {
            name: attr_name.expect("Expected NumericField attribute 'name' to be present."),
            primitive_type: attr_type
                .expect("Expected NumericField attribute 'type' to be present."),
            units: None,
        }
    }

    fn extract_fixed_record_field(
        element: &BytesStart,
        reader: &Reader<BufReader<File>>,
    ) -> FixedRecordField {
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };

        FixedRecordField {
            name: attr_name.expect("Expected FixedRecordField attribute 'name' to be present."),
            length: attr_length
                .expect("Expected FixedRecordField attribute 'length' to be present."),
            field_type: attr_type
                .expect("Expected FixedRecordField attribute 'type' to be present."),
        }
    }

    fn extract_count_field(element: &BytesStart) -> CountField {
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };

        CountField {
            name: attr_name.expect("Expected CountField attribute 'name' to be present."),
            primitive_type: attr_type.expect("Expected CountField attribute 'type' to be present."),
        }
    }

    fn extract_enum_field(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> EnumField {
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_enum_uid = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_ENUM_UID)
        {
            expand_uid_string(&extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_hd =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_HIERARCHY_DEPENDENCY) {
                Some(extract_attr_to_string(&attr))
            } else {
                None
            };
        let attr_is_discriminant =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_IS_DISCRIMINANT) {
                Some(extract_attr_to_bool(reader, &attr))
            } else {
                None
            };

        EnumField {
            name: attr_name.expect("Expected EnumField attribute 'name' to be present."),
            field_type: attr_type.expect("Expected EnumField attribute 'type' to be present."),
            enum_uid: attr_enum_uid,
            hierarchy_dependency: attr_hd,
            is_discriminant: attr_is_discriminant,
        }
    }

    fn extract_fixed_string_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> FixedStringField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };

        FixedStringField {
            name: attr_name.expect("Expected FixedStringField attribute 'name' to be present."),
            length: attr_length
                .expect("Expected FixedStringField attribute 'length' to be present."),
        }
    }

    fn extract_variable_string_element(reader: &mut Reader<BufReader<File>>) -> VariableString {
        let mut buf = Vec::new();
        let mut count_field = None;
        let mut string_field = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    COUNT_FIELD_ELEMENT => {
                        count_field = Some(extract_count_field(element));
                    }
                    VARIABLE_STRING_FIELD_ELEMENT => {
                        string_field = Some(extract_variable_string_field(element, reader));
                    }
                    element => {
                        panic!("Encountered unexpected empty element {element:?} in {VARIABLE_STRING_ELEMENT:?}.")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    VARIABLE_STRING_ELEMENT => {
                        break;
                    }
                    element => {
                        panic!("Encountered unexpected end element {element:?} {VARIABLE_STRING_ELEMENT:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {VARIABLE_STRING_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }

        VariableString {
            count_field: count_field
                .expect("Expected VariableString to have an element CountField."),
            string_field: string_field
                .expect("Expected VariableString to have an element VariableStringField."),
        }
    }

    fn extract_variable_string_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> VariableStringField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let fixed_number_of_strings =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_FIXED_NO_OF_STRINGS) {
                Some(extract_attr_to_usize(reader, &attr))
            } else {
                None
            };

        VariableStringField {
            name: attr_name.expect("Expected VariableStringField attribute 'name' to be present."),
            fixed_number_of_strings,
        }
    }

    fn extract_array_element(reader: &mut Reader<BufReader<File>>) -> Array {
        let mut buf = Vec::new();
        let mut count_field = None;
        let mut type_field = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    COUNT_FIELD_ELEMENT => {
                        count_field = Some(extract_count_field(element));
                    }
                    NUMERIC_FIELD_ELEMENT => {
                        type_field = Some(ArrayFieldEnum::Numeric(extract_numeric_field(element)));
                    }
                    ENUM_FIELD_ELEMENT => {
                        type_field =
                            Some(ArrayFieldEnum::Enum(extract_enum_field(element, reader)));
                    }
                    FIXED_STRING_FIELD_ELEMENT => {
                        type_field = Some(ArrayFieldEnum::FixedString(extract_fixed_string_field(
                            element, reader,
                        )));
                    }
                    FIXED_RECORD_FIELD_ELEMENT => {
                        type_field = Some(ArrayFieldEnum::FixedRecord(extract_fixed_record_field(
                            element, reader,
                        )));
                    }
                    BIT_RECORD_FIELD_ELEMENT => {
                        type_field = Some(ArrayFieldEnum::BitRecord(extract_bit_record_field(
                            element, reader,
                        )));
                    }
                    element => {
                        panic!("Encountered unexpected empty element {element:?} in {ARRAY_ELEMENT:?}.")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    ARRAY_ELEMENT => {
                        break;
                    }
                    element => {
                        panic!("Encountered unexpected end element {element:?} {VARIABLE_STRING_ELEMENT:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {VARIABLE_STRING_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }

        Array {
            count_field: count_field.expect("Expected Array field CountField to be present."),
            type_field: type_field
                .expect("Expected Array field defining the type of the fields to be present."),
        }
    }

    fn extract_bit_record_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> BitRecordField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_size = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_enum_uid = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_ENUM_UID)
        {
            expand_uid_string(&extract_attr_to_string(&attr))
        } else {
            None
        };

        BitRecordField {
            name: attr_name.expect("Expected BitRecordField attribute 'name' to be present."),
            size: attr_size.expect("Expected BitRecordField attribute 'size' to be present."),
            field_type: attr_type,
            enum_uid: attr_enum_uid,
        }
    }

    fn extract_adaptive_record_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> AdaptiveRecordField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(extract_attr_to_usize(reader, &attr))
        } else {
            None
        };
        let attr_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_enum_uid = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_ENUM_UID)
        {
            expand_uid_string(&extract_attr_to_string(&attr))
        } else {
            None
        };
        let attr_discriminant =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_DISCRIMINANT) {
                Some(extract_attr_to_string(&attr))
            } else {
                None
            };

        AdaptiveRecordField {
            name: attr_name.expect("Expected AdaptiveRecordField attribute 'name' to be present."),
            length: attr_length
                .expect("Expected AdaptiveRecordField attribute 'length' to be present."),
            field_type: attr_type,
            enum_uid: attr_enum_uid,
            discriminant: attr_discriminant
                .expect("Expected AdaptiveRecordField attribute 'discriminant' to be present."),
        }
    }

    fn extract_opaque_data_element(reader: &mut Reader<BufReader<File>>) -> OpaqueData {
        let mut buf = Vec::new();
        let mut count_field = None;
        let mut data_field = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    COUNT_FIELD_ELEMENT => {
                        count_field = Some(extract_count_field(element));
                    }
                    OPAQUE_DATA_FIELD_ELEMENT => {
                        data_field = Some(extract_opaque_data_field(element));
                    }
                    element => {
                        panic!("Encountered unexpected empty element {element:?} in {OPAQUE_DATA_ELEMENT:?}.")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    OPAQUE_DATA_ELEMENT => {
                        break;
                    }
                    element => {
                        panic!("Encountered unexpected end element {element:?} {OPAQUE_DATA_ELEMENT:?}.")
                    }
                },
                Ok(Event::Eof) => {
                    panic!("Unexpected end of file at element {OPAQUE_DATA_ELEMENT:?}.")
                }
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }

        OpaqueData {
            count_field: count_field.expect("Expected OpaqueData field CountField to be present."),
            opaque_data_field: data_field
                .expect("Expected OpaqueData field OpaqueDataField to be present."),
        }
    }

    fn extract_opaque_data_field(element: &BytesStart) -> OpaqueDataField {
        let attr_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(extract_attr_to_string(&attr))
        } else {
            None
        };

        OpaqueDataField {
            name: attr_name.expect("Expected OpaqueDataField attribute 'name' to be present."),
        }
    }

    #[inline]
    fn extract_padding_16_field() -> PaddingTo16 {
        PaddingTo16
    }

    #[inline]
    fn extract_padding_32_field() -> PaddingTo32 {
        PaddingTo32
    }

    #[inline]
    fn extract_padding_64_field() -> PaddingTo64 {
        PaddingTo64
    }

    /// Helper function to extract an Attribute value as a String
    #[inline]
    fn extract_attr_to_string(attr: &Attribute) -> String {
        String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8")
    }

    /// Helper function to extract an Attribute value as an usize
    #[inline]
    fn extract_attr_to_usize(reader: &Reader<BufReader<File>>, attr: &Attribute) -> usize {
        usize::from_str(
            &reader
                .decoder()
                .decode(&attr.value)
                .expect("Expected valid UTF-8"),
        )
        .expect("Expected a value able to be parsed to 'usize'")
    }

    /// Helper function to extract an Attribute value as a bool
    #[inline]
    fn extract_attr_to_bool(reader: &Reader<BufReader<File>>, attr: &Attribute) -> bool {
        bool::from_str(
            &reader
                .decoder()
                .decode(&attr.value)
                .expect("Expected valid UTF-8"),
        )
        .expect("Expected a value able to be parsed to 'bool'")
    }

    /// Helper function to expand a list of UIDs in &str format,
    /// consisting of single items and ranges, into a Vec<usize>.
    ///
    /// Returns the enum IDs as defined by the `uid_string` format
    /// contained in an `Option::Some`, and `Option::None` when
    /// the value of `uid_string` equals `"None"` (as the attribute can be optional).
    ///
    /// Example: `"1, 4, 6-8"` would expand to `1, 4, 6, 7, 8`.
    fn expand_uid_string(uid_string: &str) -> Option<Vec<usize>> {
        const NONE_STRING: &str = "None";
        const EREF_STRING: &str = "EREF"; // TODO figure out what this value means
        const TBD_STRING: &str = "TBD"; // FIXME remove when the standard has settled all UIDs
        if [NONE_STRING, EREF_STRING, TBD_STRING].contains(&uid_string) {
            return None;
        }

        Some(
            uid_string
                .split(',')
                .map(str::trim)
                .filter(|part| !part.is_empty())
                .flat_map(|part| {
                    if let Some((start_str, end_str)) = part.split_once('-') {
                        // Parse range
                        let start: usize = start_str
                            .trim()
                            .parse()
                            .unwrap_or_else(|_| panic!("Invalid range start: '{start_str}'"));

                        let end: usize = end_str
                            .trim()
                            .parse()
                            .unwrap_or_else(|_| panic!("Invalid range end: '{end_str}'"));

                        assert!(
                            start <= end,
                            "{}",
                            format!("Invalid range '{part}': start > end")
                        );
                        (start..=end).collect::<Vec<usize>>()
                    } else {
                        // Parse single number
                        let number: usize = part
                            .parse()
                            .unwrap_or_else(|_| panic!("Invalid number: '{part}'"));

                        vec![number]
                    }
                })
                .collect(),
        )
    }
}

// Approach:
// 1. V When generating SISO-REF-010 enumerations, build an index of uids to the names of the structs/enums.
//     - this can be done after extracting the values from the XML,
//          V making a loop over all GenerationItems,
//          V already formatting the decl names (instead of in the generate step, >> just applied the formatting again, could be more efficient naturally.
//          V and building the index of UID to names as used in the generated code.
//          V The resulting index must be made available to the PDU generator.
// 2.a V Build intermediate representation model (IR) based on the XSD
// 2.b Store in the IR to which rust module the generated code should be placed (common, v8, based on PduType...)
// 3. Extract and generate all basic types and records (from DIS_CommonRecords.xml)
// 4. Extract and generate all types in the other XML files that define the schema, per family/category.
// 5. Generate all serialization, deserialization, Display, Default, and (optional) builder code.
