#[derive(Debug, PartialEq, Eq)]
pub enum DisError {
    // UnsupportedProtocolVersion,
    ParseError, // the parsing of a PDU resulted in an error
    InsufficientHeaderLength(u16), // the input was too small to contain a valid DIS header; (u16 found)
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid DIS Pdu based on the header and parsing; (u16 expected, u16 found)
    StringNotAsciiError,    // the String value to serialize is not valid ASCII encoded
    StringTooLongError,     // the String value to serialize is too large for the field specification
    IffIncorrectSystemType, // the System Type in an IFF PDU is incorrect (to determine the type for parsing the basic data)
    IffUndeterminedSystemType, // the System Type in an IFF PDU does not determine whether it is an Interrogator or a Transponder
}