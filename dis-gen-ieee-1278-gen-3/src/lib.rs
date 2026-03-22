use extraction::ExtractionItem;
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
            .collect::<Vec<ExtractionItem>>();

        let (fqn_lookup, enum_fqn_lookup) = intermediate_processing::create_fqn_lookup(&generation_items, &uid_lookup);

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
        let pdu_fqn = crate::format_full_qualified_name(&pdu);
        assert_eq!(
            pdu_fqn.as_str(),
            "crate::entity_info_interaction::entity_state::EntityState"
        );
    }
}

mod intermediate_processing {
    use std::collections::HashMap;
    use dis_gen_utils::{enum_type_to_field_type, format_field_name, format_type_name};
    use crate::extraction::{EnumField, ExtractionItem};
    use crate::generation::GenerationItem;
    use crate::{FqnLookup, Lookup, UidLookup};

    pub(crate) fn create_fqn_lookup(
        items: &[ExtractionItem],
        uid_lookup: &UidLookup,
    ) -> (FqnLookup, FqnLookup) {
        let mut gen3_lookup = HashMap::new();
        let mut enum_lookup = HashMap::new();

        for item in items {
            let name = format_type_name(&item.name());
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

    fn format_full_qualified_name(item: &ExtractionItem) -> String {
        if item.is_pdu() {
            format!(
                "crate::{}::{}::{}",
                item.family(),
                dis_gen_utils::format_pdu_module_name(item.name().as_str()),
                format_type_name(item.name().as_str())
            )
        } else if item.is_extension_record() {
            format!(
                "crate::{}::extension_records::{}",
                item.family(),
                format_type_name(item.name().as_str())
            )
        } else {
            format!(
                "crate::{}::{}",
                item.family(),
                format_type_name(item.name().as_str())
            )
        }
    }

    fn lookup_first_uid<'l>(uids: &[usize], lookup: &'l Lookup) -> &'l str {
        let the_type = uids
            .iter()
            .map(|uid| lookup_uid(*uid, lookup).to_string())
            .collect::<Vec<String>>();
        let the_type = the_type
            .first()
            .expect("Expected at least one Type for an UID lookup.");
        lookup_fqn(the_type, lookup)
    }

    #[inline]
    fn lookup_uid(uid: usize, lookup: &Lookup) -> &str {
        let val = lookup
            .uid
            .get(&uid)
            .expect("Expected an existing type for uid.");
        val
    }

