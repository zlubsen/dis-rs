use std::collections::HashMap;
use std::env;
use std::fmt::Debug;

mod extraction;
mod generation;

const XML_FILE_EXTENSION: &str = "xml";
const OUT_DIR: &str = "OUT_DIR";
const TARGET_ENV_VAR: &str = "TARGET_GENERATED_SISO_1278_GEN3_FILENAME";
const TARGET_OUT_FILE: &str = "siso_1278_gen3.rs";

type UidLookup = HashMap<usize, String>;
type FqnLookup = HashMap<String, String>;

// TODO create a lookup for discriminants - field name mapped to 1) uid?, 2) enum generation item (to generate variants with contained elements), ...

struct Lookup {
    fqn: FqnLookup,
    enum_fqn: FqnLookup,
    uid: UidLookup,
}

/// This is the main entry point for the generation of the DIS v8 code units.
/// It is meant to be called from a build script.
///
/// The `schema_dir` argument is the path to the directory where the XML schema definitions
/// are located.
///
/// The `uid_lookup` argument is a `&HashMap` that associates the UIDs from SISO-REF-010
/// enumerations to the code type names of those enumerations.
///
/// When reading all schemas and generating all code is successful the code written
/// to an output file in the `OUT_DIR` location. The precise location and name are
/// set in an environment variable, which is read by the DIS library itself to include
/// the generated code.
///
/// # Panics
/// The functions in the call stack beneath this function panic when encountering
/// inconsistent states or values in the schema files.
pub fn execute(schema_dir: &str, uid_lookup: UidLookup) {
    println!("{schema_dir}");
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

        // keeps track of all unique PDU Family names, based on the schema file names.
        let mut families = vec![];
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

        let (fqn_lookup, enum_fqn_lookup) = create_fqn_lookup(&generation_items, &uid_lookup);

        for fqn in &fqn_lookup {
            println!("{} => {}", fqn.0, fqn.1);
        }

        let lookup = Lookup {
            fqn: fqn_lookup,
            enum_fqn: enum_fqn_lookup,
            uid: uid_lookup,
        };

        // TODO intermediate processing: 1) format all field/type names before generating? need to update in the Intermediate Representation model.
        // DONE intermediate processing: 2) keep track of 'fully qualified names', including the modules, for use/import statements.
        // TODO intermediate processing: 3) convert all type names (schema to rust types) - See 1).
        // TODO intermediate processing: 4) expand all UIDs to textual field names in the Intermediate Representation model.

        // Generate all items
        let generated = generation::generate(&generation_items, &families, &lookup);

        // Format generated code using prettyplease
        let generated = syn::parse_file(&generated.to_string())
            .expect("Error parsing generated code for pretty printing.");
        let contents = prettyplease::unparse(&generated);

        // Save to file
        let dest_path = std::path::Path::new(&env::var(OUT_DIR).unwrap()).join(TARGET_OUT_FILE);
        std::fs::write(dest_path, contents).unwrap();

        // Set file name to an environment variable, for inclusion in the to-be compiled library
        println!("cargo:rustc-env={TARGET_ENV_VAR}={TARGET_OUT_FILE}");
    }
}

fn create_fqn_lookup(
    items: &Vec<GenerationItem>,
    uid_lookup: &UidLookup,
) -> (FqnLookup, FqnLookup) {
    let mut gen3_lookup = HashMap::new();
    let mut enum_lookup = HashMap::new();

    for item in items {
        let name = dis_gen_utils::format_type_name(&item.name());
        let fqn_name = format_full_qualified_name(item);
        gen3_lookup.entry(name.clone()).or_insert(fqn_name);
    }

    for uid in uid_lookup {
        let fqn_name = format!("crate::enumerations::{}", uid.1);
        enum_lookup.entry(uid.1.clone()).or_insert(fqn_name);
    }

    // FIXME remove these placeholder lookups when the normal stuff works
    gen3_lookup.insert("u8".to_string(), "u8".to_string());
    gen3_lookup.insert("u16".to_string(), "u16".to_string());

    (gen3_lookup, enum_lookup)
}

fn format_full_qualified_name(item: &GenerationItem) -> String {
    if item.is_pdu() {
        format!(
            "crate::{}::{}::{}",
            item.family(),
            dis_gen_utils::format_pdu_module_name(item.name().as_str()),
            dis_gen_utils::format_type_name(item.name().as_str())
        )
    } else if item.is_extension_record() {
        format!(
            "crate::{}::extension_records::{}",
            item.family(),
            dis_gen_utils::format_type_name(item.name().as_str())
        )
    } else {
        format!(
            "crate::{}::{}",
            item.family(),
            dis_gen_utils::format_type_name(item.name().as_str())
        )
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

    /// Returns true when the item is a `PDU`
    fn is_pdu(&self) -> bool {
        matches!(self, GenerationItem::Pdu(_, _))
    }

    /// Returns true when the item is an `ExtensionRecord`
    fn is_extension_record(&self) -> bool {
        matches!(self, GenerationItem::ExtensionRecord(_, _))
    }

    /// Returns true when the item is not a PDU or an `ExtensionRecord`
    fn is_record(&self) -> bool {
        !self.is_pdu() && !self.is_extension_record()
    }
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
enum PduAndFixedRecordFieldsEnum {
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
    pub fields: Vec<PduAndFixedRecordFieldsEnum>,
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
    pub variants: Vec<AdaptiveFormatEnum>,
    pub record_type: String,
    pub length: usize,
    pub discriminant_start_value: usize,
}

#[derive(Debug, Clone)]
enum AdaptiveFormatEnum {
    #[allow(dead_code)]
    Numeric(NumericField),
    #[allow(dead_code)]
    Enum(EnumField),
    #[allow(dead_code)]
    FixedString(FixedStringField),
    #[allow(dead_code)]
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
    #[allow(dead_code)]
    PaddingTo16,
    #[allow(dead_code)]
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
    pub fields: Vec<PduAndFixedRecordFieldsEnum>,
    pub extension_record_set: ExtensionRecordSet,
}

#[cfg(test)]
mod tests {
    use crate::{CountField, ExtensionRecordSet, FixedRecordField, GenerationItem, Pdu};

    #[test]
    fn format_full_qualified_name() {
        let pdu = GenerationItem::Pdu(
            Pdu {
                name_attr: "Entity State".to_string(),
                type_attr: 0,
                protocol_family_attr: 0,
                base_length_attr: 0,
                header_field: FixedRecordField {
                    name: String::new(),
                    length: 0,
                    field_type: String::new(),
                },
                fields: vec![],
                extension_record_set: ExtensionRecordSet {
                    count_field: CountField {
                        name: String::new(),
                        primitive_type: String::new(),
                    },
                },
            },
            "entity_info_interaction".to_string(),
        );
        let pdu_fqn = crate::format_full_qualified_name(&pdu);
        assert_eq!(
            pdu_fqn.as_str(),
            "crate::entity_info_interaction::entity_state::EntityState"
        );
    }
}
