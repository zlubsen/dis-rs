use nom::{Err, InputIter, InputTake, IResult};
use nom::error::ParseError;
use nom::number::complete::{be_u32, be_u8, be_u16};
use nom::sequence::tuple;

use crate::dis::errors::DisError;
use crate::dis::v6::model::{Pdu, PDU_HEADER_MIN_LEN_BYTES, PduHeader, PduType, ProtocolFamily, ProtocolVersion};
use crate::dis::v6::entity_state::parser::entity_state_body;

fn pdu(input: &[u8]) -> IResult<&[u8], Pdu> {
    // parse the header
    // let (input, header) = parse_pdu_header(input)?;
    let header_result = pdu_header(input);
    if let Err(err) = header_result {
        match err {
            // TODO return error from this match
            Err::Incomplete(_) => {  } // working with complete data, should not happen
            Err::Error(_) => {  } // would mean a malformed or too short PDU
            Err::Failure(_) => {  } // found some invalid data
        }
    }
    let (input, header) = header_result?;
    let (input, pdu) = pdu_body(header)(input)?;

    Ok((input, pdu))
}

// TODO handle using a parser error type (Err::Error)
// TODO alternative: implement for a custom wrapper type on the buffer?
fn has_minimal_header_len(input: &[u8]) -> Result<(), DisError> {
    if input.len() >= PDU_HEADER_MIN_LEN_BYTES {
        Ok(())
    } else {
        Err(DisError::InsufficientHeaderLength(PDU_HEADER_MIN_LEN_BYTES, input.len()))
    }
}

// TODO handle using a parser error type (Err::Error)
// TODO alternative: implement for a custom wrapper type on the buffer?
fn has_minimal_pdu_len(input: &[u8], expected_len: usize) -> Result<(), DisError> {
    if input.len() >= expected_len {
        Ok(())
    } else {
        Err(DisError::InsufficientPduLength(expected_len, input.len()))
    }
}

fn pdu_header(input: &[u8]) -> IResult<&[u8], PduHeader> {
    // if let Err(parse_error) = has_minimal_header_len(buf) {
    //     Err(Error(input, DisError::InsufficientHeaderLength()))
    //     return Err(nom::error::Error::new(input, ErrorKind);
    // }
    //
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

fn protocol_version(input: &[u8]) -> IResult<&[u8], ProtocolVersion> {
    let (input, protocol_version) = be_u8(input)?;
    let protocol_version = ProtocolVersion::try_from(protocol_version)?;
    Ok((input, protocol_version))
}

fn pdu_type(input: &[u8]) -> IResult<&[u8], PduType> {
    let (input, pdu_type) = be_u8(input)?;
    let pdu_type = PduType::try_from(pdu_type)?;
    Ok((input, pdu_type))
}

fn protocol_family(input: &[u8]) -> IResult<&[u8], ProtocolFamily> {
    let (input, protocol_family) = be_u8(input)?;
    nom::bytes::complete::take(1u8)(input);
    let protocol_family = ProtocolFamily::try_from(protocol_family)?;
    Ok((input, protocol_family))
}

fn pdu_body<Input, Error: ParseError<Input>>(
    header: PduHeader,
) -> impl Fn(Input) -> IResult<Input, Pdu, Error>
    where
        Input: InputIter + InputTake,
{
    // parse the body of the PDU based on the type
    match header.pdu_type {
        PduType::OtherPdu => {},
        PduType::EntityStatePdu => { entity_state_body(input, header) }
        PduType::FirePdu => {}
        PduType::DetonationPdu => {}
        PduType::CollisionPdu => {}
        PduType::ServiceRequestPdu => {}
        PduType::ResupplyOfferPdu => {}
        PduType::ResupplyReceivedPdu => {}
        PduType::ResupplyCancelPdu => {}
        PduType::RepairCompletePdu => {}
        PduType::RepairResponsePdu => {}
        PduType::CreateEntityPdu => {}
        PduType::RemoveEntityPdu => {}
        PduType::StartResumePdu => {}
        PduType::StopFreezePdu => {}
        PduType::AcknowledgePdu => {}
        PduType::ActionRequestPdu => {}
        PduType::ActionResponsePdu => {}
        PduType::DataQueryPdu => {}
        PduType::SetDataPdu => {}
        PduType::DataPdu => {}
        PduType::EventReportPdu => {}
        PduType::CommentPdu => {}
        PduType::ElectromagneticEmissionPdu => {}
        PduType::DesignatorPdu => {}
        PduType::TransmitterPdu => {}
        PduType::SignalPdu => {}
        PduType::ReceiverPdu => {}
        PduType::AnnounceObjectPdu => {}
        PduType::DeleteObjectPdu => {}
        PduType::DescribeApplicationPdu => {}
        PduType::DescribeEventPdu => {}
        PduType::DescribeObjectPdu => {}
        PduType::RequestEventPdu => {}
        PduType::RequestObjectPdu => {}
        PduType::TimeSpacePositionIndicatorFIPdu => {}
        PduType::AppearanceFIPdu => {}
        PduType::ArticulatedPartsFIPdu => {}
        PduType::FireFIPdu => {}
        PduType::DetonationFIPdu => {}
        PduType::PointObjectStatePdu => {}
        PduType::LinearObjectStatePdu => {}
        PduType::ArealObjectStatePdu => {}
        PduType::EnvironmentPdu => {}
        PduType::TransferControlRequestPdu => {}
        PduType::TransferControlPdu => {}
        PduType::TransferControlAcknowledgePdu => {}
        PduType::IntercomControlPdu => {}
        PduType::IntercomSignalPdu => {}
        PduType::AggregatePdu => {}
    }
}

