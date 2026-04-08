use crate::extraction::ExtractionItem;
use crate::generation::models::{
    GenerationItem, PduAndFixedRecordFieldsEnum, EXTENSION_RECORDS_MODULE_NAME, PARSER_MODULE_NAME,
};
use crate::{Fqn, FqnLookup, Lookup, UidLookup};
use dis_gen_utils::{
    enum_type_to_primitive_type, format_field_name, format_pdu_module_name, format_type_name,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

const NOM_LE_PARSER_PATH: &str = "nom::number::complete::le_";

pub(crate) fn create_fqn_lookup(items: &[ExtractionItem]) -> (FqnLookup, FqnLookup, FqnLookup) {
    let mut pdu_lookup = HashMap::new();
    let mut er_lookup = HashMap::new();
    let mut rec_lookup = HashMap::new();

    for item in items {
        let name = format_type_name(&item.name());
        let field_name = format_field_name(&item.name());
        match item {
            ExtractionItem::Pdu(_pdu, _) => {
                let path = format_fqn_path_pdu(item);
                pdu_lookup.entry(name.clone()).or_insert(Fqn {
                    path,
                    type_name: name,
                    field_name,
                    // data_size: Default::default(),
                });
            }
            ExtractionItem::ExtensionRecord(_, _) => {
                let path = format_fqn_path_extension_record(item);
                er_lookup.entry(name.clone()).or_insert(Fqn {
                    path,
                    type_name: name,
                    field_name,
                    // data_size: Default::default(),
                });
            }
            _ => {
                let path = format_fqn_path_record(item);
                rec_lookup.entry(name.clone()).or_insert(Fqn {
                    path,
                    type_name: name,
                    field_name,
                    // data_size: Default::default(),
                });
            }
        }
    }

    // FIXME remove these placeholder lookups when the normal stuff works
    rec_lookup.insert(
        "u8".to_string(),
        Fqn {
            path: String::default(),
            type_name: "u8".to_string(),
            field_name: String::default(),
            // data_size: Default::default(),
        },
    );
    rec_lookup.insert(
        "u16".to_string(),
        Fqn {
            path: String::default(),
            type_name: "u16".to_string(),
            field_name: String::default(),
            // data_size: Default::default(),
        },
    );

    (pdu_lookup, er_lookup, rec_lookup)
}

pub(crate) fn create_enum_lookup(uid_lookup: &UidLookup) -> FqnLookup {
    let mut enum_lookup = HashMap::new();

    for uid in uid_lookup {
        let name = uid.1.clone();
        let path = "crate::enumerations".to_string();
        enum_lookup.entry(uid.1.clone()).or_insert(Fqn {
            path,
            type_name: name,
            field_name: String::default(), // FIXME not so nice to have this here
        });
    }

    enum_lookup
}

pub(crate) fn format_fqn_path_pdu(item: &ExtractionItem) -> String {
    format!(
        "crate::{}::{}",
        item.family(),
        dis_gen_utils::format_pdu_module_name(item.name().as_str())
    )
}

pub(crate) fn format_fqn_path_extension_record(item: &ExtractionItem) -> String {
    format!("crate::{}::{EXTENSION_RECORDS_MODULE_NAME}", item.family())
}

pub(crate) fn format_fqn_path_record(item: &ExtractionItem) -> String {
    format!("crate::{}", item.family())
}

/// Looks up the Fqn path and type of the first enumeration UID in `uids`.
fn lookup_enum_fqn_first_uid<'l>(uids: &[usize], lookup: &'l Lookup) -> &'l Fqn {
    let the_type = uids
        .iter()
        .map(|uid| lookup_uid(*uid, lookup).to_string())
        .collect::<Vec<String>>();
    let the_type = the_type
        .first()
        .expect("Expected at least one Type for an UID lookup.");
    let fqn = lookup_enum_fqn(the_type, lookup);
    fqn
}

/// Looks up the type name of the Enumeration UID `uid`.
#[inline]
fn lookup_uid(uid: usize, lookup: &Lookup) -> &str {
    let val = lookup
        .uid
        .get(&uid)
        .expect("Expected an existing type for uid.");
    val
}

/// Look up the path and name for the given _Record_ (BitRecord, FixedRecord, AdaptiveRecord) type name.
#[inline]
fn lookup_record_fqn<'fqn>(type_name: &str, lookup: &'fqn Lookup) -> &'fqn Fqn {
    if let Some(fqn) = lookup.records_fqn.get(type_name) {
        fqn
    } else if let Some(fqn) = lookup.enum_fqn.get(type_name) {
        fqn
    } else {
        panic!("Expected full qualified path for type or enum '{type_name}'")
    }
}

