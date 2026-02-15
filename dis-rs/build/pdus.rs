use std::collections::HashMap;

const XML_FILE_EXTENSION: &str = "xml";
const SISO_SCHEMA_DIR: &str = "./definitions/v8-schemas";

pub fn execute(uid_index: &HashMap<usize, String>) {
    if std::path::Path::new(SISO_SCHEMA_DIR).is_dir() {
        let file_paths = std::fs::read_dir(SISO_SCHEMA_DIR)
            .unwrap()
            .map(|a| a.unwrap())
            .filter(|a| a.path().is_file())
            .filter(|a| a.path().extension() == Some(XML_FILE_EXTENSION.as_ref()))
            .map(|a| a.path())
            .collect::<Vec<std::path::PathBuf>>();

        let generation_items = file_paths
            .iter()
            .map(|path| extraction::extract_from_file(path))
            .flatten()
            .collect::<Vec<GenerationItem>>();

        // for entry in
        //     std::fs::read_dir(SISO_SCHEMA_DIR).expect("Expected a directory")
        // {
        //     let entry = entry.expect("Expected entries to be in the directory");
        //     let path = entry.path();
        //     if path.is_file() {
        //         if path.extension()
        //         println!("{path}");
        //     }
        // }
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
    pub enum_uid: Option<String>, // TODO model that this can be a single UID or a range ('1, 4' or '1-5')
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
    pub enum_uid: Option<String>, // TODO model that this can be a single UID or a range ('1, 4' or '1-5')
    pub is_discriminant: Option<bool>,
}

#[derive(Debug, Clone)]
struct BooleanBitField {
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
    pub enum_uid: Option<String>,
}

#[derive(Debug, Clone)]
struct AdaptiveRecordField {
    pub name: String,
    pub length: usize,
    pub field_type: Option<String>,
    pub enum_uid: Option<String>,
    pub discriminant: String,
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
    pub count_field: usize,
    pub field_type: ArrayFieldEnum,
}

#[derive(Debug, Clone)]
struct VariableString {
    pub count: usize,
    pub string: VariableStringField,
}

#[derive(Debug, Clone)]
struct OpaqueData {
    pub count: usize,
    pub string: OpaqueDataField,
}

#[derive(Debug, Clone)]
struct AdaptiveFormat {
    pub format_type: AdaptiveFormatEnum,
    pub discriminant_start_value: usize,
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
    Bool(BooleanBitField),
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
    pub count: CountField,
    pub fields: Vec<ExtensionRecord>,
}

#[derive(Debug, Clone)]
enum ExtensionRecordFieldEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    VariableString(VariableStringField),
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
    pub record_type_field: EnumField,
    pub record_length_field: NumericField,
    pub fields: Vec<ExtensionRecordFieldEnum>,
    pub padding_to_64_field: Option<PaddingTo64>,
    pub name_attr: String,
    pub record_type_attr: usize,
    pub base_length_attr: usize,
    pub is_variable_attr: bool,
}

#[derive(Debug, Clone)]
struct Pdu {
    pub name_attr: String,
    pub pdu_type_attr: usize,
    pub protocol_family_attr: usize,
    pub base_length_attr: usize,
    pub fields: Vec<FixedRecordFieldsEnum>,
}

mod extraction {
    use crate::pdus::{
        AdaptiveFormatEnum, AdaptiveRecord, AdaptiveRecordField, BitRecord, BitRecordField,
        BitRecordFieldEnum, CountField, EnumField, ExtensionRecord, FixedRecord, FixedRecordField,
        FixedRecordFieldsEnum, FixedStringField, GenerationItem, NumericField, PaddingTo16,
        PaddingTo32, PaddingTo64, Pdu,
    };
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
    const ELEMENT_ATTR_ENUM_ID: QName = QName(b"enumTableUID");
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

