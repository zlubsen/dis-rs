use crate::constants::{EXTENSION_RECORD_TYPE, PARSER_MODULE_NAME, PDU_TYPE};
use crate::generation::models::{
    generate_module_with_name, AdaptiveRecord, AdaptiveRecordField, Array, ArrayFieldEnum,
    BitRecord, BitRecordField, BitRecordFieldEnum, BoolBitField, EnumBitField, EnumField,
    ExtensionRecord, ExtensionRecordFieldEnum, FixedRecord, FixedRecordField, FixedStringField,
    GenerationItem, IntBitField, NumericField, OpaqueData, Pdu, PduAndFixedRecordFieldsEnum,
    VariableString,
};
use crate::pre_processing::{
    field_length_to_primitive, field_size_to_primitive_type, finalise_type, to_tokens,
};
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

pub(crate) fn generate_common_parsers(items: &[GenerationItem]) -> TokenStream {
    let pdu_type = format_ident!("{PDU_TYPE}");
    let extension_record_type = format_ident!("{EXTENSION_RECORD_TYPE}");

    let pdu_body_parser = generate_common_pdu_body_parser(items);
    let extension_record_body_parser = generate_common_extension_record_body_parser(items);

    let contents = quote! {
        use crate::common_records::PDUHeader;
        use crate::enumerations::#pdu_type;
        use crate::enumerations::#extension_record_type;
        use nom::{IResult, Parser};

        #pdu_body_parser

        pub(crate) fn extension_record_set(input: &[u8]) -> IResult<&[u8], Vec<crate::ExtensionRecord>> {
            let (input, number_of_er) = nom::number::complete::le_u16(input)?;
            let (input, records) = nom::multi::count(extension_record, number_of_er.into()).parse(input)?;

            Ok((input, records))
        }

        pub(crate) fn extension_record(input: &[u8]) -> IResult<&[u8], crate::ExtensionRecord> {
            let (input, record_type) = nom::number::complete::le_u16(input)?;
            let record_type = crate::enumerations::ExtensionRecordTypes::from(record_type);
            let (input, record_length) = nom::number::complete::le_u16(input)?;

            let (input, body) = extension_record_body(&record_type, record_length.into())(input)?;

            Ok((input, crate::ExtensionRecord {
                record_type,
                record_length,
                body,
            }))
        }

        #extension_record_body_parser

        pub(crate) fn fixed_string_with_length(length: usize) -> impl Fn(&[u8]) -> IResult<&[u8], String> {
            move |input: &[u8]| {
                let (input, bytes) = nom::bytes::complete::take(length)(input)?;
                let string = String::from_utf8_lossy(bytes).to_string();

                Ok((input, string))
            }
        }

        pub(crate) fn opaque_data(length: usize) -> impl Fn(&[u8]) -> IResult<&[u8], Vec<u8>> {
            move |input: &[u8]| {
                let (input, bytes) = nom::bytes::complete::take(length)(input)?;
                Ok((input, bytes.to_vec()))
            }
        }
    };

    generate_module_with_name(PARSER_MODULE_NAME, &contents)
}

fn generate_common_pdu_body_parser(items: &[GenerationItem]) -> TokenStream {
    let parser_module = format_ident!("{PARSER_MODULE_NAME}");
    let pdu_type_arms = items
        .iter()
        .filter(|&it| it.is_pdu())
        .map(generate_common_pdu_body_parser_arm)
        .collect::<TokenStream>();
    let dis_pdu_type_ident = format_ident!("{PDU_TYPE}");

    quote! {
        pub fn pdu_body(header: &PDUHeader) -> impl Fn(&[u8]) -> IResult<&[u8], crate::PduBody> + '_ {
            move |input: &[u8]| {
                let (input, body) = match header.pdu_type {
                    #dis_pdu_type_ident::Other => crate::other_pdu::#parser_module::other_body(header)(input)?,
                    #pdu_type_arms
                    _ => crate::other_pdu::#parser_module::other_body(header)(input)?,
                };
                Ok((input, body))
            }
        }
    }
}

