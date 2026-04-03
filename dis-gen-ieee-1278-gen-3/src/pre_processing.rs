use crate::extraction::ExtractionItem;
use crate::generation::models::{
    GenerationItem, PduAndFixedRecordFieldsEnum, EXTENSION_RECORDS_MODULE_NAME,
};
use crate::{FqnLookup, Lookup, UidLookup};
use dis_gen_utils::{
    enum_type_to_field_type, format_field_name, format_pdu_module_name, format_type_name,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use std::collections::HashMap;

const NOM_LE_PARSER_PATH: &str = "nom::number::complete::le_";

pub(crate) fn create_fqn_lookup(
    items: &[ExtractionItem]
) -> (FqnLookup, FqnLookup, FqnLookup) {
    let mut pdu_lookup = HashMap::new();
    let mut er_lookup = HashMap::new();
    let mut rec_lookup = HashMap::new();

    for item in items {
        match item {
            ExtractionItem::Pdu(pdu, _) => {
                let name = format_type_name(&item.name());
                let fqn_name = format_fqn_pdu(item);
                pdu_lookup.entry(name.clone()).or_insert(fqn_name);
            }
            ExtractionItem::ExtensionRecord(_, _) => {
                let name = format_type_name(&item.name());
                let fqn_name = format_fqn_extension_record(item);
                er_lookup.entry(name.clone()).or_insert(fqn_name);
            }
            _ => {
                let name = format_type_name(&item.name());
                let fqn_name = format_fqn_record(item);
                rec_lookup.entry(name.clone()).or_insert(fqn_name);
            }
        }
    }

    // FIXME remove these placeholder lookups when the normal stuff works
    rec_lookup.insert("u8".to_string(), "u8".to_string());
    rec_lookup.insert("u16".to_string(), "u16".to_string());

    (pdu_lookup, er_lookup, rec_lookup)
}

pub(crate) fn create_enum_lookup(uid_lookup: &UidLookup) -> FqnLookup {
    let mut enum_lookup = HashMap::new();

    for uid in uid_lookup {
        let fqn_name = format!("crate::enumerations::{}", uid.1);
        enum_lookup.entry(uid.1.clone()).or_insert(fqn_name);
    }

    enum_lookup
}

pub(crate) fn format_fqn_pdu(item: &ExtractionItem) -> String {
    format!(
        "crate::{}::{}::{}",
        item.family(),
        dis_gen_utils::format_pdu_module_name(item.name().as_str()),
        format_type_name(item.name().as_str())
    )
}

pub(crate) fn format_fqn_extension_record(item: &ExtractionItem) -> String {
    format!(
        "crate::{}::extension_records::{}",
        item.family(),
        format_type_name(item.name().as_str())
    )
}

pub(crate) fn format_fqn_record(item: &ExtractionItem) -> String {
    format!(
        "crate::{}::{}",
        item.family(),
        format_type_name(item.name().as_str())
    )
}

fn lookup_first_uid<'l>(uids: &[usize], lookup: &'l Lookup) -> &'l str {
    let the_type = uids
        .iter()
        .map(|uid| lookup_uid(*uid, lookup).to_string())
        .collect::<Vec<String>>();
    let the_type = the_type
        .first()
        .expect("Expected at least one Type for an UID lookup.");
    lookup_record_fqn(the_type, lookup)
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
fn lookup_record_fqn<'fqn>(type_name: &str, lookup: &'fqn Lookup) -> &'fqn str {
    if let Some(fqn) = lookup.records_fqn.get(type_name) {
        fqn
    } else if let Some(fqn) = lookup.enum_fqn.get(type_name) {
        fqn
    } else {
        panic!("Expected full qualified name for type or enum '{type_name}'")
    }
}

fn lookup_er_fqn<'fqn>(er_name: &str, lookup: &'fqn Lookup) -> &'fqn str {
    lookup.er_fqn.get(er_name).unwrap_or_else(|| {
        panic!("Expected full qualified name for Extension Record '{er_name}'")
    })
}

fn lookup_pdu_fqn<'fqn>(pdu_name: &str, lookup: &'fqn Lookup) -> &'fqn str {
    lookup.pdu_fqn.get(pdu_name).unwrap_or_else(|| {
        panic!("Expected full qualified name for PDU '{pdu_name}'")
    })
}

fn lookup_enum_fqn<'fqn>(type_name: &str, lookup: &'fqn Lookup) -> &'fqn str {
    lookup.enum_fqn.get(type_name).unwrap_or_else(|| {
        panic!("Expected full qualified enumeration name for type '{type_name}'")
    })
}

fn lookup_er_type<'a>(uid: &usize, lookup: &'a Lookup) -> &'a str {
    lookup.er_types.get(uid).unwrap_or_else(|| {
        panic!("Expected a ExtensionRecord name for Record Type Enum value {uid}")
    })
}