/// Look up the path and name for the given _Extension Record_ type name.
fn lookup_er_fqn<'fqn>(er_name: &str, lookup: &'fqn Lookup) -> &'fqn Fqn {
    lookup
        .er_fqn
        .get(er_name)
        .unwrap_or_else(|| panic!("Expected full qualified path for Extension Record '{er_name}'"))
}

/// Look up the path and name for the given _PDU_ name.
fn lookup_pdu_fqn<'fqn>(pdu_name: &str, lookup: &'fqn Lookup) -> &'fqn Fqn {
    lookup
        .pdu_fqn
        .get(pdu_name)
        .unwrap_or_else(|| panic!("Expected full qualified path for PDU '{pdu_name}'"))
}

/// Look up the path for the given _Enumeration_ type name.
fn lookup_enum_fqn<'fqn>(type_name: &str, lookup: &'fqn Lookup) -> &'fqn Fqn {
    lookup.enum_fqn.get(type_name).unwrap_or_else(|| {
        panic!("Expected full qualified enumeration path for type '{type_name}'")
    })
}

/// Lookup the name for the `ExtensionRecordTypes` variant with UID `uid` (UID 99)
fn lookup_er_type(uid: usize, lookup: &Lookup) -> &str {
    lookup.er_types.get(&uid).unwrap_or_else(|| {
        panic!("Expected a ExtensionRecord name for Record Type Enum value {uid}")
    })
}

/// Lookup the name for the `DIS-PDUType` variant with UID `uid` (UID 4)
fn lookup_pdu_type(uid: usize, lookup: &Lookup) -> &str {
    lookup
        .pdu_types
        .get(&uid)
        .unwrap_or_else(|| panic!("Expected a PDU name for DIS-PDU Type Enum value {uid}"))
}

/// Helper function to create a `TokenStream` from a `&str`.
///
/// # Panics
/// The passed `token_string` value must be valid Rust code, otherwise the function panics.
pub(crate) fn to_tokens(token_string: &str) -> TokenStream {
    token_string
        .parse()
        .expect(format!("Could not parse TokenStream '{token_string}'").as_str())
}

pub(crate) fn finalise_type<'a>(path: &'a TokenStream, name: &'a TokenStream) -> TokenStream {
    if path.is_empty() {
        quote! { #name }
    } else {
        quote! { #path::#name }
    }
}

/// Maps the DIS schema primitive data types to Rust primitive data types.
fn type_for_primitive_type_field(dis_primitive_type: &str) -> &'static str {
    match dis_primitive_type {
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
        _ => panic!("Invalid primitive type '{dis_primitive_type}'"),
    }
}

/// Maps DIS field sizes (in number of bits) to Rust primitive data types.
pub(crate) fn field_size_to_primitive_type(size: usize) -> &'static str {
    if size > 64 {
        "u128"
    } else if size <= 64 && size > 32 {
        "u64"
    } else if size <= 32 && size > 16 {
        "u32"
    } else if size <= 16 && size > 8 {
        "u16"
    } else {
        "u8"
    }
}

/// Maps DIS field lengths (in number of bytes) to Rust primitive data types.
fn field_length_to_primitive(length: usize) -> &'static str {
    match length {
        1 => "u8",
        2 => "u16",
        4 => "u32",
        _ => panic!("Invalid length, cannot convert to a Rust primitive type"),
    }
}

#[derive(PartialEq)]
enum EnumFieldType<'e, 'p> {
    Enum(&'e Fqn),
    Primitive(&'p str),
}

/// Determine the Type of an Enum field. Presence of a UID has precedence over a primitive type attribute.
/// Returns the either the found Enum `&Fqn` type data or a primitive type (as `&str`).
#[inline]
fn type_for_enum_field<'l>(
    field: &crate::extraction::EnumField,
    lookup: &'l Lookup,
) -> EnumFieldType<'l, 'static> {
    if let Some(uids) = &field.enum_uid {
        let fqn = lookup_enum_fqn_first_uid(&uids, lookup);
        EnumFieldType::Enum(fqn)
    } else {
        let ty = enum_type_to_primitive_type(&field.field_type)
            .expect("Expected a valid type for an EnumField declaration.");
        EnumFieldType::Primitive(ty)
    }
}

enum BitRecordFieldType<'l> {
    Enum(&'l Fqn),
    Record(&'l Fqn),
}

/// Determine the Type of a Bit Record field. Presence of a UID has precedence over a Bit Record type.
/// Returns the either the found Enum `&Fqn` type data or a Bit Record `&Fqn` type data.
#[inline]
fn type_for_bit_record_field<'l>(
    field: &crate::extraction::BitRecordField,
    lookup: &'l Lookup,
) -> BitRecordFieldType<'l> {
    if let Some(uids) = &field.enum_uid {
        let fqn = lookup_enum_fqn_first_uid(uids, lookup);
        BitRecordFieldType::Enum(fqn)
    } else {
        let field_fqn = lookup_record_fqn(
            &format_type_name(field.field_type.as_ref().expect(
                "Expected a type name for BitRecordField to be present as there is also no UID.",
            )),
            lookup,
        );
        BitRecordFieldType::Record(field_fqn)
    }
}

