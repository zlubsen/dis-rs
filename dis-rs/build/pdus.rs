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
    pub size: usize,
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
    pub size: usize,
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

enum BasicFieldEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
}

#[derive(Debug, Clone)]
enum FixedRecordFieldEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
    AdaptiveRecord(AdaptiveRecordField), // TODO - not for Array and AdaptiveFormat, but for FixedRecord and PDU
}

struct Array {
    pub count_field: usize,
    pub field_type: BasicFieldEnum,
}

struct VariableString {
    pub count: usize,
    pub string: VariableStringField,
}

struct OpaqueData {
    pub count: usize,
    pub string: OpaqueDataField,
}

struct AdaptiveFormat {
    pub format_type: BasicFieldEnum,
    pub discriminant_start_value: usize,
}

struct FixedRecord {
    pub fields: Vec<FixedRecordFieldEnum>,
    pub record_type: String,
    pub length: usize,
}

enum BitRecordFieldEnum {
    Enum(EnumBitField),
    Int(IntBitField),
    Bool(BooleanBitField),
}

struct BitRecord {
    pub fields: Vec<BitFieldEnum>,
    pub record_type: String,
    pub size: usize,
}

struct AdaptiveRecord {
    pub fields: Vec<AdaptiveFormat>,
    pub record_type: String,
    pub length: usize,
}

enum AdaptiveFormatEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
}

struct ExtensionRecordSet {
    pub count: CountField,
    pub fields: Vec<ExtensionRecord>,
}

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

struct PaddingTo16;
struct PaddingTo32;
struct PaddingTo64;

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
    pub fields: Vec<FixedRecordFieldEnum>,
}

mod extraction {
    use crate::pdus::{
        AdaptiveFormatEnum, AdaptiveRecord, BitRecord, BitRecordFieldEnum, CountField,
        ExtensionRecord, FixedRecord, FixedRecordField, FixedRecordFieldEnum, GenerationItem,
        NumericField, PaddingTo16, PaddingTo32, PaddingTo64, Pdu,
    };
    use quick_xml::events::{BytesStart, Event};
    use quick_xml::name::QName;
    use quick_xml::Reader;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;
    use std::str::FromStr;

    // Top-level elements
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
        let mut reader = Reader::from_file(a).unwrap();
        reader.config_mut().trim_text(true);