fn lookup_pdu_type<'a>(uid: &usize, lookup: &'a Lookup) -> &'a str {
    lookup
        .pdu_types
        .get(uid)
        .unwrap_or_else(|| panic!("Expected a PDU name for DIS-PDU Type Enum value {uid}"))
}

fn to_tokens(value: &str) -> TokenStream {
    value
        .parse()
        .expect(format!("Could not parse TokenStream '{value}'").as_str())
}

fn type_for_primitive_type_field(primitive_type: &str) -> &'static str {
    match primitive_type {
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
        _ => panic!("Invalid primitive type"),
    }
}

pub(crate) fn field_size_to_primitive_type(size: usize) -> TokenStream {
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
    field_type
        .parse()
        .expect(&format!("Expected a valid Rust primitive type for a bit field declaration with a given size, found size {size}."))
}

fn field_length_to_primitive(length: usize) -> &'static str {
    match length {
        1 => "u8",
        2 => "u16",
        4 => "u32",
        _ => panic!("Invalid length, cannot convert to a Rust primitive type"),
    }
}

#[inline]
fn type_for_enum_field<'l>(
    field: &crate::extraction::EnumField,
    lookup: &'l Lookup,
) -> (&'l str, bool) {
    if let Some(uids) = &field.enum_uid {
        let enum_type = uids
            .iter()
            .map(|uid| lookup_uid(*uid, lookup).to_string())
            .collect::<Vec<String>>();
        let enum_type = enum_type
            .first()
            .expect("Expected at least one type for an EnumField declaration.");
        let enum_type = lookup_enum_fqn(enum_type, lookup);
        (enum_type, false)
    } else {
        let ty = enum_type_to_field_type(&field.field_type)
            .expect("Expected a valid type for an EnumField declaration.");
        (ty, true)
    }
}

#[inline]
fn type_for_fixed_record_field(
    field: &crate::extraction::FixedRecordField,
    lookup: &Lookup,
) -> TokenStream {
    let fqn_field_type = lookup_record_fqn(format_type_name(&field.field_type).as_str(), lookup);
    fqn_field_type.parse().expect(&format!(
        "Expected a valid Type for a FixedRecordField declaration, found '{fqn_field_type}'."
    ))
}

#[inline]
fn type_for_bit_record_field(
    field: &crate::extraction::BitRecordField,
    lookup: &Lookup,
) -> TokenStream {
    if let Some(uids) = &field.enum_uid {
        let enum_type = lookup_first_uid(&uids, lookup);
        to_tokens(enum_type)
    } else {
        let field_type = lookup_record_fqn(
            format_type_name(field.field_type.as_ref().expect(
                "Expected a type name for BitRecordField to be present as there is also no UID.",
            ))
            .as_str(),
            lookup,
        );
        to_tokens(field_type)
    }
}

#[inline]
fn type_for_adaptive_record_field(
    field: &crate::extraction::AdaptiveRecordField,
    lookup: &Lookup,
) -> TokenStream {
    if let Some(uids) = &field.enum_uid {
        let enum_type = lookup_first_uid(&uids, lookup);
        to_tokens(enum_type)
    } else {
        let field_type = lookup_record_fqn(format_type_name(field.field_type.as_ref().expect(
            "Expected a type name for AdaptiveRecordField to be present as there is also no UID.",
        )).as_str(), lookup);
        to_tokens(field_type)
    }
}

#[inline]
fn must_skip_field_decl(field_name: &str) -> bool {
    ["Padding", "Padding1", "Padding2", "Not used"].contains(&field_name)
}

pub fn process_extracted(extracts: &[ExtractionItem], lookup: &Lookup) -> Vec<GenerationItem> {
    extracts
        .iter()
        .map(|e| match e {
            ExtractionItem::Pdu(pdu, family) => {
                GenerationItem::Pdu(process_pdu(pdu, family, lookup), family.clone())
            }
            ExtractionItem::FixedRecord(record, family) => GenerationItem::FixedRecord(
                process_fixed_record(record, family, lookup),
                family.clone(),
            ),
            ExtractionItem::BitRecord(record, family) => GenerationItem::BitRecord(
                process_bit_record(record, family, lookup),
                family.clone(),
            ),
            ExtractionItem::AdaptiveRecord(record, family) => GenerationItem::AdaptiveRecord(
                process_adaptive_record(record, family, lookup),
                family.clone(),
            ),
            ExtractionItem::ExtensionRecord(record, family) => GenerationItem::ExtensionRecord(
                process_extension_record(record, family, lookup),
                family.clone(),
            ),
        })
        .collect()
}