/// Determine and retrieve the associated `Fqn` data for an Adaptive Record field
#[inline]
fn type_for_adaptive_record_field<'l>(
    field: &crate::extraction::AdaptiveRecordField,
    lookup: &'l Lookup,
) -> &'l Fqn {
    if let Some(uids) = &field.enum_uid {
        lookup_enum_fqn_first_uid(&uids, lookup)
    } else {
        let field_fqn = lookup_record_fqn(format_type_name(field.field_type.as_ref().expect(
            "Expected a type name for AdaptiveRecordField to be present as there is also no UID.",
        )).as_str(), lookup);
        field_fqn
    }
}

/// Returns true when the `field_name` contains name patterns that indicate the field is not used or used as padding.
#[inline]
fn must_skip_field_decl(field_name: &str) -> bool {
    ["Padding", "Padding1", "Padding2", "Not used"].contains(&field_name)
}

pub fn process_extracted(extracts: &[ExtractionItem], lookup: &Lookup) -> Vec<GenerationItem> {
    extracts
        .iter()
        .map(|e| match e {
            ExtractionItem::Pdu(pdu, family) => {
                GenerationItem::Pdu(process_pdu(pdu, lookup), family.clone())
            }
            ExtractionItem::FixedRecord(record, family) => {
                GenerationItem::FixedRecord(process_fixed_record(record, lookup), family.clone())
            }
            ExtractionItem::BitRecord(record, family) => {
                GenerationItem::BitRecord(process_bit_record(record, lookup), family.clone())
            }
            ExtractionItem::AdaptiveRecord(record, family) => GenerationItem::AdaptiveRecord(
                process_adaptive_record(record, lookup),
                family.clone(),
            ),
            ExtractionItem::ExtensionRecord(record, family) => GenerationItem::ExtensionRecord(
                process_extension_record(record, lookup),
                family.clone(),
            ),
        })
        .collect()
}

fn process_numeric_field(
    field: &crate::extraction::NumericField,
) -> crate::generation::models::NumericField {
    let field_primitive_type = type_for_primitive_type_field(&field.primitive_type);
    crate::generation::models::NumericField {
        field_name: format_field_name(&field.name),
        primitive_type: to_tokens(field_primitive_type),
        units: field.units.clone(),
        is_padding: must_skip_field_decl(&field.name),
        parser_function: to_tokens(format!("{NOM_LE_PARSER_PATH}{field_primitive_type}").as_str()),
    }
}

fn process_count_field(
    field: &crate::extraction::CountField,
) -> crate::generation::models::CountField {
    let field_primitive_type = type_for_primitive_type_field(&field.primitive_type);
    crate::generation::models::CountField {
        field_name: format_field_name(&field.name),
        primitive_type: to_tokens(field_primitive_type),
        parser_function: to_tokens(format!("{NOM_LE_PARSER_PATH}{field_primitive_type}").as_str()),
    }
}

fn process_enum_field(
    field: &crate::extraction::EnumField,
    lookup: &Lookup,
) -> crate::generation::models::EnumField {
    let field_name = format_field_name(&field.name);
    let field_type = type_for_enum_field(field, lookup);
    let (type_path, type_name, parser_must_convert_to_enum) = match field_type {
        EnumFieldType::Enum(fqn) => (to_tokens(&fqn.path), to_tokens(&fqn.type_name), true),
        EnumFieldType::Primitive(fqn) => (quote! {}, to_tokens(fqn), false),
    };
    println!(
        "enumfield: {:?}, {:?}",
        type_path.to_string(),
        type_name.to_string()
    );
    let enum_data_size = format_ident!(
        "{}",
        enum_type_to_primitive_type(&field.field_type).expect("Expected a valid enum data size")
    );
    let parser_function = to_tokens(&format!("{NOM_LE_PARSER_PATH}{enum_data_size}"));

    crate::generation::models::EnumField {
        field_name,
        type_name,
        type_path,
        is_discriminant: field.is_discriminant.unwrap_or(false),
        parser_function,
        parser_must_convert_to_enum,
    }
}

