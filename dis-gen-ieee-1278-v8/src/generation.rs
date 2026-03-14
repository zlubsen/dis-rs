use super::{numeric_type_to_field_type, AdaptiveRecord, AdaptiveRecordField, Array, ArrayFieldEnum, BitRecord, BitRecordField, BitRecordFieldEnum, BoolBitField, EnumBitField, EnumField, ExtensionRecord, ExtensionRecordFieldEnum, FixedRecord, FixedRecordField, FixedStringField, GenerationItem, IntBitField, Lookup, NumericField, Pdu, PduAndFixedRecordFieldsEnum, VariableStringField};
use dis_gen_utils::{
    enum_type_to_field_type, format_field_name, format_pdu_module_name, format_type_name,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
// Module tree of generated sources:
// src/v8
//    common_records/           // Containing all common records
//        builder.rs
//        model.rs
//        parser.rs
//        writer.rs
//    family_x/
//        common/       // Containing all records for this family
//            builder.rs
//            model.rs
//            parser.rs
//            writer.rs
//        pdu_x/        // Containing specific PDU x.
//            builder.rs
//            model.rs
//            parser.rs
//            writer.rs

// TODO
// - experiment with xsd_parser to generate the intermediate representation, again in separate crates

pub fn generate(items: &[GenerationItem], families: &[String], lookup: &Lookup) -> TokenStream {
    let core_contents = generate_core_units();
    let family_contents: Vec<TokenStream> = families
        .iter()
        .map(|family| generate_family_module(items, family.as_str(), lookup))
        .collect();
    let contents = quote! {

        #core_contents

        #(#family_contents)*

    };

    generate_module_with_name("v8", &contents)
}

fn generate_core_units() -> TokenStream {
    // FIXME these parts should be in the lib itself as regular code, whenever possible
    // TODO design required core data structures
    // TODO PduBody: list all PDUs (and their headers) in a main enum, analogous to v7
    // TODO ExtensionRecordBody: list all extension records in an enum, analogous to PduBody
    quote! {
        use crate::v8::common_records::PduHeader;

        #[derive(Debug, Clone)]
        pub struct Pdu {
            pub header: PduHeader,
            pub body: PduBody,
        }

        #[derive(Debug, Clone)]
        pub enum PduBody {
            Dummy,
        }

        pub struct ExtensionRecord {
            pub record_type: crate::enumerations::ExtensionRecordTypes,
            pub record_length: usize,
            pub body: ExtensionRecordBody,
        }

        pub enum ExtensionRecordBody {
            ...
        }
    }
}

/// Helper function that generates a module structure with the provided `name`,
/// and is filled with the provided `TokenStream` as contents.
fn generate_module_with_name(name: &str, contents: &TokenStream) -> TokenStream {
    let name_ident = format_ident!("{name}");
    quote! {

        pub mod #name_ident {
            #contents
        }

    }
}

/// Generates a module for a PDU Family of PDUs and records, plus all its contents
fn generate_family_module(items: &[GenerationItem], family: &str, lookup: &Lookup) -> TokenStream {
    // 1. Filter the PDUs for this family and generate these in separate modules
    let pdus = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_pdu())
        .map(|pdu| {
            if let GenerationItem::Pdu(pdu, _) = pdu {
                generate_pdu_module(pdu, lookup)
            } else {
                panic!("GenerationItem is not a PDU.")
            }
        })
        .collect::<TokenStream>();

    // 2. Filter the non-PDU items for this family and generate the records in the family module
    let records = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && !item.is_pdu())
        .map(|item| match item {
            GenerationItem::FixedRecord(record, _family) => generate_fixed_record(record, lookup),
            GenerationItem::ExtensionRecord(record, _family) => {
                generate_extension_record(record, lookup)
            }
            GenerationItem::BitRecord(record, _family) => generate_bit_record(record, lookup),
            GenerationItem::AdaptiveRecord(record, _family) => {
                generate_adaptive_record(record, lookup)
            }
            GenerationItem::Pdu(_, _) => panic!("GenerationItem is not a Record."),
        })
        .collect::<TokenStream>();

    // 3. Merge resulting TokenStreams
    let contents = quote! { #pdus #records };

    generate_module_with_name(family, &contents)
}

