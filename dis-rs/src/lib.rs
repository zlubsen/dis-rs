mod common;
mod v6;
mod v7;
mod constants;

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
V Revise writing pdu's based on version
V Finish model of entity_state PDU (with v6 and v7 capabilities)
V Add entity appearance v7 to entity state
V Parse v7 entity_state PDU
- Finalize way to handle builders (how to build for different protocol versions, various fields with different layout)
- Common function for calculating body length based on header data (pdu_length - header_length); now at several places
- Incorporate Symbolic names from the standard (v7 - table 25)
- Dead-reckoning algorithms
- Improve error handling (specific errors for parsing and writing pdus, with possible validation checks)

DOCUMENTATION:
- Document behaviour that PduBodies try to model V7 as much as possible (parse v6 into), and use compatibility conversions to serialize (impl From<V7> for V6).

TESTS:
- Build Other PDU
V Read Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header

FIXES / ERROR HANDLING CASES:
- Serialize OtherPdu: check for sufficient buffer size?

*/