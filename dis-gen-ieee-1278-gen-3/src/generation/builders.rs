use crate::generation::models::{Pdu, PduAndFixedRecordFieldsEnum};
use crate::pre_processing::{finalise_type, to_tokens};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

pub fn generate_pdu_builder(item: &Pdu, builder_name_ident: &Ident) -> TokenStream {
    let type_name = to_tokens(&item.type_name);
    let final_type = finalise_type(&item.type_path, &type_name);
    let with_functions = item
        .fields
        .iter()
        .filter(|field| !field.is_padding())
        .map(generate_pdu_builder_functions)
        .collect::<Vec<TokenStream>>();

    quote! {
        pub struct #builder_name_ident(#final_type);

        impl Default for #builder_name_ident {
            fn default() -> Self {
                Self::new()
            }
        }

        impl #builder_name_ident {
            #[must_use]
            pub fn new() -> Self {
                #builder_name_ident(#final_type::default())
            }

            #[must_use]
            pub fn new_from_body(body: #final_type) -> Self {
                #builder_name_ident(body)
            }

            #[must_use]
            pub fn build(self) -> #final_type {
                self.0
            }

            #(#with_functions)*

            #[must_use]
            pub fn with_extension_record(mut self, record: crate::ExtensionRecord) -> Self {
                self.0.extension_records.push(record);
                self
            }

            #[must_use]
            pub fn with_extension_records(mut self, records: Vec<crate::ExtensionRecord>) -> Self {
                self.0.extension_records = records;
                self
            }
        }
    }
}

fn generate_pdu_builder_functions(field: &PduAndFixedRecordFieldsEnum) -> TokenStream {
    match field {
        PduAndFixedRecordFieldsEnum::Numeric(field) => {
            generate_pdu_builder_with_function(&field.field_name, &field.primitive_type, false)
        }
        PduAndFixedRecordFieldsEnum::Enum(field) => {
            let final_type = finalise_type(&field.type_path, &field.type_name);
            generate_pdu_builder_with_function(&field.field_name, &quote! { #final_type }, false)
        }
        PduAndFixedRecordFieldsEnum::FixedString(field) => generate_pdu_builder_with_function(
            &field.field_name,
            &quote! { impl Into<String> },
            true,
        ),
        PduAndFixedRecordFieldsEnum::FixedRecord(field) => {
            let final_type = finalise_type(&field.type_path, &field.type_name);
            generate_pdu_builder_with_function(&field.field_name, &quote! { #final_type }, false)
        }
        PduAndFixedRecordFieldsEnum::BitRecord(field) => {
            let final_type = finalise_type(&field.type_path, &field.type_name);
            generate_pdu_builder_with_function(&field.field_name, &quote! { #final_type }, false)
        }
        PduAndFixedRecordFieldsEnum::AdaptiveRecord(field) => {
            let final_type = finalise_type(&field.type_path, &field.type_name);
            generate_pdu_builder_with_function(&field.field_name, &quote! { #final_type }, false)
        }
    }
}

fn generate_pdu_builder_with_function(
    field_name: &str,
    field_path_and_type: &TokenStream,
    into: bool,
) -> TokenStream {
    let function_name_ident = format_ident!("with_{field_name}");
    let field_ident = format_ident!("{field_name}");
    let assignment_value = if into {
        quote! { #field_ident.into() }
    } else {
        quote! { #field_ident }
    };

    quote! {
            #[must_use]
            pub fn #function_name_ident(mut self, #field_ident: #field_path_and_type) -> Self {
                self.0.#field_ident = #assignment_value;
                self
            }
    }
}