    #[inline]
    fn lookup_fqn<'fqn>(type_name: &str, lookup: &'fqn Lookup) -> &'fqn str {
        if let Some(fqn) = lookup.fqn.get(type_name) {
            fqn
        } else if let Some(fqn) = lookup.enum_fqn.get(type_name) {
            fqn
        } else {
            panic!("Expected full qualified name for type '{type_name}'")
        }
    }

    fn lookup_enum_fqn<'fqn>(type_name: &str, lookup: &'fqn Lookup) -> &'fqn str {
        lookup.enum_fqn.get(type_name).unwrap_or_else(|| {
            panic!("Expected full qualified enumeration name for type '{type_name}'")
        })
    }

    #[inline]
    fn type_for_primitive_type_field(primitive_type: &str) -> syn::Type {
        let ty = match primitive_type {
            "uint8" => "u8",
            "uint16" => "u16",
            "uint32" => "u32",
            "uint64" => "u64",
            "int8" => "i8",
            "int16" => "i16",
            "int32" => "i32",
            "int64" => "i64",
            "float32" => "f32",
            "float64" => "f64",
            _ => panic!("Invalid primitive type in NumericField"),
        };
        syn::parse_str(ty).unwrap_or_else(|_| {
            panic!(
                "Expected a valid Type for a NumericField, found {primitive_type}."
            )}
        )
    }

    #[inline]
    fn type_for_enum_field(field: &crate::extraction::EnumField, lookup: &Lookup) -> syn::Type {
        if let Some(uids) = &field.enum_uid {
            let enum_type = uids
                .iter()
                .map(|uid| lookup_uid(*uid, lookup).to_string())
                .collect::<Vec<String>>();
            let enum_type = enum_type
                .first()
                .expect("Expected at least one type for an EnumField declaration.");
            let enum_type = lookup_enum_fqn(enum_type, lookup);
            let ty: syn::Type =
                syn::parse_str(enum_type).expect("Expected a valid type for an EnumField declaration.");
            ty
        } else {
            let ty: syn::Type = syn::parse_str(
                &enum_type_to_field_type(&field.field_type)
                    .expect("Expected a valid type for an EnumField declaration."),
            )
                .expect("Expected valid input to parse a Type for an EnumField declaration.");
            ty
        }
    }

    #[inline]
    fn type_for_fixed_string_field() -> syn::Type {
        syn::parse_str("String").expect("Expected valid type 'String' for FixedString field.")
    }

    #[inline]
    fn type_for_fixed_record_field(field: &crate::extraction::FixedRecordField, lookup: &Lookup) -> syn::Type {
        let fqn_field_type = lookup_fqn(format_type_name(&field.field_type).as_str(), lookup);
        fqn_field_type).unwrap_or_else(|_| {
            panic!(
                "Expected a valid Type for a FixedRecordField declaration, found '{fqn_field_type}'."
            )
        })
    }

    #[inline]
    fn type_for_bit_record_field(field: &crate::extraction::BitRecordField, lookup: &Lookup) -> syn::Type {
        if let Some(uids) = &field.enum_uid {
            let enum_type = uids
                .iter()
                .map(|uid| lookup_uid(*uid, lookup).to_string())
                .collect::<Vec<String>>();
            let enum_type = enum_type
                .first()
                .expect("Expected at least one Type for a BitRecordField declaration.");
            let enum_type = lookup_enum_fqn(enum_type, lookup);
            let ty: syn::Type = syn::parse_str(enum_type)
                .expect("Expected a valid Type for a BitRecordField declaration.");
            ty
        } else {
            let ty: syn::Type = syn::parse_str(lookup_fqn(
                format_type_name(field.field_type.as_ref().expect(
                    "Expected a type name for BitRecordField to be present as there is also no UID.",
                ))
                    .as_str(),
                lookup,
            ))
                .expect("Expected valid input to parse a Type for a BitRecordField declaration.");
            ty
        }
    }

    #[inline]
    fn type_for_adaptive_record_field(field: &crate::extraction::AdaptiveRecordField, lookup: &Lookup) -> syn::Type {
        if let Some(uids) = &field.enum_uid {
            let enum_type = uids
                .iter()
                .map(|uid| lookup_uid(*uid, lookup).to_string())
                .collect::<Vec<String>>();
            let enum_type = enum_type
                .first()
                .expect("Expected at least one Type for an AdaptiveRecordField declaration.");
            let enum_type = lookup_enum_fqn(enum_type, lookup);
            let ty: syn::Type = syn::parse_str(enum_type)
                .expect("Expected a valid Type for an AdaptiveRecordField declaration.");
            ty
        } else {
            let ty: syn::Type = syn::parse_str(lookup_fqn(format_type_name(field.field_type.as_ref().expect(
                "Expected a type name for AdaptiveRecordField to be present as there is also no UID.",
            )).as_str(), lookup))
                .expect("Expected valid input to parse a Type for a AdaptiveRecordField declaration.");
            ty
        }
    }

    #[inline]
    fn must_skip_field_decl(field_name: &str) -> bool {
        ["Padding", "Padding1", "Padding2", "Not used"].contains(&field_name)
    }

    pub fn process_extracted(extracts: &[ExtractionItem]) -> Vec<GenerationItem> {
        todo!()
    }

    fn process_numeric_field(field: crate::extraction::NumericField) -> crate::generation::NumericField {
        crate::generation::NumericField {
            field_name: format_field_name(&field.name),
            primitive_type: type_for_primitive_type_field(&field.primitive_type),
            units: field.units,
            is_padding: must_skip_field_decl(&field.name),
        }
    }

    fn process_count_field(field: crate::extraction::CountField) -> crate::generation::CountField {
        crate::generation::CountField {
            field_name: format_field_name(&field.name),
            primitive_type: type_for_primitive_type_field(&field.primitive_type),
        }
    }

    fn process_enum_field(field: crate::extraction::EnumField, lookup: &Lookup) -> crate::generation::EnumField {
        crate::generation::EnumField {
            field_name: format_field_name(&field.name),
            field_type_fqn: type_for_enum_field(&field, lookup),
            is_discriminant: false,
        }
    }

    fn process_fixed_string_field(field: crate::extraction::FixedStringField) -> crate::generation::FixedStringField {
        crate::generation::FixedStringField {
            field_name: format_field_name(&field.name),
            field_type: "String",
            length: field.length,
        }
    }


}
