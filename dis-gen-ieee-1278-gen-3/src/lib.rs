mod extraction;
mod generation;
mod pre_processing;

use extraction::ExtractionItem;
use std::collections::HashMap;
use std::env;

const XML_FILE_EXTENSION: &str = "xml";
const OUT_DIR: &str = "OUT_DIR";
const TARGET_ENV_VAR: &str = "TARGET_GENERATED_SISO_1278_GEN3_FILENAME";
const TARGET_OUT_FILE: &str = "siso_1278_gen3.rs";

/// Hardcoded values for the discriminant type values of Adaptive Records.
/// Tuple consists of ( <type of the AdaptiveRecord> , <UID of the discriminant> )
const ADAPTIVE_RECORD_DISCRIMINANT_TYPES: [(&str, usize, &str); 2] = [
    ("Net ID", 590, "u8"),                                  // Net ID Type
    ("IFF Interactive Transmission Parameters", 372, "u8"), // Transmission Indicator
];

type UidLookup = HashMap<usize, String>;
type BodyTypeLookup = HashMap<usize, String>; // TODO merge with Uidlookup again
type FqnLookup = HashMap<String, Fqn>;

#[derive(PartialEq)]
struct Fqn {
    path: String,
    type_name: String,
    field_name: String,
}

impl Fqn {
    pub(crate) fn to_full_type(&self) -> String {
        if self.path.is_empty() {
            self.type_name.clone()
        } else {
            format!("{}::{}", self.path, self.type_name)
        }
    }
}

struct Lookup {
    pdu_fqn: FqnLookup,
    er_fqn: FqnLookup,
    records_fqn: FqnLookup,
    enum_fqn: FqnLookup,
    uid: UidLookup,
    pdu_types: BodyTypeLookup,
    er_types: BodyTypeLookup,
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
    pdu_types_lookup: BodyTypeLookup,
    er_types_lookup: BodyTypeLookup,
) {
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

        let (pdu_fqn_lookup, er_fqn_lookup, records_fqn_lookup) =
            pre_processing::create_fqn_lookup(&extracted_items);
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
