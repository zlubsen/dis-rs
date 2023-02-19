#[derive(Debug, PartialEq, Eq)]
pub enum DisError {
    // UnsupportedProtocolVersion,
    ParseError, // the parsing of a PDU resulted in an error
    InsufficientHeaderLength(u16), // the input was too small to contain a valid DIS header; (u16 found)
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid DIS Pdu based on the header and parsing; (u16 expected, u16 found)
}