fn process_numeric_field(
    field: &crate::extraction::NumericField,
) -> crate::generation::models::NumericField {
    let field_type = type_for_primitive_type_field(&field.primitive_type);
    crate::generation::models::NumericField {
        field_name: format_field_name(&field.name),
        primitive_type: to_tokens(field_type),
        units: field.units.clone(),
        is_padding: must_skip_field_decl(&field.name),
        parser_function: to_tokens(format!("{NOM_LE_PARSER_PATH}{field_type}").as_str()),
    }
}

fn process_count_field(
    field: &crate::extraction::CountField,
) -> crate::generation::models::CountField {
    let field_type = type_for_primitive_type_field(&field.primitive_type);
    crate::generation::models::CountField {
        field_name: format_field_name(&field.name),
        primitive_type: to_tokens(field_type),
        parser_function: to_tokens(format!("{NOM_LE_PARSER_PATH}{field_type}").as_str()),
    }
}

fn process_enum_field(
    field: &crate::extraction::EnumField,
    lookup: &Lookup,
) -> crate::generation::models::EnumField {
    let field_name = format_field_name(&field.name);
    let (enum_type, is_primitive_type) = type_for_enum_field(field, lookup);
    let field_type_fqn = to_tokens(enum_type);
    let enum_data_size = format_ident!(
        "{}",
        enum_type_to_field_type(&field.field_type).expect("Expected a valid enum data size")
    );
    let parser_function = to_tokens(&format!("{NOM_LE_PARSER_PATH}{enum_data_size}"));
    let parser_must_convert_to_enum = !is_primitive_type;

    crate::generation::models::EnumField {
        field_name,
        field_type_fqn,
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
            "crate::parser::fixed_string({} as usize)",
            field.length
        )),
    }
}

fn process_int_bitfield(
    field: &crate::extraction::IntBitField,
) -> crate::generation::models::IntBitField {
    crate::generation::models::IntBitField {
        field_name: format_field_name(&field.name),
        field_type: field_size_to_primitive_type(field.size.unwrap_or(1)),
        bit_position: field.bit_position,
        size: field.size.unwrap_or(1),
        units: field.units.clone(),
        is_padding: must_skip_field_decl(&field.name),
    }
}

fn process_enum_bitfield(
    field: &crate::extraction::EnumBitField,
    lookup: &Lookup,
) -> crate::generation::models::EnumBitField {
    let field_type_fqn = match (field.size, &field.enum_uid) {
        (Some(size), None) => {
            let primitive = field_size_to_primitive_type(size);
            primitive
        }
        (_, Some(uids)) => {
            let field_type = lookup_first_uid(uids, lookup);
            to_tokens(field_type)
        }
        (None, None) => {
            panic!("EnumBitField neither has a size or an enumTableUid attribute");
        }
    };
    crate::generation::models::EnumBitField {
        field_name: format_field_name(&field.name),
        field_type: field_type_fqn.clone(),
        field_type_fqn,
        bit_position: field.bit_position,
        size: field.size.unwrap_or(1),
        is_discriminant: field.is_discriminant.unwrap_or(false),
    }
}

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
    let parser_function = to_tokens(&field_name);
    crate::generation::models::FixedRecordField {
        field_name,
        field_type_fqn: type_for_fixed_record_field(field, lookup),
        length: field.length,
        parser_function,
    }
}

fn process_bit_record_field(
    field: &crate::extraction::BitRecordField,
    lookup: &Lookup,
) -> crate::generation::models::BitRecordField {
    let field_name = format_field_name(&field.name);
    let parser_function = to_tokens(&field_name);
    crate::generation::models::BitRecordField {
        field_name: format_field_name(&field.name),
        field_type_fqn: type_for_bit_record_field(field, lookup),
        as_variant_name: format_type_name(&field.name),
        size: field.size,
        parser_function,
    }
}

