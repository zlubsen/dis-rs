use crate::generation::models::{generate_module_with_name, AdaptiveRecordField, Array, ArrayFieldEnum, BitRecordField, EnumField, ExtensionRecord, ExtensionRecordFieldEnum, FixedRecordField, FixedStringField, GenerationItem, NumericField, OpaqueDataField, Pdu, PduAndFixedRecordFieldsEnum, VariableString, PARSER_MODULE_NAME};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

// TODO:
// - separate fqn path and name in the GenerationItems, store as String or TokenStream
// - parsers for 'Other' PDU and Extension Records(?)

const PDU_HEADER: &str = "PDUHeader";
const PDU_TYPE: &str = "DISPDUType";
const EXTENSION_RECORD_TYPE: &str = "ExtensionRecordTypes";

pub fn generate_common_parsers(items: &[GenerationItem]) -> TokenStream {
    let pdu_type = format_ident!("{PDU_TYPE}");
    let extension_record_type = format_ident!("{EXTENSION_RECORD_TYPE}");

    let pdu_body_parser = generate_common_pdu_body_parser(items);
    let extension_record_body_parser = generate_common_extension_record_body_parser(items);

    // TODO 'Other' parsers

    let contents = quote! {
        use crate::PduBody;
        use crate::common_records::PDUHeader;
        use crate::enumerations::#pdu_type;
        use crate::enumerations::#extension_record_type;
        use nom::IResult;

        #pdu_body_parser

        pub fn extension_record_set(input: &[u8]) -> IResult<&[u8], Vec<crate::ExtensionRecord> {
            let (input, number_of_er) = nom::number::complete::le_u16(input)?;
            let (input, records) = nom::multi::count(extension_record(input), number_of_er)?;

            Ok((input, records))
        }

        pub fn extension_record(input: &[u8]) -> IResult<&[u8], crate::ExtensionRecord> {
            let (input, record_type) = nom::number::complete::le_u16(input)?;
            let record_type = crate::enumerations::ExtensionRecordTypes::from(record_type);
            let (input, record_length) = nom::number::complete::le_u16(input)?;

            let (input, body) = extension_record_body(&record_type)?;

            Ok((input, crate::ExtensionRecord {
                record_type,
                record_length,
                body,
            }))
        }

        #extension_record_body_parser
    };

    generate_module_with_name(PARSER_MODULE_NAME, &contents)
}

pub fn generate_common_pdu_body_parser(items: &[GenerationItem]) -> TokenStream {
    let parser_module = format_ident!("{PARSER_MODULE_NAME}");
    let pdu_type_arms = items
        .iter()
        .filter(|&it| it.is_pdu())
        .map(generate_common_pdu_body_parser_arm)
        .collect::<TokenStream>();

    quote! {
        pub fn pdu_body(header: &PDUHeader) -> impl Fn(&[u8]) -> IResult<&[u8], crate::PduBody> + '_ {
            move |input: &[u8]| {
                let (input, body) = match header.pdu_type {
                    DISPDUType::Other => crate::other::#parser_module::other_body(input)?,
                    #pdu_type_arms
                    _ => crate::other::#parser_module::other_body(input)?,
                };
                Ok((input, body))
            }
        }
    }
}

fn generate_common_pdu_body_parser_arm(pdu: &GenerationItem) -> TokenStream {
    if let GenerationItem::Pdu(pdu, _) = pdu {
        let pdu_type: TokenStream = format!("{PDU_TYPE}::{}", pdu.pdu_name)
            .parse()
            .expect("Expected valid Rust code for PDUType variant");
        let pdu_path = pdu.fqn_path.clone();
        let parser_function = pdu.parser_function.clone();
        let parser_module = format_ident!("{PARSER_MODULE_NAME}");
        quote! {
            #pdu_type => #pdu_path::#parser_module::#parser_function(input)?,
        }
    } else {
        panic!("GenerationItem is not a PDU.")
    }
}

