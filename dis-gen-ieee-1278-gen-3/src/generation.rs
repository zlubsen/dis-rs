use super::Lookup;
// use crate::extraction::{
//     AdaptiveFormatEnum, AdaptiveRecord, AdaptiveRecordField, Array, ArrayFieldEnum, BitRecord,
//     BitRecordField, BitRecordFieldEnum, BoolBitField, EnumBitField, EnumField, ExtensionRecord,
//     ExtensionRecordFieldEnum, ExtractionItem, FixedRecord, FixedRecordField, FixedStringField,
//     IntBitField, NumericField, OpaqueData, Pdu, PduAndFixedRecordFieldsEnum, VariableStringField,
// };
use dis_gen_utils::{
    enum_type_to_field_type, format_field_name, format_pdu_module_name, format_type_name,
};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
// Module tree of generated sources:
// src/
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

pub const EXTENSION_RECORDS_MODULE_NAME: &str = "extension_records";
pub const BUILDER_MODULE_NAME: &str = "builder";
pub const BUILDER_TYPE_SUFFIX: &str = "Builder";

pub enum GenerationItem {
    Pdu(Pdu, String),
    FixedRecord(FixedRecord, String),
    BitRecord(BitRecord, String),
    AdaptiveRecord(AdaptiveRecord, String),
    ExtensionRecord(ExtensionRecord, String),
}

impl GenerationItem {
    pub(crate) fn family(&self) -> String {
        match self {
            GenerationItem::Pdu(_, fam) => fam.clone(),
            GenerationItem::FixedRecord(_, fam) => fam.clone(),
            GenerationItem::BitRecord(_, fam) => fam.clone(),
            GenerationItem::AdaptiveRecord(_, fam) => fam.clone(),
            GenerationItem::ExtensionRecord(_, fam) => fam.clone(),
        }
    }

    pub(crate) fn name(&self) -> String {
        match self {
            GenerationItem::Pdu(item, _) => item.name_attr.clone(),
            GenerationItem::FixedRecord(item, _) => item.record_type.clone(),
            GenerationItem::BitRecord(item, _) => item.record_type.clone(),
            GenerationItem::AdaptiveRecord(item, _) => item.record_type.clone(),
            GenerationItem::ExtensionRecord(item, _) => item.name_attr.clone(),
        }
    }

    /// Returns true when the item is a `PDU`
    pub(crate) fn is_pdu(&self) -> bool {
        matches!(self, GenerationItem::Pdu(_, _))
    }

    /// Returns true when the item is an `ExtensionRecord`
    pub(crate) fn is_extension_record(&self) -> bool {
        matches!(self, GenerationItem::ExtensionRecord(_, _))
    }

    /// Returns true when the item is not a PDU or an `ExtensionRecord`
    fn is_record(&self) -> bool {
        !self.is_pdu() && !self.is_extension_record()
    }
}

#[derive(Debug, Clone)]
pub struct NumericField {
    pub field_name: String,
    pub primitive_type: syn::Type,
    pub units: Option<String>,
    pub is_padding: bool,
}

#[derive(Debug, Clone)]
pub struct CountField {
    pub field_name: String,
    pub primitive_type: syn::Type,
}

#[derive(Debug, Clone)]
pub struct EnumField {
    pub field_name: String,
    pub field_type_fqn: syn::Type,
    pub is_discriminant: bool,
}