fn generate_common_pdu_body_parser_arm(pdu: &GenerationItem) -> TokenStream {
    if let GenerationItem::Pdu(pdu, _) = pdu {
        let dis_pdu_type_ident = format_ident!("{PDU_TYPE}");
        let pdu_type = &pdu.pdu_type_name;
        let pdu_path = &pdu.type_path;
        let parser_module = format_ident!("{PARSER_MODULE_NAME}");
        let parser_function = &pdu.parser_function;
        quote! {
            #dis_pdu_type_ident::#pdu_type => #pdu_path::#parser_module::#parser_function(input)?,
        }
    } else {
        panic!("GenerationItem is not a PDU.")
    }
}

fn generate_common_extension_record_body_parser(items: &[GenerationItem]) -> TokenStream {
    let extension_record_type = format_ident!("{EXTENSION_RECORD_TYPE}");
    let extension_record_types_arms = items
        .iter()
        .filter(|&it| it.is_extension_record())
        .map(generate_common_extension_record_body_parser_arm)
        .collect::<TokenStream>();

    quote! {
        pub fn extension_record_body(record_type: &ExtensionRecordTypes, record_length: usize) -> impl Fn(&[u8]) -> IResult<&[u8], crate::ExtensionRecordBody> + '_ {
            move |input: &[u8]| {
                let (input, body) = match record_type {
                    #extension_record_type::NotSpecified => crate::other_extension_record::parser::other_body(record_length)(input)?,
                    // TODO reserved for C-DIS, range 1-255
                    #extension_record_types_arms
                    _ => crate::other_extension_record::parser::other_body(record_length)(input)?,
                };
                Ok((input, body))
            }
        }
    }
}

fn generate_common_extension_record_body_parser_arm(er: &GenerationItem) -> TokenStream {
    if let GenerationItem::ExtensionRecord(er, _) = er {
        let er_record_type_ident = format_ident!("{EXTENSION_RECORD_TYPE}");
        let er_variant_name = format_ident!("{}", er.record_type_variant_name);
        let er_path = &er.type_path;
        let parser_module = format_ident!("{PARSER_MODULE_NAME}");
        let parser_function = &er.parser_function;
        quote! {
            #er_record_type_ident::#er_variant_name => #er_path::#parser_module::#parser_function(record_length)(input)?,
        }
    } else {
        panic!("GenerationItem is not an Extension Record.")
    }
}

pub(crate) fn generate_pdu_body_parser(pdu: &Pdu) -> TokenStream {
    let parser_function = &pdu.parser_function;
    let type_name = to_tokens(&pdu.type_name);
    let type_path = &pdu.type_path;
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
    let parser_module = to_tokens(PARSER_MODULE_NAME);

    quote! {
        use crate::BodyRaw;
        use nom::IResult;
        #[allow(unused_imports, reason = "Imported in every parser module instead of finding out the use of specific nom functions per module")]
        use nom::Parser;

        pub fn #parser_function(input: &[u8]) -> IResult<&[u8], crate::PduBody> {
            #(#field_parsers)*
            let (input, extension_records) = crate::#parser_module::extension_record_set(input)?;

            let body = #type_path::#type_name {
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
        PduAndFixedRecordFieldsEnum::FixedString(f) => generate_fixed_string_field_parser(f),
        PduAndFixedRecordFieldsEnum::FixedRecord(f) => generate_fixed_record_field_parser(f),
        PduAndFixedRecordFieldsEnum::BitRecord(f) => generate_bit_record_field_parser(f),
        PduAndFixedRecordFieldsEnum::AdaptiveRecord(f) => generate_adaptive_record_field_parser(f),
    }
}