pub fn generate_common_extension_record_body_parser(items: &[GenerationItem]) -> TokenStream {
    let extension_record_type = format_ident!("{EXTENSION_RECORD_TYPE}");
    let extension_record_types_arms = items
        .iter()
        .filter(|&it| it.is_extension_record())
        .map(generate_common_extension_record_body_parser_arm)
        .collect::<TokenStream>();

    quote! {
        fn temp_other_parser(input: &[u8]) -> IResult<&[u8], ()> {
            todo!("Implement parser for Other PDU")
        }

        pub fn extension_record_body(record_type: &ExtensionRecordTypes) -> impl Fn(&[u8]) -> IResult<&[u8], crate::ExtensionRecordBody> + '_ {
            move |input: &[u8]| {
                let (input, body) = match record_type {
                    // FIXME parser for 'Other' PDU
                    #extension_record_type::NotSpecified => temp_other_parser(input)?,//crate::other::parser::other_body(input)?,
                    // TODO reserved for C-DIS, range 1-255
                    #extension_record_types_arms
                    _ => temp_other_parser(input)?,//crate::other::parser::other_body(input)?,
                };
                Ok((input, body))
            }
        }
    }
}

fn generate_common_extension_record_body_parser_arm(er: &GenerationItem) -> TokenStream {
    if let GenerationItem::ExtensionRecord(er, _) = er {
        let er_record_type_ident = format_ident!("{}", EXTENSION_RECORD_TYPE);
        let er_variant_name = &er.record_type_variant_name;
        let er_path = er.fqn_path.clone();
        let parser_function = er.parser_function.clone();
        quote! {
            #er_record_type_ident::#er_variant_name => #er_path::parser::#parser_function(input)?,
        }
    } else {
        panic!("GenerationItem is not an Extension Record.")
    }
}

pub fn generate_pdu_body_parser(pdu: &Pdu) -> TokenStream {
    let parser_function = pdu.parser_function.clone();
    let pdu_name_fqn = &pdu.pdu_name_fqn;
    let field_parsers = pdu
        .fields
        .iter()
        .map(generate_pdu_field_parser)
        .collect::<Vec<TokenStream>>();
    let field_builders = pdu
        .fields
        .iter()
        .filter(|field| !field.is_padding())
        .map(|field| field.field_name().parse::<TokenStream>().unwrap())
        .collect::<Vec<TokenStream>>();
    let parser_module = format_ident!("{PARSER_MODULE_NAME}");

    quote! {
        use crate::BodyRaw;
        use nom::IResult;

        pub fn #parser_function(input: &[u8]) -> IResult<&[u8], crate::PduBody> {
            #(#field_parsers)*
            let (input, extension_records) = crate::#parser_module::extension_record_set(input)?;

            let body = #pdu_name_fqn {
                #(#field_builders),*,
                extension_records,
            };
            Ok((input, body.into_pdu_body()))
        }
    }
}

fn generate_pdu_field_parser(field: &PduAndFixedRecordFieldsEnum) -> TokenStream {
    match field {
        PduAndFixedRecordFieldsEnum::Numeric(f) => generate_numeric_field_parser(f),
        PduAndFixedRecordFieldsEnum::Enum(f) => generate_enum_field_parser(f),
        PduAndFixedRecordFieldsEnum::FixedString(f) => generate_fixed_string_parser(f),
        PduAndFixedRecordFieldsEnum::FixedRecord(f) => generate_fixed_record_field_parser(f),
        PduAndFixedRecordFieldsEnum::BitRecord(f) => generate_bit_record_field_parser(f),
        PduAndFixedRecordFieldsEnum::AdaptiveRecord(f) => generate_adaptive_record_field_parser(f),
    }
}

pub fn generate_extension_record_body_parser(record: &ExtensionRecord) -> TokenStream {
    let parser_function = &record.parser_function;
    let record_name_fqn = &record.record_name_fqn;

    let field_parsers = record.fields
        .iter()
        .map(generate_extension_record_field_parser)
        .collect::<Vec<TokenStream>>();
    let fields = record.fields
        .iter()
        // TODO list all records fields, comma separated
        .map(|field| field.field_name().parse::<TokenStream>().expect("Failed to tokenise field name for extension record parser") )
        .collect::<Vec<TokenStream>>();

    quote! {
        pub fn #parser_function(input: &[u8]) -> IResult<&[u8], crate::ExtensionRecordBody> {
            #(#field_parsers)*

            let body = #record_name_fqn {
                #(#fields),*,
            };
            Ok((input, body.into_pdu_body()))
        }
    }
}