/// Generates all code related a PDU
fn generate_pdu_module(item: &Pdu, lookup: &Lookup) -> TokenStream {
    let formatted_pdu_name = format_type_name(item.name_attr.as_str());
    let ident_pdu_name = format_ident!("{}", formatted_pdu_name);
    let pdu_module_name = format_pdu_module_name(item.name_attr.as_str());

    // TODO design PduBody traits: size, family, pduType. See BodyRaw, BodyInfo, blanket impls, serialisation, Interaction.

    let fields = item
        .fields
        .iter()
        .map(|field| generate_pdu_and_fixed_field_decl(field, lookup))
        .collect::<Vec<TokenStream>>();

    let contents = quote! {
        pub struct #ident_pdu_name {
            #(#fields)*
            pub extension_record_count: usize,
        }
    };

    generate_module_with_name(pdu_module_name.as_str(), &contents)
}

#[inline]
fn must_skip_field_decl(field_name: &str) -> bool {
    ["Padding", "Padding1", "Padding2", "Not used"].contains(&field_name)
}

fn generate_fixed_record(item: &FixedRecord, lookup: &Lookup) -> TokenStream {
    let record_name = format_ident!("{}", format_type_name(&item.record_type));

    let fields = item
        .fields
        .iter()
        .map(|field| generate_pdu_and_fixed_field_decl(field, lookup))
        .collect::<Vec<TokenStream>>();

    quote! {
        pub struct #record_name {
            #(#fields)*
        }
    }
}

fn generate_extension_record(item: &ExtensionRecord, lookup: &Lookup) -> TokenStream {
    let record_name = format_ident!("{}", format_type_name(&item.name_attr));
    let record_type_doc_comment = format!("Record Type Enum {}", item.record_type_attr);

    let fields = item
        .fields
        .iter()
        .map(|field| generate_extension_record_field_decl(field, lookup));

    quote! {
        pub struct #record_name {
            #(#fields)*
        }
    }
}

fn generate_pdu_and_fixed_field_decl(
    field: &PduAndFixedRecordFieldsEnum,
    lookup: &Lookup,
) -> TokenStream {
    match field {
        PduAndFixedRecordFieldsEnum::Numeric(field) => generate_numeric_field_decl(field),
        PduAndFixedRecordFieldsEnum::Enum(field) => generate_enum_field_decl(field, lookup),
        PduAndFixedRecordFieldsEnum::FixedString(field) => generate_fixed_string_field_decl(field),
        PduAndFixedRecordFieldsEnum::FixedRecord(field) => {
            generate_fixed_record_field_decl(field, lookup)
        }
        PduAndFixedRecordFieldsEnum::BitRecord(field) => {
            generate_bit_record_field_decl(field, lookup)
        }
        PduAndFixedRecordFieldsEnum::AdaptiveRecord(field) => {
            generate_adaptive_record_field_decl(field, lookup)
        }
    }
}

fn generate_extension_record_field_decl(
    field: &ExtensionRecordFieldEnum,
    lookup: &Lookup,
) -> TokenStream {
    match field {
        ExtensionRecordFieldEnum::Numeric(field) => generate_numeric_field_decl(field),
        ExtensionRecordFieldEnum::Enum(field) => generate_enum_field_decl(field, lookup),
        ExtensionRecordFieldEnum::FixedString(field) => generate_fixed_string_field_decl(field),
        ExtensionRecordFieldEnum::VariableString(field) => {
            generate_variable_string_field_decl(&field.string_field)
        }
        ExtensionRecordFieldEnum::FixedRecord(field) => {
            generate_fixed_record_field_decl(field, lookup)
        }
        ExtensionRecordFieldEnum::BitRecord(field) => generate_bit_record_field_decl(field, lookup),
        ExtensionRecordFieldEnum::Array(field) => generate_array_field_decl(field),
        ExtensionRecordFieldEnum::AdaptiveRecord(field) => {
            generate_adaptive_record_field_decl(field, lookup)
        }
        ExtensionRecordFieldEnum::Opaque(field) => generate_opaque_field_decl(field),
        ExtensionRecordFieldEnum::PaddingTo16 => {
            quote! {}
        }
        ExtensionRecordFieldEnum::PaddingTo32 => {
            quote! {}
        }
    }
}

