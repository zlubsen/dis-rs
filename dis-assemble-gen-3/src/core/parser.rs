use crate::common_records::parser::pdu_header;
use crate::common_records::PDUHeader;
use crate::core::errors::DisError;
use crate::core::Pdu;
use crate::enumerations::DISProtocolVersion;
use crate::parser::pdu_body;
use crate::PDU_HEADER_LEN_BYTES;
use nom::bytes::complete::take;
use nom::combinator::peek;
use nom::error::ErrorKind::Eof;
use nom::multi::many1;
use nom::number::complete::be_u8;
use nom::Err;
use nom::IResult;
use nom::Parser;

pub(crate) fn parse_multiple_pdu(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    match many1(pdu).parse(input) {
        Ok((_, pdus)) => Ok(pdus),
        Err(err) => Err(DisError::ParseError(err.to_string())),
    }
}

#[allow(dead_code)]
pub(crate) fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    match pdu(input) {
        Ok((_, pdu)) => Ok(pdu),
        Err(err) => Err(DisError::ParseError(err.to_string())),
    }
}

#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn parse_multiple_header(input: &[u8]) -> Result<Vec<PDUHeader>, DisError> {
    match many1(pdu_header_skip_body).parse(input) {
        Ok((_, headers)) => Ok(headers),
        Err(parse_error) => {
            if let Err::Error(ref error) = parse_error {
                if error.code == Eof {
                    return Err(DisError::InsufficientHeaderLength(input.len() as u16));
                }
            }
            Err(DisError::ParseError(parse_error.to_string()))
        }
    }
}

/// Parse the input for a PDU header, and skip the rest of the pdu body in the input
#[allow(dead_code)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn parse_header(input: &[u8]) -> Result<PDUHeader, DisError> {
    match pdu_header(input) {
        Ok((input, header)) => {
            let skipped = skip_body(header.pdu_length)(input); // Discard the body
            if let Err(Err::Error(error)) = skipped {
                return if error.code == Eof {
                    Err(DisError::InsufficientPduLength(
                        header.pdu_length - PDU_HEADER_LEN_BYTES,
                        input.len() as u16,
                    ))
                } else {
                    Err(DisError::ParseError(
                        "ParseError while parsing a pdu header and skipping body.".to_string(),
                    ))
                };
            }
            Ok(header)
        }
        Err(parse_error) => {
            if let Err::Error(ref error) = parse_error {
                if error.code == Eof {
                    return Err(DisError::InsufficientHeaderLength(input.len() as u16));
                }
            }
            Err(DisError::ParseError(parse_error.to_string()))
        }
    }
}

#[cfg_attr(
    all(feature = "hotpath", not(feature = "_test_no_instrumentation")),
    hotpath::measure
)]
fn pdu(input: &[u8]) -> IResult<&[u8], Pdu> {
    // parse the header
    let (input, header) = pdu_header(input)?;

    // if (header.pdu_length - PDU_HEADER_LEN_BYTES) as usize > input.len() {
    //     // FIXME signal correct sort of error when the input is too small for the indicated PDU length
    //     return nom::error::make_error(input, nom::error::ErrorKind::Eof);
    // }

    // parse the body based on the type
    // and produce the final pdu combined with the header
    let (input, body) = pdu_body(&header)(input)?;

    Ok((input, Pdu { header, body }))
}

#[allow(dead_code)]
fn pdu_header_skip_body(input: &[u8]) -> IResult<&[u8], PDUHeader> {
    let (input, header) = pdu_header(input)?;
    let (input, _) = skip_body(header.pdu_length)(input)?;
    Ok((input, header))
}

#[allow(dead_code)]
pub(crate) fn parse_peek_protocol_version(input: &[u8]) -> Result<DISProtocolVersion, DisError> {
    let parse_result = peek_protocol_version(input);
    match parse_result {
        Ok((_, protocol_version)) => Ok(protocol_version),
        Err(err) => Err(DisError::ParseError(err.to_string())),
    }
}

/// Function tries to peek the protocol version field of the DIS header
#[allow(dead_code)]
fn peek_protocol_version(input: &[u8]) -> IResult<&[u8], DISProtocolVersion> {
    let (input, protocol_version) = peek(be_u8).parse(input)?;
    let protocol_version = DISProtocolVersion::from(protocol_version);
    Ok((input, protocol_version))
}

/// Skip the bytes of a PDU's body, by calculating the total length minus the length of a header.
/// The function will skip zero bytes when the total length provided is less than the length of a header (12 bytes).
#[allow(dead_code)]
pub(crate) fn skip_body(total_bytes: u16) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    // if total_bytes <= PDU_HEADER_LEN_BYTES {
    //     return Err(nom::error::Error {
    //         input: (),
    //         code: ErrorKind::Tag,
    //     } )
    // }
    let bytes_to_skip = total_bytes.saturating_sub(PDU_HEADER_LEN_BYTES);
    move |input| take(bytes_to_skip)(input)
}