fn process_adaptive_record_field(
    field: &crate::extraction::AdaptiveRecordField,
    lookup: &Lookup,
) -> crate::generation::models::AdaptiveRecordField {
    let discriminant_field_type = if let Some(name) = &field.enum_uid {
        to_tokens(lookup_first_uid(name, lookup))
    } else {
        quote! {}
    };
    let parser_function = to_tokens(&format!(
        "{NOM_LE_PARSER_PATH}{}",
        field_length_to_primitive(field.length)
    ));
    crate::generation::models::AdaptiveRecordField {
        field_name: format_field_name(&field.name),
        field_type_fqn: type_for_adaptive_record_field(field, lookup),
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
        parser_function: to_tokens("crate::parser::fixed_string"),
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

// TODO parser
fn process_fixed_record(
    record: &crate::extraction::FixedRecord,
    family: &str,
    lookup: &Lookup,
) -> crate::generation::models::FixedRecord {
    let formatted_record_type = format_type_name(&record.record_type);
    let fields = record
        .fields
        .iter()
        .map(|field| process_pdu_fixed_record_fields_enum(field, lookup))
        .collect::<Vec<PduAndFixedRecordFieldsEnum>>();

    // determine if the record has fields with a discriminant
    let discriminants = fields
        .iter()
        .filter(|&f| f.has_discriminant())
        .map(|f| {
            if let PduAndFixedRecordFieldsEnum::AdaptiveRecord(arf) = f {
                let discriminant_name = format_ident!("{}", arf.discriminant_field_name);
                let field_type = &arf.discriminant_field_type;
                quote! { #discriminant_name: #field_type, }
            } else { panic!("Expected only AdaptiveRecords to have discriminant dependencies on fields outside of the record itself") }
        } )
        .collect::<TokenStream>();

    let parser_name = to_tokens(&format_field_name(&record.record_type));
    let record_type_fqn = to_tokens(lookup_record_fqn(&formatted_record_type, lookup));

    let parser_function = if discriminants.is_empty() {
        quote! { #parser_name(input: &[u8]) -> IResult<&[u8], #record_type_fqn> }
    } else {
        quote! {
            #parser_name(#discriminants) -> impl Fn(&[u8]) -> IResult<&[u8], #record_type_fqn>
        }
    };

    crate::generation::models::FixedRecord {
        fields,
        record_type: to_tokens(&formatted_record_type),
        record_type_fqn,
        length: record.length,
        parser_function,
        external_discriminants: !discriminants.is_empty(),
    }
}

// TODO parser
fn process_bit_record(
    record: &crate::extraction::BitRecord,
    family: &str,
    lookup: &Lookup,
) -> crate::generation::models::BitRecord {
    let formatted_record_type = format_type_name(&record.record_type);
    crate::generation::models::BitRecord {
        fields: record
            .fields
            .iter()
            .map(|field| process_bit_record_field_enum(field, lookup))
            .collect(),
        record_type: to_tokens(&formatted_record_type),
        record_type_fqn: to_tokens(lookup_record_fqn(&formatted_record_type, lookup)),
        size: record.size,
    }
}

// TODO parser
fn process_adaptive_record(
    record: &crate::extraction::AdaptiveRecord,
    family: &str,
    lookup: &Lookup,
) -> crate::generation::models::AdaptiveRecord {
    let formatted_record_type = format_type_name(&record.record_type);
    crate::generation::models::AdaptiveRecord {
        variants: record
            .variants
            .iter()
            .map(|variant| process_adaptive_format_enum(variant, lookup))
            .collect(),
        record_type: to_tokens(&formatted_record_type),
        record_type_fqn: to_tokens(lookup_record_fqn(&formatted_record_type, lookup)),
        length: record.length,
        discriminant_start_value: record.discriminant_start_value,
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
    family: &str,
    lookup: &Lookup,
) -> crate::generation::models::ExtensionRecord {
    let formatted_record_name = format_type_name(&record.name_attr);
    let formatted_function_name = format_field_name(&record.name_attr);
    let record_type_variant_name = lookup_er_type(&record.record_type_attr, lookup);
    crate::generation::models::ExtensionRecord {
        record_name: formatted_record_name.clone(),
        record_name_fqn: to_tokens(lookup_er_fqn(&formatted_record_name, lookup)),
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
        fqn_path: format!("crate::{family}::{EXTENSION_RECORDS_MODULE_NAME}")
            .parse()
            .unwrap(),
        parser_function: format!("{formatted_function_name}_body").parse().unwrap(),
    }
}

fn process_pdu(
    pdu: &crate::extraction::Pdu,
    family: &str,
    lookup: &Lookup,
) -> crate::generation::models::Pdu {
    let formatted_pdu_name = format_type_name(&pdu.name_attr);
    let formatted_pdu_module_name = format_pdu_module_name(&pdu.name_attr);
    crate::generation::models::Pdu {
        pdu_module_name: formatted_pdu_module_name.clone(),
        pdu_name: formatted_pdu_name.clone(),
        pdu_name_fqn: to_tokens(lookup_pdu_fqn(&formatted_pdu_name, lookup)),
        pdu_type: pdu.type_attr,
        pdu_type_name: to_tokens(lookup_pdu_type(&pdu.type_attr, lookup)),
        protocol_family: pdu.protocol_family_attr,
        base_length: pdu.base_length_attr,
        header_field: process_fixed_record_field(&pdu.header_field, lookup),
        fields: pdu
            .fields
            .iter()
            .map(|field| process_pdu_fixed_record_fields_enum(field, lookup))
            .collect(),
        extension_record_set: process_extension_record_set(&pdu.extension_record_set),
        fqn_path: format!("crate::{family}::{formatted_pdu_module_name}")
            .parse()
            .unwrap(),
        parser_function: format!("{formatted_pdu_module_name}_body").parse().unwrap(),
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
