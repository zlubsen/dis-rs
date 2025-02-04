use thiserror::Error;

use crate::constants::PDU_HEADER_LEN_BYTES;

#[derive(Debug, PartialEq, Eq, Error)]
pub enum DisError {
    // UnsupportedProtocolVersion,
    #[error("{0}")]
    ParseError(String), // the parsing of a PDU resulted in an error
    #[error("The buffer does not contain enough bytes for a valid DIS header. {0} bytes available, needed {PDU_HEADER_LEN_BYTES}")]
    InsufficientHeaderLength(u16), // the input was too small to contain a valid DIS header; (u16 found)
    #[error("PDU has insufficient length. Expected {0}, found {1}")]
    InsufficientPduLength(u16, u16), // the input was too small to contain a valid DIS Pdu based on the header and parsing; (u16 expected, u16 found)
    #[error("PDU is larger than size of the buffer for serialisation. Needs {0} bytes, available {1} bytes")]
    InsufficientBufferSize(u16, usize), // the buffer for serialisation has insufficient capacity to hold the provided PDU; (u16 PDU size, usize available capacity)
    #[error("Provided String is not valid ASCII encoded.")]
    StringNotAsciiError, // the String value to serialize is not valid ASCII encoded
    #[error("Provided String is too long.")]
    StringTooLongError, // the String value to serialize is too large for the field specification
    #[error("IFF PDU - Incorrect System Time provided.")]
    IffIncorrectSystemType, // the System Type in an IFF PDU is incorrect (to determine the type for parsing the basic data)
    #[error("IFF PDU - Undetermined System Time.")]
    IffUndeterminedSystemType, // the System Type in an IFF PDU does not determine whether it is an Interrogator or a Transponder
}
