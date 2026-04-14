use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};
use crate::constants::{PDU_TYPE, WRITER_MODULE_NAME};
use crate::generation::models::{generate_module_with_name, AdaptiveRecordField, BitRecordField, EnumField, FixedRecordField, FixedStringField, GenerationItem, NumericField, Pdu, PduAndFixedRecordFieldsEnum};
use crate::pre_processing::to_tokens;

pub(crate) fn generate_common_writers(items: &[GenerationItem]) -> TokenStream {
    let pdu_body_writer = generate_common_pdu_body_writer(items);

    let contents = quote! {
        use crate::PduBody;
        use crate::core::writer::Serialize;
        use bytes::BytesMut;

        #pdu_body_writer
    };

    generate_module_with_name(WRITER_MODULE_NAME, &contents)
}

fn generate_common_pdu_body_writer(items: &[GenerationItem]) -> TokenStream {
    let pdu_body_arms = items
        .iter()
        .filter(|&it| it.is_pdu())
        .map(generate_common_pdu_body_writer_arm)
        .collect::<TokenStream>();

    quote! {
        impl Serialize for PduBody {
            fn serialize(&self, buf: &mut BytesMut) -> u16 {
                match self {
                    PduBody::Other(body) => body.serialize(buf),
                    #pdu_body_arms
                }
            }
        }
    }
}

fn generate_common_pdu_body_writer_arm(pdu: &GenerationItem) -> TokenStream {
    let pdu_variant = to_tokens(pdu.variant_name().expect("PDUs have variant names"));
    quote! {
        PduBody::#pdu_variant(body) => body.serialize(buf),
    }
}

fn generate_pdu_body_writer(pdu: &Pdu) -> TokenStream {
    let type_name = to_tokens(&pdu.type_name);
    let type_path = &pdu.type_path;
    let field_writers = pdu.fields
        .iter()
        .map(generate_pdu_field_writer)
        .collect::<Vec<TokenStream>>();

    quote! {
        impl Serialize for #type_path::#type_name {
            fn serialize(&self, buf: &mut BytesMut) -> u16 {
                #(#field_writers)*
                self.extension_records.serialize(buf)

                self.body_length()
            }
        }
    }
}

fn generate_pdu_field_writer(field: &PduAndFixedRecordFieldsEnum) -> TokenStream {
    match field {
        PduAndFixedRecordFieldsEnum::Numeric(f) => generate_numeric_field_writer(f),
        PduAndFixedRecordFieldsEnum::Enum(f) => generate_enum_field_writer(f),
        PduAndFixedRecordFieldsEnum::FixedString(f) => generate_fixed_string_field_writer(f),
        PduAndFixedRecordFieldsEnum::FixedRecord(f) => generate_fixed_record_field_writer(f),
        PduAndFixedRecordFieldsEnum::BitRecord(f) => generate_bit_record_field_writer(f),
        PduAndFixedRecordFieldsEnum::AdaptiveRecord(f) => generate_adaptive_record_writer(f),
    }
}

fn generate_numeric_field_writer(field: &NumericField) -> TokenStream {
    let field_name = to_tokens(&field.field_name);
    let writer_function = &field.writer_function;

    quote! {
        buf.#writer_function(self.#field_name);
    }
}

fn generate_enum_field_writer(field: &EnumField) -> TokenStream {
    let field_name = to_tokens(&field.field_name);
    let writer_function = &field.writer_function;

    quote! {
        buf.#writer_function(self.#field_name.into());
    }
}

fn generate_fixed_string_field_writer(field: &FixedStringField) -> TokenStream {
    let field_name = to_tokens(&field.field_name);
    let length = Literal::usize_unsuffixed(field.length);

    quote! {
        let num_pad = #length.saturating_sub(self.#field_name.len());

        buf.put_slice(self.#field_name.as_bytes()[..]);
        (0..num_pad).for_each(|_i| buf.put_u8(0x00));
    }
}

fn generate_fixed_record_field_writer(field: &FixedRecordField) -> TokenStream {
    let field_name = to_tokens(&field.field_name);

    quote! {
        self.#field_name.serialize(buf);
    }
}

fn generate_bit_record_field_writer(field: &BitRecordField) -> TokenStream {
    let field_name = to_tokens(&field.field_name);

    quote! {
        self.#field_name.serialize(buf);
    }
}

fn generate_adaptive_record_writer(field: &AdaptiveRecordField) -> TokenStream {
    let field_name = to_tokens(&field.field_name);
    let discriminant_field = to_tokens(&field.discriminant_field_name);

    quote! {
        self.#field_name.serialize(buf, &self.#discriminant_field);
    }
}
