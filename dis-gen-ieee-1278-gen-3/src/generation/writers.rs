use proc_macro2::TokenStream;
use quote::quote;
use crate::constants::WRITER_MODULE_NAME;
use crate::generation::models::{generate_module_with_name, GenerationItem};

pub(crate) fn generate_common_writers(items: &[GenerationItem]) -> TokenStream {
    let contents = quote! {
        
    };
    
    generate_module_with_name(WRITER_MODULE_NAME, &contents)
}

fn generate_common_pdu_body_writer(items: &[GenerationItem]) -> TokenStream {
    
}

fn generate_common_pdu_body_writer_arm(pdu: &GenerationItem) -> TokenStream {
    
}