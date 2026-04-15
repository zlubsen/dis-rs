use crate::constants::{
    BUILDER_MODULE_NAME, BUILDER_TYPE_SUFFIX, EXTENSION_RECORDS_MODULE_NAME, PARSER_MODULE_NAME,
    WRITER_MODULE_NAME,
};
use crate::generation::parsers::generate_extension_record_body_parser;
use crate::generation::writers::generate_extension_record_body_writer;
use crate::pre_processing::{finalise_type, to_tokens};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

/// Module tree of generated sources:
/// `src/`
///    `common_records/`           // Containing all common records
///        `parser`
///        `writer`
///    `family_x/`
///        `parser`
///        `writer`
///        `pdu_x/`                // Containing specific PDU x.
///            `builder`
///            `model`
///            `parser`
///            `writer`

pub(crate) enum GenerationItem {
    Pdu(Pdu, String),
    FixedRecord(FixedRecord, String),
    BitRecord(BitRecord, String),
    AdaptiveRecord(AdaptiveRecord, String),
    ExtensionRecord(ExtensionRecord, String),
}

impl GenerationItem {
    pub(crate) fn family(&self) -> String {
        #[allow(clippy::match_same_arms)]
        match self {
            GenerationItem::Pdu(_, fam) => fam.clone(),
            GenerationItem::FixedRecord(_, fam) => fam.clone(),
            GenerationItem::BitRecord(_, fam) => fam.clone(),
            GenerationItem::AdaptiveRecord(_, fam) => fam.clone(),
            GenerationItem::ExtensionRecord(_, fam) => fam.clone(),
        }
    }

    pub(crate) fn variant_name(&self) -> Option<&str> {
        match self {
            GenerationItem::Pdu(item, _) => Some(&item.type_name),
            GenerationItem::FixedRecord(_item, _) => None,
            GenerationItem::BitRecord(_item, _) => None,
            GenerationItem::AdaptiveRecord(_item, _) => None,
            GenerationItem::ExtensionRecord(item, _) => Some(&item.record_type_variant_name),
        }
    }

    pub(crate) fn type_name(&self) -> Option<&str> {
        match self {
            GenerationItem::Pdu(item, _) => Some(&item.type_name),
            GenerationItem::FixedRecord(_item, _) => None,
            GenerationItem::BitRecord(_item, _) => None,
            GenerationItem::AdaptiveRecord(_item, _) => None,
            GenerationItem::ExtensionRecord(item, _) => Some(&item.type_name),
        }
    }

