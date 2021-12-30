pub mod errors;
pub mod common;
pub mod v6;
pub mod v7;

pub enum Version {
    V6,
    V7,
    UNSUPPORTED,
}

pub enum Pdu {
    V6Pdu(v6::model::Pdu),
    V7Pdu(v7::model::Pdu),
}

/// Parses the contents of the input, determining the DIS version by itself.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
pub fn parse(input: &[u8]) -> Result<Vec<Pdu>, errors::DisError> {
    todo!()
}

/// Parses the contents of the input, based on the provided DIS version.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
pub fn parse_as_version(input: &[u8], version: Version) -> Result<Vec<Pdu>, errors::DisError> {
    todo!()
}

/// Parses the contents of the input as DIS version 6.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
pub fn parse_v6(input: &[u8]) -> Result<Vec<v6::model::Pdu>, errors::DisError> {
    todo!()
}

/// Parses the contents of the input as DIS version 7.
/// This function tries to parse as many PDUs as there are in the buffer,
/// assuming there are only complete PDUs present in the input.
pub fn parse_v7(input: &[u8]) -> Result<Vec<v7::model::Pdu>, errors::DisError> {
    todo!()
}