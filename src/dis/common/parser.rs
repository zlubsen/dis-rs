use nom::combinator::peek;
use nom::IResult;
use nom::number::complete::be_u8;

/// Function tries to peek the protocol version field of the DIS header
/// and return the raw value when successful.
pub fn peek_protocol_version(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, protocol_version) = peek(be_u8)(input)?;
    Ok((input, protocol_version))
}