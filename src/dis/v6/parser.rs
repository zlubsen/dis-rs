use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::many1;
use nom::number::complete::{be_u32, be_u8, be_u16};
use nom::sequence::tuple;

use crate::dis::common::model::ProtocolVersion;
use crate::dis::errors::DisError;
use crate::dis::v6::model::{Pdu, PDU_HEADER_LEN_BYTES, PduHeader, PduType, ProtocolFamily};
use crate::dis::v6::entity_state::parser::entity_state_body;
use crate::dis::v6::other::parser::other_body;

pub fn parse_multiple_pdu(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    match many1(pdu)(input) {
        Ok((_, pdus)) => { Ok(pdus) }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

pub fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    match pdu(input) {
        Ok((_, pdu)) => { Ok(pdu) }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

pub fn parse_multiple_header(input: &[u8]) -> Result<Vec<PduHeader>, DisError> {
    match many1(pdu_header_skip_body)(input) {
        Ok((_, headers)) => { Ok(headers) }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

/// Parse the input for a PDU header, and skip the rest of the pdu body in the input
pub fn parse_header(input: &[u8]) -> Result<PduHeader, DisError> {
    match pdu_header(input) {
        Ok((input, header)) => {
            let skipped = skip_body(&header)(input); // Discard the body
            if let Err(_) = skipped {
                return Err(DisError::ParseError) // TODO not very descriptive / error means we can not skip enough bytes for the body
            }
            Ok(header)
        }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

fn pdu(input: &[u8]) -> IResult<&[u8], Pdu> {
    // parse the header
    let (input, header) = pdu_header(input)?;
    // parse the body based on the type
    // and produce the final pdu combined with the header
    let (input, pdu) = pdu_body(header)(input)?;

    Ok((input, pdu))
}

fn pdu_header(input: &[u8]) -> IResult<&[u8], PduHeader> {
    let protocol_version = protocol_version;
    let exercise_id = be_u8;
    let pdu_type = pdu_type;
    let protocol_family = protocol_family;
    let time_stamp= be_u32;
    let pdu_length = be_u16;
    let padding = be_u16;

    let (input, (protocol_version, exercise_id, pdu_type, protocol_family, time_stamp, pdu_length, padding)) =
    tuple((protocol_version, exercise_id, pdu_type, protocol_family, time_stamp, pdu_length, padding))(input)?;

    Ok((input,
        PduHeader {
            protocol_version,
            exercise_id,
            pdu_type,
            protocol_family,
            time_stamp,
            pdu_length,
            padding,
        }))
}

fn pdu_header_skip_body(input: &[u8]) -> IResult<&[u8], PduHeader> {
    let (input, header) = pdu_header(input)?;
    let (input, _) = skip_body(&header)(input)?;
    Ok((input, header))
}

fn skip_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    let bytes_to_skip = header.pdu_length as usize - PDU_HEADER_LEN_BYTES;
    move |input| {
        take(bytes_to_skip)(input)
    }
}

fn protocol_version(input: &[u8]) -> IResult<&[u8], ProtocolVersion> {
    let (input, protocol_version) = be_u8(input)?;
    let protocol_version = ProtocolVersion::from(protocol_version);
    Ok((input, protocol_version))
}

fn pdu_type(input: &[u8]) -> IResult<&[u8], PduType> {
    let (input, pdu_type) = be_u8(input)?;
    let pdu_type = PduType::from(pdu_type);
    Ok((input, pdu_type))
}

fn protocol_family(input: &[u8]) -> IResult<&[u8], ProtocolFamily> {
    let (input, protocol_family) = be_u8(input)?;
    let protocol_family = ProtocolFamily::from(protocol_family);
    Ok((input, protocol_family))
}

fn pdu_body(header: PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], Pdu> {
    move | input: &[u8] | {
        // parse the body of the PDU based on the type
        let (input, pdu) = match header.pdu_type {
            PduType::OtherPdu => { other_body(header)(input)? }
            PduType::EntityStatePdu => { entity_state_body(header)(input)? }
            _ => { other_body(header)(input)? }
            // PduType::FirePdu => {}
            // PduType::DetonationPdu => {}
            // PduType::CollisionPdu => {}
            // PduType::ServiceRequestPdu => {}
            // PduType::ResupplyOfferPdu => {}
            // PduType::ResupplyReceivedPdu => {}
            // PduType::ResupplyCancelPdu => {}
            // PduType::RepairCompletePdu => {}
            // PduType::RepairResponsePdu => {}
            // PduType::CreateEntityPdu => {}
            // PduType::RemoveEntityPdu => {}
            // PduType::StartResumePdu => {}
            // PduType::StopFreezePdu => {}
            // PduType::AcknowledgePdu => {}
            // PduType::ActionRequestPdu => {}
            // PduType::ActionResponsePdu => {}
            // PduType::DataQueryPdu => {}
            // PduType::SetDataPdu => {}
            // PduType::DataPdu => {}
            // PduType::EventReportPdu => {}
            // PduType::CommentPdu => {}
            // PduType::ElectromagneticEmissionPdu => {}
            // PduType::DesignatorPdu => {}
            // PduType::TransmitterPdu => {}
            // PduType::SignalPdu => {}
            // PduType::ReceiverPdu => {}
            // PduType::AnnounceObjectPdu => {}
            // PduType::DeleteObjectPdu => {}
            // PduType::DescribeApplicationPdu => {}
            // PduType::DescribeEventPdu => {}
            // PduType::DescribeObjectPdu => {}
            // PduType::RequestEventPdu => {}
            // PduType::RequestObjectPdu => {}
            // PduType::TimeSpacePositionIndicatorFIPdu => {}
            // PduType::AppearanceFIPdu => {}
            // PduType::ArticulatedPartsFIPdu => {}
            // PduType::FireFIPdu => {}
            // PduType::DetonationFIPdu => {}
            // PduType::PointObjectStatePdu => {}
            // PduType::LinearObjectStatePdu => {}
            // PduType::ArealObjectStatePdu => {}
            // PduType::EnvironmentPdu => {}
            // PduType::TransferControlRequestPdu => {}
            // PduType::TransferControlPdu => {}
            // PduType::TransferControlAcknowledgePdu => {}
            // PduType::IntercomControlPdu => {}
            // PduType::IntercomSignalPdu => {}
            // PduType::AggregatePdu => {}
        };
        // TODO handle result of pdu variable
        Ok((input, pdu))
    }
}

#[cfg(test)]
mod tests {
    use crate::dis::common::model::ProtocolVersion;
    use crate::dis::v6::model::{PDU_HEADER_LEN_BYTES, PduType, ProtocolFamily};
    use crate::parse_v6_header;

    #[test]
    fn parse_header() {
        let bytes : [u8;12] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x60, 0x00, 0x00];

        let header = parse_v6_header(&bytes);
        assert!(header.is_ok());
        let header = header.unwrap();
        assert_eq!(header.protocol_version, ProtocolVersion::Ieee1278_1a1998);
        assert_eq!(header.exercise_id, 1);
        assert_eq!(header.pdu_type, PduType::EntityStatePdu);
        assert_eq!(header.protocol_family, ProtocolFamily::EntityInformationInteraction);
        assert_eq!(header.time_stamp, 1323973472);
        assert_eq!(header.pdu_length, PDU_HEADER_LEN_BYTES as u16); // only the header, 0-bytes pdu body
    }

    #[test]
    fn parse_header_unspecified_version() {
        let bytes : [u8;12] = [0x1F, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x60, 0x00, 0x00];

        let header = parse_v6_header(&bytes);
        assert!(header.is_ok());
        let header = header.unwrap();
        assert_eq!(header.protocol_version, ProtocolVersion::Other);
        assert_eq!(header.exercise_id, 1);
        assert_eq!(header.pdu_type, PduType::EntityStatePdu);
        assert_eq!(header.protocol_family, ProtocolFamily::EntityInformationInteraction);
        assert_eq!(header.time_stamp, 1323973472);
        assert_eq!(header.pdu_length, PDU_HEADER_LEN_BYTES as u16); // only the header, 0-bytes pdu body
    }
}