pub(crate) fn generate_extension_record_body_parser(record: &ExtensionRecord) -> TokenStream {
    let parser_function = &record.parser_function;
    let type_name = to_tokens(&record.type_name);
    let type_path = &record.type_path;
    let record_type_variant = to_tokens(&record.record_type_variant_name);

    let field_parsers = record
        .fields
        .iter()
        .map(generate_extension_record_field_parser)
        .collect::<Vec<TokenStream>>();
    let fields = record
        .fields
        .iter()
        .filter(|field| !field.is_padding())
        .map(|field| {
            field
                .field_name()
                .parse::<TokenStream>()
                .expect("Failed to tokenise field name for Extension Record body parser")
        })
        .collect::<Vec<TokenStream>>();

    let (record_length_argument, padding_to_64) = if record.is_variable {
        (
            quote! {
                record_length
            },
            quote! {
                let padded_record = crate::utils::length_padded_to_num(record_length, 8);
                let (input, _padding_to_64) = nom::bytes::complete::take(padded_record.padding_length)(input)?;
            },
        )
    } else {
        let length = Literal::usize_suffixed(record.base_length);

        (
            quote! {
                _record_length
            },
            quote! {
                let (input, _padding_to_64) = nom::bytes::complete::take(#length)(input)?;
            },
        )
    };

    quote! {
        pub fn #parser_function(#record_length_argument: usize) -> impl Fn(&[u8]) -> IResult<&[u8], crate::ExtensionRecordBody> {
            move |input: &[u8]| {
                #(#field_parsers)*

                #padding_to_64

                let body = #type_path::#type_name {
                    #(#fields),*,
                };

                Ok((input, crate::ExtensionRecordBody::#record_type_variant(body)))
            }
        }
    }
}

pub(crate) fn generate_fixed_record_parser(record: &FixedRecord) -> TokenStream {
    let type_name = &record.type_name;
    let type_path = &record.type_path;
    let parser_function = &record.parser_function;

    let field_parsers = record
        .fields
        .iter()
        .map(generate_pdu_field_parser)
        .collect::<Vec<TokenStream>>();

    let fields = record
        .fields
        .iter()
        .filter(|field| !field.is_padding())
        .map(|field| {
            field
                .field_name()
                .parse::<TokenStream>()
                .expect("Failed to tokenise field name for Fixed Record parser")
        })
        .collect::<Vec<TokenStream>>();

    let body = quote! {
        #(#field_parsers)*

        let record = #type_path::#type_name {
                #(#fields),*,
            };

        Ok((input, record))
    };
    let body = if record.has_external_discriminants {
        quote! {
            move |input: &[u8]| {
                #body
            }
        }
    } else {
        body
    };

    quote! {
        pub(crate) fn #parser_function {
            #body
        }
    }
}

pub(crate) fn generate_bit_record_parser(record: &BitRecord) -> TokenStream {
    let type_name = &record.type_name;
    let type_path = &record.type_path;
    let size_primitive = to_tokens(field_size_to_primitive_type(record.size));

    let field_extractors = record
        .fields
        .iter()
        .map(bit_record_field_enum_parser)
        .collect::<TokenStream>();

    let fields = record
        .fields
        .iter()
        .filter(|field| !field.is_padding())
        .map(|field| {
            field
                .field_name()
                .parse::<TokenStream>()
                .expect("Failed to tokenise field name for BitRecord From<primitive> impl")
        })
        .collect::<Vec<TokenStream>>();

    quote! {
        impl From<#size_primitive> for #type_path::#type_name {
            fn from(value: #size_primitive) -> Self {
                #field_extractors

                #type_path::#type_name {
                    #(#fields),*
                }
            }
        }
    }
}

