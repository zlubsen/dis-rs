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
enum GenerationItem {}

mod extraction {
    use crate::pdus::GenerationItem;
    use quick_xml::events::Event;
    use quick_xml::name::QName;
    use quick_xml::Reader;
    use std::path::PathBuf;

    const PDU_ELEMENT: QName = QName(b"PDU");
    const FIXED_RECORD_FIELD_ELEMENT: QName = QName(b"FixedRecordField");
    const NUMERIC_FIELD_ELEMENT: QName = QName(b"NumericField");
    const ENUM_FIELD_ELEMENT: QName = QName(b"EnumField");
    const FIXED_STRING_FIELD_ELEMENT: QName = QName(b"FixedStringField");
    const BIT_RECORD_FIELD_ELEMENT: QName = QName(b"BitRecordField");
    const ADAPTIVE_RECORD_FIELD_ELEMENT: QName = QName(b"AdaptiveRecordField");
    const EXTENSION_RECORD_SET_ELEMENT: QName = QName(b"ExtensionRecordSet");
    const ARRAY_ELEMENT: QName = QName(b"Array");
    const COUNT_FIELD_ELEMENT: QName = QName(b"CountField");
    const OPAQUE_DATA_ELEMENT: QName = QName(b"OpaqueData");
    const PADDING_TO_16_ELEMENT: QName = QName(b"PaddingTo16");
    const PADDING_TO_32_ELEMENT: QName = QName(b"PaddingTo32");
    const PADDING_TO_64_ELEMENT: QName = QName(b"PaddingTo64");

    // Primitive types mapping:
    // numeric_t
    // enum_t
    // count_t

    struct NumericField {
        pub name: String,
        pub primitive_type: String,
        pub units: Option<String>,
    }

    struct CountField {
        pub name: String,
        pub primitive_type: String,
    }

    struct EnumField {
        pub name: String,
        pub size: usize,
        pub enum_uid: Option<String>, // TODO model that this can be a single UID or a range ('1, 4' or '1-5')
        pub hierarchy_dependency: Option<String>,
        pub is_discriminant: Option<bool>,
    }

    struct FixedStringField {
        pub name: String,
        pub length: usize,
    }

    struct IntBitField {
        pub name: String,
        pub bit_position: usize,
        pub size: Option<usize>,
        pub units: Option<String>,
    }

    struct EnumBitField {
        pub name: String,
        pub bit_position: usize,
        pub size: Option<usize>,
        pub enum_uid: Option<String>, // TODO model that this can be a single UID or a range ('1, 4' or '1-5')
        pub is_discriminant: Option<bool>,
    }

    struct BooleanBitField {
        pub name: String,
        pub bit_position: usize,
    }

    struct FixedRecordField {
        pub name: String,
        pub length: usize,
        pub field_type: String,
    }

    struct BitRecordField {
        pub name: String,
        pub size: usize,
        pub field_type: Option<String>,
        pub enum_uid: Option<String>,
    }

    struct AdaptiveRecordField {
        pub name: String,
        pub size: usize,
        pub field_type: Option<String>,
        pub enum_uid: Option<String>,
        pub discriminant: String,
    }

    struct VariableStringField {
        pub name: String,
        pub fixed_number_of_strings: Option<usize>,
    }

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

    enum BitFieldEnum {
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

    struct Pdu {
        pub name_attr: String,
        pub pdu_type_attr: usize,
        pub protocol_family_attr: usize,
        pub base_length_attr: usize,
        pub fields: Vec<FixedRecordFieldEnum>,
    }

    pub(crate) fn extract_from_file(path: &PathBuf) -> Vec<GenerationItem> {
        let mut reader = Reader::from_file(a).unwrap();
        reader.config_mut().trim_text(true);

        todo!()
    }

    fn extract() -> Vec<GenerationItem> {
        let mut buf = Vec::new();
        let mut items = Vec::new();
        let mut current_item = None;

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref element)) => match element.name() {
                    // TODO match start elements
                    _ => (),
                },
                Ok(Event::End(ref element)) => {
                    match element.name() {
                        // TODO match end elements
                        _ => (),
                    }
                }
                Ok(Event::Empty(ref element)) => match element.name() {
                    ENUM_ROW_ELEMENT => {
                        current_item = if let (
                            Some(crate::enumerations::GenerationItem::Enum(mut current)),
                            Ok(item),
                        ) = (
                            current_item,
                            crate::enumerations::extraction::extract_enum_item(element, reader),
                        ) {
                            current.items.push(item);
                            Some(crate::enumerations::GenerationItem::Enum(current))
                        } else {
                            None
                        };
                    }
                    ENUM_ROW_RANGE_ELEMENT => {
                        current_item = if let (
                            Some(crate::enumerations::GenerationItem::Enum(mut current)),
                            Ok(item),
                        ) = (
                            current_item,
                            crate::enumerations::extraction::extract_enum_range_item(
                                element, reader,
                            ),
                        ) {
                            current.items.push(item);
                            Some(crate::enumerations::GenerationItem::Enum(current))
                        } else {
                            None
                        };
                    }
                    BITFIELD_ROW_ELEMENT => {
                        current_item = if let (
                            Some(crate::enumerations::GenerationItem::Bitfield(mut current)),
                            Ok(item),
                        ) = (
                            current_item,
                            crate::enumerations::extraction::extract_bitfield_item(element, reader),
                        ) {
                            current.fields.push(item);
                            Some(crate::enumerations::GenerationItem::Bitfield(current))
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