        let generation_items = extract(&mut reader);
        todo!()
    }

    fn extract(reader: &mut Reader<BufReader<File>>) -> Vec<GenerationItem> {
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
                    DIS_SYNTAX_ELEMENT | COPYRIGHT_ELEMENT => (),
                    element => {
                        panic!("Encountered unexpected start element {element:?} at top-level.")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    // Pass on the main structure elements that are not interesting
                    DIS_SYNTAX_ELEMENT | COPYRIGHT_ELEMENT => (),
                    element => {
                        panic!("Encountered unexpected end element {element:?} at top-level.")
                    }
                },
                Ok(Event::Empty(ref element)) => {
                    let name = element.name();
                    panic!("Unexpected closed element '{name:?}' encountered at top-level")
                }
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            };
        }
        items
    }

    fn extract_dis_syntax(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Pdu {
        todo!()
    }

    fn extract_pdu(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Pdu {
        todo!()
    }

    fn extract_fixed_record(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> FixedRecord {
        let record_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let record_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(usize::from_str(&reader.decoder().decode(&attr.value).unwrap()).unwrap())
        } else {
            None
        };
        let fields = extract_fixed_record_fields(reader);

        FixedRecord {
            record_type: record_type.expect("Expected record attribute 'type' to be present."),
            length: record_length.expect("Expected record attribute 'length' to be present."),
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
                            element, reader,
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
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        fields
    }

    fn extract_bit_record(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> BitRecord {
        let record_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let record_size = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(usize::from_str(&reader.decoder().decode(&attr.value).unwrap()).unwrap())
        } else {
            None
        };
        let fields = extract_bit_record_fields(reader);

        BitRecord {
            record_type: record_type.expect("Expected BitRecord attribute 'type' to be present."),
            size: record_size.expect("Expected BitRecord attribute 'size' to be present."),
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
                        todo!();
                        // fields.push(BitRecordFieldEnum::Enum(extract_enum_bit_field(element, reader)));
                    }
                    INT_BIT_FIELD_ELEMENT => {
                        todo!();
                        // fields.push(BitRecordFieldEnum::Int(extract_int_bit_field(element, reader)));
                    }
                    BOOL_BIT_FIELD_ELEMENT => {
                        todo!();
                        // fields.push(BitRecordFieldEnum::Bool(extract_bool_bit_field(element, reader)));
                    }
                    element => {
                        panic!("Unexpected element '{element:?}' in {BIT_RECORD_ELEMENT:?}")
                    }
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        fields
    }

    fn extract_adaptive_record(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> AdaptiveRecord {
        let record_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let record_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(usize::from_str(&reader.decoder().decode(&attr.value).unwrap()).unwrap())
        } else {
            None
        };
        let fields = extract_adaptive_record_formats(reader);

        AdaptiveRecord {
            record_type: record_type
                .expect("Expected AdaptiveRecord attribute 'type' to be present."),
            length: record_length
                .expect("Expected AdaptiveRecord attribute 'length' to be present."),
            fields,
        }
    }

    fn extract_adaptive_record_formats(
        reader: &mut Reader<BufReader<File>>,
    ) -> Vec<AdaptiveFormatEnum> {
        let mut buf = Vec::new();
        let mut fields = vec![];
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    NUMERIC_FIELD_ELEMENT => {
                        fields.push(AdaptiveFormatEnum::Numeric(extract_numeric_field(
                            element, reader,
                        )));
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
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        fields
    }

    fn extract_extension_record(
        element: &BytesStart,
        reader: &Reader<BufReader<File>>,
    ) -> ExtensionRecord {
        todo!()
    }

    fn extract_numeric_field(
        element: &BytesStart,
        reader: &Reader<BufReader<File>>,
    ) -> NumericField {
        let field_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };

        NumericField {
            name: field_name.expect("Expected NumericField attribute 'name' to be present."),
            primitive_type: field_type
                .expect("Expected NumericField attribute 'type' to be present."),
            units: None,
        }
    }

    fn extract_fixed_record_field(
        element: &BytesStart,
        reader: &Reader<BufReader<File>>,
    ) -> FixedRecordField {
        let field_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(
                usize::from_str(
                    &reader
                        .decoder()
                        .decode(&attr.value)
                        .expect("Expected valid UTF-8"),
                )
                .expect("Expected a value able to be parsed to 'usize'"),
            )
        } else {
            None
        };

        FixedRecordField {
            name: field_name.expect("Expected FixedRecordField attribute 'name' to be present."),
            length: length.expect("Expected FixedRecordField attribute 'length' to be present."),
            field_type: field_type
                .expect("Expected FixedRecordField attribute 'type' to be present."),
        }
    }

    fn extract_count_field(element: &BytesStart, reader: &Reader<BufReader<File>>) -> CountField {
        let field_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };

        CountField {
            name: field_name.expect("Expected CountField attribute 'name' to be present."),
            primitive_type: field_type
                .expect("Expected CountField attribute 'type' to be present."),
        }
    }

    fn extract_enum_field(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> EnumField {
        let field_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_enum_uid = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_ENUM_ID)
        {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_hd =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_HIERARCHY_DEPENDENCY) {
                Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
            } else {
                None
            };
        let field_is_discriminant =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_IS_DISCRIMINANT) {
                let value = String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8");
                Some(
                    bool::from_str(&value)
                        .expect("Expected 'true' or 'false' value for attribute 'isDiscriminant'."),
                )
            } else {
                None
            };

        EnumField {
            name: field_name.expect("Expected EnumField attribute 'name' to be present."),
            field_type: field_type.expect("Expected EnumField attribute 'type' to be present."),
            enum_uid: field_enum_uid,
            hierarchy_dependency: field_hd,
            is_discriminant: field_is_discriminant,
        }
    }

    fn extract_fixed_string_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> FixedStringField {
        let field_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_length = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(usize::from_str(&reader.decoder().decode(&attr.value).unwrap()).unwrap())
        } else {
            None
        };

        FixedStringField {
            name: field_name.expect("Expected FixedStringField attribute 'name' to be present."),
            length: field_length
                .expect("Expected FixedStringField attribute 'length' to be present."),
        }
    }

    fn extract_bit_record_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> BitRecordField {
        let field_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_size = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(usize::from_str(&reader.decoder().decode(&attr.value).unwrap()).unwrap())
        } else {
            None
        };
        let field_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_enum_uid = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_ENUM_ID)
        {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };

        BitRecordField {
            name: field_name.expect("Expected BitRecordField attribute 'name' to be present."),
            size: field_size.expect("Expected BitRecordField attribute 'size' to be present."),
            field_type,
            enum_uid: field_enum_uid,
        }
    }

    fn extract_adaptive_record_field(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> AdaptiveRecordField {
        let field_name = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_length =
            if let Ok(Some(attr_size)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
                Some(usize::from_str(&reader.decoder().decode(&attr_size.value).unwrap()).unwrap())
            } else {
                None
            };
        let field_type = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_enum_uid = if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_ENUM_ID)
        {
            Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_discriminant =
            if let Ok(Some(attr)) = element.try_get_attribute(ELEMENT_ATTR_DISCRIMINANT) {
                Some(String::from_utf8(attr.value.to_vec()).expect("Expected valid UTF-8"))
            } else {
                None
            };

        AdaptiveRecordField {
            name: field_name.expect("Expected AdaptiveRecordField attribute 'name' to be present."),
            length: field_length
                .expect("Expected AdaptiveRecordField attribute 'length' to be present."),
            field_type,
            enum_uid: field_enum_uid,
            discriminant: field_discriminant
                .expect("Expected AdaptiveRecordField attribute 'discriminant' to be present."),
        }
    }

    fn extract_padding_16_field() -> PaddingTo16 {
        PaddingTo16
    }

    fn extract_padding_32_field() -> PaddingTo32 {
        PaddingTo32
    }

    fn extract_padding_64_field() -> PaddingTo64 {
        PaddingTo64
    }
}

// Approach:
// 1. When generating SISO-REF-010 enumerations, build an index of uids to the names of the structs/enums.
//     - this can be done after extracting the values from the XML,
//          V making a loop over all GenerationItems,
//          V already formatting the decl names (instead of in the generate step, >> just applied the formatting again, could be more efficient naturally.
//          V and building the index of UID to names as used in the generated code.
//          V The resulting index must be made available to the PDU generator.
// 2. Build intermediate model based on the XSD, either by hand or using https://github.com/Bergmann89/xsd-parser >> xsd-parser would require a multi-phase build script.
// 3. Extract and generate all basic types and records (from DIS_CommonRecords.xml)
// 4. Extract and generate all types in the other XML files that define the schema, per family/category.
// 5. Generate all serialization, deserialization, Display, Default, and (optional) builder code.