        let generation_items = extract(&mut reader);
        todo!()
    }

    fn extract(reader: &mut Reader<BufReader<File>>) -> Vec<GenerationItem> {
        let mut buf = Vec::new();
        let mut items = Vec::new();
        let mut current_pdu = None;
        let mut current_fixed_item = None;
        let mut current_pdu = None;
        let mut current_pdu = None;
        let mut current_pdu = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref element)) => match element.name() {
                    PDU_ELEMENT => Some(GenerationItem::Pdu(extract_pdu(element, reader))),
                    FIXED_RECORD_ELEMENT => {
                        todo!()
                    }
                    BIT_RECORD_ELEMENT => {
                        todo!()
                    }
                    ADAPTIVE_RECORD_ELEMENT => {
                        todo!()
                    }
                    EXTENSION_RECORD_ELEMENT => {
                        todo!()
                    }
                    ARRAY_ELEMENT => {
                        todo!()
                    }
                    OPAQUE_DATA_ELEMENT => {
                        todo!()
                    }
                    element => {
                        eprintln!("encountered unmatched XML Element {element:?}")
                    }
                },
                Ok(Event::End(ref element)) => match element.name() {
                    PDU_ELEMENT => {
                        todo!()
                    }
                    FIXED_RECORD_ELEMENT => {
                        todo!()
                    }
                    BIT_RECORD_ELEMENT => {
                        todo!()
                    }
                    ADAPTIVE_RECORD_ELEMENT => {
                        todo!()
                    }
                    EXTENSION_RECORD_ELEMENT => {
                        todo!()
                    }
                    _ => (),
                },
                Ok(Event::Empty(ref element)) => match element.name() {
                    // TODO do empty elements exist at the top level?
                    _ => (),
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        items
    }

    fn extract_pdu(element: &BytesStart, reader: &Reader<BufReader<File>>) -> Pdu {
        todo!()
    }

    fn extract_fixed_record(
        element: &BytesStart,
        reader: &mut Reader<BufReader<File>>,
    ) -> FixedRecord {
        let record_type = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_TYPE)
        {
            Some(String::from_utf8(attr_name.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let length = if let Ok(Some(attr_length)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(usize::from_str(&reader.decoder().decode(&attr_length.value).unwrap()).unwrap())
        } else {
            None
        };
        let fields = extract_fixed_record_fields(reader);

        FixedRecord {
            fields,
            record_type: record_type.expect("Expected record attribute 'type' to be present."),
            length: length.expect("Expected record attribute 'length' to be present."),
        }
    }

    fn extract_fixed_record_fields(
        reader: &mut Reader<BufReader<File>>,
    ) -> Vec<FixedRecordFieldEnum> {
        let mut fields = vec![];
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    NUMERIC_FIELD_ELEMENT => extract_numeric_field(reader),
                    ENUM_FIELD_ELEMENT => {
                        todo!()
                    }
                    FIXED_STRING_FIELD_ELEMENT => {
                        todo!()
                    }
                    FIXED_RECORD_FIELD_ELEMENT => {
                        todo!()
                    }
                    BIT_RECORD_FIELD_ELEMENT => {
                        todo!()
                    }
                    ADAPTIVE_RECORD_FIELD_ELEMENT => {
                        todo!()
                    }
                    _ => (),
                },
                Ok(Event::Eof) => break, // exits the loop when reaching end of file
                Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
                _ => (), // There are several other `Event`s we do not consider here
            }
        }
        fields
    }

    fn extract_bit_record(element: &BytesStart, reader: &mut Reader<BufReader<File>>) -> BitRecord {
        let record_type = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_TYPE)
        {
            Some(String::from_utf8(attr_name.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let size = if let Ok(Some(attr_size)) = element.try_get_attribute(ELEMENT_ATTR_SIZE) {
            Some(usize::from_str(&reader.decoder().decode(&attr_size.value).unwrap()).unwrap())
        } else {
            None
        };
        let fields = extract_bit_record_fields(reader);

        BitRecord {
            fields,
            record_type: record_type.expect("Expected BitRecord attribute 'type' to be present."),
            size: size.expect("Expected BitRecord attribute 'size' to be present."),
        }
    }

    fn extract_bit_record_fields(reader: &mut Reader<BufReader<File>>) -> Vec<BitRecordFieldEnum> {
        let mut fields = vec![];
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    ENUM_BIT_FIELD_ELEMENT => {
                        todo!()
                    }
                    INT_BIT_FIELD_ELEMENT => {
                        todo!()
                    }
                    BOOL_BIT_FIELD_ELEMENT => {
                        todo!()
                    }
                    _ => (),
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
        let record_type = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_TYPE)
        {
            Some(String::from_utf8(attr_name.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let length = if let Ok(Some(attr_length)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(usize::from_str(&reader.decoder().decode(&attr_length.value).unwrap()).unwrap())
        } else {
            None
        };
        // TODO this is not yet okay
        let fields = extract_adaptive_record_formats(reader);

        AdaptiveRecord {
            fields: vec![],
            record_type: record_type
                .expect("Expected AdaptiveRecord attribute 'type' to be present."),
            length: length.expect("Expected AdaptiveRecord attribute 'length' to be present."),
        }
    }

    fn extract_adaptive_record_formats(
        reader: &mut Reader<BufReader<File>>,
    ) -> Vec<AdaptiveFormatEnum> {
        // TODO this is not yet okay
        let mut fields = vec![];
        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Empty(ref element)) => match element.name() {
                    ENUM_BIT_FIELD_ELEMENT => {
                        todo!()
                    }
                    INT_BIT_FIELD_ELEMENT => {
                        todo!()
                    }
                    BOOL_BIT_FIELD_ELEMENT => {
                        todo!()
                    }
                    _ => (),
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
        let field_type = if let Ok(Some(attr_type)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr_type.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_name = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr_name.value.to_vec()).expect("Expected valid UTF-8"))
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
        let field_type = if let Ok(Some(attr_type)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr_type.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_name = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr_name.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let length = if let Ok(Some(attr_length)) = element.try_get_attribute(ELEMENT_ATTR_LENGTH) {
            Some(
                usize::from_str(
                    &reader
                        .decoder()
                        .decode(&attr_length.value)
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
        let field_type = if let Ok(Some(attr_type)) = element.try_get_attribute(ELEMENT_ATTR_TYPE) {
            Some(String::from_utf8(attr_type.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };
        let field_name = if let Ok(Some(attr_name)) = element.try_get_attribute(ELEMENT_ATTR_NAME) {
            Some(String::from_utf8(attr_name.value.to_vec()).expect("Expected valid UTF-8"))
        } else {
            None
        };

        CountField {
            name: field_name.expect("Expected CountField attribute 'name' to be present."),
            primitive_type: field_type
                .expect("Expected CountField attribute 'type' to be present."),
        }
    }

    fn extract_padding_16_field(element: &BytesStart) -> PaddingTo16 {
        PaddingTo16
    }

    fn extract_padding_32_field(element: &BytesStart) -> PaddingTo32 {
        PaddingTo32
    }

    fn extract_padding_64_field(element: &BytesStart) -> PaddingTo64 {
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