fn generate_extension_record_field_parser(field: &ExtensionRecordFieldEnum) -> TokenStream {
    match field {
        ExtensionRecordFieldEnum::Numeric(f) => { generate_numeric_field_parser(f) }
        ExtensionRecordFieldEnum::Enum(f) => { generate_enum_field_parser(f) }
        ExtensionRecordFieldEnum::FixedString(f) => { generate_fixed_string_parser(f) }
        ExtensionRecordFieldEnum::VariableString(f) => { generate_variable_string_parser(f) }
        ExtensionRecordFieldEnum::FixedRecord(f) => { generate_fixed_record_field_parser(f) }
        ExtensionRecordFieldEnum::BitRecord(f) => { generate_bit_record_field_parser(f) }
        ExtensionRecordFieldEnum::Array(f) => { generate_array_field_parser(f) }
        ExtensionRecordFieldEnum::AdaptiveRecord(f) => { generate_adaptive_record_field_parser(f) }
        ExtensionRecordFieldEnum::Opaque(f) => { generate_opaque_field_parser(f) }
        ExtensionRecordFieldEnum::PaddingTo16 => { generate_padding_to_16_parser() }
        ExtensionRecordFieldEnum::PaddingTo32 => { generate_padding_to_32_parser() }
    }
}

fn generate_numeric_field_parser(field: &NumericField) -> TokenStream {
    let field_name = if field.is_padding {
        format_ident!("_{}", &field.field_name)
    } else {
        format_ident!("{}", &field.field_name)
    };
    let parser = &field.parser_function;

    quote! {
        let (input, #field_name) = #parser(input)?;
    }
}

fn generate_enum_field_parser(field: &EnumField) -> TokenStream {
    let field_name = format_ident!("{}", &field.field_name);
    let field_type_fqn = &field.field_type_fqn;
    let parser = &field.parser_function;
    let conversion = if field.parser_must_convert_to_enum {
        quote! { let #field_name = #field_type_fqn::from(#field_name); }
    } else {
        quote! {}
    };
    let a = quote! {
        let (input, #field_name) = #parser(input)?;
        #conversion
    };
    println!("enum field parser {a}");
    a
}

fn generate_fixed_string_parser(field: &FixedStringField) -> TokenStream {
    let field_name = format_ident!("{}", &field.field_name);
    let length = field.length;

    quote! {
        let (input, #field_name) = nom::bytes::complete::take(#length)(input)?;
        let #field_name = String::from_utf8_lossy(#field_name).unwrap_or("Invalid UTF8");
    }
}

fn generate_fixed_record_field_parser(field: &FixedRecordField) -> TokenStream {
    let field_name = format_ident!("{}", &field.field_name);
    let parser = &field.parser_function;

    quote! {
        let (input, #field_name) = #parser(input)?;
    }
}

fn generate_bit_record_field_parser(field: &BitRecordField) -> TokenStream {
    let field_name = format_ident!("{}", &field.field_name);
    let parser = &field.parser_function;

    quote! {
        let (input, #field_name) = #parser(input)?;
    }
}

fn generate_adaptive_record_field_parser(field: &AdaptiveRecordField) -> TokenStream {
    let field_name = format_ident!("{}", &field.field_name);
    let field_type = &field.field_type_fqn;
    let discriminant_field_name = format_ident!("{}", &field.discriminant_field_name);
    let parser = &field.parser_function;

    quote! {
        let (input, #field_name) = #parser(input)?;
        let #field_name = #field_type::from((#discriminant_field_name, #field_name));
    }
}

fn generate_variable_string_parser(field: &VariableString) -> TokenStream {
    todo!()
}

fn generate_array_field_parser(array: &Array) -> TokenStream {
    let count_parser_function = &array.count_field.parser_function;
    let field_name = format_ident!("{}", array.type_field.field_name());
    let type_parser_function = generate_array_field_type_parser_function(&array.type_field);
    quote! {
        let (input, count) = #count_parser_function(input)?;
        let (input, #field_name) = nom::multi::count(#parser_function(input), count)?;
    }
}

fn generate_array_field_type_parser_function(e: &ArrayFieldEnum) -> &TokenStream {
    match e {
        ArrayFieldEnum::Numeric(f) => { &f.parser_function }
        ArrayFieldEnum::Enum(f) => { &f.parser_function }
        // FIXME crate a 'built-in' parser function for fixed and variable strings, store it in the GenerationItem
        ArrayFieldEnum::FixedString(f) => { &f.parser_function }
        ArrayFieldEnum::FixedRecord(f) => { &f.parser_function }
        ArrayFieldEnum::BitRecord(f) => { &f.parser_function }
    }
}

fn generate_opaque_field_parser(field: &OpaqueDataField) -> TokenStream {
    todo!()
}

fn generate_padding_to_16_parser() -> TokenStream {
    todo!()
}

fn generate_padding_to_32_parser() -> TokenStream {
    todo!()
}

fn generate_padding_to_64_parser() -> TokenStream {
    todo!()
}