fn generate_numeric_field_decl(field: &NumericField) -> TokenStream {
    if must_skip_field_decl(field.name.as_str()) {
        return quote! {};
    }
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl = format_ident!(
        "{}",
        numeric_type_to_field_type(&field.primitive_type)
            .expect("Expected valid numeric field type.")
    );
    let doc_units = &field
        .units
        .as_ref()
        .map(|f| quote! { #[doc= #f] })
        .unwrap_or_default();

    quote! {
        #doc_units
        pub #field_ident : #type_decl,
    }
}

fn generate_enum_field_decl(field: &EnumField, lookup: &Lookup) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl = if let Some(uids) = &field.enum_uid {
        let enum_type = uids
            .iter()
            .map(|uid| lookup_uid(*uid, lookup).to_string())
            .collect::<Vec<String>>();
        let enum_type = enum_type
            .first()
            .expect("Expected at least one type for an EnumField declaration.");
        let enum_type = lookup_fqn(enum_type, lookup);
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
    };

    quote! {
        pub #field_ident : #type_decl,
    }
}

fn generate_fixed_string_field_decl(field: &FixedStringField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let length_doc_comment = format!("Fixed String with length {}", field.length);

    quote! {
        #[doc = #length_doc_comment]
        pub #field_ident : String,
    }
}

fn generate_fixed_record_field_decl(field: &FixedRecordField, lookup: &Lookup) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let fqn_field_type = lookup_fqn(format_type_name(&field.field_type).as_str(), lookup);
    let type_decl: syn::Type = syn::parse_str(fqn_field_type)
        .expect("Expected a valid Type for a FixedRecordField declaration.");

    quote! {
        pub #field_ident : #type_decl,
    }
}

fn generate_bit_record_field_decl(field: &BitRecordField, lookup: &Lookup) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl =
        if let Some(uids) = &field.enum_uid {
            let enum_type = uids
                .iter()
                .map(|uid| lookup_uid(*uid, lookup).to_string())
                .collect::<Vec<String>>();
            let enum_type = enum_type
                .first()
                .expect("Expected at least one Type for an EnumField declaration.");
            let enum_type = lookup_fqn(enum_type, lookup);
            let ty: syn::Type = syn::parse_str(enum_type)
                .expect("Expected a valid Type for an EnumField declaration.");
            ty
        } else {
            let ty: syn::Type = syn::parse_str(format_type_name(field.field_type.as_ref().expect(
                "Expected a type name for BitRecordField to be present as there is also no UID.",
            )).as_str())
                .expect("Expected valid input to parse a Type for a BitRecordField declaration.");
            ty
        };

    quote! {
        pub #field_ident : #type_decl,
    }
}

fn generate_adaptive_record_field_decl(
    field: &AdaptiveRecordField,
    lookup: &Lookup,
) -> TokenStream {
    // TODO figure out where the discriminant attribute is needed.
    // It is what makes the record 'adaptive', determining the contents (based on UID) of what follows in the record.
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl = if let Some(uids) = &field.enum_uid {
        let enum_type = uids
            .iter()
            .map(|uid| lookup_uid(*uid, lookup).to_string())
            .collect::<Vec<String>>();
        let enum_type = enum_type
            .first()
            .expect("Expected at least one Type for an AdaptiveRecordField declaration.");
        let enum_type = lookup_fqn(enum_type, lookup);
        let ty: syn::Type = syn::parse_str(enum_type)
            .expect("Expected a valid Type for an AdaptiveRecordField declaration.");
        ty
    } else {
        let ty: syn::Type = syn::parse_str(format_type_name(field.field_type.as_ref().expect(
            "Expected a type name for AdaptiveRecordField to be present as there is also no UID.",
        )).as_str())
            .expect("Expected valid input to parse a Type for a AdaptiveRecordField declaration.");
        ty
    };

    quote! {
        pub #field_ident : #type_decl,
    }
}

fn generate_variable_string_field_decl(field: &VariableStringField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    quote! {
        #[doc = "Variable String"]
        pub #field_ident : String,
    }
}

fn generate_array_field_decl(field: &Array) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.count_field.name));
    let type_decl = match field.type_field {
        ArrayFieldEnum::Numeric(inner) => { &inner.primitive_type }
        ArrayFieldEnum::Enum(inner) => { inner. }
        ArrayFieldEnum::FixedString(inner) => { "String" }
        ArrayFieldEnum::FixedRecord(inner) => { &inner.field_type }
        // TODO create macros / helper functions to easily get the right type for a field
        ArrayFieldEnum::BitRecord(inner) => { inner. }
    };

    quote! {
        pub #field_ident : Vec<#type_decl>,
    }
}

