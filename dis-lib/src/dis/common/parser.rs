use nom::combinator::peek;
use nom::IResult;
use nom::number::complete::be_u8;
use crate::dis::common::model::ProtocolVersion;
use crate::dis::errors::DisError;

pub fn parse_peek_protocol_version(input: &[u8]) -> Result<ProtocolVersion,DisError> {
    let parse_result = peek_protocol_version(input);
    match parse_result {
        Ok((_, protocol_version)) => Ok(ProtocolVersion::from(protocol_version)),
        Err(_err) => Err(DisError::ParseError),
    }
}

/// Function tries to peek the protocol version field of the DIS header
/// and return the raw value when successful.
fn peek_protocol_version(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, protocol_version) = peek(be_u8)(input)?;
    Ok((input, protocol_version))
}