#[derive(Debug, Clone)]
pub struct FixedStringField {
    pub field_name: String,
    pub field_type: &'static str, // `String`
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct IntBitField {
    pub field_name: String,
    pub field_type: syn::Type,
    pub bit_position: usize,
    pub size: usize,
    pub units: Option<String>,
    pub is_padding: bool,
}

#[derive(Debug, Clone)]
pub struct EnumBitField {
    pub field_name: String,
    pub field_type: syn::Type,
    pub field_type_fqn: syn::Type,
    pub bit_position: usize,
    pub size: usize,
    pub is_discriminant: bool, // FIXME 'true' does not occur in the schemas
}

#[derive(Debug, Clone)]
pub struct BoolBitField {
    pub field_name: String,
    pub bit_position: usize,
}

#[derive(Debug, Clone)]
pub struct FixedRecordField {
    pub field_name: String,
    pub field_type_fqn: syn::Type,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct BitRecordField {
    pub field_name: String,
    pub field_type_fqn: syn::Type,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct AdaptiveRecordField {
    pub field_name: String,
    pub field_type_fqn: syn::Type,
    pub length: usize,
    pub discriminant_field_name: String,
}

#[derive(Debug, Clone)]
pub struct VariableString {
    pub count_field: CountField,
    pub string_field: VariableStringField,
}

#[derive(Debug, Clone)]
pub struct VariableStringField {
    pub field_name: String,
    pub field_type: &'static str,       // `String`
    pub fixed_number_of_strings: usize, // FIXME attribute does not occur in the schemas
}

#[derive(Debug, Clone)]
pub struct OpaqueDataField {
    pub field_name: String,
    pub field_type: &'static str, // `Vec<u8>`
}

#[derive(Debug, Clone)]
pub enum PduAndFixedRecordFieldsEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
    AdaptiveRecord(AdaptiveRecordField),
}

#[derive(Debug, Clone)]
pub enum ArrayFieldEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
}

#[derive(Debug, Clone)]
pub enum BitRecordFieldEnum {
    Enum(EnumBitField),
    Int(IntBitField),
    Bool(BoolBitField),
}

#[derive(Debug, Clone)]
pub enum AdaptiveFormatEnum {
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
pub enum ExtensionRecordFieldEnum {
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
pub struct Array {
    pub count_field: CountField,
    pub type_field: ArrayFieldEnum,
}

#[derive(Debug, Clone)]
pub struct OpaqueData {
    pub count_field: CountField,
    pub opaque_data_field: OpaqueDataField,
}

#[derive(Debug, Clone)]
pub struct FixedRecord {
    pub fields: Vec<PduAndFixedRecordFieldsEnum>,
    pub record_type: syn::Type,
    pub record_type_fqn: syn::Type,
    pub length: usize,
}

#[derive(Debug, Clone)]
pub struct BitRecord {
    pub fields: Vec<BitRecordFieldEnum>,
    pub record_type: syn::Type,
    pub record_type_fqn: syn::Type,
    pub size: usize,
}

#[derive(Debug, Clone)]
pub struct AdaptiveRecord {
    pub variants: Vec<AdaptiveFormatEnum>,
    pub record_type: syn::Type,
    pub record_type_fqn: syn::Type,
    pub length: usize,
    pub discriminant_start_value: usize,
}

#[derive(Debug, Clone)]
pub struct ExtensionRecordSet {
    pub count_field: CountField,
}

#[derive(Debug, Clone)]
struct PaddingTo16;

#[derive(Debug, Clone)]
struct PaddingTo32;

#[derive(Debug, Clone)]
struct PaddingTo64;

#[derive(Debug, Clone)]
pub struct ExtensionRecord {
    pub record_name: String,
    pub record_name_fqn: syn::Type,
    pub record_type_enum: usize,
    pub base_length: usize,
    pub is_variable: bool,
    pub record_type_field: EnumField,
    pub record_length_field: NumericField,
    pub fields: Vec<ExtensionRecordFieldEnum>,
    pub padding_to_64_field: Option<PaddingTo64>,
}

#[derive(Debug, Clone)]
pub struct Pdu {
    pub pdu_name: String,
    pub pdu_name_fqn: syn::Type,
    pub pdu_type: usize,
    pub protocol_family: usize,
    pub base_length: usize,
    pub header_field: FixedRecordField,
    pub fields: Vec<PduAndFixedRecordFieldsEnum>,
    pub extension_record_set: ExtensionRecordSet,
}

pub fn generate(items: &[ExtractionItem], families: &[String], lookup: &Lookup) -> TokenStream {
    let core_contents = generate_core_units(items, lookup);
    let family_contents: Vec<TokenStream> = families
        .iter()
        .map(|family| {
            let generated = generate_family_module(items, family.as_str(), lookup);
            let _ = syn::parse_file(&generated.to_string())
                .expect("Error parsing intermediate generated code for pretty printing.");
            generated
        })
        .collect();
    quote! {
        #[expect(arithmetic_overflow, reason = "Intentionally trigger a lint warning")]

        #core_contents

        #(#family_contents)*

    }
}

fn generate_core_units(items: &[ExtractionItem], lookup: &Lookup) -> TokenStream {
    // FIXME these parts should be in the lib itself as regular code, whenever possible
    // TODO design required core data structures
    // TODO PduBody: list all PDUs (and their headers) in a main enum, analogous to v7
    // TODO ExtensionRecordBody: list all extension records in an enum, analogous to PduBody

    let pdu_body_variants = items
        .iter()
        .filter(|&it| it.is_pdu())
        .map(|pdu| generate_body_variant(pdu, lookup))
        .collect::<TokenStream>();

    let extension_record_variants = items
        .iter()
        .filter(|&it| it.is_extension_record())
        .map(|er| generate_body_variant(er, lookup))
        .collect::<TokenStream>();

    quote! {
        use crate::common_records::PDUHeader;

        pub trait BodyRaw {
            type Builder;

            #[must_use]
            fn builder() -> Self::Builder;

            #[must_use]
            fn into_builder(self) -> Self::Builder;

            #[must_use]
            fn into_pdu_body(self) -> PduBody;
        }

        impl<T: BodyRaw> From<T> for PduBody {
            #[inline]
            fn from(value: T) -> Self {
                value.into_pdu_body()
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct Pdu {
            pub header: PDUHeader,
            pub body: PduBody,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum PduBody {
            Other(Vec<u8>),
            #pdu_body_variants
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct ExtensionRecord {
            pub record_type: crate::enumerations::ExtensionRecordTypes,
            pub record_length: usize,
            pub body: crate::ExtensionRecordBody,
        }

        #[derive(Debug, Clone, PartialEq)]
        pub enum ExtensionRecordBody {
            #extension_record_variants
        }
    }
}

fn generate_body_variant(variant: &ExtractionItem, lookup: &Lookup) -> TokenStream {
    let variant_name = format_type_name(&variant.name());
    let fqn_name = lookup_fqn(&variant_name, lookup);
    let variant_name = format_ident!("{variant_name}");
    let variant_type: syn::Type = syn::parse_str(fqn_name)
        .expect("Expected a valid PDU/ExtensionRecord FQN name to build PduBody or ExtensionRecordBody variants.");

    quote! {
        #variant_name ( #variant_type ),
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
fn generate_family_module(items: &[ExtractionItem], family: &str, lookup: &Lookup) -> TokenStream {
    // 1. Filter the PDUs for this family and generate these in separate modules
    let pdus = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_pdu())
        .map(|pdu| {
            if let ExtractionItem::Pdu(pdu, _) = pdu {
                generate_pdu_module(pdu, lookup)
            } else {
                panic!("ExtractionItem is not a PDU.")
            }
        })
        .collect::<TokenStream>();

    // 3. Filter the ExtensionRecord items for this family and generate the records in a separate (sub)module
    let extension_records = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_extension_record())
        .map(|item| {
            if let ExtractionItem::ExtensionRecord(record, _family) = item {
                generate_extension_record(record, lookup)
            } else {
                panic!("ExtractionItem is not an ExtensionRecord.")
            }
        })
        .collect::<TokenStream>();
    let extension_records =
        generate_module_with_name(EXTENSION_RECORDS_MODULE_NAME, &extension_records);

    // 3. Filter the remaining non-PDU items for this family and generate the records in the family module
    let records = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_record())
        .map(|item| match item {
            ExtractionItem::FixedRecord(record, _family) => generate_fixed_record(record, lookup),
            ExtractionItem::BitRecord(record, _family) => generate_bit_record(record, lookup),
            ExtractionItem::AdaptiveRecord(record, _family) => {
                generate_adaptive_record(record, lookup)
            }
            ExtractionItem::ExtensionRecord(record, _family) => {
                panic!("ExtractionItem is an ExtensionRecord.")
            }
            ExtractionItem::Pdu(_, _) => panic!("ExtractionItem is not a Record."),
        })
        .collect::<TokenStream>();

    // 3. Merge resulting TokenStreams
    let contents = quote! { #pdus #extension_records #records };

    generate_module_with_name(family, &contents)
}

/// Generates all code related a PDU
fn generate_pdu_module(item: &Pdu, lookup: &Lookup) -> TokenStream {
    let formatted_pdu_name = format_type_name(item.name_attr.as_str());
    let fqn_pdu_name = lookup_fqn(&formatted_pdu_name, lookup);
    let pdu_name_ident = format_ident!("{}", formatted_pdu_name);
    let fqn_pdu_name_ident: syn::Type =
        syn::parse_str(&fqn_pdu_name).expect("Expected a valid FQN type");
    let pdu_module_name = format_pdu_module_name(item.name_attr.as_str());
    let builder_name_ident = format_ident!("{}{BUILDER_TYPE_SUFFIX}", formatted_pdu_name);

    // TODO design PduBody traits: size, family, pduType. See BodyRaw, BodyInfo, blanket impls, serialisation, Interaction.
    let pdu_trait_impls = generate_pdu_trait_impls(&pdu_name_ident, &builder_name_ident);
    let builder_content =
        builders::generate_pdu_builder(item, &fqn_pdu_name_ident, &builder_name_ident, lookup);
    let builder_module = generate_module_with_name(BUILDER_MODULE_NAME, &builder_content);

    let fields = item
        .fields
        .iter()
        .map(|field| generate_pdu_and_fixed_field_decl(field, lookup))
        .collect::<Vec<TokenStream>>();

    let contents = quote! {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct #pdu_name_ident {
            #(#fields)*
            pub extension_records: Vec<crate::ExtensionRecord>,
        }

        #pdu_trait_impls

        #builder_module
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
        #[derive(Debug, Default, Clone, PartialEq)]
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
        #[doc = #record_type_doc_comment]
        #[derive(Debug, Default, Clone, PartialEq)]
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
        ExtensionRecordFieldEnum::Array(field) => generate_array_field_decl(field, lookup),
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
    let type_decl = type_for_numeric_field(field);
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
    let type_decl = type_for_enum_field(field, lookup);

    quote! {
        pub #field_ident : #type_decl,
    }
}

fn generate_fixed_string_field_decl(field: &FixedStringField) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl = type_for_fixed_string_field();
    let length_doc_comment = format!("Fixed String with length {}", field.length);

    quote! {
        #[doc = #length_doc_comment]
        pub #field_ident : #type_decl,
    }
}

fn generate_fixed_record_field_decl(field: &FixedRecordField, lookup: &Lookup) -> TokenStream {
    if must_skip_field_decl(field.name.as_str()) {
        return quote! {};
    }

    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl = type_for_fixed_record_field(field, lookup);

    quote! {
        pub #field_ident : #type_decl,
    }
}

fn generate_bit_record_field_decl(field: &BitRecordField, lookup: &Lookup) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl = type_for_bit_record_field(field, lookup);

    quote! {
        pub #field_ident : #type_decl,
    }
}

fn generate_adaptive_record_field_decl(
    field: &AdaptiveRecordField,
    lookup: &Lookup,
) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.name));
    let type_decl = type_for_adaptive_record_field(field, lookup);

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

fn generate_array_field_decl(field: &Array, lookup: &Lookup) -> TokenStream {
    let (field_ident, type_decl) = match &field.type_field {
        ArrayFieldEnum::Numeric(inner) => (
            format_ident!("{}", format_field_name(&inner.name)),
            type_for_numeric_field(inner),
        ),
        ArrayFieldEnum::Enum(inner) => (
            format_ident!("{}", format_field_name(&inner.name)),
            type_for_enum_field(inner, lookup),
        ),
        ArrayFieldEnum::FixedString(inner) => (
            format_ident!("{}", format_field_name(&inner.name)),
            type_for_fixed_string_field(),
        ),
        ArrayFieldEnum::FixedRecord(inner) => (
            format_ident!("{}", format_field_name(&inner.name)),
            type_for_fixed_record_field(inner, lookup),
        ),
        ArrayFieldEnum::BitRecord(inner) => (
            format_ident!("{}", format_field_name(&inner.name)),
            type_for_bit_record_field(inner, lookup),
        ),
    };

    quote! {
        pub #field_ident : Vec<#type_decl>,
    }
}

fn generate_opaque_field_decl(field: &OpaqueData) -> TokenStream {
    let field_ident = format_ident!("{}", format_field_name(&field.opaque_data_field.name));

    quote! {
        pub #field_ident : Vec<u8>,
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
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct #record_name {
            #(#fields)*
        }
    }
}

fn generate_bit_record_item_decl(item: &BitRecordFieldEnum, lookup: &Lookup) -> TokenStream {
    match item {
        BitRecordFieldEnum::Enum(field) => generate_enum_bit_field_decl(field, lookup),
        BitRecordFieldEnum::Int(field) => generate_int_bit_field_decl(field),
        BitRecordFieldEnum::Bool(field) => generate_bool_bit_field_decl(field),
    }
}

fn generate_enum_bit_field_decl(field: &EnumBitField, lookup: &Lookup) -> TokenStream {
    if must_skip_field_decl(field.name.as_str()) {
        return quote! {};
    }

    let field_name = format_ident!("{}", format_field_name(field.name.as_str()));
    let field_type = match (field.size, &field.enum_uid) {
        (Some(size), None) => {
            let field_type = field_size_to_primitive_type(size);
            quote! { #field_type }
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

fn generate_bool_bit_field_decl(field: &BoolBitField) -> TokenStream {
    if must_skip_field_decl(field.name.as_str()) {
        return quote! {};
    }

    let field_name = format_ident!("{}", format_field_name(field.name.as_str()));
    quote! { pub #field_name: bool, }
}

fn generate_int_bit_field_decl(field: &IntBitField) -> TokenStream {
    if must_skip_field_decl(field.name.as_str()) {
        return quote! {};
    }

    let field_name = format_ident!("{}", format_field_name(field.name.as_str()));
    let field_type = field_size_to_primitive_type(field.size.unwrap_or(1));
    quote! { pub #field_name: #field_type, }
}

fn generate_adaptive_record(item: &AdaptiveRecord, lookup: &Lookup) -> TokenStream {
    let record_name = format_ident!("{}", format_type_name(&item.record_type));

    let variants = item
        .variants
        .iter()
        .map(|field| generate_adaptive_record_variant(field, lookup))
        .collect::<Vec<TokenStream>>();

    quote! {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub enum #record_name {
            #[default]
            None,
            #(#variants)*
        }
    }
}

fn generate_adaptive_record_variant(variant: &AdaptiveFormatEnum, lookup: &Lookup) -> TokenStream {
    if let AdaptiveFormatEnum::BitRecord(bit_variant) = variant {
        let variant_name = format_ident!("{}", format_type_name(&bit_variant.name));
        let variant_type = match (&bit_variant.enum_uid, &bit_variant.field_type) {
            (Some(uids), None) => lookup_fqn(lookup_first_uid(uids, lookup), lookup),
            (None, Some(type_name)) => lookup_fqn(&format_type_name(type_name), lookup),
            (_, _) => {
                panic!(
                    "Cannot determine the type of AdaptiveRecord BitRecordField {}",
                    bit_variant.name
                );
            }
        };
        let variant_type: syn::Type = syn::parse_str(variant_type)
            .expect("Expected a valid Type for an AdaptiveRecord enum variant declaration.");

        quote! {
            #variant_name ( #variant_type ),
        }
    } else {
        todo!("There are no AdaptiveRecords having variants other than BitRecordFields in the schema definitions at this moment.")
    }
}

fn generate_pdu_trait_impls(pdu_name_ident: &Ident, builder_name_ident: &Ident) -> TokenStream {
    // TODO further develop the core traits (as in gen 2: BodyInfo, Interaction)
    quote! {
        impl crate::BodyRaw for #pdu_name_ident {
            type Builder = builder::#builder_name_ident;

            fn builder() -> Self::Builder {
                Self::Builder::new()
            }

            fn into_builder(self) -> Self::Builder {
                Self::Builder::new_from_body(self)
            }

            fn into_pdu_body(self) -> crate::PduBody {
                crate::PduBody::#pdu_name_ident(self)
            }
        }
    }
}

fn field_size_to_primitive_type(size: usize) -> syn::Type {
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
fn type_for_numeric_field(field: &NumericField) -> syn::Type {
    match field.primitive_type.as_str() {
        "uint8" => syn::parse_str("u8"),
        "uint16" => syn::parse_str("u16"),
        "uint32" => syn::parse_str("u32"),
        "uint64" => syn::parse_str("u64"),
        "int8" => syn::parse_str("i8"),
        "int16" => syn::parse_str("i16"),
        "int32" => syn::parse_str("i32"),
        "int64" => syn::parse_str("i64"),
        "float32" => syn::parse_str("f32"),
        "float64" => syn::parse_str("f64"),
        _ => syn::parse_str("Type Unknown"), // fail on purpose
    }
    .unwrap_or_else(|_| {
        panic!(
            "Expected a valid Type for NumericField {}, found {}.",
            field.name, field.primitive_type
        )
    })
}

#[inline]
fn type_for_enum_field(field: &EnumField, lookup: &Lookup) -> syn::Type {
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
fn type_for_fixed_record_field(field: &FixedRecordField, lookup: &Lookup) -> syn::Type {
    let fqn_field_type = lookup_fqn(format_type_name(&field.field_type).as_str(), lookup);
    syn::parse_str(fqn_field_type).unwrap_or_else(|_| {
        panic!(
            "Expected a valid Type for a FixedRecordField declaration, found '{fqn_field_type}'."
        )
    })
}

#[inline]
fn type_for_bit_record_field(field: &BitRecordField, lookup: &Lookup) -> syn::Type {
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
fn type_for_adaptive_record_field(field: &AdaptiveRecordField, lookup: &Lookup) -> syn::Type {
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

mod builders {
    use crate::extraction::{Pdu, PduAndFixedRecordFieldsEnum};
    use crate::generation::{lookup_first_uid, lookup_fqn, type_for_enum_field};
    use crate::Lookup;
    use dis_gen_utils::{format_field_name, format_type_name};
    use proc_macro2::{Ident, TokenStream};
    use quote::{format_ident, quote};

    pub fn generate_pdu_builder(
        item: &Pdu,
        fqn_pdu_name_ident: &syn::Type,
        builder_name_ident: &Ident,
        lookup: &Lookup,
    ) -> TokenStream {
        // TODO generate with_ functions for all fields
        let with_functions = item
            .fields
            .iter()
            .map(|field| generate_pdu_builder_functions(field, lookup))
            .collect::<Vec<TokenStream>>();

        quote! {
            pub struct #builder_name_ident(#fqn_pdu_name_ident);

            impl Default for #builder_name_ident {
                fn default() -> Self {
                    Self::new()
                }
            }

            impl #builder_name_ident {
                #[must_use]
                pub fn new() -> Self {
                    #builder_name_ident(#fqn_pdu_name_ident::default())
                }

                #[must_use]
                pub fn new_from_body(body: #fqn_pdu_name_ident) -> Self {
                    #builder_name_ident(body)
                }

                #[must_use]
                pub fn build(self) -> #fqn_pdu_name_ident {
                    self.0
                }

                #(#with_functions)*
            }
        }
    }

    fn generate_pdu_builder_functions(
        field: &PduAndFixedRecordFieldsEnum,
        lookup: &Lookup,
    ) -> TokenStream {
        println!("{field:?}");
        let tokens = match field {
            PduAndFixedRecordFieldsEnum::Numeric(field) => generate_pdu_builder_with_function(
                &format_field_name(&field.name),
                &field.primitive_type,
            ),
            PduAndFixedRecordFieldsEnum::Enum(field) => generate_pdu_builder_with_function(
                &format_field_name(&field.name),
                type_for_enum_field(field, lookup),
            ),
            PduAndFixedRecordFieldsEnum::FixedString(field) => {
                generate_pdu_builder_with_function(&format_field_name(&field.name), "Into<String>")
            }
            PduAndFixedRecordFieldsEnum::FixedRecord(field) => generate_pdu_builder_with_function(
                &format_field_name(&field.name),
                lookup_fqn(&format_type_name(&field.field_type), lookup),
            ),
            PduAndFixedRecordFieldsEnum::BitRecord(field) => {
                let type_name = match (&field.enum_uid, &field.field_type) {
                    (Some(uids), None) => lookup_first_uid(uids, lookup),
                    (None, Some(enum_type)) => lookup_fqn(&format_type_name(enum_type), lookup),
                    _ => panic!("BitRecordField has no valid type"),
                };
                generate_pdu_builder_with_function(&format_field_name(&field.name), type_name)
            }
            PduAndFixedRecordFieldsEnum::AdaptiveRecord(field) => {
                let type_name = match (&field.enum_uid, &field.field_type) {
                    (Some(uids), None) => lookup_first_uid(uids, lookup),
                    (None, Some(enum_type)) => lookup_fqn(&format_type_name(enum_type), lookup),
                    _ => panic!("BitRecordField has no valid type"),
                };
                generate_pdu_builder_with_function(&format_field_name(&field.name), type_name)
            }
        };
        println!("{tokens}");
        syn::parse_file(&tokens.to_string()).unwrap();
        tokens
    }

    fn generate_pdu_builder_with_function(field_name: &str, type_name: &str) -> TokenStream {
        let function_name_ident = format_ident!("with_{field_name}");
        let field_ident = format_ident!("{field_name}");
        let field_type: syn::Type = syn::parse_str(type_name).unwrap_or_else(|_| {
            panic!("Expected a valid field Type for Builder function 'with_{field_name}', found '{type_name}'")
        });
        let assignment_value = if type_name.starts_with("Into<") {
            quote! { #field_ident.into() }
        } else {
            quote! { #field_ident }
        };
        quote! {
                #[must_use]
                pub fn #function_name_ident(mut self, #field_ident: #field_type) -> Self {
                    self.0.#field_ident = #assignment_value;
                    self
                }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::generation::field_size_to_primitive_type;
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
