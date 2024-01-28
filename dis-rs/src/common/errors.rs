use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Eq)]
pub enum DisError {
    // UnsupportedProtocolVersion,
    ParseError(String), // the parsing of a PDU resulted in an error
    InsufficientHeaderLength(u16), // the input was too small to contain a valid DIS header; (u16 found)
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid DIS Pdu based on the header and parsing; (u16 expected, u16 found)
    StringNotAsciiError,    // the String value to serialize is not valid ASCII encoded
    StringTooLongError,     // the String value to serialize is too large for the field specification
    IffIncorrectSystemType, // the System Type in an IFF PDU is incorrect (to determine the type for parsing the basic data)
    IffUndeterminedSystemType, // the System Type in an IFF PDU does not determine whether it is an Interrogator or a Transponder
}

impl Display for DisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DisError::ParseError(message) => { f.write_str(message.as_str()) }
            DisError::InsufficientHeaderLength(found_length) => { f.write_fmt(format_args!("Found length {}", found_length)) }
            DisError::InsufficientPduLength(expected, found) => { f.write_fmt(format_args!("PDU has insufficient length. Expected {}, found {}", expected, found)) }
            DisError::StringNotAsciiError => { f.write_str("Provided String is not valid ASCII encoded.") }
            DisError::StringTooLongError => { f.write_str("Provided String is too long.") }
            DisError::IffIncorrectSystemType => { f.write_str("IFF PDU - Incorrect System Time provided.") }
            DisError::IffUndeterminedSystemType => { f.write_str("IFF PDU - Undetermined System Time.") }
        }
    }
}