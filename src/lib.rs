mod dis;

pub use dis::parse;
pub use dis::parse_as_version;
pub use dis::parse_v6 as parse_v6_pdus;
pub use dis::parse_v7 as parse_v7_pdus;

pub use dis::v6::parse_header as parse_v6_header;
pub use dis::v6::parse_multiple_header as parse_v6_headers;

/*
TODO:
- Finish EntityState model and builder functions (such as Appearance impl)
- Finish parsing EntityState PDU
- Finish parsing Other PDU
- Build DIS v7 Header; model, builder, parser
- Writing headers and pdus to buffer/network

TESTS:
- Build Other PDU
- Read Other PDU
- Build EntityState PDU
- Reading EntityState PDU
- DIS v7 header

ISSUES:
- Decide on use of 'Unspecified' enum values

*/