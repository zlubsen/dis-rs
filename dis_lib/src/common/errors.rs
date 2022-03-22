#[derive(Debug, PartialEq)]
pub enum DisError {
    // UnsupportedProtocolVersion,
    ParseError, // the parsing of a PDU resulted in an error
    InsufficientHeaderLength(usize), // the input was too small to contain a valid DIS header; (usize found)
    InsufficientPduLength(usize, usize), // the input was too small to contain a valid DIS Pdu based on the header and parsing; (usize expected, usize found)
}