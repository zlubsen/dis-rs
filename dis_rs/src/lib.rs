mod common;
mod v6;
mod v7;

include!(concat!(env!("OUT_DIR"), "/enumerations.rs"));

pub use enumerations::*;

pub use common::parse;
pub use common::parse_v6 as parse_v6_pdus;
pub use common::parse_v7 as parse_v7_pdus;

// TODO review required exports for the final API that the lib exposes
pub use common::model::*;
pub use common::symbolic_names::*;
pub use common::errors::*;

pub use common::entity_state::*;
pub use common::entity_state::model::*;
pub use common::other::*;

/*
TODO:
- Parse v7 entity_state PDU
- finalize way to handle builders
- revise writing pdu's based on version
- add entity appearance v7 to entity state
- common function for calculating body length based on header data (pdu_length - header_length); now at several places
- Incorporate Symbolic names from the standard (v7 - table 25)
- Dead-reckoning algorithms

TESTS:
- Build Other PDU
V Read Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header

ISSUES:
- Serialize OtherPdu: check for sufficient buffer size?

*/