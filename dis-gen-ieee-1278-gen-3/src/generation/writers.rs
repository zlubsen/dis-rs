use crate::constants::{ARRAY_ELEMENT_IDENT, WRITER_MODULE_NAME};
use crate::generation::models::{
    generate_module_with_name, AdaptiveRecordField, Array, ArrayFieldEnum, BitRecord,
    BitRecordField, BitRecordFieldEnum, EnumField, ExtensionRecord, ExtensionRecordFieldEnum,
    FixedRecord, FixedRecordField, FixedStringField, GenerationItem, NumericField, OpaqueData, Pdu,
    PduAndFixedRecordFieldsEnum, VariableString,
};
use crate::pre_processing::to_tokens;
use itertools::Itertools;
use proc_macro2::{Literal, TokenStream};
use quote::{format_ident, quote};

pub(crate) fn generate_common_writers(items: &[GenerationItem]) -> TokenStream {
    let pdu_body_writer = generate_common_pdu_body_writer(items);

    let contents = quote! {
        use crate::PduBody;
        use crate::core::writer::Serialize;
        use bytes::{BytesMut, BufMut};

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

pub(crate) fn generate_pdu_body_writer(pdu: &Pdu) -> TokenStream {
    let type_name = to_tokens(&pdu.type_name);
    let type_path = &pdu.type_path;
    let field_writers = pdu
        .fields
        .iter()
        .map(generate_pdu_and_fixed_record_field_writer)
        .collect::<Vec<TokenStream>>();

    quote! {
        use bytes::{BytesMut, BufMut};
        use crate::core::BodyRaw;
        use crate::core::writer::Serialize;

        impl Serialize for #type_path::#type_name {
            fn serialize(&self, buf: &mut BytesMut) -> u16 {
                #(#field_writers)*
                self.extension_records.serialize(buf);

                self.body_length()
            }
        }
    }
}

pub(crate) fn generate_extension_record_body_writer(record: &ExtensionRecord) -> TokenStream {
    let type_name = to_tokens(&record.type_name);
    let type_path = &record.type_path;
    let field_writers = record
        .fields
        .iter()
        .map(generate_extension_record_field_writer)
        .collect::<Vec<TokenStream>>();

    quote! {
        impl Serialize for #type_path::#type_name {
            fn serialize(&self, buf: &mut BytesMut) -> u16 {
                #(#field_writers)*

                self.record_length()
            }
        }
    }
}

