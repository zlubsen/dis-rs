use crate::generation::parsers::generate_extension_record_body_parser;
use crate::pre_processing::{finalise_type, to_tokens};
use proc_macro2::{Ident, TokenStream};
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

pub(crate) const EXTENSION_RECORDS_MODULE_NAME: &str = "extension_records";
pub(crate) const BUILDER_MODULE_NAME: &str = "builder";
pub(crate) const BUILDER_TYPE_SUFFIX: &str = "Builder";
pub(crate) const PARSER_MODULE_NAME: &str = "parser";

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
}

#[derive(Clone)]
pub(crate) struct CountField {
    pub field_name: String,
    pub primitive_type: TokenStream,
    pub parser_function: TokenStream,
}

#[derive(Clone, Debug)]
pub(crate) struct EnumField {
    pub field_name: String,
    pub type_name: TokenStream,
    pub type_path: TokenStream,
    pub is_discriminant: bool,
    pub parser_function: TokenStream,
    pub parser_must_convert_to_enum: bool,
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

    println!("{parsers}");
    let _ = syn::parse_file(&parsers.to_string())
        .expect("Error parsing 'parsers' intermediate generated code for pretty printing.");

    quote! {
        #[expect(arithmetic_overflow, reason = "Intentionally trigger a lint warning")]

        #core_contents

        #(#family_model_contents)*

        #parsers
    }
}

fn generate_core_units(items: &[GenerationItem]) -> TokenStream {
    // FIXME these parts should be in the lib itself as regular code, whenever possible
    // TODO Other PDU
    // TODO Other ExtensionRecord

    // TODO Generate From<(discriminant, value)> for ... - Adaptive records, to be able to parse it

    let pdu_body_variants = items
        .iter()
        .filter(|&it| it.is_pdu())
        .map(generate_body_variant)
        .collect::<TokenStream>();

    let extension_record_variants = items
        .iter()
        .filter(|&it| it.is_extension_record())
        .map(generate_body_variant)
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
    let pdus = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_pdu())
        .map(|pdu| {
            if let GenerationItem::Pdu(pdu, _) = pdu {
                let module = generate_pdu_module(pdu);

                println!("{module}");
                let _ = syn::parse_file(&module.to_string())
                    .unwrap_or_else(|_|panic!("Error parsing 'pdu module - {}' intermediate generated code for pretty printing.", &pdu.type_name));

                module
            } else {
                panic!("GenerationItem is not a PDU.")
            }
        })
        .collect::<TokenStream>();

    let _ = syn::parse_file(&pdus.to_string())
        .expect("Error parsing 'PDU' intermediate generated code for pretty printing.");

    // 3. Filter the ExtensionRecord items for this family and generate the records in a separate (sub)module
    // TODO extract into separate function
    let generated = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_extension_record())
        .map(|item| {
            if let GenerationItem::ExtensionRecord(record, _family) = item {
                (
                    generate_extension_record(record),
                    generate_extension_record_body_parser(record),
                )
            } else {
                panic!("GenerationItem is not an ExtensionRecord.")
            }
        })
        .collect::<Vec<(TokenStream, TokenStream)>>();

    let (extension_records, extension_record_parsers): (Vec<_>, Vec<_>) =
        itertools::multiunzip(generated);

    let extension_records = extension_records.into_iter().collect::<TokenStream>();
    // TODO remove
    let _ = syn::parse_file(&extension_records.to_string()).expect(
        "Error parsing 'extension_records models' intermediate generated code for pretty printing.",
    );
    let extension_record_parsers = extension_record_parsers
        .into_iter()
        .flatten()
        .collect::<TokenStream>();
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
    let er_parser_module = generate_module_with_name(PARSER_MODULE_NAME, &extension_record_parsers);
    let extension_records = quote! { #extension_records #er_parser_module };
    let extension_records =
        generate_module_with_name(EXTENSION_RECORDS_MODULE_NAME, &extension_records);

    // 3. Filter the remaining non-PDU items for this family and generate the records in the family module
    // TODO extract into separate function
    let (records, record_parsers) = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_record())
        .map(|item| match item {
            GenerationItem::FixedRecord(record, _family) => (
                generate_fixed_record(record),
                super::parsers::generate_fixed_record_parser(record),
            ),
            GenerationItem::BitRecord(record, _family) => (
                generate_bit_record(record),
                super::parsers::generate_bit_record_parser(record),
            ),
            GenerationItem::AdaptiveRecord(record, _family) => (
                generate_adaptive_record(record),
                super::parsers::generate_adaptive_record_parser(record),
            ),
            GenerationItem::ExtensionRecord(_record, _family) => {
                panic!("GenerationItem is not a Record (found ExtensionRecord).")
            }
            GenerationItem::Pdu(_, _) => panic!("GenerationItem is not a Record (found PDU)."),
        })
        .collect::<(Vec<TokenStream>, Vec<TokenStream>)>();
    let records = records.into_iter().collect::<TokenStream>();
    let record_parsers = record_parsers
        .into_iter()
        .flatten()
        .collect::<TokenStream>();
    let record_parsers = quote! { use nom::IResult; #record_parsers };
    let records_parser_module = generate_module_with_name(PARSER_MODULE_NAME, &record_parsers);
    // TODO remove
    println!("{records}");
    println!("{records_parser_module}");
    let _ = syn::parse_file(&records_parser_module.to_string())
        .expect("Error parsing 'family record parser module' intermediate generated code for pretty printing.");
    let records = quote! { #records #records_parser_module };

    println!("{records}");
    let _ = syn::parse_file(&records.to_string())
        .expect("Error parsing 'family records and parsers' intermediate generated code for pretty printing.");

    let contents = quote! { #pdus #extension_records #records };

    println!("{contents}");
    let _ = syn::parse_file(&contents.to_string())
        .expect("Error parsing 'family contents' intermediate generated code for pretty printing.");

    generate_module_with_name(family, &contents)
}

/// Generates all code related a PDU
fn generate_pdu_module(pdu: &Pdu) -> TokenStream {
    let pdu_name_ident = format_ident!("{}", pdu.type_name);
    let pdu_module_name = &pdu.pdu_module_name;
    let builder_name_ident = format_ident!("{}{BUILDER_TYPE_SUFFIX}", pdu.type_name);

    // TODO design PduBody traits: size, family, pduType. See BodyRaw, BodyInfo, blanket impls, serialisation, Interaction.
    let pdu_trait_impls = generate_pdu_trait_impls(&pdu_name_ident, &builder_name_ident);
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

    let fields = pdu
        .fields
        .iter()
        .map(generate_pdu_and_fixed_field_decl)
        .collect::<Vec<TokenStream>>();

    let contents = quote! {
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct #pdu_name_ident {
            #(#fields)*
            pub extension_records: Vec<crate::ExtensionRecord>,
        }

        #pdu_trait_impls

        #builder_module

        #parser_module
    };

    generate_module_with_name(pdu_module_name, &contents)
}

fn generate_extension_record(record: &ExtensionRecord) -> TokenStream {
    let record_name = format_ident!("{}", record.type_name);
    let record_type_doc_comment = format!("Record Type Enum {}", record.record_type_enum);

    let fields = record
        .fields
        .iter()
        .map(generate_extension_record_field_decl);

    quote! {
        #[doc = #record_type_doc_comment]
        #[derive(Debug, Default, Clone, PartialEq)]
        pub struct #record_name {
            #(#fields)*
        }
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
