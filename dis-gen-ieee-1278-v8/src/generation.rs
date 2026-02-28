use super::{
    enum_type_to_field_type, format_field_name, format_type_name, numeric_type_to_field_type,
    AdaptiveRecordField, BitRecordField, EnumField, FixedRecordField, FixedStringField, FqnLookup,
    GenerationItem, NumericField, Pdu, PduFieldsEnum, UidLookup,
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

pub fn generate(
    items: &[GenerationItem],
    families: &[String],
    fqn_lookup: &FqnLookup,
    uid_lookup: &UidLookup,
) -> TokenStream {
    let core_contents = generate_core_units();
    let family_contents: Vec<TokenStream> = families
        .iter()
        .map(|family| generate_family_module(items, family.as_str()))
        .collect();
    let contents = quote! {

        #core_contents

        #(#family_contents)*

    };

    generate_module_with_name("v8", &contents)
}

fn generate_core_units() -> TokenStream {
    // TODO design required core data structures
    // TODO list all PDUs (and their headers) in this main enum, analogous to v7
    quote! {
        // use crate::v8::common_records::PduHeader;

        #[derive(Debug, Clone)]
        pub struct Pdu {
            // pub header: PduHeader,
            pub body: PduBody,
        }

        #[derive(Debug, Clone)]
        pub enum PduBody {
            Dummy,
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
fn generate_family_module(items: &[GenerationItem], family: &str) -> TokenStream {
    // 1. Filter items for this family
    // 2. Filter the PDUs and generate these in separate modules
    // 3. Filter the non-PDU items and generate the records in the family module
    // 4. Merge resulting TokenStreams

    let pdus = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_pdu())
        .map(|pdu| {
            if let GenerationItem::Pdu(pdu, _) = pdu {
                generate_pdu_module(pdu)
            } else {
                quote!( compile error; )
            }
        })
        .collect::<TokenStream>();

    let records = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && !item.is_pdu())
        .collect::<Vec<&GenerationItem>>();

    println!("{records:?}");

    generate_module_with_name(family, &pdus)
}

/// Generates all code related a PDU
fn generate_pdu_module(item: &Pdu) -> TokenStream {
    let formatted_pdu_name = format_type_name(item.name_attr.as_str());
    println!(
        "generate_pdu_module - {} - {}",
        item.name_attr, formatted_pdu_name
    );
    let ident_pdu_name = format_ident!("{}", formatted_pdu_name);

    // TODO required imports/uses to common fields
    // TODO decide if the header is part of the PDU struct >> No, consistent with v6/v7 implementation
    // TODO generate the pdu field declarations, with correct uses of enumerations and Records (needs the imports/uses)
    // TODO design PduBody traits: size, family, pduType. See BodyRaw, BodyInfo, blanket impls, serialisation, Interaction.

    let fields = item
        .fields
        .iter()
        .map(generate_field_decl)
        .collect::<Vec<TokenStream>>();
    let contents = quote! {
        pub struct #ident_pdu_name {
            #(#fields)*
        }
    };

    generate_module_with_name(formatted_pdu_name.as_str(), &contents)
}

fn generate_field_decl(field: &PduFieldsEnum) -> TokenStream {
    let decl = match field {
        PduFieldsEnum::Numeric(field) => generate_numeric_field_decl(field),
        PduFieldsEnum::Enum(field) => generate_enum_field_decl(field),
        PduFieldsEnum::FixedString(field) => generate_fixed_string_field_decl(field),
        PduFieldsEnum::FixedRecord(field) => generate_fixed_record_field_decl(field),
        PduFieldsEnum::BitRecord(field) => generate_bit_record_field_decl(field),
        PduFieldsEnum::AdaptiveRecord(field) => generate_adaptive_record_field_decl(field),
    };

    quote! {
        #decl
    }
}

fn generate_numeric_field_decl(field: &NumericField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_ident = format_ident!(
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
        #field_ident : #type_ident,
    }
}

fn generate_enum_field_decl(field: &EnumField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_ident = if let Some(uids) = &field.enum_uid {
        // FIXME get Enum type name that goes with the UID
        // format_ident!("{}", uids.first().unwrap().to_string())
        format_ident!("u8") // PLACEHOLDER
    } else {
        format_ident!(
            "{}",
            enum_type_to_field_type(&field.field_type).expect("Expected valid enum field type.")
        )
    };

    quote! {
        #field_ident : #type_ident,
    }
}

fn generate_fixed_string_field_decl(field: &FixedStringField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));

    quote! {
        #field_ident : String,
    }
}

fn generate_fixed_record_field_decl(field: &FixedRecordField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_ident = format_ident!("{}", format_type_name(&field.field_type));

    quote! {
        #field_ident : #type_ident,
    }
}

fn generate_bit_record_field_decl(field: &BitRecordField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_ident = if let Some(uids) = &field.enum_uid {
        // FIXME get Enum name that goes with the UID
        // format_ident!("{}", uids.first().unwrap().to_string())
        format_ident!("u8") // PLACEHOLDER
    } else {
        format_ident!(
            "{}",
            format_type_name(field.field_type.as_ref().expect(
                "Expected a type name for BitRecordField to be present as there is also no UID."
            ))
        )
    };

    quote! {
        #field_ident : #type_ident,
    }
}

fn generate_adaptive_record_field_decl(field: &AdaptiveRecordField) -> TokenStream {
    // TODO figure out where the discriminant attribute is needed.
    // It is what makes the record 'adaptive', determining the contents (based on UID) of what follows in the record.
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_ident = if let Some(uids) = &field.enum_uid {
        // FIXME get Enum name that goes with the UID
        // format_ident!("{}", uids.first().unwrap().to_string())
        format_ident!("u8") // PLACEHOLDER
    } else {
        format_ident!(
            "{}",
            format_type_name(field.field_type.as_ref().expect(
                "Expected a type name for AdaptiveRecordField to be present as there is also no UID."
            ))
        )
    };

    quote! {
        #field_ident : #type_ident,
    }
}