fn process_fixed_string_field(
    field: &crate::extraction::FixedStringField,
) -> crate::generation::models::FixedStringField {
    crate::generation::models::FixedStringField {
        field_name: format_field_name(&field.name),
        field_type: "String",
        length: field.length,
        parser_function: to_tokens(&format!(
            "crate::parser::fixed_string_with_length({} as usize)",
            field.length
        )),
    }
}

// TODO parser impl and pre-processing
fn process_int_bitfield(
    field: &crate::extraction::IntBitField,
) -> crate::generation::models::IntBitField {
    crate::generation::models::IntBitField {
        field_name: format_field_name(&field.name),
        field_type: to_tokens(field_size_to_primitive_type(field.size.unwrap_or(1))),
        bit_position: field.bit_position,
        size: field.size.unwrap_or(1),
        units: field.units.clone(),
        is_padding: must_skip_field_decl(&field.name),
    }
}

// TODO parser impl and pre-processing
fn process_enum_bitfield(
    field: &crate::extraction::EnumBitField,
    lookup: &Lookup,
) -> crate::generation::models::EnumBitField {
    let (type_path, type_name) = match (field.size, &field.enum_uid) {
        (Some(size), None) => {
            let primitive = to_tokens(field_size_to_primitive_type(size));
            (quote! {}, primitive)
        }
        (_, Some(uids)) => {
            let fqn = lookup_enum_fqn_first_uid(uids, lookup);
            (to_tokens(&fqn.path), to_tokens(&fqn.type_name))
        }
        (None, None) => {
            panic!("EnumBitField neither has a size or an enumTableUid attribute");
        }
    };

    crate::generation::models::EnumBitField {
        field_name: format_field_name(&field.name),
        type_name,
        type_path,
        bit_position: field.bit_position,
        size: field.size.unwrap_or(1),
        is_discriminant: field.is_discriminant.unwrap_or(false),
    }
}

// TODO parser impl and pre-processing
fn process_bool_bitfield(
    field: &crate::extraction::BoolBitField,
) -> crate::generation::models::BoolBitField {
    crate::generation::models::BoolBitField {
        field_name: format_field_name(&field.name),
        bit_position: field.bit_position,
    }
}

