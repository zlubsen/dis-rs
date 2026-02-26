use super::{GenerationItem, Pdu};
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

// Module tree of generated sources:
// src/v8
//    common_records/           // Containing all common records
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

// TODO
// - create separate crates for siso_ref_010 and siso_1278_v8 code generation, to make them testable.
// - experiment with xsd_parser to generate the intermediate representation, again in separate crates
// - annotate the GenerationItems with module structure context (family, pdu, common).

// Approach
// Construct a tree structure, where each node represents a code unit (module, struct, enum, function, impl block).
// Each node in the tree contains the children contained in that code unit. E.g modules, which are parents, and Structs, impl blocks, etc are the leafs.
// Tree construction is an intermediate step before code generation.
// 1. Create a Vec<(String, TokenStream)>, filled with all pieces of generated code.
// 2. Walk the tree and concat all produced TokenStreams into one.
// 3. Write the final code into the output file.

struct Tree {
    next_id: usize,
    root: Vec<Node>,
}

impl Tree {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            root: vec![],
        }
    }

    pub fn new_with_root(root: Node) -> Self {
        Self {
            next_id: 1,
            root: vec![root],
        }
    }
}

struct Node {
    id: usize,
    item: NodeItem,
    children: Vec<usize>,
}

enum NodeItem {
    Module(String),
    Unit(GenerationItem),
}

pub fn generate(items: &[GenerationItem], families: &[String]) -> TokenStream {
    let contents = families
        .iter()
        .map(|family| generate_family_module(items, family.as_str()))
        .collect();
    let generated = generate_named_module("v8", &contents);

    println!("{generated}");

    generated
}

fn generate_v8_module(families: &Vec<&str>) -> TokenStream {
    let families: Vec<Ident> = families
        .iter()
        .map(|&family| format_ident!("{family}"))
        .collect();

    let contents = quote!(
        pub const HELLO_NAMED: &str = "Hello from a module";
    );
    let b = quote!(
        pub const GOODBYE_NAMED: &str = "Ciao!";
    );

    let together = quote!(
        #contents
        #b
    );
    let common_module = generate_named_module("common", &together);

    // FIXME remove HELLO_V8 placeholder
    quote!(
        pub mod v8 {
            pub const HELLO_V8: &str = "Hello from v8";

            #common_module

            // #(pub mod #families; )*
        }
    )
}

fn generate_named_module(name: &str, contents: &TokenStream) -> TokenStream {
    let name_literal = format_ident!("{name}");
    quote!(
        pub mod #name_literal {
            #contents
        }
    )
}

fn generate_family_module(items: &[GenerationItem], family: &str) -> TokenStream {
    // 1. Filter items for this family
    // 2. Filter the PDUs and generate these in separate modules
    // 3. Filter the non-PDU items and generate the records in the family module
    // 4. Merge resulting TokenStreams

    let pdus = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && item.is_pdu())
        .map(|pdu| {
            if let GenerationItem::Pdu(pdu, _) = pdu {
                generate_pdu_module(pdu)
            } else {
                quote!( compile error; )
            }
        })
        .collect::<TokenStream>();

    let records = items
        .iter()
        .filter(|&item| (item.family().as_str() == family) && !item.is_pdu())
        .collect::<Vec<&GenerationItem>>();

    println!("{records:?}");

    generate_named_module(family, &pdus)
}

fn generate_pdu_module(item: &Pdu) -> TokenStream {
    let formatted_name = item.name_attr.replace(' ', "");
    println!(
        "generate_pdu_module - {} - {}",
        item.name_attr, formatted_name
    );
    let ident = format_ident!("{}", formatted_name);
    let contents = quote!(
        pub const NAME: &str = #ident;
    );

    generate_named_module(formatted_name.as_str(), &contents)
}
