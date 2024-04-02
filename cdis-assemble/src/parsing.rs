use nom::IResult;
use nom::multi::many1;
use dis_rs::enumerations::PduType;
use crate::{CdisBody, CdisError, CdisPdu};
use crate::entity_state::parser::entity_state_body;
use crate::records::model::CdisHeader;
use crate::records::parser::cdis_header;
use crate::unsupported::Unsupported;
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

pub(crate) fn cdis_pdu(input: BitInput) -> IResult<BitInput, CdisPdu> {
    let (input, header) = cdis_header(input)?;
    let (input, body) = cdis_body(&header)(input)?;

    Ok((input, CdisPdu {
        header,
        body,
    }))
}

pub(crate) fn cdis_body(header: &CdisHeader) -> impl Fn(BitInput) -> IResult<BitInput, CdisBody> + '_ {
    move | input: BitInput | {
        let (input, body) : (BitInput, CdisBody) = match header.pdu_type {
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
            // Unsupported PDUs in CDIS v1
            PduType::Other => { (input, CdisBody::Unsupported(Unsupported)) }
            PduType::Unspecified(_val) => { (input, CdisBody::Unsupported(Unsupported)) }
            _val => { (input, CdisBody::Unsupported(Unsupported)) }
        };

        Ok((input, body))
    }
}