fn process_fixed_record_field(
    field: &crate::extraction::FixedRecordField,
    lookup: &Lookup,
) -> crate::generation::models::FixedRecordField {
    let field_name = format_field_name(&field.name);
    let fqn = lookup_record_fqn(&format_type_name(&field.field_type), lookup);
    let type_name = to_tokens(&fqn.type_name);
    let type_path = to_tokens(&fqn.path);
    let parser_name = to_tokens(&fqn.field_name);
    // TODO determine if this fixed record parser needs to be called with discriminant fields; information needs to be collected and stored during extraction
    let parser_module = to_tokens(PARSER_MODULE_NAME);
    let parser_function = quote! { #type_path::#parser_module::#parser_name };

    crate::generation::models::FixedRecordField {
        field_name,
        type_name,
        type_path,
        length: field.length,
        parser_function,
    }
}

fn process_bit_record_field(
    field: &crate::extraction::BitRecordField,
    lookup: &Lookup,
) -> crate::generation::models::BitRecordField {
    let field_name = format_field_name(&field.name);
    let field_type = type_for_bit_record_field(field, lookup);
    let (type_path, type_name, parser_must_convert_to_enum, parser_function) = match field_type {
        BitRecordFieldType::Enum(fqn) => {
            let primitive_type = field_size_to_primitive_type(field.size);
            let function = to_tokens(&format!("{NOM_LE_PARSER_PATH}{primitive_type}"));
            (
                to_tokens(&fqn.path),
                to_tokens(&fqn.type_name),
                true,
                function,
            )
        }
        BitRecordFieldType::Record(fqn) => {
            let type_path = to_tokens(&fqn.path);
            let parser_name = to_tokens(&fqn.field_name);
            let parser_module = to_tokens(PARSER_MODULE_NAME);
            let function = quote! { #type_path::#parser_module::#parser_name };
            (type_path, to_tokens(&fqn.type_name), false, function)
        }
    };

    crate::generation::models::BitRecordField {
        field_name,
        type_name,
        type_path,
        size: field.size,
        parser_function,
        parser_must_convert_to_enum,
    }
}

fn process_adaptive_record_field(
    field: &crate::extraction::AdaptiveRecordField,
    lookup: &Lookup,
) -> crate::generation::models::AdaptiveRecordField {
    let field_type_fqn = type_for_adaptive_record_field(field, lookup);
    let discriminant_field_type = if let Some(d_uids) = &field.enum_uid {
        let d_fqn = lookup_enum_fqn_first_uid(d_uids, lookup);
        to_tokens(&d_fqn.to_full_type())
    } else {
        quote! {}
    };
    let parser_function = to_tokens(&format!(
        "{NOM_LE_PARSER_PATH}{}",
        field_length_to_primitive(field.length)
    ));
    crate::generation::models::AdaptiveRecordField {
        field_name: format_field_name(&field.name),
        type_name: to_tokens(&field_type_fqn.type_name),
        type_path: to_tokens(&field_type_fqn.path),
        length: field.length,
        discriminant_field_name: format_field_name(&field.discriminant),
        discriminant_field_type,
        parser_function,
    }
}

fn process_variable_string_field(
    field: &crate::extraction::VariableStringField,
) -> crate::generation::models::VariableStringField {
    crate::generation::models::VariableStringField {
        field_name: format_field_name(&field.name),
        field_type: "String",
        fixed_number_of_strings: field.fixed_number_of_strings.unwrap_or(0),
        parser_function: to_tokens("crate::parser::fixed_string_with_length"),
    }
}

fn process_variable_string(
    element: &crate::extraction::VariableString,
) -> crate::generation::models::VariableString {
    crate::generation::models::VariableString {
        count_field: process_count_field(&element.count_field),
        string_field: process_variable_string_field(&element.string_field),
    }
}

fn process_opaque_data_field(
    field: &crate::extraction::OpaqueDataField,
) -> crate::generation::models::OpaqueDataField {
    crate::generation::models::OpaqueDataField {
        field_name: format_field_name(&field.name),
        field_type: "Vec<u8>",
        parser_function: to_tokens("crate::parser::opaque_data"),
    }
}

fn process_array(
    array: &crate::extraction::Array,
    lookup: &Lookup,
) -> crate::generation::models::Array {
    crate::generation::models::Array {
        count_field: process_count_field(&array.count_field),
        type_field: process_array_field_enum(&array.type_field, lookup),
    }
}

fn process_opaque_data(o: &crate::extraction::OpaqueData) -> crate::generation::models::OpaqueData {
    crate::generation::models::OpaqueData {
        count_field: process_count_field(&o.count_field),
        opaque_data_field: process_opaque_data_field(&o.opaque_data_field),
    }
}

fn process_fixed_record(
    record: &crate::extraction::FixedRecord,
    lookup: &Lookup,
) -> crate::generation::models::FixedRecord {
    let record_type_fqn = lookup_record_fqn(&format_type_name(&record.record_type), lookup);
    let fields = record
        .fields
        .iter()
        .map(|field| process_pdu_fixed_record_fields_enum(field, lookup))
        .collect::<Vec<PduAndFixedRecordFieldsEnum>>();

    // Determine if the record has (AdaptiveRecordField) fields that depend on a discriminant
    // Only AdaptiveRecordFields can be dependent on EnumField discriminants
    let depending_on_discriminants = fields
        .iter()
        .filter_map(|field| field.has_discriminant())
        .map(|field| field.discriminant_field_name.as_str())
        .collect::<Vec<&str>>();

    // Determine the fields that are defining discriminants
    let defining_discriminants = fields
        .iter()
        .filter_map(|field| field.is_discriminant())
        .map(|field| field.field_name.as_str())
        .collect::<Vec<&str>>();

    // Determine if dependent fields are not matched by a defining field, e.g. the discriminant is external to the record
    let external_discriminants = depending_on_discriminants
        .into_iter()
        .filter(|&item| !defining_discriminants.contains(&item))
        .collect::<Vec<&str>>();

    // Find the externally dependent AdaptiveRecordField and retrieve the discriminant info as `discriminant_name: discriminant_type`
    // E.g. a TokenStream that fits the parser arguments for this record.
    let external_discriminants = external_discriminants
        .iter()
        .filter_map(|&discriminant| {
            let dependent_field = fields.iter().find(|field| {
                if let PduAndFixedRecordFieldsEnum::AdaptiveRecord(arf) = field {
                    arf.discriminant_field_name == discriminant
                } else {
                    false
                }
            });
            dependent_field
        })
        .filter_map(|field| {
            if let PduAndFixedRecordFieldsEnum::AdaptiveRecord(arf) = field {
                let discriminant_field = format_ident!("{}", arf.discriminant_field_name);
                let discriminant_type = &arf.discriminant_field_type;
                let argument = quote! {
                    #discriminant_field: #discriminant_type
                };
                Some(argument)
            } else {
                None
            }
        })
        .collect::<Vec<TokenStream>>();

    let parser_name = to_tokens(&format_field_name(&record.record_type));
    let record_full_type = to_tokens(&record_type_fqn.to_full_type());

    let parser_function = if external_discriminants.is_empty() {
        quote! { #parser_name(input: &[u8]) -> IResult<&[u8], #record_full_type> }
    } else {
        quote! {
            #parser_name(#(#external_discriminants),*) -> impl Fn(&[u8]) -> IResult<&[u8], #record_full_type>
        }
    };

    let has_external_discriminants = !external_discriminants.is_empty();

    crate::generation::models::FixedRecord {
        fields,
        type_name: to_tokens(&record_type_fqn.type_name),
        type_path: to_tokens(&record_type_fqn.path),
        length: record.length,
        parser_function,
        has_external_discriminants,
    }
}

fn process_bit_record(
    record: &crate::extraction::BitRecord,
    lookup: &Lookup,
) -> crate::generation::models::BitRecord {
    let record_type_fqn = lookup_record_fqn(&format_type_name(&record.record_type), lookup);
    let fields = record
        .fields
        .iter()
        .map(|field| process_bit_record_field_enum(field, lookup))
        .collect();
    let type_name = to_tokens(&record_type_fqn.type_name);
    let type_path = to_tokens(&record_type_fqn.path);
    let parser_function = to_tokens(&record_type_fqn.field_name);

    let value_primitive_type = field_size_to_primitive_type(record.size);
    let value_parser = to_tokens(&format!("{NOM_LE_PARSER_PATH}{value_primitive_type}"));

    crate::generation::models::BitRecord {
        fields,
        type_name,
        type_path,
        size: record.size,
        parser_function,
        value_parser,
    }
}

fn process_adaptive_record(
    record: &crate::extraction::AdaptiveRecord,
    lookup: &Lookup,
) -> crate::generation::models::AdaptiveRecord {
    let record_type_fqn = lookup_record_fqn(&format_type_name(&record.record_type), lookup);
    let variants = record
        .variants
        .iter()
        .map(|variant| process_adaptive_format_enum(variant, lookup))
        .collect();
    let type_name = to_tokens(&record_type_fqn.type_name);
    let type_path = to_tokens(&record_type_fqn.path);
    let parser_name = to_tokens(&format_field_name(&record.record_type));
    let record_full_type = to_tokens(&record_type_fqn.to_full_type());
    let parser_function =
        quote! { #parser_name(input: &[u8]) -> IResult<&[u8], #record_full_type> };

    crate::generation::models::AdaptiveRecord {
        variants,
        type_name,
        type_path,
        length: record.length,
        discriminant_start_value: record.discriminant_start_value,
        parser_function,
    }
}

fn process_extension_record_set(
    record: &crate::extraction::ExtensionRecordSet,
) -> crate::generation::models::ExtensionRecordSet {
    crate::generation::models::ExtensionRecordSet {
        count_field: process_count_field(&record.count_field),
    }
}

fn process_extension_record(
    record: &crate::extraction::ExtensionRecord,
    lookup: &Lookup,
) -> crate::generation::models::ExtensionRecord {
    let formatted_record_name = format_type_name(&record.name_attr);
    let fqn = lookup_er_fqn(&formatted_record_name, lookup);
    let formatted_function_name = format_field_name(&record.name_attr);
    let record_type_variant_name = lookup_er_type(record.record_type_attr, lookup);
    crate::generation::models::ExtensionRecord {
        type_name: fqn.type_name.clone(),
        type_path: to_tokens(&fqn.path),
        record_type_enum: record.record_type_attr,
        record_type_variant_name: record_type_variant_name.to_string(),
        base_length: record.base_length_attr,
        is_variable: record.is_variable_attr,
        record_type_field: process_enum_field(&record.record_type_field, lookup),
        record_length_field: process_numeric_field(&record.record_length_field),
        fields: record
            .fields
            .iter()
            .map(|field| process_extension_record_field_enum(field, lookup))
            .collect(),
        padding_to_64_field: record.padding_to_64_field.map(|_p| process_padding_64()),
        parser_function: format!("{formatted_function_name}_body")
            .parse()
            .expect("Could not parse a formatted function name."),
    }
}

fn process_pdu(pdu: &crate::extraction::Pdu, lookup: &Lookup) -> crate::generation::models::Pdu {
    let pdu_type_name = format_type_name(&pdu.name_attr);
    let pdu_module_name = format_pdu_module_name(&pdu.name_attr);
    let fqn = lookup_pdu_fqn(&pdu_type_name, lookup);
    crate::generation::models::Pdu {
        pdu_module_name: pdu_module_name.clone(),
        type_name: pdu_type_name.clone(),
        type_path: to_tokens(&fqn.path),
        pdu_type: pdu.type_attr,
        pdu_type_name: to_tokens(lookup_pdu_type(pdu.type_attr, lookup)),
        protocol_family: pdu.protocol_family_attr,
        base_length: pdu.base_length_attr,
        header_field: process_fixed_record_field(&pdu.header_field, lookup),
        fields: pdu
            .fields
            .iter()
            .map(|field| process_pdu_fixed_record_fields_enum(field, lookup))
            .collect(),
        extension_record_set: process_extension_record_set(&pdu.extension_record_set),
        parser_function: format!("{pdu_module_name}_body").parse().unwrap(),
    }
}

#[allow(dead_code)]
fn process_padding_16() -> crate::generation::models::PaddingTo16 {
    crate::generation::models::PaddingTo16
}

#[allow(dead_code)]
fn process_padding_32() -> crate::generation::models::PaddingTo32 {
    crate::generation::models::PaddingTo32
}

fn process_padding_64() -> crate::generation::models::PaddingTo64 {
    crate::generation::models::PaddingTo64
}

fn process_pdu_fixed_record_fields_enum(
    e: &crate::extraction::PduAndFixedRecordFieldsEnum,
    lookup: &Lookup,
) -> crate::generation::models::PduAndFixedRecordFieldsEnum {
    match e {
        crate::extraction::PduAndFixedRecordFieldsEnum::Numeric(f) => {
            crate::generation::models::PduAndFixedRecordFieldsEnum::Numeric(process_numeric_field(
                f,
            ))
        }
        crate::extraction::PduAndFixedRecordFieldsEnum::Enum(f) => {
            crate::generation::models::PduAndFixedRecordFieldsEnum::Enum(process_enum_field(
                f, lookup,
            ))
        }
        crate::extraction::PduAndFixedRecordFieldsEnum::FixedString(f) => {
            crate::generation::models::PduAndFixedRecordFieldsEnum::FixedString(
                process_fixed_string_field(f),
            )
        }
        crate::extraction::PduAndFixedRecordFieldsEnum::FixedRecord(f) => {
            crate::generation::models::PduAndFixedRecordFieldsEnum::FixedRecord(
                process_fixed_record_field(f, lookup),
            )
        }
        crate::extraction::PduAndFixedRecordFieldsEnum::BitRecord(f) => {
            crate::generation::models::PduAndFixedRecordFieldsEnum::BitRecord(
                process_bit_record_field(f, lookup),
            )
        }
        crate::extraction::PduAndFixedRecordFieldsEnum::AdaptiveRecord(f) => {
            crate::generation::models::PduAndFixedRecordFieldsEnum::AdaptiveRecord(
                process_adaptive_record_field(f, lookup),
            )
        }
    }
}

fn process_extension_record_field_enum(
    e: &crate::extraction::ExtensionRecordFieldEnum,
    lookup: &Lookup,
) -> crate::generation::models::ExtensionRecordFieldEnum {
    match e {
        crate::extraction::ExtensionRecordFieldEnum::Numeric(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::Numeric(process_numeric_field(f))
        }
        crate::extraction::ExtensionRecordFieldEnum::Enum(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::Enum(process_enum_field(f, lookup))
        }
        crate::extraction::ExtensionRecordFieldEnum::FixedString(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::FixedString(
                process_fixed_string_field(f),
            )
        }
        crate::extraction::ExtensionRecordFieldEnum::VariableString(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::VariableString(
                process_variable_string(f),
            )
        }
        crate::extraction::ExtensionRecordFieldEnum::FixedRecord(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::FixedRecord(
                process_fixed_record_field(f, lookup),
            )
        }
        crate::extraction::ExtensionRecordFieldEnum::BitRecord(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::BitRecord(
                process_bit_record_field(f, lookup),
            )
        }
        crate::extraction::ExtensionRecordFieldEnum::Array(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::Array(process_array(f, lookup))
        }
        crate::extraction::ExtensionRecordFieldEnum::AdaptiveRecord(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::AdaptiveRecord(
                process_adaptive_record_field(f, lookup),
            )
        }
        crate::extraction::ExtensionRecordFieldEnum::Opaque(f) => {
            crate::generation::models::ExtensionRecordFieldEnum::Opaque(process_opaque_data(f))
        }
        crate::extraction::ExtensionRecordFieldEnum::PaddingTo16 => {
            crate::generation::models::ExtensionRecordFieldEnum::PaddingTo16
        }
        crate::extraction::ExtensionRecordFieldEnum::PaddingTo32 => {
            crate::generation::models::ExtensionRecordFieldEnum::PaddingTo32
        }
    }
}

fn process_bit_record_field_enum(
    e: &crate::extraction::BitRecordFieldEnum,
    lookup: &Lookup,
) -> crate::generation::models::BitRecordFieldEnum {
    match e {
        crate::extraction::BitRecordFieldEnum::Enum(f) => {
            crate::generation::models::BitRecordFieldEnum::Enum(process_enum_bitfield(f, lookup))
        }
        crate::extraction::BitRecordFieldEnum::Int(f) => {
            crate::generation::models::BitRecordFieldEnum::Int(process_int_bitfield(f))
        }
        crate::extraction::BitRecordFieldEnum::Bool(f) => {
            crate::generation::models::BitRecordFieldEnum::Bool(process_bool_bitfield(f))
        }
    }
}

fn process_adaptive_format_enum(
    e: &crate::extraction::AdaptiveFormatEnum,
    lookup: &Lookup,
) -> crate::generation::models::AdaptiveFormatEnum {
    match e {
        crate::extraction::AdaptiveFormatEnum::Numeric(f) => {
            crate::generation::models::AdaptiveFormatEnum::Numeric(process_numeric_field(f))
        }
        crate::extraction::AdaptiveFormatEnum::Enum(f) => {
            crate::generation::models::AdaptiveFormatEnum::Enum(process_enum_field(f, lookup))
        }
        crate::extraction::AdaptiveFormatEnum::FixedString(f) => {
            crate::generation::models::AdaptiveFormatEnum::FixedString(process_fixed_string_field(
                f,
            ))
        }
        crate::extraction::AdaptiveFormatEnum::FixedRecord(f) => {
            crate::generation::models::AdaptiveFormatEnum::FixedRecord(process_fixed_record_field(
                f, lookup,
            ))
        }
        crate::extraction::AdaptiveFormatEnum::BitRecord(f) => {
            crate::generation::models::AdaptiveFormatEnum::BitRecord(process_bit_record_field(
                f, lookup,
            ))
        }
    }
}

fn process_array_field_enum(
    e: &crate::extraction::ArrayFieldEnum,
    lookup: &Lookup,
) -> crate::generation::models::ArrayFieldEnum {
    match e {
        crate::extraction::ArrayFieldEnum::Numeric(f) => {
            crate::generation::models::ArrayFieldEnum::Numeric(process_numeric_field(f))
        }
        crate::extraction::ArrayFieldEnum::Enum(f) => {
            crate::generation::models::ArrayFieldEnum::Enum(process_enum_field(f, lookup))
        }
        crate::extraction::ArrayFieldEnum::FixedString(f) => {
            crate::generation::models::ArrayFieldEnum::FixedString(process_fixed_string_field(f))
        }
        crate::extraction::ArrayFieldEnum::FixedRecord(f) => {
            crate::generation::models::ArrayFieldEnum::FixedRecord(process_fixed_record_field(
                f, lookup,
            ))
        }
        crate::extraction::ArrayFieldEnum::BitRecord(f) => {
            crate::generation::models::ArrayFieldEnum::BitRecord(process_bit_record_field(
                f, lookup,
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::field_size_to_primitive_type;
    use std::any::Any;

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
        let pdu_fqn = crate::pre_processing::format_fqn_path_pdu(&pdu);
        assert_eq!(
            pdu_fqn.as_str(),
            "crate::entity_info_interaction::entity_state::EntityState"
        );
    }

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
