use super::{
    AdaptiveRecordField, BitRecordField, EnumField, FixedRecordField, FixedStringField,
    GenerationItem, NumericField, Pdu, PduFieldsEnum,
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

// Approach
// Construct a tree structure, where each node represents a code unit (module, struct, enum, function, impl block).
// Each node in the tree contains the children contained in that code unit. E.g modules, which are parents, and Structs, impl blocks, etc are the leafs.
// Tree construction is an intermediate step before code generation.
// 1. Create a Vec<(String, TokenStream)>, filled with all pieces of generated code.
// 2. Walk the tree and concat all produced TokenStreams into one.
// 3. Write the final code into the output file.

struct Tree {
    next_id: usize,
    root: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            root: vec![],
        }
    }

    pub fn new_with_root(root: Node) -> Self {
        Self {
            next_id: 1,
            root: vec![root],
        }
    }
}

struct Node {
    id: usize,
    item: NodeItem,
    children: Vec<usize>,
}

enum NodeItem {
    Module(String),
    Unit(GenerationItem),
}

pub fn generate(items: &[GenerationItem], families: &[String]) -> TokenStream {
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
    let formatted_pdu_name = format_pdu_name(item.name_attr.as_str());
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
    let type_ident = format_ident!("{}", format_field_type(&field.field_type));

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
            format_field_type(field.field_type.as_ref().expect(
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
            format_field_type(field.field_type.as_ref().expect(
                "Expected a type name for AdaptiveRecordField to be present as there is also no UID."
            ))
        )
    };

    quote! {
        #field_ident : #type_ident,
    }
}

fn format_full_qualified_name(item: &GenerationItem, family: &str) -> String {
    if item.is_pdu() {
        format!(
            "crate::v8::{}::{}::{};",
            family,
            format_field_name(item.name().as_str()),
            item.name().as_str()
        )
    } else {
        format!("crate::v8::{}::{};", family, item.name().as_str())
    }
}

/// Formats the name of a PDU from the defined format into the code representation
/// The basic approach is to remove non-alphabetic characters and whitespace.
///
/// Examples:
/// - Create Entity -> CreateEntity
/// - Electromagnetic Emission -> ElectromagneticEmission
/// - Start/Resume -> StartResume
fn format_pdu_name(name: &str) -> String {
    name.replace(['/', ' '], "")
}

/// Formats the name of a field into the code representation
/// The basic approach is to remove non-alphabetic characters and convert `CamelCase` to `snake_case`.
fn format_field_name(name: &str) -> String {
    name.replace(['/', '-', '(', ')'], "")
        .replace([' '], "_")
        .to_lowercase()
}

/// Formats the name of a field type into the code representation
/// Types are formatted in CamelCase, without non-alphabetic characters and whitespace
fn format_field_type(type_name: &str) -> String {
    type_name.replace(['/', ' '], "")
}

fn numeric_type_to_field_type(ty: &str) -> Result<String, ()> {
    let field_type = match ty {
        "uint8" => Ok("u8"),
        "uint16" => Ok("u16"),
        "uint32" => Ok("u32"),
        "uint64" => Ok("u64"),
        "int8" => Ok("i8"),
        "int16" => Ok("i16"),
        "int32" => Ok("i32"),
        "int64" => Ok("i64"),
        "float32" => Ok("f32"),
        "float64" => Ok("f64"),
        _ => Err(()),
    };

    field_type.map(|t| t.to_string())
}

fn enum_type_to_field_type(ty: &str) -> Result<String, ()> {
    let field_type = match ty {
        "enum8" => Ok("u8"),
        "enum16" => Ok("u16"),
        "enum32" => Ok("u32"),
        _ => Err(()),
    };

    field_type.map(|t| t.to_string())
}

#[cfg(test)]
mod tests {
    use crate::generation::{format_field_name, format_field_type, format_pdu_name};

    #[test]
    fn test_format_pdu_name() {
        assert_eq!(format_pdu_name("Create Entity"), "CreateEntity");
        assert_eq!(
            format_pdu_name("Electromagnetic Emission"),
            "ElectromagneticEmission"
        );
        assert_eq!(format_pdu_name("Start/Resume"), "StartResume");
    }

    #[test]
    fn test_format_field_name() {
        assert_eq!(format_field_name("Entity Location"), "entity_location");
        assert_eq!(format_field_name("Ez"), "ez");
        assert_eq!(
            format_field_name("Emitter Name (Jammer)"),
            "emitter_name_jammer"
        );
    }

    #[test]
    fn test_format_field_type() {
        assert_eq!(format_field_type("Euler Angles"), "EulerAngles");
        assert_eq!(format_field_type("Entity Location"), "EntityLocation");
    }
}
