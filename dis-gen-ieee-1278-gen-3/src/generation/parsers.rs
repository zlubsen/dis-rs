use crate::generation::models::{
    generate_module_with_name, AdaptiveRecordField, BitRecordField, EnumField, FixedRecordField,
    FixedStringField, GenerationItem, NumericField, Pdu, PduAndFixedRecordFieldsEnum,
    PARSER_MODULE_NAME,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

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

        #extension_record_body_parser
    };

    generate_module_with_name(PARSER_MODULE_NAME, &contents)
}

pub fn generate_common_pdu_body_parser(items: &[GenerationItem]) -> TokenStream {
    let pdu_type_arms = items
        .iter()
        .filter(|&it| it.is_pdu())
        .map(generate_common_pdu_body_parser_arm)
        .collect::<TokenStream>();

    quote! {
        pub fn pdu_body(header: &PDUHeader) -> impl Fn(&[u8]) -> IResult<&[u8], crate::PduBody> + '_ {
            move |input: &[u8]| {
                let (input, body) = match header.pdu_type {
                    DISPDUType::Other => crate::other::parser::other_body(input)?,
                    #pdu_type_arms
                    _ => crate::other::parser::other_body(input)?,
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
        quote! {
            #pdu_type => #pdu_path::parser::#parser_function(input)?,
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

        pub fn extension_record_body(er_record_type: &ExtensionRecordTypes) -> impl Fn(&[u8]) -> IResult<&[u8], crate::ExtensionRecordBody> + '_ {
            move |input: &[u8]| {
                let (input, body) = match er_record_type {
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
        let er_variant_name = &er.record_type_variant_name;
        let er_path = er.fqn_path.clone();
        let parser_function = er.parser_function.clone();
        quote! {
            #EXTENSION_RECORD_TYPE::#er_variant_name => #er_path::parser::#parser_function(input)?,
        }
    } else {
        panic!("GenerationItem is not an Extension Record.")
    }
}

// TODO:
// - separate fqn path and name in the GenerationItems, store as String or TokenStream
// -

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
        let #field_name = String::from_utf8(#field_name).unwrap_or("Invalid UTF8");
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
