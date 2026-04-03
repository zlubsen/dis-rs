use extraction::ExtractionItem;
use std::collections::HashMap;
use std::env;

mod extraction;
mod generation;
mod pre_processing;

const XML_FILE_EXTENSION: &str = "xml";
const OUT_DIR: &str = "OUT_DIR";
const TARGET_ENV_VAR: &str = "TARGET_GENERATED_SISO_1278_GEN3_FILENAME";
const TARGET_OUT_FILE: &str = "siso_1278_gen3.rs";

type UidLookup = HashMap<usize, String>;
type FqnLookup = HashMap<String, String>;

struct Lookup {
    pdu_fqn: FqnLookup,
    er_fqn: FqnLookup,
    records_fqn: FqnLookup,
    enum_fqn: FqnLookup,
    uid: UidLookup,
    pdu_types: UidLookup,
    er_types: UidLookup,
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
pub fn execute(
    schema_dir: &str,
    uid_lookup: UidLookup,
    pdu_types_lookup: UidLookup,
    er_types_lookup: UidLookup,
) {
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
        let extracted_items = file_paths
            .iter()
            .flat_map(|path| {
                // The extractor extracts and formats the PDU Family name, which is also in the path.
                // But it is already formatted nicely this way.
                let (items, family) = extraction::extract_from_file(path);
                families.push(family);
                items
            })
            .collect::<Vec<ExtractionItem>>();

        let (pdu_fqn_lookup, er_fqn_lookup, records_fqn_lookup) = pre_processing::create_fqn_lookup(&extracted_items);
        let enum_fqn_lookup = pre_processing::create_enum_lookup(&uid_lookup);

        let lookup = Lookup {
            pdu_fqn: pdu_fqn_lookup,
            er_fqn: er_fqn_lookup,
            records_fqn: records_fqn_lookup,
            enum_fqn: enum_fqn_lookup,
            uid: uid_lookup,
            pdu_types: pdu_types_lookup,
            er_types: er_types_lookup,
        };

        // Pre-process all items (formatting, expanding of field, type, variant names)
        let generation_items = pre_processing::process_extracted(&extracted_items, &lookup);

        // Generate all items
        let generated = generation::models::generate(&generation_items, &families);

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

#[cfg(test)]
mod tests {
    use crate::extraction::{
        CountField, ExtensionRecordSet, ExtractionItem, FixedRecordField, Pdu,
    };

    #[test]
    fn format_full_qualified_name() {
        let pdu = ExtractionItem::Pdu(
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
        let pdu_fqn = crate::pre_processing::format_fqn_pdu(&pdu);
        assert_eq!(
            pdu_fqn.as_str(),
            "crate::entity_info_interaction::entity_state::EntityState"
        );
    }
}
