mod common;
mod v7;

pub use common::parse;
pub use common::parse_v6 as parse_v6_pdus;
pub use common::parse_v7 as parse_v7_pdus;

// TODO required exports for the final API that the lib exposes
pub use common::model::*;
pub use common::symbolic_names::*;
pub use common::errors::*;

pub use common::entity_state::*;
pub use common::other::*;


/*
TODO:
V Finish EntityState model and builder functions (such as Appearance impl)
V Tests for parsing EntityState PDU
V Tests for parsing Other PDU
V Tests for parsing incomplete input / some santity checks on pdu lengths
- common function for calculating body length based on header data (pdu_length - header_length); now at several places
V Writing headers and pdus to buffer/network
V Build DIS v7 Header; model, builder, parser
V Factor out PDU types into common
- Build DIS v7 EntityState
- Incorporate versions of enumeration document into the lib (domain types, country codes, etc). Possibly set version to use as config option; default to latest.
- Incorporate Symbolic names from the standard (v7 - table 25)
- Dead-reckoning algorithms

TESTS:
- Build Other PDU
V Read Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header

ISSUES:
- Decide on use of 'Unspecified' enum values
- Serialize OtherPdu: check for sufficient buffer size?

*/