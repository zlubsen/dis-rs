use nom::bytes::complete::take;
use nom::{Err, IResult};
use nom::error::ErrorKind::Eof;
use nom::multi::many1;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::sequence::tuple;

use crate::dis::common::model::{PDU_HEADER_LEN_BYTES, PduType, ProtocolFamily, ProtocolVersion};
use crate::dis::errors::DisError;
use crate::dis::v6::model::{Pdu, PduHeader};
use crate::dis::v6::entity_state::parser::entity_state_body;
use crate::dis::v6::other::parser::other_body;

pub fn parse_multiple_pdu(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    match many1(pdu)(input) {
        Ok((_, pdus)) => { Ok(pdus) }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

#[allow(dead_code)]
pub fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    match pdu(input) {
        Ok((_, pdu)) => { Ok(pdu) }
        Err(_) => { Err(DisError::ParseError) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

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

/// Parse the input for a PDU header, and skip the rest of the pdu body in the input
pub fn parse_header(input: &[u8]) -> Result<PduHeader, DisError> {
    match pdu_header(input) {
        Ok((input, header)) => {
            let skipped = skip_body(&header)(input); // Discard the body
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
    use crate::dis::common::model::{PDU_HEADER_LEN_BYTES, PduType, ProtocolFamily, ProtocolVersion};
    use crate::dis::errors::DisError;
    use crate::dis::v6::entity_state::model::{Afterburner, AirPlatformsRecord, ApTypeDesignator, ApTypeMetric, Country, DrAlgorithm, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityKind, EntityLights, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityTrailingEffect, EntityType, ForceId, FrozenStatus, GeneralAppearance, ParameterTypeVariant, PowerPlantStatus, SpecificAppearance, State};
    use crate::dis::v6::model::Pdu;
    use crate::dis::v6::{parse_multiple_header, parse_pdu};
    use crate::parse_v6_header;

    #[test]
    fn parse_header() {
        let bytes : [u8;12] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00];

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
    fn parse_header_too_short() {
        let bytes : [u8;10] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x60];

        let header = parse_v6_header(&bytes);
        assert!(header.is_err());
        let error = header.expect_err("Should be Err");
        assert_eq!(error, DisError::InsufficientHeaderLength(10));
    }

    #[test]
    fn parse_header_unspecified_version() {
        let bytes : [u8;12] = [0x1F, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00];

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

    #[test]
    fn parse_header_body_too_short() {
        // PDU whit header that states that the total length is 208 bytes, but only contains a 2 bytes body;
        let bytes: [u8; 14] =
            [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0xd0, 0x00, 0x00, 0x01, 0xf4];

        let header = parse_v6_header(&bytes);
        assert!(header.is_err());
        let error = header.expect_err("Should be Err");
        assert_eq!(error, DisError::InsufficientPduLength(208-PDU_HEADER_LEN_BYTES,2));
    }

    #[test]
    fn parse_pdu_entity_state() {
        let bytes : [u8;208] =
            [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0xd0, 0x00, 0x00, 0x01, 0xf4, 0x03, 0x84,
            0x00, 0x0e, 0x01, 0x04, 0x01, 0x02, 0x00, 0x99, 0x32, 0x04, 0x04, 0x00, 0x01, 0x02, 0x00, 0x99,
            0x32, 0x04, 0x04, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x41, 0x50, 0xc4, 0x1a, 0xde, 0xa4, 0xbe, 0xcc, 0x41, 0x50, 0xc9, 0xfa, 0x13, 0x3c, 0xf0, 0x5d,
            0x41, 0x35, 0x79, 0x16, 0x9e, 0x7a, 0x16, 0x78, 0xbf, 0x3e, 0xdd, 0xfa, 0x3e, 0x2e, 0x36, 0xdd,
            0x3f, 0xe6, 0x27, 0xc9, 0x00, 0x40, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x01, 0x3f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x4d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let pdu = parse_pdu(&bytes);
        assert!(pdu.is_ok());
        let pdu = pdu.unwrap();
        if let Pdu::EntityState(pdu) = pdu {
            assert_eq!(pdu.header.pdu_type, PduType::EntityStatePdu);
            assert_eq!(pdu.header.pdu_length, 208u16);
            assert_eq!(pdu.entity_id.simulation_address.site_id, 500u16);
            assert_eq!(pdu.entity_id.simulation_address.application_id, 900u16);
            assert_eq!(pdu.entity_id.entity_id, 14u16);
            assert_eq!(pdu.force_id, ForceId::Friendly);
            assert_eq!(pdu.articulated_parts_no, 4u8);
            assert_eq!(pdu.entity_type, EntityType {
                kind: EntityKind::Platform,
                domain: 2,
                country: Country::Netherlands,
                category: 50,
                subcategory: 4,
                specific: 4,
                extra: 0
            });
            assert_eq!(pdu.entity_appearance.general_appearance, GeneralAppearance {
                entity_paint_scheme: EntityPaintScheme::UniformColor,
                entity_mobility_kill: EntityMobilityKill::NoMobilityKill,
                entity_fire_power: EntityFirePower::NoFirePowerKill,
                entity_damage: EntityDamage::NoDamage,
                entity_smoke: EntitySmoke::NotSmoking,
                entity_trailing_effect: EntityTrailingEffect::None,
                entity_hatch_state: EntityHatchState::Open,
                entity_lights: EntityLights::None,
                entity_flaming_effect: EntityFlamingEffect::None,
            });
            if let SpecificAppearance::AirPlatform(record) = pdu.entity_appearance.specific_appearance {
                assert_eq!(record, AirPlatformsRecord {
                    afterburner: Afterburner::NotOn,
                    frozen_status: FrozenStatus::NotFrozen,
                    power_plant_status: PowerPlantStatus::Off,
                    state: State::Active,
                })
            } else { assert!(false) };
            assert_eq!(pdu.dead_reckoning_parameters.algorithm, DrAlgorithm::DrmRVW);
            assert_eq!(pdu.entity_marking.marking_string, String::from("EYE 10"));
            assert_eq!(pdu.entity_capabilities, EntityCapabilities {
                ammunition_supply: false,
                fuel_supply: false,
                recovery: false,
                repair: false,
            });
            let articulation_parameters = pdu.articulation_parameter.unwrap();
            assert_eq!(articulation_parameters.len(), 4);
            let parameter_1 = articulation_parameters.get(0).unwrap();
            assert_eq!(parameter_1.parameter_type_designator, ApTypeDesignator::Articulated);
            if let ParameterTypeVariant::ArticulatedParts(type_varient) = &parameter_1.parameter_type_variant {
                assert_eq!(type_varient.type_metric, ApTypeMetric::Position);
                assert_eq!(type_varient.type_class, 3072); // landing gear
            } else { assert!(false) }
            assert_eq!(parameter_1.articulation_parameter_value, 1f32);
        } else { assert!(false) }
    }

    #[test]
    fn parse_multiple_headers() {
        let bytes : [u8;24] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00
            ,0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00];

        let headers = parse_multiple_header(&bytes);
        assert!(headers.is_ok());
        let headers = headers.unwrap();
        assert_eq!(headers.len(), 2);
    }

    #[test]
    fn parse_multiple_headers_1st_too_short() {
        let bytes : [u8;23] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00
            ,0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00];
        // two pdus, headers with 0-byte body's; first header is one-byte short

        let headers = parse_multiple_header(&bytes);
        assert!(headers.is_ok());
        let headers = headers.unwrap();
        assert_eq!(headers.len(), 1);
        let header = headers.get(0).unwrap();
        assert_ne!(header.padding, 0);  // padding is not zero, because first byte of the next headers is there
    }

    #[test]
    fn parse_multiple_headers_too_short() {
        let bytes : [u8;11] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x60, 0x00];
        // buffer is too short for one pdu, let alone multiple.

        let headers = parse_multiple_header(&bytes);
        assert!(headers.is_err());
        let error = headers.expect_err("Should be Err");
        assert_eq!(error, DisError::InsufficientHeaderLength(11));
    }
}