fn generate_bit_record(item: &BitRecord, lookup: &Lookup) -> TokenStream {
    let record_name = format_ident!("{}", format_type_name(&item.record_type));

    let fields = item
        .fields
        .iter()
        .map(|field| generate_bit_record_item_decl(field, lookup))
        .collect::<Vec<TokenStream>>();

    quote! {
        pub struct #record_name {
            #(#fields)*
        }
    }
}

fn generate_bit_record_item_decl(item: &BitRecordFieldEnum, lookup: &Lookup) -> TokenStream {
    match item {
        BitRecordFieldEnum::Enum(field) => generate_enum_bit_field_decl(field, lookup),
        BitRecordFieldEnum::Int(field) => generate_int_bit_field_decl(field, lookup),
        BitRecordFieldEnum::Bool(field) => generate_bool_bit_field_decl(field, lookup),
    }
}

fn generate_enum_bit_field_decl(field: &EnumBitField, lookup: &Lookup) -> TokenStream {
    let field_name = format_ident!("{}", format_field_name(field.name.as_str()));
    let field_type = match (field.size, &field.enum_uid) {
        (Some(size), None) => {
            let field_type = bit_field_size_to_primitive_type(size);
            let ty: syn::Type = syn::parse_str(field_type.as_str())
                .expect("Expected a valid Rust primitive type for a bit field declaration with a given size.");
            quote! { #ty }
        }
        (_, Some(uids)) => {
            let field_type = lookup_first_uid(uids, lookup);
            let ty: syn::Type = syn::parse_str(field_type)
                .expect("Expected a valid Type for an EnumBitField declaration.");
            quote! { #ty }
        }
        (None, None) => {
            quote! { bool }
        }
    };
    quote! { pub #field_name: #field_type, }
}

fn generate_bool_bit_field_decl(field: &BoolBitField, lookup: &Lookup) -> TokenStream {
    let field_name = format_ident!("{}", format_field_name(field.name.as_str()));
    quote! { pub #field_name: bool, }
}

fn generate_int_bit_field_decl(field: &IntBitField, lookup: &Lookup) -> TokenStream {
    let field_name = format_ident!("{}", format_field_name(field.name.as_str()));
    let field_type = bit_field_size_to_primitive_type(field.size.unwrap_or(1));
    let ty: syn::Type = syn::parse_str(field_type.as_str())
        .expect("Expected a valid primitive type for an IntBitField declaration.");
    quote! { pub #field_name: #ty, }
}

fn generate_adaptive_record(item: &AdaptiveRecord, lookup: &Lookup) -> TokenStream {
    todo!()
}

fn bit_field_size_to_primitive_type(size: usize) -> String {
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
    .to_string()
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
    println!("lookup_uid({uid})");
    let val = lookup
        .uid
        .get(&uid)
        .expect("Expected an existing type for uid.");

    val
}

#[inline]
fn lookup_fqn<'fqn>(type_name: &str, lookup: &'fqn Lookup) -> &'fqn str {
    lookup
        .fqn
        .get(type_name)
        .unwrap_or_else(|| panic!("Expected full qualified name for type {type_name}"))
}

#[cfg(test)]
mod tests {
    use crate::generation::bit_field_size_to_primitive_type;

    #[test]
    fn test_int_bit_field_type_to_primitive() {
        assert_eq!(bit_field_size_to_primitive_type(4), "u8".to_string());
        assert_eq!(bit_field_size_to_primitive_type(8), "u8".to_string());
        assert_eq!(bit_field_size_to_primitive_type(9), "u16".to_string());
        assert_eq!(bit_field_size_to_primitive_type(16), "u16".to_string());
        assert_eq!(bit_field_size_to_primitive_type(17), "u32".to_string());
        assert_eq!(bit_field_size_to_primitive_type(32), "u32".to_string());
        assert_eq!(bit_field_size_to_primitive_type(33), "u64".to_string());
        assert_eq!(bit_field_size_to_primitive_type(100), "u128".to_string());
    }
}
