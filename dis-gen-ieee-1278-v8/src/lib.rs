use std::collections::HashMap;
use std::env;
use std::fmt::{Debug, Display, Write};

mod extraction;
mod generation;

const XML_FILE_EXTENSION: &str = "xml";
const OUT_DIR: &str = "OUT_DIR";
const TARGET_ENV_VAR: &str = "TARGET_GENERATED_SISO_1278_V8_FILENAME";
const TARGET_OUT_FILE: &str = "siso_1278_v8.rs";

/// This is the main entry point for the generation of the DIS v8 code units.
/// It is meant to be called from a build script.
///
/// When reading all schemas and generating all code is successful the code written
/// to an output file in the `OUT_DIR` location. The precise location and name are
/// set in an environment variable, which is read by the DIS library itself to include
/// the generated code.
///
/// # Panics
/// The functions in the call stack beneath this function panic when encountering
/// inconsistent states or values in the schema files.
pub fn execute(schema_dir: &str, uid_index: &HashMap<usize, String>) {
    if std::path::Path::new(schema_dir).is_dir() {
        // Find all files in the provided directory. Files must be .xml files.
        let mut file_paths = std::fs::read_dir(schema_dir)
            .expect("Cannot read provided directory.")
            .map(|a| a.expect("Cannot access `DirEntry`"))
            .filter(|a| a.path().is_file())
            .filter(|a| a.path().extension() == Some(XML_FILE_EXTENSION.as_ref()))
            .map(|a| a.path())
            .collect::<Vec<std::path::PathBuf>>();
        file_paths.sort();

        let mut families = vec![]; // keeps track of all unique PDU Family names, based on the schema file names.
                                   // Extract the items to be generated from all identified files
        let generation_items = file_paths
            .iter()
            .flat_map(|path| {
                // The extractor extracts and formats the PDU Family name, which is also in the path.
                // But it is already formatted nicely this way.
                let (items, family) = extraction::extract_from_file(path);
                families.push(family);
                items
            })
            .collect::<Vec<GenerationItem>>();

        // Generate all items
        let generated = generation::generate(&generation_items, &families);

        // Format generated code using prettyplease
        let ast = syn::parse_file(&generated.to_string())
            .expect("Error parsing generated code for pretty printing.");
        let contents = prettyplease::unparse(&ast);

        // Save to file
        let dest_path = std::path::Path::new(&env::var(OUT_DIR).unwrap()).join(TARGET_OUT_FILE);
        std::fs::write(dest_path, contents).unwrap();

        // Set file name to an environment variable, for inclusion in the to-be compiled library
        println!("cargo:rustc-env={TARGET_ENV_VAR}={TARGET_OUT_FILE}");
    }
}

#[derive(Debug, Clone)]
enum GenerationItem {
    Pdu(Pdu, String),
    FixedRecord(FixedRecord, String),
    BitRecord(BitRecord, String),
    AdaptiveRecord(AdaptiveRecord, String),
    ExtensionRecord(ExtensionRecord, String),
}

impl GenerationItem {
    fn family(&self) -> String {
        match self {
            GenerationItem::Pdu(_, fam) => fam.clone(),
            GenerationItem::FixedRecord(_, fam) => fam.clone(),
            GenerationItem::BitRecord(_, fam) => fam.clone(),
            GenerationItem::AdaptiveRecord(_, fam) => fam.clone(),
            GenerationItem::ExtensionRecord(_, fam) => fam.clone(),
        }
    }

    fn name(&self) -> String {
        match self {
            GenerationItem::Pdu(item, _) => item.name_attr.clone(),
            GenerationItem::FixedRecord(item, _) => item.record_type.clone(),
            GenerationItem::BitRecord(item, _) => item.record_type.clone(),
            GenerationItem::AdaptiveRecord(item, _) => item.record_type.clone(),
            GenerationItem::ExtensionRecord(item, _) => item.name_attr.clone(),
        }
    }

    fn is_pdu(&self) -> bool {
        match self {
            GenerationItem::Pdu(_, _) => true,
            _ => false,
        }
    }
}

// enum Items {
//     Pdu(Pdu),
//     Record(),
//     Field,
//     SpecialType,
//     Type,
// }

// enum Record {
//     FixedRecord,
//     BitRecord,
//     AdaptiveRecord,
//     ExtensionRecord,
// }

// enum Field {
//     NumericField,
//     CountField,
//     EnumField,
//     FixedStringField,
//     // IntBitField,
//     // EnumBitField,
//     // BoolBitField,
//     FixedRecordField,
//     BitRecordField(BitRecordField),
//     AdaptiveRecordField, // AdaptiveRecordField can occur in other records
//     AdaptiveFormat,      // Can occur within an AdaptiveRecord
//     VariableString,      // VariableStringField is contained within VariableString
//     OpaqueData,          // OpaqueDataField is contained within OpaqueData
//     Array,
// }

// enum SpecialType {
//     Array,
//     VariableString,
//     OpaqueData,
//     AdaptiveFormat,
// }

// enum Primitive {
//     Numeric,
//     Enum,
//     Count,
// }

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

// TODO define a scope attribute for generation items so we can fit them into the module tree

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

#[cfg(test)]
mod tests {

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }
}
