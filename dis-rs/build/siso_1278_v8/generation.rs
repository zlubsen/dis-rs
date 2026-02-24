use super::GenerationItem;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::str::FromStr;

// Module tree of generated sources:
// src/v8
//    common/           // Containing all common records
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

pub fn generate(items: &[GenerationItem]) -> TokenStream {
    let families = vec![
        "acoustic",
        "communications",
        "ee",
        "entity_info_interaction",
    ];
    // let mut generated_items = vec![];
    let generated = generate_v8_module(&families);

    println!("{generated}");

    generated
}

fn generate_v8_module(families: &Vec<&str>) -> TokenStream {
    let families: Vec<Ident> = families
        .iter()
        .map(|&family| format_ident!("{family}"))
        .collect();

    let common_mod = generate_common_module();
    quote!(
        pub mod v8 {
            // pub mod common;

            // #(pub mod #families; )*

            pub const HELLO_V8: &str = "Hello from v8";

            #common_mod
        }
    )
}

fn generate_common_module() -> TokenStream {
    let aap = TokenStream::from_str("// dit is wat code").unwrap();
    quote!(
        pub mod common {
            #aap
            pub const HELLO_COMMON: &str = "Hello from common";
        }
    )
}