fn bit_record_field_enum_parser(field: &BitRecordFieldEnum) -> TokenStream {
    match field {
        BitRecordFieldEnum::Enum(f) => generate_enum_bit_field_parser(f),
        BitRecordFieldEnum::Int(f) => generate_int_bit_field_parser(f),
        BitRecordFieldEnum::Bool(f) => generate_bool_bit_field_parser(f),
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "Exponent value will not overflow u32::MAX"
)]
fn generate_enum_bit_field_parser(field: &EnumBitField) -> TokenStream {
    let field_name = format_ident!("{}", field.field_name);
    let type_name = &field.type_name;
    let type_path = &field.type_path;
    let bit_type = to_tokens(field_size_to_primitive_type(field.size));

    let shift_literal = Literal::usize_unsuffixed(field.bit_position);
    let bitmask_literal = Literal::usize_unsuffixed(2usize.pow(field.size as u32) - 1);

    let final_type = if type_path.is_empty() {
        quote! { #type_name }
    } else {
        quote! { #type_path::#type_name }
    };

    quote! {
        let #field_name = ((value >> #shift_literal ) & #bitmask_literal) as #bit_type;
        let #field_name = #final_type::from(#field_name);
    }
}

#[allow(
    clippy::cast_possible_truncation,
    reason = "Exponent value will not overflow u32::MAX"
)]
fn generate_int_bit_field_parser(field: &IntBitField) -> TokenStream {
    let field_name = if field.is_padding {
        format_ident!("_{}", &field.field_name)
    } else {
        format_ident!("{}", &field.field_name)
    };
    let field_type = &field.field_type;

    let shift_literal = Literal::usize_unsuffixed(field.bit_position);
    let bitmask_literal = Literal::usize_unsuffixed(2usize.pow(field.size as u32) - 1);

    quote! {
        let #field_name = ((value >> #shift_literal ) & #bitmask_literal) as #field_type;
    }
}

fn generate_bool_bit_field_parser(field: &BoolBitField) -> TokenStream {
    let field_name = format_ident!("{}", field.field_name);

    let shift_literal = Literal::usize_unsuffixed(field.bit_position);
    let bitmask_literal = Literal::usize_unsuffixed(1);

    quote! {
        let #field_name = ((value >> #shift_literal ) & #bitmask_literal) != 0;
    }
}

