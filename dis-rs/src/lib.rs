#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "Parsing, writing, encoding, decoding PDUs uses many valid conversions"
)]

extern crate core;

mod common;
mod constants;
mod fixed_parameters;
pub mod utils;
mod v6;
mod v7;
mod variable_parameters;

include!(concat!(env!("OUT_DIR"), "/enumerations.rs"));

pub use common::entity_state::parser::dr_other_parameters as parse_dr_other_parameters;
pub use common::parse;
pub use common::parse_v6 as parse_v6_pdus;
pub use common::parse_v7 as parse_v7_pdus;
pub use common::Serialize;
pub use v7::entity_state::entity_capabilities_from_bytes;
pub use v7::parser::parse_pdu_status_fields;
pub use v7::writer::serialize_pdu_status;

pub use common::*;

pub use fixed_parameters::*;
pub use variable_parameters::VariableParameters;

pub use common::errors::*;

/*
TODO:
- Common function for calculating body length based on header data (pdu_length - header_length); now at several places
- Dead-reckoning algorithms

TESTS:
- Build Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header, PduStatus fields

*/
