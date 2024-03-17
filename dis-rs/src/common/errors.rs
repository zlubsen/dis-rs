use std::fmt::{Display, Formatter};
use crate::constants::PDU_HEADER_LEN_BYTES;

#[derive(Debug, PartialEq, Eq)]
pub enum DisError {
    // UnsupportedProtocolVersion,
    ParseError(String), // the parsing of a PDU resulted in an error
    InsufficientHeaderLength(u16), // the input was too small to contain a valid DIS header; (u16 found)
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid DIS Pdu based on the header and parsing; (u16 expected, u16 found)
    InsufficientBufferSize(u16, usize), // the buffer for serialisation has insufficient capacity to hold the provided PDU; (u16 PDU size, usize available capacity)
    StringNotAsciiError,    // the String value to serialize is not valid ASCII encoded
    StringTooLongError,     // the String value to serialize is too large for the field specification
    IffIncorrectSystemType, // the System Type in an IFF PDU is incorrect (to determine the type for parsing the basic data)
    IffUndeterminedSystemType, // the System Type in an IFF PDU does not determine whether it is an Interrogator or a Transponder
}

impl Display for DisError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            DisError::ParseError(message) => { f.write_str(message.as_str()) }
            DisError::InsufficientHeaderLength(found_length) => { f.write_fmt(format_args!("The buffer does not contain enough bytes for a valid DIS header. {found_length} bytes available, needed {PDU_HEADER_LEN_BYTES}")) }
            DisError::InsufficientPduLength(expected, found) => { f.write_fmt(format_args!("PDU has insufficient length. Expected {expected}, found {found}")) }
            DisError::InsufficientBufferSize(needed, available) => { f.write_fmt(format_args!("PDU is larger than size of the buffer for serialisation. Needs {needed} bytes, available {available} bytes"))},
            DisError::StringNotAsciiError => { f.write_str("Provided String is not valid ASCII encoded.") }
            DisError::StringTooLongError => { f.write_str("Provided String is too long.") }
            DisError::IffIncorrectSystemType => { f.write_str("IFF PDU - Incorrect System Time provided.") }
            DisError::IffUndeterminedSystemType => { f.write_str("IFF PDU - Undetermined System Time.") }
        }
    }
}