pub(crate) fn generate_adaptive_record_parser(record: &AdaptiveRecord) -> TokenStream {
    let type_path = &record.type_path;
    let type_name = &record.type_name;

    let discriminant_type = &record.discriminant_type;
    let discriminant_primitive = &record.discriminant_primitive_type;
    let primitive_size = to_tokens(field_length_to_primitive(record.length));

    let variant_arms = record
        .variants
        .iter()
        .enumerate()
        .map(|(index, variant)| {
            let index = Literal::usize_unsuffixed(index + record.discriminant_start_value);
            let variant_type_path = &variant.type_path;
            let variant_type_name = &variant.type_name;
            quote! { #index => #type_path::#type_name::#variant_type_name(#variant_type_path::#variant_type_name::from(value)), }
        })
        .collect::<Vec<TokenStream>>();

    quote! {
        impl From<(#discriminant_type, #primitive_size)> for #type_path::#type_name {
            fn from((discriminant, value): (#discriminant_type, #primitive_size)) -> Self {
                let discriminant_value = #discriminant_primitive::from(discriminant);
                match discriminant_value {
                    #(#variant_arms)*
                    _ => { Self::None }
                }
            }
        }
    }
}

fn generate_extension_record_field_parser(field: &ExtensionRecordFieldEnum) -> TokenStream {
    match field {
        ExtensionRecordFieldEnum::Numeric(f) => generate_numeric_field_parser(f),
        ExtensionRecordFieldEnum::Enum(f) => generate_enum_field_parser(f),
        ExtensionRecordFieldEnum::FixedString(f) => generate_fixed_string_field_parser(f),
        ExtensionRecordFieldEnum::VariableString(f) => generate_variable_string_field_parser(f),
        ExtensionRecordFieldEnum::FixedRecord(f) => generate_fixed_record_field_parser(f),
        ExtensionRecordFieldEnum::BitRecord(f) => generate_bit_record_field_parser(f),
        ExtensionRecordFieldEnum::Array(f) => generate_array_field_parser(f),
        ExtensionRecordFieldEnum::AdaptiveRecord(f) => generate_adaptive_record_field_parser(f),
        ExtensionRecordFieldEnum::Opaque(f) => generate_opaque_data_parser(f),
        ExtensionRecordFieldEnum::PaddingTo16 => unimplemented!(
            "Unimplemented as element <PaddingTo16> does not occur in the schema definitions"
        ),
        ExtensionRecordFieldEnum::PaddingTo32 => unimplemented!(
            "Unimplemented as element <PaddingTo32> does not occur in the schema definitions"
        ),
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
    let type_name = &field.type_name;
    let type_path = &field.type_path;
    let parser = &field.parser_function;
    let conversion = if field.parser_must_convert_to_enum {
        quote! { let #field_name = #type_path::#type_name::from(#field_name); }
    } else {
        quote! {}
    };

    quote! {
        let (input, #field_name) = #parser(input)?;
        #conversion
    }
}

fn generate_fixed_string_field_parser(field: &FixedStringField) -> TokenStream {
    let field_name = format_ident!("{}", &field.field_name);
    let parser_function = &field.parser_function;

    quote! {
        let (input, #field_name) = #parser_function(input)?;
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
    let final_type = finalise_type(&field.type_path, &field.type_name);

    quote! {
        let (input, #field_name) = #parser(input)?;
        let #field_name = #final_type::from(#field_name);
    }
}

fn generate_adaptive_record_field_parser(field: &AdaptiveRecordField) -> TokenStream {
    let field_name = format_ident!("{}", &field.field_name);
    let type_name = &field.type_name;
    let type_path = &field.type_path;
    let discriminant_field_name = format_ident!("{}", &field.discriminant_field_name);
    let parser = &field.parser_function;

    quote! {
        let (input, #field_name) = #parser(input)?;
        let #field_name = #type_path::#type_name::from((#discriminant_field_name, #field_name));
    }
}

fn generate_variable_string_field_parser(field: &VariableString) -> TokenStream {
    let string_field_name = format_ident!("{}", field.string_field.field_name);
    let count_field_name = format_ident!("{}", field.count_field.field_name);
    let count_parser = &field.count_field.parser_function;
    let string_parser = &field.string_field.parser_function;

    quote! {
        let (input, #count_field_name) = #count_parser(input)?;
        let (input, #string_field_name) = #string_parser(#count_field_name as usize)(input)?;
    }
}

fn generate_array_field_parser(array: &Array) -> TokenStream {
    let count_parser_function = &array.count_field.parser_function;
    let field_name = format_ident!("{}", array.type_field.field_name());
    let type_parser_function = generate_array_field_type_parser_function(&array.type_field);
    let conversion_function = generate_array_field_type_conversion_function(&array.type_field);

    quote! {
        let (input, count) = #count_parser_function(input)?;
        let (input, #field_name) = nom::multi::count(#type_parser_function, count.into()).parse(input)?;
        #conversion_function
    }
}

fn generate_array_field_type_parser_function(e: &ArrayFieldEnum) -> &TokenStream {
    match e {
        ArrayFieldEnum::Numeric(f) => &f.parser_function,
        ArrayFieldEnum::Enum(f) => &f.parser_function,
        ArrayFieldEnum::FixedString(f) => &f.parser_function,
        ArrayFieldEnum::FixedRecord(f) => &f.parser_function,
        ArrayFieldEnum::BitRecord(f) => &f.parser_function,
    }
}

fn generate_array_field_type_conversion_function(e: &ArrayFieldEnum) -> TokenStream {
    match e {
        ArrayFieldEnum::Enum(f) => {
            let field_name = to_tokens(&f.field_name);
            let field_type = finalise_type(&f.type_path, &f.type_name);
            quote! {
                let #field_name = #field_name.iter().map(|&item| #field_type::from(item) ).collect::<Vec<#field_type>>();
            }
        }
        ArrayFieldEnum::BitRecord(f) => {
            let field_name = to_tokens(&f.field_name);
            let field_type = finalise_type(&f.type_path, &f.type_name);
            quote! {
                let #field_name = #field_name.iter().map(|&item| #field_type::from(item) ).collect::<Vec<#field_type>>();
            }
        }
        _ => quote! {},
    }
}

fn generate_opaque_data_parser(field: &OpaqueData) -> TokenStream {
    let count_field_name = format_ident!("{}", &field.count_field.field_name);
    let count_parser_function = &field.count_field.parser_function;
    let data_field_name = format_ident!("{}", &field.opaque_data_field.field_name);
    let data_parser_function = &field.opaque_data_field.parser_function;

    quote! {
        let (input, #count_field_name) = #count_parser_function(input)?;
        let (input, #data_field_name) = #data_parser_function(#count_field_name as usize)(input)?;
    }
}
