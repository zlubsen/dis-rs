use nom::{Finish, IResult};
use nom::multi::many1;
use dis_rs::DisError;
use dis_rs::enumerations::PduType;
use crate::{CdisBody, CdisError, CdisPdu};
use crate::entity_state::parser::entity_state_body;
use crate::records::model::CdisHeader;
use crate::records::parser::cdis_header;
use crate::utils::BitInput;

/// Attempts to parse the provided buffer for CDIS PDUs
pub fn parse(input: &[u8]) -> Result<Vec<CdisPdu>, CdisError> {
    parse_multiple_cdis_pdu(input)
}

pub(crate) fn parse_multiple_cdis_pdu(input: &[u8]) -> Result<Vec<CdisPdu>, CdisError> {
    match many1(cdis_pdu)((input, 0)) {
        Ok((_, pdus)) => { Ok(pdus) }
        Err(err) => { Err(CdisError::ParseError(err.to_string())) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

pub(crate) fn cdis_pdu(input: BitInput) -> IResult<BitInput, CdisPdu, CdisError> {
    let (input, header) = cdis_header(input)?;
    let (input, body) = cdis_body(&header)(input)?;

    Ok((input, CdisPdu {
        header,
        body,
    }))
}

pub(crate) fn cdis_body(header: &CdisHeader) -> impl Fn(BitInput) -> IResult<BitInput, CdisBody> {
    move | input: BitInput | {
        let (input, header) = cdis_header(input)?;
        let (input, body) : (BitInput, Result<CdisBody, DisError>) = match header.pdu_type {
            PduType::EntityState => { entity_state_body(input)? }
            // PduType::Fire => {}
            // PduType::Detonation => {}
            // PduType::Collision => {}
            // PduType::CreateEntity => {}
            // PduType::RemoveEntity => {}
            // PduType::StartResume => {}
            // PduType::StopFreeze => {}
            // PduType::Acknowledge => {}
            // PduType::ActionRequest => {}
            // PduType::ActionResponse => {}
            // PduType::DataQuery => {}
            // PduType::SetData => {}
            // PduType::Data => {}
            // PduType::EventReport => {}
            // PduType::Comment => {}
            // PduType::ElectromagneticEmission => {}
            // PduType::Designator => {}
            // PduType::Transmitter => {}
            // PduType::Signal => {}
            // PduType::Receiver => {}
            // PduType::IFF => {}
            PduType::Other => { Err(CdisError::UnsupportedPdu(0)) }            // TODO make an implementation for Other PDUs
            PduType::Unspecified(_val) => { Err(CdisError::UnsupportedPdu(_val.into())) }
            _val => { Err(CdisError::UnsupportedPdu(_val.into())) } // Unsupported PDUs in CDIS v1
        };
        let body = body?;
        Ok((input, body))
    }
}