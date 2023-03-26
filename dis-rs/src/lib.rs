extern crate core;

mod common;
mod v6;
mod v7;
mod constants;
mod fixed_parameters;
mod variable_parameters;

include!(concat!(env!("OUT_DIR"), "/enumerations.rs"));

pub use enumerations::*;

pub use common::parse;
pub use common::parse_v6 as parse_v6_pdus;
pub use common::parse_v7 as parse_v7_pdus;

// TODO review required exports for the API that the lib exposes
pub use fixed_parameters::*;
pub use variable_parameters::VariableParameters;

pub use common::model::*;
pub use common::errors::*;

pub use common::entity_state::*;
pub use common::entity_state::model::*;
pub use common::electromagnetic_emission::*;
pub use common::electromagnetic_emission::model::*;
pub use common::other::*;
pub use common::other::model::*;

/*
TODO:
- Common function for calculating body length based on header data (pdu_length - header_length); now at several places
- Dead-reckoning algorithms
- Improve error handling (specific errors for parsing and writing pdus, with possible validation checks)

DOCUMENTATION:
- Document behaviour that PduBodies try to model V7 as much as possible (parse v6 into), and use compatibility conversions to serialize (impl From<V7> for V6).

TESTS:
- Build Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header

FIXES / ERROR HANDLING CASES:
- Serialize OtherPdu: check for sufficient buffer size?

*/