use nom::{Err, IResult};
use nom::error::ErrorKind::Eof;
use nom::multi::many1;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::sequence::tuple;
use crate::dis::common::model::{PDU_HEADER_LEN_BYTES, PduType};
use crate::dis::common::parser::{pdu_type, protocol_family, protocol_version, skip_body};

use crate::dis::errors::DisError;
use crate::dis::v7::entity_state::parser::entity_state_body;
use crate::dis::v7::model::{ActiveInterrogationIndicator, build_pdu_status_aii_ism_cei_lvc_tei, build_pdu_status_cei_lvc, build_pdu_status_cei_lvc_tei, build_pdu_status_dti_cei_lvc, build_pdu_status_fti_cei_lvc, build_pdu_status_iai_cei_lvc_tei, build_pdu_status_lvc, build_pdu_status_rai_cei_lvc_tei, CoupledExtensionIndicator, DetonationTypeIndicator, FireTypeIndicator, IffSimulationMode, IntercomAttachedIndicator, LvcIndicator, Pdu, PduHeader, PduStatus, RadioAttachedIndicator, TransferredEntityIndicator};
use crate::dis::v7::other::parser::other_body;

// FIXME refactor v6 and v7 similarities: pass correct parser to generic function
pub fn parse_multiple_pdu(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    match many1(pdu)(input) {
        Ok((_, pdus)) => { Ok(pdus) }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

// FIXME refactor v6 and v7 similarities: pass correct parser to generic function
#[allow(dead_code)]
pub fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    match pdu(input) {
        Ok((_, pdu)) => { Ok(pdu) }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

// FIXME refactor v6 and v7 similarities: pass correct parser to generic function
pub fn parse_multiple_header(input: &[u8]) -> Result<Vec<PduHeader>, DisError> {
    match many1(pdu_header_skip_body)(input) {
        Ok((_, headers)) => { Ok(headers) }
        Err(parse_error) => {
            if let Err::Error(error) = parse_error {
                if error.code == Eof {
                    return Err(DisError::InsufficientHeaderLength(input.len()));
                }
            }
            Err(DisError::ParseError)
        }
    }
}

// FIXME refactor v6 and v7 similarities: pass correct parser to generic function
/// Parse the input for a PDU header, and skip the rest of the pdu body in the input
pub fn parse_header(input: &[u8]) -> Result<PduHeader, DisError> {
    match pdu_header(input) {
        Ok((input, header)) => {
            let skipped = skip_body(header.pdu_length)(input); // Discard the body
            if let Err(Err::Error(error)) = skipped {
                return if error.code == Eof {
                    Err(DisError::InsufficientPduLength(header.pdu_length as usize - PDU_HEADER_LEN_BYTES, input.len()))
                } else { Err(DisError::ParseError) }
            }
            Ok(header)
        }
        Err(parse_error) => {
            if let Err::Error(error) = parse_error {
                if error.code == Eof {
                    return Err(DisError::InsufficientHeaderLength(input.len()));
                }
            }
            Err(DisError::ParseError)
        }
    }
}

// FIXME refactor v6 and v7 similarities: pass correct parser to generic function
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
    let pdu_status = be_u8;
    let padding = be_u8;

    let (input, (protocol_version, exercise_id, pdu_type, protocol_family, time_stamp, pdu_length, pdu_status_bits, padding)) =
        tuple((protocol_version, exercise_id, pdu_type, protocol_family, time_stamp, pdu_length, pdu_status, padding))(input)?;
    let pdu_status = parse_pdu_status(pdu_type.into(), pdu_status_bits);

    Ok((input,
        PduHeader {
            protocol_version,
            exercise_id,
            pdu_type,
            protocol_family,
            time_stamp,
            pdu_length,
            pdu_status,
            padding,
        }))
}

// FIXME refactor v6 and v7 similarities: pass correct parser to generic function
fn pdu_header_skip_body(input: &[u8]) -> IResult<&[u8], PduHeader> {
    let (input, header) = pdu_header(input)?;
    let (input, _) = skip_body(header.pdu_length)(input)?;
    Ok((input, header))
}

/// Parses the pdu status field into a PduStatus struct, depending on the PduType
///
/// Note: parser should not be fed input that is consumed, but a copy of the pdu status byte (as it is already parsed earlier)
fn parse_pdu_status(pdu_type: u8, input : u8) -> PduStatus {
    let tei = status_tei(input);
    let lvc = status_lvc(input);
    let cei = status_cei(input);
    let fti = status_fti(input);
    let dti = status_dti(input);
    let rai = status_rai(input);
    let iai = status_iai(input);
    let ism = status_ism(input);
    let aii = status_aii(input);

    // TODO create (more) ergonomic builder for PduStatus struct
    match pdu_type {
        1 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        2 => { build_pdu_status_fti_cei_lvc(fti, cei, lvc) }
        3 => { build_pdu_status_dti_cei_lvc(dti, cei, lvc) }
        4..=22 => { build_pdu_status_cei_lvc(cei, lvc) }
        23 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        24 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        25 => { build_pdu_status_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        26 => { build_pdu_status_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        27 => { build_pdu_status_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        28 => { build_pdu_status_aii_ism_cei_lvc_tei(aii, ism, cei, lvc, tei) }
        29..=30 => { build_pdu_status_cei_lvc(cei, lvc) }
        31 => { build_pdu_status_iai_cei_lvc_tei(iai, cei, lvc, tei) }
        32 => { build_pdu_status_iai_cei_lvc_tei(iai, cei, lvc, tei) }
        33..=40 => { build_pdu_status_cei_lvc(cei, lvc) }
        41 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        42..=66 => { build_pdu_status_cei_lvc(cei, lvc) }
        67 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        68 => { build_pdu_status_cei_lvc(cei, lvc) }
        69 => { build_pdu_status_cei_lvc(cei, lvc) }
        70 => { build_pdu_status_cei_lvc(cei, lvc) }
        71 => { build_pdu_status_cei_lvc(cei, lvc) }
        72 => { build_pdu_status_lvc(lvc) }
        73..=255 | _ => {
            PduStatus {
                transferred_entity_indicator: None,
                lvc_indicator: None,
                coupled_extension_indicator: None,
                fire_type_indicator: None,
                detonation_type_indicator: None,
                radio_attached_indicator: None,
                intercom_attached_indicator: None,
                iff_simulation_mode: None,
                active_interrogation_indicator: None
            }
        }
    }
}

fn status_tei(pdu_status_field : u8) -> TransferredEntityIndicator {
    let tei = pdu_status_field & 0x01;
    TransferredEntityIndicator::from(tei)
}

fn status_lvc(pdu_status_field : u8) -> LvcIndicator {
    let lvc = pdu_status_field & 0x06;
    LvcIndicator::from(lvc)
}

fn status_cei(pdu_status_field : u8) -> CoupledExtensionIndicator {
    let cei = pdu_status_field & 0x08;
    CoupledExtensionIndicator::from(cei)
}

fn status_fti(pdu_status_field : u8) -> FireTypeIndicator {
    let fti = pdu_status_field & 0x10;
    FireTypeIndicator::from(fti)
}

fn status_dti(pdu_status_field : u8) -> DetonationTypeIndicator {
    let dti = pdu_status_field & 0x30;
    DetonationTypeIndicator::from(dti)
}

fn status_rai(pdu_status_field : u8) -> RadioAttachedIndicator {
    let rai = pdu_status_field & 0x30;
    RadioAttachedIndicator::from(rai)
}

fn status_iai(pdu_status_field : u8) -> IntercomAttachedIndicator {
    let iai = pdu_status_field & 0x30;
    IntercomAttachedIndicator::from(iai)
}

fn status_ism(pdu_status_field : u8) -> IffSimulationMode {
    let ism = pdu_status_field & 0x10;
    IffSimulationMode::from(ism)
}

fn status_aii(pdu_status_field : u8) -> ActiveInterrogationIndicator {
    let aii = pdu_status_field & 0x20;
    ActiveInterrogationIndicator::from(aii)
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