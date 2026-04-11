use crate::core::Pdu;

pub(crate) fn parse_multiple_pdu(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    match many1(pdu).parse(input) {
        Ok((_, pdus)) => Ok(pdus),
        Err(err) => Err(DisError::ParseError(err.to_string())), // TODO not very descriptive / error means we can not match any PDUs
    }
}

#[allow(dead_code)]
pub(crate) fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    match pdu(input) {
        Ok((_, pdu)) => Ok(pdu),
        Err(err) => Err(DisError::ParseError(err.to_string())), // TODO not very descriptive / error means we can not match any PDUs
    }
}

#[allow(dead_code)]
pub(crate) fn parse_multiple_header(input: &[u8]) -> Result<Vec<PduHeader>, DisError> {
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
pub(crate) fn parse_header(input: &[u8]) -> Result<PduHeader, DisError> {
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
