use extraction::ExtractionItem;
use std::collections::HashMap;
use std::env;

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

        let (fqn_lookup, enum_fqn_lookup) = pre_processing::create_fqn_lookup(&generation_items, &uid_lookup);

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

mod pre_processing {
    use std::collections::HashMap;
    use dis_gen_utils::{enum_type_to_field_type, format_field_name, format_type_name};
    use crate::extraction::ExtractionItem;
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

    pub(crate) fn field_size_to_primitive_type(size: usize) -> syn::Type {
        let field_type = if size > 64 {
            "u128"
        } else if size <= 64 && size > 32 {
            "u64"
        } else if size <= 32 && size > 16 {
            "u32"
        } else if size <= 16 && size > 8 {
            "u16"
        } else {
            "u8"
        };
        syn::parse_str(field_type)
            .unwrap_or_else(|_| panic!("Expected a valid Rust primitive type for a bit field declaration with a given size, found size {size}."))
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
        syn::parse_str(fqn_field_type).unwrap_or_else(|_| {
            panic!(
                "Expected a valid Type for a FixedRecordField declaration, found '{fqn_field_type}'."
            )
        })
    }

    #[inline]
    fn type_for_bit_record_field(field: &crate::extraction::BitRecordField, lookup: &Lookup) -> syn::Type {
        if let Some(uids) = &field.enum_uid {
            let enum_type = lookup_first_uid(&uids, lookup);
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
            let enum_type = lookup_first_uid(&uids, lookup);
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

    pub fn process_extracted(extracts: &[ExtractionItem], lookup: &Lookup) -> Vec<GenerationItem> {
        extracts.iter().map(|e| {
            match e {
                ExtractionItem::Pdu(pdu, s) => { GenerationItem::Pdu(process_pdu(pdu, lookup), s.clone()) }
                ExtractionItem::FixedRecord(record, s) => { GenerationItem::FixedRecord(process_fixed_record(record, lookup), s.clone()) }
                ExtractionItem::BitRecord(record, s) => { GenerationItem::BitRecord(process_bit_record(record, lookup), s.clone()) }
                ExtractionItem::AdaptiveRecord(record, s) => { GenerationItem::AdaptiveRecord(process_adaptive_record(record, lookup), s.clone()) }
                ExtractionItem::ExtensionRecord(record, s) => { GenerationItem::ExtensionRecord(process_extension_record(record, lookup), s.clone()) }
            }
        }).collect()
    }

    fn process_numeric_field(field: &crate::extraction::NumericField) -> crate::generation::NumericField {
        crate::generation::NumericField {
            field_name: format_field_name(&field.name),
            primitive_type: type_for_primitive_type_field(&field.primitive_type),
            units: field.units.clone(),
            is_padding: must_skip_field_decl(&field.name),
        }
    }

    fn process_count_field(field: &crate::extraction::CountField) -> crate::generation::CountField {
        crate::generation::CountField {
            field_name: format_field_name(&field.name),
            primitive_type: type_for_primitive_type_field(&field.primitive_type),
        }
    }

    fn process_enum_field(field: &crate::extraction::EnumField, lookup: &Lookup) -> crate::generation::EnumField {
        crate::generation::EnumField {
            field_name: format_field_name(&field.name),
            field_type_fqn: type_for_enum_field(&field, lookup),
            is_discriminant: field.is_discriminant.unwrap_or(false),
        }
    }

    fn process_fixed_string_field(field: &crate::extraction::FixedStringField) -> crate::generation::FixedStringField {
        crate::generation::FixedStringField {
            field_name: format_field_name(&field.name),
            field_type: "String",
            length: field.length,
        }
    }

    fn process_int_bitfield(field: &crate::extraction::IntBitField) -> crate::generation::IntBitField {
        crate::generation::IntBitField {
            field_name: format_field_name(&field.name),
            field_type: field_size_to_primitive_type(field.size.unwrap_or(1)),
            bit_position: field.bit_position,
            size: field.size.unwrap_or(1),
            units: field.units.clone(),
            is_padding: must_skip_field_decl(&field.name),
        }
    }

    fn process_enum_bitfield(field: &crate::extraction::EnumBitField, lookup: &Lookup) -> crate::generation::EnumBitField {
        let (field_type, field_type_fqn) = match (field.size, &field.enum_uid) {
            (Some(size), None) => {
                let primitive = field_size_to_primitive_type(size);
                (primitive.clone(), primitive)
            }
            (_, Some(uids)) => {
                let field_type = lookup_first_uid(uids, lookup);
                let ty: syn::Type = syn::parse_str(field_type)
                    .expect("Expected a valid Type for an EnumBitField declaration.");
                (ty, syn::parse_str(lookup_enum_fqn(field_type, lookup)).expect("Expected a valid Enum Type"))
            }
            (None, None) => {
                panic!("EnumBitField neither has a size or an enumTableUid attribute");
            }
        };
        crate::generation::EnumBitField {
            field_name: format_field_name(&field.name),
            field_type,
            field_type_fqn,
            bit_position: field.bit_position,
            size: field.size.unwrap_or(1),
            is_discriminant: field.is_discriminant.unwrap_or(false),
        }
    }

    fn process_bool_bitfield(field: &crate::extraction::BoolBitField) -> crate::generation::BoolBitField {
        crate::generation::BoolBitField {
            field_name: format_field_name(&field.name),
            bit_position: field.bit_position,
        }
    }

    fn process_fixed_record_field(field: &crate::extraction::FixedRecordField, lookup: &Lookup) -> crate::generation::FixedRecordField {
        crate::generation::FixedRecordField {
            field_name: format_field_name(&field.name),
            field_type_fqn: type_for_fixed_record_field(&field, lookup),
            length: field.length,
        }
    }

    fn process_bit_record_field(field: &crate::extraction::BitRecordField, lookup: &Lookup) -> crate::generation::BitRecordField {
        crate::generation::BitRecordField {
            field_name: format_field_name(&field.name),
            field_type_fqn: type_for_bit_record_field(field, lookup),
            size: field.size,
        }
    }

    fn process_adaptive_record_field(field: &crate::extraction::AdaptiveRecordField, lookup: &Lookup) -> crate::generation::AdaptiveRecordField {
        crate::generation::AdaptiveRecordField {
            field_name: format_field_name(&field.name),
            field_type_fqn: type_for_adaptive_record_field(field, lookup),
            length: field.length,
            discriminant_field_name: format_field_name(&field.discriminant),
        }
    }

    fn process_variable_string_field(field: &crate::extraction::VariableStringField) -> crate::generation::VariableStringField {
        crate::generation::VariableStringField {
            field_name: format_field_name(&field.name),
            field_type: "String",
            fixed_number_of_strings: field.fixed_number_of_strings.unwrap_or(0),
        }
    }

    fn process_variable_string(element: &crate::extraction::VariableString) -> crate::generation::VariableString {
        crate::generation::VariableString {
            count_field: process_count_field(&element.count_field),
            string_field: process_variable_string_field(&element.string_field),
        }
    }

    fn process_opaque_data_field(field: &crate::extraction::OpaqueDataField) -> crate::generation::OpaqueDataField {
        crate::generation::OpaqueDataField {
            field_name: format_field_name(&field.name),
            field_type: "Vec<u8>",
        }
    }

    fn process_array(array: &crate::extraction::Array, lookup: &Lookup) -> crate::generation::Array {
        crate::generation::Array {
            count_field: process_count_field(&array.count_field),
            type_field: process_array_field_enum(&array.type_field, lookup),
        }
    }

    fn process_opaque_data(o: &crate::extraction::OpaqueData) -> crate::generation::OpaqueData {
        crate::generation::OpaqueData {
            count_field: process_count_field(&o.count_field),
            opaque_data_field: process_opaque_data_field(&o.opaque_data_field),
        }
    }

    fn process_fixed_record(record: &crate::extraction::FixedRecord, lookup: &Lookup) -> crate::generation::FixedRecord {
        crate::generation::FixedRecord {
            fields: record.fields.iter().map(|field| process_pdu_fixed_record_fields_enum(field, lookup) ).collect(),
            record_type: syn::parse_str(&format_type_name(&record.record_type)).expect("Expected a valid FixedRecord Type"),
            record_type_fqn: syn::parse_str(lookup_fqn(&record.record_type, lookup)).expect("Expected a valid FQN FixedRecord Type"),
            length: record.length,
        }
    }

    fn process_bit_record(record: &crate::extraction::BitRecord, lookup: &Lookup) -> crate::generation::BitRecord {
        crate::generation::BitRecord {
            fields: record.fields.iter().map(|field| process_bit_record_field_enum(field, lookup) ).collect(),
            record_type: syn::parse_str(&format_type_name(&record.record_type)).expect("Expected a valid BitRecord Type"),
            record_type_fqn: syn::parse_str(lookup_fqn(&record.record_type, lookup)).expect("Expected a valid FQN FixedRecord Type"),
            size: record.size,
        }
    }

    fn process_adaptive_record(record: &crate::extraction::AdaptiveRecord, lookup: &Lookup) -> crate::generation::AdaptiveRecord {
        crate::generation::AdaptiveRecord {
            variants: record.variants.iter().map(|variant| process_adaptive_format_enum(variant, lookup) ).collect(),
            record_type: syn::parse_str(&format_type_name(&record.record_type)).expect("Expected a valid AdaptiveRecord Type"),
            record_type_fqn: syn::parse_str(lookup_fqn(&record.record_type, lookup)).expect("Expected a valid FQN AdaptiveRecord Type"),
            length: record.length,
            discriminant_start_value: record.discriminant_start_value,
        }
    }

    fn process_extension_record_set(record: &crate::extraction::ExtensionRecordSet) -> crate::generation::ExtensionRecordSet {
        crate::generation::ExtensionRecordSet {
            count_field: process_count_field(&record.count_field)
        }
    }

    fn process_extension_record(record: &crate::extraction::ExtensionRecord, lookup: &Lookup) -> crate::generation::ExtensionRecord {
        crate::generation::ExtensionRecord {
            record_name: format_type_name(&record.name_attr),
            record_name_fqn: syn::parse_str(lookup_fqn(&record.name_attr, lookup)).expect("Expected a valid FQN ExtensionRecord Type"),
            record_type_enum: record.record_type_attr,
            base_length: record.base_length_attr,
            is_variable: record.is_variable_attr,
            record_type_field: process_enum_field(&record.record_type_field, lookup),
            record_length_field: process_numeric_field(&record.record_length_field),
            fields: vec![],
            padding_to_64_field: record.padding_to_64_field.map(|p| process_padding_64(&p)),
        }
    }

    fn process_pdu(pdu: &crate::extraction::Pdu, lookup: &Lookup) -> crate::generation::Pdu {
        crate::generation::Pdu {
            pdu_name: format_type_name(&pdu.name_attr),
            pdu_name_fqn: syn::parse_str(lookup_fqn(&pdu.name_attr, lookup)).expect("Expected a valid FQN PDU Type"),
            pdu_type: pdu.type_attr,
            protocol_family: pdu.protocol_family_attr,
            base_length: pdu.base_length_attr,
            header_field: process_fixed_record_field(&pdu.header_field, lookup),
            fields: pdu.fields.iter().map(|field| process_pdu_fixed_record_fields_enum(field, lookup) ).collect(),
            extension_record_set: process_extension_record_set(&pdu.extension_record_set),
        }
    }

    fn process_padding_16(_pad: &crate::extraction::PaddingTo16) -> crate::generation::PaddingTo16 {
        crate::generation::PaddingTo16
    }

    fn process_padding_32(_pad: &crate::extraction::PaddingTo32) -> crate::generation::PaddingTo32 {
        crate::generation::PaddingTo32
    }

    fn process_padding_64(_pad: &crate::extraction::PaddingTo64) -> crate::generation::PaddingTo64 {
        crate::generation::PaddingTo64
    }

    fn process_pdu_fixed_record_fields_enum(e: &crate::extraction::PduAndFixedRecordFieldsEnum, lookup: &Lookup) -> crate::generation::PduAndFixedRecordFieldsEnum {
        match e {
            crate::extraction::PduAndFixedRecordFieldsEnum::Numeric(f) => { crate::generation::PduAndFixedRecordFieldsEnum::Numeric(process_numeric_field(f)) }
            crate::extraction::PduAndFixedRecordFieldsEnum::Enum(f) => { crate::generation::PduAndFixedRecordFieldsEnum::Enum(process_enum_field(f, lookup)) }
            crate::extraction::PduAndFixedRecordFieldsEnum::FixedString(f) => { crate::generation::PduAndFixedRecordFieldsEnum::FixedString(process_fixed_string_field(f)) }
            crate::extraction::PduAndFixedRecordFieldsEnum::FixedRecord(f) => { crate::generation::PduAndFixedRecordFieldsEnum::FixedRecord(process_fixed_record_field(f, lookup)) }
            crate::extraction::PduAndFixedRecordFieldsEnum::BitRecord(f) => { crate::generation::PduAndFixedRecordFieldsEnum::BitRecord(process_bit_record_field(f, lookup)) }
            crate::extraction::PduAndFixedRecordFieldsEnum::AdaptiveRecord(f) => { crate::generation::PduAndFixedRecordFieldsEnum::AdaptiveRecord(process_adaptive_record_field(f, lookup)) }
        }
    }

    fn process_bit_record_field_enum(e: &crate::extraction::BitRecordFieldEnum, lookup: &Lookup) -> crate::generation::BitRecordFieldEnum {
        match e {
            crate::extraction::BitRecordFieldEnum::Enum(f) => { crate::generation::BitRecordFieldEnum::Enum(process_enum_bitfield(f, lookup)) }
            crate::extraction::BitRecordFieldEnum::Int(f) => { crate::generation::BitRecordFieldEnum::Int(process_int_bitfield(f)) }
            crate::extraction::BitRecordFieldEnum::Bool(f) => { crate::generation::BitRecordFieldEnum::Bool(process_bool_bitfield(f)) }
        }
    }

    fn process_adaptive_format_enum(e: &crate::extraction::AdaptiveFormatEnum, lookup: &Lookup) -> crate::generation::AdaptiveFormatEnum {
        match e {
            crate::extraction::AdaptiveFormatEnum::Numeric(f) => { crate::generation::AdaptiveFormatEnum::Numeric(process_numeric_field(f)) }
            crate::extraction::AdaptiveFormatEnum::Enum(f) => { crate::generation::AdaptiveFormatEnum::Enum(process_enum_field(f, lookup)) }
            crate::extraction::AdaptiveFormatEnum::FixedString(f) => { crate::generation::AdaptiveFormatEnum::FixedString(process_fixed_string_field(f)) }
            crate::extraction::AdaptiveFormatEnum::FixedRecord(f) => { crate::generation::AdaptiveFormatEnum::FixedRecord(process_fixed_record_field(f, lookup)) }
            crate::extraction::AdaptiveFormatEnum::BitRecord(f) => { crate::generation::AdaptiveFormatEnum::BitRecord(process_bit_record_field(f, lookup)) }
        }
    }

    fn process_array_field_enum(e: &crate::extraction::ArrayFieldEnum, lookup: &Lookup) -> crate::generation::ArrayFieldEnum {
        match e {
            crate::extraction::ArrayFieldEnum::Numeric(f) => { crate::generation::ArrayFieldEnum::Numeric(process_numeric_field(f)) }
            crate::extraction::ArrayFieldEnum::Enum(f) => { crate::generation::ArrayFieldEnum::Enum(process_enum_field(f, lookup)) }
            crate::extraction::ArrayFieldEnum::FixedString(f) => { crate::generation::ArrayFieldEnum::FixedString(process_fixed_string_field(f)) }
            crate::extraction::ArrayFieldEnum::FixedRecord(f) => { crate::generation::ArrayFieldEnum::FixedRecord(process_fixed_record_field(f, lookup)) }
            crate::extraction::ArrayFieldEnum::BitRecord(f) => { crate::generation::ArrayFieldEnum::BitRecord(process_bit_record_field(f, lookup)) }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::field_size_to_primitive_type;
        use std::any::Any;

        #[test]
        fn test_int_bit_field_type_to_primitive() {
            let type_u8: syn::Type = syn::parse_str("u8").unwrap();
            let type_u16: syn::Type = syn::parse_str("u16").unwrap();
            let type_u32: syn::Type = syn::parse_str("u32").unwrap();
            let type_u64: syn::Type = syn::parse_str("u64").unwrap();
            let type_u128: syn::Type = syn::parse_str("u128").unwrap();

            assert_eq!(field_size_to_primitive_type(4).type_id(), type_u8.type_id());
            assert_eq!(field_size_to_primitive_type(8).type_id(), type_u8.type_id());
            assert_eq!(
                field_size_to_primitive_type(9).type_id(),
                type_u16.type_id()
            );
            assert_eq!(
                field_size_to_primitive_type(16).type_id(),
                type_u16.type_id()
            );
            assert_eq!(
                field_size_to_primitive_type(17).type_id(),
                type_u32.type_id()
            );
            assert_eq!(
                field_size_to_primitive_type(32).type_id(),
                type_u32.type_id()
            );
            assert_eq!(
                field_size_to_primitive_type(33).type_id(),
                type_u64.type_id()
            );
            assert_eq!(
                field_size_to_primitive_type(100).type_id(),
                type_u128.type_id()
            );
        }
    }
}