    pub(crate) fn type_path(&self) -> &TokenStream {
        match self {
            GenerationItem::Pdu(item, _) => &item.type_path,
            GenerationItem::FixedRecord(item, _) => &item.type_path,
            GenerationItem::BitRecord(item, _) => &item.type_path,
            GenerationItem::AdaptiveRecord(item, _) => &item.type_path,
            GenerationItem::ExtensionRecord(item, _) => &item.type_path,
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

#[derive(Clone)]
pub(crate) struct NumericField {
    pub field_name: String,
    pub primitive_type: TokenStream,
    pub units: Option<String>,
    pub is_padding: bool,
    pub parser_function: TokenStream,
    pub writer_function: TokenStream,
    pub length: usize,
}

#[derive(Clone)]
pub(crate) struct CountField {
    pub field_name: String,
    pub primitive_type: TokenStream,
    pub parser_function: TokenStream,
    pub writer_function: TokenStream,
}

#[derive(Clone, Debug)]
pub(crate) struct EnumField {
    pub field_name: String,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub is_discriminant: bool,
    pub parser_function: TokenStream,
    pub parser_must_convert_to_enum: bool,
    pub writer_function: TokenStream,
    pub length: usize,
}

#[derive(Clone)]
pub(crate) struct FixedStringField {
    pub field_name: String,
    pub field_type: &'static str, // `String`
    pub length: usize,
    pub parser_function: TokenStream,
}

#[derive(Clone)]
pub(crate) struct IntBitField {
    pub field_name: String,
    pub field_type: TokenStream,
    pub bit_position: usize,
    pub size: usize,
    pub units: Option<String>,
    pub is_padding: bool,
}

#[derive(Clone)]
pub(crate) struct EnumBitField {
    pub field_name: String,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub bit_position: usize,
    pub size: usize,
    pub is_discriminant: bool, // FIXME 'true' does not occur in the schemas
}

#[derive(Clone)]
pub(crate) struct BoolBitField {
    pub field_name: String,
    pub bit_position: usize,
}

#[derive(Clone)]
pub(crate) struct FixedRecordField {
    pub field_name: String,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub length: usize,
    pub parser_function: TokenStream,
}

#[derive(Clone)]
pub(crate) struct BitRecordField {
    pub field_name: String,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub size: usize,
    pub parser_function: TokenStream,
}

#[derive(Clone)]
pub(crate) struct AdaptiveRecordField {
    pub field_name: String,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub length: usize,
    pub discriminant_field_name: String,
    pub discriminant_field_type: TokenStream,
    pub parser_function: TokenStream,
}

#[derive(Clone)]
pub(crate) struct VariableString {
    pub count_field: CountField,
    pub string_field: VariableStringField,
}

#[derive(Clone)]
pub(crate) struct VariableStringField {
    pub field_name: String,
    pub field_type: &'static str,       // `String`
    pub fixed_number_of_strings: usize, // FIXME attribute does not occur in the schemas
    pub parser_function: TokenStream,
}

#[derive(Clone)]
pub(crate) struct OpaqueDataField {
    pub field_name: String,
    pub field_type: &'static str, // `Vec<u8>`
    pub parser_function: TokenStream,
}

#[derive(Clone)]
pub(crate) enum PduAndFixedRecordFieldsEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
    AdaptiveRecord(AdaptiveRecordField),
}

impl PduAndFixedRecordFieldsEnum {
    pub(crate) fn field_name(&self) -> &str {
        match self {
            PduAndFixedRecordFieldsEnum::Numeric(f) => &f.field_name,
            PduAndFixedRecordFieldsEnum::Enum(f) => &f.field_name,
            PduAndFixedRecordFieldsEnum::FixedString(f) => &f.field_name,
            PduAndFixedRecordFieldsEnum::FixedRecord(f) => &f.field_name,
            PduAndFixedRecordFieldsEnum::BitRecord(f) => &f.field_name,
            PduAndFixedRecordFieldsEnum::AdaptiveRecord(f) => &f.field_name,
        }
    }

    pub(crate) fn is_discriminant(&self) -> Option<&EnumField> {
        match self {
            PduAndFixedRecordFieldsEnum::Enum(field) => {
                if field.is_discriminant {
                    Some(field)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub(crate) fn has_discriminant(&self) -> Option<&AdaptiveRecordField> {
        match self {
            PduAndFixedRecordFieldsEnum::AdaptiveRecord(field) => Some(field),
            _ => None,
        }
    }

    pub(crate) fn is_padding(&self) -> bool {
        match self {
            PduAndFixedRecordFieldsEnum::Numeric(f) => f.is_padding,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub enum ArrayFieldEnum {
    Numeric(NumericField),
    Enum(EnumField),
    FixedString(FixedStringField),
    FixedRecord(FixedRecordField),
    BitRecord(BitRecordField),
}

impl ArrayFieldEnum {
    pub fn field_name(&self) -> &str {
        match self {
            ArrayFieldEnum::Numeric(f) => &f.field_name,
            ArrayFieldEnum::Enum(f) => &f.field_name,
            ArrayFieldEnum::FixedString(f) => &f.field_name,
            ArrayFieldEnum::FixedRecord(f) => &f.field_name,
            ArrayFieldEnum::BitRecord(f) => &f.field_name,
        }
    }
}

#[derive(Clone)]
pub(crate) enum BitRecordFieldEnum {
    Enum(EnumBitField),
    Int(IntBitField),
    Bool(BoolBitField),
}

impl BitRecordFieldEnum {
    pub(crate) fn field_name(&self) -> &str {
        match self {
            BitRecordFieldEnum::Enum(f) => &f.field_name,
            BitRecordFieldEnum::Int(f) => &f.field_name,
            BitRecordFieldEnum::Bool(f) => &f.field_name,
        }
    }

    pub(crate) fn is_padding(&self) -> bool {
        match self {
            BitRecordFieldEnum::Enum(_f) => false,
            BitRecordFieldEnum::Int(f) => f.is_padding,
            BitRecordFieldEnum::Bool(_f) => false,
        }
    }
}

#[derive(Clone)]
pub(crate) enum AdaptiveFormatEnum {
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

#[derive(Clone)]
pub(crate) enum ExtensionRecordFieldEnum {
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

impl ExtensionRecordFieldEnum {
    pub(crate) fn field_name(&self) -> &str {
        match self {
            ExtensionRecordFieldEnum::Numeric(f) => &f.field_name,
            ExtensionRecordFieldEnum::Enum(f) => &f.field_name,
            ExtensionRecordFieldEnum::FixedString(f) => &f.field_name,
            ExtensionRecordFieldEnum::VariableString(f) => &f.string_field.field_name,
            ExtensionRecordFieldEnum::FixedRecord(f) => &f.field_name,
            ExtensionRecordFieldEnum::BitRecord(f) => &f.field_name,
            ExtensionRecordFieldEnum::Array(f) => f.type_field.field_name(),
            ExtensionRecordFieldEnum::AdaptiveRecord(f) => &f.field_name,
            ExtensionRecordFieldEnum::Opaque(f) => &f.opaque_data_field.field_name,
            // TODO are these correct names?
            ExtensionRecordFieldEnum::PaddingTo16 => "padding",
            ExtensionRecordFieldEnum::PaddingTo32 => "padding",
        }
    }

    pub(crate) fn is_padding(&self) -> bool {
        match self {
            ExtensionRecordFieldEnum::Numeric(f) => f.is_padding,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub(crate) struct Array {
    pub count_field: CountField,
    pub type_field: ArrayFieldEnum,
}

#[derive(Clone)]
pub(crate) struct OpaqueData {
    pub count_field: CountField,
    pub opaque_data_field: OpaqueDataField,
}

#[derive(Clone)]
pub(crate) struct FixedRecord {
    pub fields: Vec<PduAndFixedRecordFieldsEnum>,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub length: usize,
    pub parser_function: TokenStream,
    pub has_external_discriminants: bool,
}

#[derive(Clone)]
pub(crate) struct BitRecord {
    pub fields: Vec<BitRecordFieldEnum>,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub size: usize,
    pub parser_function: TokenStream,
    pub value_parser: TokenStream,
}

#[derive(Clone)]
pub(crate) struct AdaptiveRecord {
    pub variants: Vec<BitRecordField>,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub length: usize,
    pub discriminant_start_value: usize,
    pub discriminant_type: TokenStream,
    pub discriminant_primitive_type: TokenStream,
    pub parser_function: TokenStream,
}

#[derive(Clone)]
pub(crate) struct ExtensionRecordSet {
    pub count_field: CountField,
}

#[derive(Clone)]
pub(crate) struct PaddingTo16;

#[derive(Clone)]
pub(crate) struct PaddingTo32;

#[derive(Clone)]
pub(crate) struct PaddingTo64;

#[derive(Clone)]
pub(crate) struct ExtensionRecord {
    pub type_name: String,
    pub type_path: TokenStream,
    pub record_type_enum: usize,
    pub record_type_variant_name: String,
    pub base_length: usize,
    pub is_variable: bool,
    pub record_type_field: EnumField,
    pub record_length_field: NumericField,
    pub fields: Vec<ExtensionRecordFieldEnum>,
    pub padding_to_64_field: Option<PaddingTo64>,
    pub parser_function: TokenStream,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone)]
pub(crate) struct Pdu {
    pub pdu_module_name: String,
    pub type_name: String,
    pub type_path: TokenStream,
    pub pdu_type: usize,
    pub pdu_type_name: TokenStream,
    pub protocol_family: usize,
    pub base_length: usize,
    pub header_field: FixedRecordField,
    pub fields: Vec<PduAndFixedRecordFieldsEnum>,
    pub extension_record_set: ExtensionRecordSet,
    pub parser_function: TokenStream,
}

pub(crate) fn generate(items: &[GenerationItem], families: &[String]) -> TokenStream {
    let core_contents = generate_core_units(items);
    let family_model_contents: Vec<TokenStream> = families
        .iter()
        .map(|family| {
            let generated = generate_family_module(items, family.as_str());

            // FIXME remove when finished
            let _ = syn::parse_file(&generated.to_string()).expect(
                "Error parsing 'family modules' intermediate generated code for pretty printing.",
            );
            generated
        })
        .collect();
    let parsers = super::parsers::generate_common_parsers(items);
    let writers = super::writers::generate_common_writers(items);

    println!("{parsers}");
    let _ = syn::parse_file(&parsers.to_string())
        .expect("Error parsing 'parsers' intermediate generated code for pretty printing.");

    println!("{writers}");
    let _ = syn::parse_file(&writers.to_string())
        .expect("Error parsing 'writers' intermediate generated code for pretty printing.");

    quote! {
        #[expect(arithmetic_overflow, reason = "Intentionally trigger a lint warning")]
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #core_contents

        #(#family_model_contents)*

        #parsers

        #writers
    }
}

fn generate_core_units(items: &[GenerationItem]) -> TokenStream {
    let (pdu_body_variants, pdu_body_type_variants, pdu_body_length_variants) = items
        .iter()
        .filter(|&it| it.is_pdu())
        .map(|pdu| {
            (
                generate_body_variant(pdu),
                generate_body_type_variant(pdu),
                generate_body_length_variant(pdu),
            )
        })
        .collect::<(TokenStream, TokenStream, TokenStream)>();

    let (
        extension_record_variants,
        extension_record_type_variants, // TODO record_type impl for determining header values
        extension_record_length_variants,
    ) = items
        .iter()
        .filter(|&it| it.is_extension_record())
        .map(|er| {
            (
                generate_body_variant(er),
                generate_body_type_variant(er),
                generate_body_length_variant(er),
            )
        })
        .collect::<(TokenStream, TokenStream, TokenStream)>();

    quote! {
        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
        #[cfg_attr(feature = "serde", serde(tag = "type"))]
        pub enum PduBody {
            Other(crate::other_pdu::model::Other),
            #pdu_body_variants
        }

        impl PduBody {
            pub fn body_length(&self) -> u16 {
                match self {
                    #pdu_body_length_variants
                    PduBody::Other(body) => body.body_length(),
                }
            }

            pub fn body_type(&self) -> crate::enumerations::DISPDUType {
                match self {
                    #pdu_body_type_variants
                    PduBody::Other(body) => body.body_type(),
                }
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
        #[cfg_attr(feature = "serde", serde(tag = "type"))]
        pub enum ExtensionRecordBody {
            Other(crate::other_extension_record::model::Other),
            #extension_record_variants
        }

        impl ExtensionRecordBody {
            pub fn record_length(&self) -> u16 {
                match self {
                    #extension_record_length_variants
                    ExtensionRecordBody::Other(body) => body.record_length(),
                }
            }
        }
    }
}

fn generate_body_variant(variant: &GenerationItem) -> TokenStream {
    let variant_name = variant
        .variant_name()
        .expect("Variant name for variants can only be called for PDUs and ExtensionRecords");
    let variant_name = format_ident!("{variant_name}");
    let type_path = variant.type_path();
    let type_name = to_tokens(
        variant
            .type_name()
            .expect("Type name for variants can only be called for PDUs and ExtensionRecords"),
    );

    quote! {
        #variant_name ( #type_path::#type_name ),
    }
}

fn generate_body_type_variant(variant: &GenerationItem) -> TokenStream {
    let variant_name = to_tokens(
        variant
            .variant_name()
            .expect("Variant name for variants can only be called for PDUs and ExtensionRecords"),
    );
    let (body_type, function) = match variant {
        GenerationItem::Pdu(_, _) => (quote! { PduBody }, quote! { body_type }),
        GenerationItem::ExtensionRecord(_, _) => {
            (quote! { ExtensionRecordBody }, quote! { record_type })
        }
        _ => panic!("Body type for variants can only be called for PDUs and ExtensionRecords"),
    };

    quote! {
        #body_type::#variant_name(body) => body.#function(),
    }
}

fn generate_body_length_variant(variant: &GenerationItem) -> TokenStream {
    let variant_name = to_tokens(
        variant
            .variant_name()
            .expect("Variant name for variants can only be called for PDUs and ExtensionRecords"),
    );
    let (body_type, function) = match variant {
        GenerationItem::Pdu(_, _) => (quote! { PduBody }, quote! { body_length }),
        GenerationItem::ExtensionRecord(_, _) => {
            (quote! { ExtensionRecordBody }, quote! { record_length })
        }
        _ => panic!("Body length for variants can only be called for PDUs and ExtensionRecords"),
    };

    quote! {
        #body_type::#variant_name(body) => body.#function(),
    }
}

/// Helper function that generates a module structure with the provided `name`,
/// and is filled with the provided `TokenStream` as contents.
pub(crate) fn generate_module_with_name(name: &str, contents: &TokenStream) -> TokenStream {
    let name_ident = format_ident!("{name}");
    quote! {

        pub mod #name_ident {
            #contents
        }

    }
}

/// Generates a module for a PDU Family of PDUs and records, plus all its contents
fn generate_family_module(items: &[GenerationItem], family: &str) -> TokenStream {
    // 1. Filter the PDUs for this family and generate these in separate modules
    let pdus = generate_family_pdus(items, family);

    let _ = syn::parse_file(&pdus.to_string())
        .expect("Error parsing 'PDU' intermediate generated code for pretty printing.");

    let extension_records = generate_family_extension_records(items, family);

    let records = generate_family_records(items, family);

    let contents = quote! { #pdus #extension_records #records };

    println!("{contents}");
    let _ = syn::parse_file(&contents.to_string())
        .expect("Error parsing 'family contents' intermediate generated code for pretty printing.");

    generate_module_with_name(family, &contents)
}

fn generate_family_pdus(items: &[GenerationItem], family: &str) -> TokenStream {
    items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_pdu())
        .map(|pdu| {
            if let GenerationItem::Pdu(pdu, _) = pdu {
                generate_pdu_module(pdu)
            } else {
                panic!("GenerationItem is not a PDU.")
            }
        })
        .collect::<TokenStream>()
}

fn generate_family_extension_records(items: &[GenerationItem], family: &str) -> TokenStream {
    // Filter the ExtensionRecord items for this family and generate the records in a separate (sub)module
    let generated = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_extension_record())
        .map(|item| {
            if let GenerationItem::ExtensionRecord(record, _family) = item {
                (
                    generate_extension_record(record),
                    generate_extension_record_body_parser(record),
                    generate_extension_record_body_writer(record),
                )
            } else {
                panic!("GenerationItem is not an ExtensionRecord.")
            }
        })
        .collect::<Vec<(TokenStream, TokenStream, TokenStream)>>();

    let (extension_records, extension_record_parsers, extension_record_writers): (
        Vec<_>,
        Vec<_>,
        Vec<_>,
    ) = itertools::multiunzip(generated);

    // Flatten the Vec<TokenStream> to a TokenStream
    let extension_records = extension_records.into_iter().collect::<TokenStream>();

    // TODO remove
    let _ = syn::parse_file(&extension_records.to_string()).expect(
        "Error parsing 'extension_records models' intermediate generated code for pretty printing.",
    );

    // Flatten the Vec<TokenStream> to a TokenStream
    let extension_record_parsers = extension_record_parsers
        .into_iter()
        .flatten()
        .collect::<TokenStream>();

    // Add imports for parsers
    let extension_record_parsers = quote! {
        use nom::IResult;
        #[allow(unused_imports, reason = "Imported in every parser module instead of finding out the use of specific nom functions per module")]
        use nom::Parser;

        #extension_record_parsers
    };

    // TODO remove
    let _ = syn::parse_file(&extension_record_parsers.to_string()).expect(
        "Error parsing 'extension_records parsers' intermediate generated code for pretty printing.",
    );

    // Put parsers into a module
    let er_parser_module = generate_module_with_name(PARSER_MODULE_NAME, &extension_record_parsers);

    // Flatten the Vec<TokenStream> to a TokenStream
    let extension_record_writers = extension_record_writers
        .into_iter()
        .flatten()
        .collect::<TokenStream>();

    // Add imports for writers
    let extension_record_writers = quote! {
        use bytes::{BytesMut, BufMut};
        use crate::core::writer::Serialize;

        #extension_record_writers
    };

    // TODO remove
    println!("{extension_record_writers}");
    let _ = syn::parse_file(&extension_record_writers.to_string()).expect(
        "Error parsing 'extension_records writers' intermediate generated code for pretty printing.",
    );

    // Put writers into a module
    let er_writer_module = generate_module_with_name(WRITER_MODULE_NAME, &extension_record_writers);

    // Finalise ExtensionRecords imports and module
    let extension_records = quote! {
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #extension_records
        #er_parser_module
        #er_writer_module
    };

    generate_module_with_name(EXTENSION_RECORDS_MODULE_NAME, &extension_records)
}

fn generate_family_records(items: &[GenerationItem], family: &str) -> TokenStream {
    // Filter the remaining non-PDU items for this family and generate the records in the family module
    let (records, record_parsers, record_writers) = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_record())
        .map(|item| match item {
            GenerationItem::FixedRecord(record, _family) => (
                generate_fixed_record(record),
                super::parsers::generate_fixed_record_parser(record),
                super::writers::generate_fixed_record_writer(record),
            ),
            GenerationItem::BitRecord(record, _family) => (
                generate_bit_record(record),
                super::parsers::generate_bit_record_parser(record),
                quote! {}, // TODO bit record writer
            ),
            GenerationItem::AdaptiveRecord(record, _family) => (
                generate_adaptive_record(record),
                super::parsers::generate_adaptive_record_parser(record),
                quote! {}, // TODO adaptive record writer
            ),
            GenerationItem::ExtensionRecord(_record, _family) => {
                panic!("GenerationItem is not a Record (found ExtensionRecord).")
            }
            GenerationItem::Pdu(_, _) => panic!("GenerationItem is not a Record (found PDU)."),
        })
        .collect::<(Vec<TokenStream>, Vec<TokenStream>, Vec<TokenStream>)>();
    // Flatten the Vec<TokenStream> to a TokenStream
    let records = records.into_iter().collect::<TokenStream>();
    let record_parsers = record_parsers
        .into_iter()
        .flatten()
        .collect::<TokenStream>();
    // Add imports for the parser module
    let record_parsers = quote! {
        #[allow(unused_imports, reason = "It is so much easier to allow than figuring out which modules use specific nom imports")]
        use nom::IResult;
        #record_parsers
    };
    let records_parser_module = generate_module_with_name(PARSER_MODULE_NAME, &record_parsers);

    // TODO remove
    println!("{records_parser_module}");
    let _ = syn::parse_file(&records_parser_module.to_string())
        .expect("Error parsing 'family record parser module' intermediate generated code for pretty printing.");

    let record_writers = record_writers
        .into_iter()
        .flatten()
        .collect::<TokenStream>();
    // Add imports for the parser module
    let record_writers = quote! {
        use bytes::{BytesMut, BufMut};
        use crate::core::writer::Serialize;
        #record_writers
    };
    let records_writer_module = generate_module_with_name(WRITER_MODULE_NAME, &record_writers);

    // TODO remove
    println!("{records_writer_module}");
    let _ = syn::parse_file(&records_writer_module.to_string())
        .expect("Error parsing 'family record parser module' intermediate generated code for pretty printing.");

    // Add imports for the records (not wrapped in a module)
    let records = quote! {
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #records
        #records_parser_module
        #records_writer_module
    };

    println!("{records}");
    let _ = syn::parse_file(&records.to_string())
        .expect("Error parsing 'family records and parsers' intermediate generated code for pretty printing.");

    records
}

/// Generates all code related a PDU
fn generate_pdu_module(pdu: &Pdu) -> TokenStream {
    let pdu_name_ident = format_ident!("{}", pdu.type_name);
    let pdu_module_name = &pdu.pdu_module_name;
    let builder_name_ident = format_ident!("{}{BUILDER_TYPE_SUFFIX}", pdu.type_name);

    let pdu_trait_impls = generate_pdu_trait_impls(pdu);
    let builder_content = super::builders::generate_pdu_builder(pdu, &builder_name_ident);
    let builder_module = generate_module_with_name(BUILDER_MODULE_NAME, &builder_content);

    // TODO remove
    println!("PDU: {}", pdu.type_name);
    let _ = syn::parse_file(&builder_module.to_string()).expect(
        "Error parsing 'pdu builder module' intermediate generated code for pretty printing.",
    );

    let parser_content = super::parsers::generate_pdu_body_parser(pdu);
    let parser_module = generate_module_with_name(PARSER_MODULE_NAME, &parser_content);

    // TODO remove
    println!("{parser_module}");
    let _ = syn::parse_file(&parser_module.to_string()).expect(
        "Error parsing 'pdu parser module' intermediate generated code for pretty printing.",
    );

    let writer_content = super::writers::generate_pdu_body_writer(pdu);
    let writer_module = generate_module_with_name(WRITER_MODULE_NAME, &writer_content);

    // TODO remove
    println!("{writer_module}");
    let _ = syn::parse_file(&writer_module.to_string()).expect(
        "Error parsing 'pdu writer module' intermediate generated code for pretty printing.",
    );

    let fields = pdu
        .fields
        .iter()
        .map(generate_pdu_and_fixed_field_decl)
        .collect::<Vec<TokenStream>>();

    let contents = quote! {
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Default, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct #pdu_name_ident {
            #(#fields)*
            pub extension_records: Vec<crate::ExtensionRecord>,
        }

        #pdu_trait_impls

        #builder_module

        #parser_module

        #writer_module
    };

    generate_module_with_name(pdu_module_name, &contents)
}

// TODO further develop the core traits (as in gen 2: BodyInfo, Interaction)
fn generate_pdu_trait_impls(pdu: &Pdu) -> TokenStream {
    const PDU_HEADER_LEN_BYTES: usize = 16;

    let pdu_name_ident = format_ident!("{}", pdu.type_name);
    let builder_name_ident = format_ident!("{}{BUILDER_TYPE_SUFFIX}", pdu.type_name);
    let pdu_type = &pdu.pdu_type_name;
    #[expect(
        clippy::cast_possible_truncation,
        reason = "Length are within u16::MAX "
    )]
    let base_body_length = Literal::u16_suffixed((pdu.base_length - PDU_HEADER_LEN_BYTES) as u16);
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

            fn body_length(&self) -> u16 {
                #base_body_length + self.extension_records.iter().map(|er| er.record_length() as u16).sum::<u16>()
            }

            fn body_type(&self) -> crate::enumerations::DISPDUType {
                crate::enumerations::DISPDUType::#pdu_type
            }
        }
    }
}

fn generate_extension_record(record: &ExtensionRecord) -> TokenStream {
    let record_name = format_ident!("{}", record.type_name);
    let record_type_doc_comment = format!("Record Type Enum {}", record.record_type_enum);

    let fields = record
        .fields
        .iter()
        .map(generate_extension_record_field_decl);

    let record_type = to_tokens(&record.record_type_variant_name);

    #[expect(
        clippy::cast_possible_truncation,
        reason = "Length are within u16::MAX "
    )]
    let base_length = Literal::u16_suffixed(record.base_length as u16);
    let length_calculation = if record.is_variable {
        let variable_fields = record
            .fields
            .iter()
            .filter_map(generate_extension_record_variable_field_length_calculation)
            .collect::<Vec<TokenStream>>();
        quote! { #base_length + #(#variable_fields)+* }
    } else {
        quote! { #base_length }
    };

    quote! {
        #[doc = #record_type_doc_comment]
        #[derive(Debug, Default, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct #record_name {
            #(#fields)*
        }

        impl #record_name {
            pub fn record_length(&self) -> u16 {
                #length_calculation
            }

            pub fn record_type(&self) -> crate::enumerations::ExtensionRecordTypes {
                crate::enumerations::ExtensionRecordTypes::#record_type
            }
        }
    }
}

fn generate_extension_record_variable_field_length_calculation(
    field: &ExtensionRecordFieldEnum,
) -> Option<TokenStream> {
    const OCTET_IN_BITS: usize = 8;

    match field {
        ExtensionRecordFieldEnum::Numeric(_) => None,
        ExtensionRecordFieldEnum::Enum(_) => None,
        ExtensionRecordFieldEnum::FixedString(_) => None,
        ExtensionRecordFieldEnum::VariableString(f) => {
            let field_name = to_tokens(&f.string_field.field_name);
            Some(quote! { (self.#field_name.len() as u16) })
        }
        ExtensionRecordFieldEnum::FixedRecord(_) => None,
        ExtensionRecordFieldEnum::BitRecord(_) => None,
        ExtensionRecordFieldEnum::Array(f) => {
            let element_length = match &f.type_field {
                ArrayFieldEnum::Numeric(af) => af.length,
                ArrayFieldEnum::Enum(af) => af.length,
                ArrayFieldEnum::FixedString(af) => af.length,
                ArrayFieldEnum::FixedRecord(af) => af.length,
                ArrayFieldEnum::BitRecord(af) => af.size / OCTET_IN_BITS,
            };
            let field_name = to_tokens(f.type_field.field_name());
            Some(quote! { ((self.#field_name.len() * #element_length) as u16) })
        }
        ExtensionRecordFieldEnum::AdaptiveRecord(_) => None,
        ExtensionRecordFieldEnum::Opaque(f) => {
            let field_name = to_tokens(&f.opaque_data_field.field_name);
            Some(quote! { (self.#field_name.len() as u16) })
        }
        ExtensionRecordFieldEnum::PaddingTo16 => None,
        ExtensionRecordFieldEnum::PaddingTo32 => None,
    }
}

fn generate_fixed_record(record: &FixedRecord) -> TokenStream {
    let record_name = &record.type_name;

    let fields = record
        .fields
        .iter()
        .map(generate_pdu_and_fixed_field_decl)
        .collect::<Vec<TokenStream>>();

    quote! {
        #[derive(Debug, Default, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct #record_name {
            #(#fields)*
        }
    }
}

fn generate_pdu_and_fixed_field_decl(field: &PduAndFixedRecordFieldsEnum) -> TokenStream {
    match field {
        PduAndFixedRecordFieldsEnum::Numeric(field) => generate_numeric_field_decl(field),
        PduAndFixedRecordFieldsEnum::Enum(field) => generate_enum_field_decl(field),
        PduAndFixedRecordFieldsEnum::FixedString(field) => generate_fixed_string_field_decl(field),
        PduAndFixedRecordFieldsEnum::FixedRecord(field) => generate_fixed_record_field_decl(field),
        PduAndFixedRecordFieldsEnum::BitRecord(field) => generate_bit_record_field_decl(field),
        PduAndFixedRecordFieldsEnum::AdaptiveRecord(field) => {
            generate_adaptive_record_field_decl(field)
        }
    }
}

fn generate_extension_record_field_decl(field: &ExtensionRecordFieldEnum) -> TokenStream {
    match field {
        ExtensionRecordFieldEnum::Numeric(field) => generate_numeric_field_decl(field),
        ExtensionRecordFieldEnum::Enum(field) => generate_enum_field_decl(field),
        ExtensionRecordFieldEnum::FixedString(field) => generate_fixed_string_field_decl(field),
        ExtensionRecordFieldEnum::VariableString(field) => {
            generate_variable_string_field_decl(&field.string_field)
        }
        ExtensionRecordFieldEnum::FixedRecord(field) => generate_fixed_record_field_decl(field),
        ExtensionRecordFieldEnum::BitRecord(field) => generate_bit_record_field_decl(field),
        ExtensionRecordFieldEnum::Array(field) => generate_array_field_decl(field),
        ExtensionRecordFieldEnum::AdaptiveRecord(field) => {
            generate_adaptive_record_field_decl(field)
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
    if field.is_padding {
        return quote! {};
    }
    let field_ident = format_ident!("{}", field.field_name);
    let field_type = &field.primitive_type;
    let doc_units = &field
        .units
        .as_ref()
        .map(|f| quote! { #[doc= #f] })
        .unwrap_or_default();

    quote! {
        #doc_units
        pub #field_ident : #field_type,
    }
}

fn generate_enum_field_decl(field: &EnumField) -> TokenStream {
    let field_ident = format_ident!("{}", field.field_name);
    let final_type = finalise_type(&field.type_path, &field.type_name);

    quote! {
        pub #field_ident : #final_type,
    }
}

fn generate_fixed_string_field_decl(field: &FixedStringField) -> TokenStream {
    let field_ident = format_ident!("{}", field.field_name);
    let length_doc_comment = format!("Fixed String with length {}", field.length);

    quote! {
        #[doc = #length_doc_comment]
        pub #field_ident : String,
    }
}

fn generate_fixed_record_field_decl(field: &FixedRecordField) -> TokenStream {
    let field_ident = format_ident!("{}", field.field_name);
    let final_type = finalise_type(&field.type_path, &field.type_name);

    quote! {
        pub #field_ident : #final_type,
    }
}

fn generate_bit_record_field_decl(field: &BitRecordField) -> TokenStream {
    let field_ident = format_ident!("{}", field.field_name);
    let final_type = finalise_type(&field.type_path, &field.type_name);

    quote! {
        pub #field_ident : #final_type,
    }
}

fn generate_adaptive_record_field_decl(field: &AdaptiveRecordField) -> TokenStream {
    let field_ident = format_ident!("{}", field.field_name);
    let final_type = finalise_type(&field.type_path, &field.type_name);

    quote! {
        pub #field_ident : #final_type,
    }
}

fn generate_variable_string_field_decl(field: &VariableStringField) -> TokenStream {
    let field_ident = format_ident!("{}", field.field_name);
    quote! {
        #[doc = "Variable String"]
        pub #field_ident : String,
    }
}

fn generate_array_field_decl(field: &Array) -> TokenStream {
    let (field_ident, field_type) = match &field.type_field {
        ArrayFieldEnum::Numeric(inner) => {
            (format_ident!("{}", inner.field_name), &inner.primitive_type)
        }
        ArrayFieldEnum::Enum(inner) => {
            let final_type = finalise_type(&inner.type_path, &inner.type_name);
            (
                format_ident!("{}", inner.field_name),
                &quote! { #final_type },
            )
        }
        ArrayFieldEnum::FixedString(inner) => (
            format_ident!("{}", inner.field_name),
            &syn::parse_str(inner.field_type).unwrap(),
        ),
        ArrayFieldEnum::FixedRecord(inner) => {
            let final_type = finalise_type(&inner.type_path, &inner.type_name);
            (
                format_ident!("{}", inner.field_name),
                &quote! { #final_type },
            )
        }
        ArrayFieldEnum::BitRecord(inner) => {
            let final_type = finalise_type(&inner.type_path, &inner.type_name);
            (
                format_ident!("{}", inner.field_name),
                &quote! { #final_type },
            )
        }
    };

    quote! {
        pub #field_ident : Vec<#field_type>,
    }
}

fn generate_opaque_field_decl(field: &OpaqueData) -> TokenStream {
    let field_ident = format_ident!("{}", field.opaque_data_field.field_name);

    quote! {
        pub #field_ident : Vec<u8>,
    }
}

fn generate_bit_record(item: &BitRecord) -> TokenStream {
    let record_name = &item.type_name;

    let fields = item
        .fields
        .iter()
        .map(generate_bit_record_item_decl)
        .collect::<Vec<TokenStream>>();

    quote! {
        #[derive(Debug, Default, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub struct #record_name {
            #(#fields)*
        }
    }
}

fn generate_bit_record_item_decl(item: &BitRecordFieldEnum) -> TokenStream {
    match item {
        BitRecordFieldEnum::Enum(field) => generate_enum_bit_field_decl(field),
        BitRecordFieldEnum::Int(field) => generate_int_bit_field_decl(field),
        BitRecordFieldEnum::Bool(field) => generate_bool_bit_field_decl(field),
    }
}

fn generate_enum_bit_field_decl(field: &EnumBitField) -> TokenStream {
    let field_name = format_ident!("{}", field.field_name);
    let final_type = finalise_type(&field.type_path, &field.type_name);

    quote! { pub #field_name: #final_type, }
}

fn generate_bool_bit_field_decl(field: &BoolBitField) -> TokenStream {
    let field_name = format_ident!("{}", field.field_name);

    quote! { pub #field_name: bool, }
}

fn generate_int_bit_field_decl(field: &IntBitField) -> TokenStream {
    if field.is_padding {
        return quote! {};
    }

    let field_name = format_ident!("{}", field.field_name);
    let field_type = &field.field_type;
    let units_doc_comment = if let Some(units) = &field.units {
        units.as_str()
    } else {
        "No units defined"
    };

    quote! {
        #[doc = #units_doc_comment]
        pub #field_name: #field_type,
    }
}

fn generate_adaptive_record(item: &AdaptiveRecord) -> TokenStream {
    let record_name = &item.type_name;

    let variants = item
        .variants
        .iter()
        .map(generate_adaptive_record_variant)
        .collect::<Vec<TokenStream>>();

    quote! {
        #[derive(Debug, Default, Clone, PartialEq)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum #record_name {
            #[default]
            None,
            #(#variants)*
        }
    }
}

/// Note: this function takes a `BitRecordField` as argument instead of an `AdaptiveFormatEnum`, because
/// at the time of writing the schema definitions only have `BitRecord`s occurring in `AdaptiveRecord`s.
fn generate_adaptive_record_variant(variant: &BitRecordField) -> TokenStream {
    let variant_name = &variant.type_name;
    let variant_type_path = &variant.type_path;

    quote! {
        #variant_name ( #variant_type_path::#variant_name ),
    }
}
