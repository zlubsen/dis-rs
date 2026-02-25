use super::GenerationItem;
use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

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

pub fn generate(items: &[GenerationItem]) -> TokenStream {
    let families = vec![
        "acoustic",
        "communications",
        "ee",
        "entity_info_interaction",
    ];

    let generated = generate_v8_module(&families);

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