fn generate_pdu_and_fixed_record_field_writer(field: &PduAndFixedRecordFieldsEnum) -> TokenStream {
    match field {
        PduAndFixedRecordFieldsEnum::Numeric(f) => {
            generate_numeric_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        PduAndFixedRecordFieldsEnum::Enum(f) => {
            generate_enum_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        PduAndFixedRecordFieldsEnum::FixedString(f) => {
            generate_fixed_string_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        PduAndFixedRecordFieldsEnum::FixedRecord(f) => {
            generate_fixed_record_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        PduAndFixedRecordFieldsEnum::BitRecord(f) => {
            generate_bit_record_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        PduAndFixedRecordFieldsEnum::AdaptiveRecord(f) => generate_adaptive_record_field_writer(f),
    }
}

fn generate_extension_record_field_writer(field: &ExtensionRecordFieldEnum) -> TokenStream {
    match field {
        ExtensionRecordFieldEnum::Numeric(f) => {
            generate_numeric_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        ExtensionRecordFieldEnum::Enum(f) => {
            generate_enum_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        ExtensionRecordFieldEnum::FixedString(f) => {
            generate_fixed_string_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        ExtensionRecordFieldEnum::VariableString(f) => generate_variable_string_field_writer(f),
        ExtensionRecordFieldEnum::FixedRecord(f) => {
            generate_fixed_record_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        ExtensionRecordFieldEnum::BitRecord(f) => {
            generate_bit_record_field_writer(f, &to_tokens(&format!("self.{}", &f.field_name)))
        }
        ExtensionRecordFieldEnum::Array(f) => generate_array_field_writer(f),
        ExtensionRecordFieldEnum::AdaptiveRecord(f) => generate_adaptive_record_field_writer(f),
        ExtensionRecordFieldEnum::Opaque(f) => generate_opaque_field_writer(f),
        ExtensionRecordFieldEnum::PaddingTo16 => {
            unimplemented!(
                "Unimplemented as element <PaddingTo16> does not occur in the schema definitions"
            )
        }
        ExtensionRecordFieldEnum::PaddingTo32 => {
            unimplemented!(
                "Unimplemented as element <PaddingTo32> does not occur in the schema definitions"
            )
        }
    }
}

fn generate_bit_record_field_enum_writer(field: &BitRecordFieldEnum) -> TokenStream {
    match field {
        BitRecordFieldEnum::Enum(e) => {
            let field_name = to_tokens(&e.field_name);
            quote! {
                // 1- enum to primitive based on it's size
                // 2- optionally convert to record size
                // 3- shift into position
                let #field_name =
            }
        }
        BitRecordFieldEnum::Int(i) => {
            todo!()
        }
        BitRecordFieldEnum::Bool(b) => {
            todo!()
        }
    }
}

pub(crate) fn generate_fixed_record_writer(record: &FixedRecord) -> TokenStream {
    let type_name = &record.type_name;
    let type_path = &record.type_path;

    let field_writers = record
        .fields
        .iter()
        .map(generate_pdu_and_fixed_record_field_writer)
        .collect::<Vec<TokenStream>>();

    quote! {
        impl Serialize for #type_path::#type_name {
            fn serialize(&self, buf: &mut BytesMut) -> u16 {
                #(#field_writers)*
                self.record_length()
            }
        }
    }
}

pub(crate) fn generate_bit_record_writer(record: &BitRecord) -> TokenStream {
    let type_name = &record.type_name;
    let type_path = &record.type_path;
    let writer_function = &record.writer_function;

    let field_values = record
        .fields
        .iter()
        .map(generate_bit_record_field_enum_writer)
        .collect::<Vec<TokenStream>>();

    let fields_or = to_tokens(
        &record
            .fields
            .iter()
            .map(|field| field.field_name())
            .join(" | "),
    );

    quote! {
        impl Serialize for #type_path::#type_name {
            fn serialize(&self, buf: &mut BytesMut) -> u16 {
                #(#field_values)*

                let value = #fields_or;
                buf.#writer_function(value)

                self.record_length()
            }
        }
    }
}

fn generate_numeric_field_writer(field: &NumericField, field_name: &TokenStream) -> TokenStream {
    let writer_function = &field.writer_function;

    if field.is_padding {
        quote! {
            buf.#writer_function(0);
        }
    } else {
        quote! {
            buf.#writer_function(#field_name);
        }
    }
}

fn generate_enum_field_writer(field: &EnumField, field_name: &TokenStream) -> TokenStream {
    let writer_function = &field.writer_function;

    quote! {
        buf.#writer_function(#field_name.into());
    }
}

fn generate_fixed_string_field_writer(
    field: &FixedStringField,
    field_name: &TokenStream,
) -> TokenStream {
    let length = Literal::usize_suffixed(field.length);

    // Too long strings will be truncated, leaving no padding in the fixed string field
    quote! {
        let num_pad = #length.saturating_sub(#field_name.len());
        let string_length = #length - num_pad;

        buf.put(&#field_name.as_bytes()[..string_length]);
        (0..num_pad).for_each(|_i| buf.put_u8(0x00));
    }
}

fn generate_fixed_record_field_writer(
    _field: &FixedRecordField,
    field_name: &TokenStream,
) -> TokenStream {
    quote! {
        #field_name.serialize(buf);
    }
}

fn generate_bit_record_field_writer(
    _field: &BitRecordField,
    field_name: &TokenStream,
) -> TokenStream {
    quote! {
        #field_name.serialize(buf);
    }
}

fn generate_adaptive_record_field_writer(field: &AdaptiveRecordField) -> TokenStream {
    let field_name = to_tokens(&field.field_name);
    let discriminant_field = to_tokens(&field.discriminant_field_name);

    quote! {
        self.#field_name.serialize(buf, &self.#discriminant_field);
    }
}

fn generate_variable_string_field_writer(field: &VariableString) -> TokenStream {
    let count_writer_function = &field.count_field.writer_function;
    let count_primitive_type = &field.count_field.primitive_type;
    let str_field_name = to_tokens(&field.string_field.field_name);

    quote! {
        buf.#count_writer_function(self.#str_field_name.len() as #count_primitive_type);
        buf.put(&self.#str_field_name.as_bytes()[..]);
    }
}

fn generate_array_field_writer(field: &Array) -> TokenStream {
    let count_writer_function = &field.count_field.writer_function;
    let count_primitive_type = &field.count_field.primitive_type;
    let type_field_name = to_tokens(field.type_field.field_name());
    let field_writer_function = generate_array_field_type_writer_function(&field.type_field);
    let item_ident = format_ident!("{ARRAY_ELEMENT_IDENT}");

    quote! {
        buf.#count_writer_function(self.#type_field_name.len() as #count_primitive_type);
        self.#type_field_name.iter().for_each(|#item_ident| { #field_writer_function } );
    }
}

fn generate_array_field_type_writer_function(field: &ArrayFieldEnum) -> TokenStream {
    match field {
        ArrayFieldEnum::Numeric(f) => {
            generate_numeric_field_writer(f, &to_tokens(&format!("*{ARRAY_ELEMENT_IDENT}")))
        }
        ArrayFieldEnum::Enum(f) => {
            generate_enum_field_writer(f, &to_tokens(&format!("*{ARRAY_ELEMENT_IDENT}")))
        }
        ArrayFieldEnum::FixedString(f) => {
            generate_fixed_string_field_writer(f, &to_tokens(ARRAY_ELEMENT_IDENT))
        }
        ArrayFieldEnum::FixedRecord(f) => {
            generate_fixed_record_field_writer(f, &to_tokens(ARRAY_ELEMENT_IDENT))
        }
        ArrayFieldEnum::BitRecord(f) => {
            generate_bit_record_field_writer(f, &to_tokens(ARRAY_ELEMENT_IDENT))
        }
    }
}

fn generate_opaque_field_writer(field: &OpaqueData) -> TokenStream {
    let count_writer_function = &field.count_field.writer_function;
    let count_primitive_type = &field.count_field.primitive_type;
    let field_name = to_tokens(&field.opaque_data_field.field_name);

    quote! {
        buf.#count_writer_function(self.#field_name.len() as #count_primitive_type);
        buf.put(&self.#field_name[..]);
    }
}
