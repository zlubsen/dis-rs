use nom::combinator::peek;
use nom::Err;
use nom::IResult;
use nom::number::complete::{be_f32, be_f64, be_u16, be_u32, be_u8};
use nom::bytes::complete::take;
use nom::error::ErrorKind::Eof;
use nom::multi::{count, many1};
use nom::sequence::tuple;
use crate::common::entity_state::parser::entity_state_body;
use crate::common::model::{Pdu, PduBody, PduHeader, ProtocolVersion};
use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;
use crate::common::errors::DisError;
use crate::common::other::parser::other_body;
use crate::{Country, EntityId, EntityKind, EntityType, EventId, Location, Orientation, SimulationAddress, VectorF32};
use crate::common::fire::parser::fire_body;
use crate::v7::parser::parse_pdu_status;
use crate::enumerations::{PduType, ProtocolFamily, PlatformDomain};

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

#[allow(dead_code)]
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
#[allow(dead_code)]
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

fn pdu(input: &[u8]) -> IResult<&[u8], Pdu> {
    // parse the header
    let (input, header) = pdu_header(input)?;
    // parse the body based on the type
    // and produce the final pdu combined with the header
    let (input, body) = pdu_body(&header)(input)?;

    Ok((input, Pdu {
        header,
        body
    }))
}

fn pdu_header(input: &[u8]) -> IResult<&[u8], PduHeader> {
    let protocol_version = protocol_version;
    let exercise_id = be_u8;
    let pdu_type = pdu_type;
    let protocol_family = protocol_family;
    let time_stamp= be_u32;
    let pdu_length = be_u16;

    let (input, (protocol_version, exercise_id, pdu_type, protocol_family, time_stamp, pdu_length)) =
        tuple((protocol_version, exercise_id, pdu_type, protocol_family, time_stamp, pdu_length))(input)?;
    let type_u8 : u8 = pdu_type.into();
    let (input, pdu_status, padding) = match protocol_version {
        ProtocolVersion::Ieee1278_1_2012 => {
            let (input, status) = be_u8(input)?;
            let (input, padding) = be_u8(input)?;
            (input, Some(
                parse_pdu_status(type_u8, status)), padding as u16) }
        _ => {
            let (input, padding) = be_u16(input)?;
            (input, None, padding as u16)
        }
    };

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

#[allow(dead_code)]
fn pdu_header_skip_body(input: &[u8]) -> IResult<&[u8], PduHeader> {
    let (input, header) = pdu_header(input)?;
    let (input, _) = skip_body(header.pdu_length)(input)?;
    Ok((input, header))
}

fn pdu_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move | input: &[u8] | {
        // parse the body of the PDU based on the type
        // TODO only process supported PduTypes; process others as 'Other'
        let (input, body) = match header.pdu_type {
            PduType::Other => { other_body(header)(input)? }
            PduType::EntityState => { entity_state_body()(input)? }
            PduType::Fire => { fire_body()(input)? }
            PduType::Unspecified(_type_number) => { other_body(header)(input)? }
            _ => { other_body(header)(input)? }
            // PduType::Detonation => {}
            // PduType::Collision => {}
            // PduType::ServiceRequest => {}
            // PduType::ResupplyOffer => {}
            // PduType::ResupplyReceived => {}
            // PduType::ResupplyCancel => {}
            // PduType::RepairComplete => {}
            // PduType::RepairResponse => {}
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
            // PduType::UnderwaterAcoustic => {}
            // PduType::SupplementalEmissionEntityState => {}
            // PduType::IntercomSignal => {}
            // PduType::IntercomControl => {}
            // PduType::AggregateState => {}
            // PduType::IsGroupOf => {}
            // PduType::TransferOwnership => {}
            // PduType::IsPartOf => {}
            // PduType::MinefieldState => {}
            // PduType::MinefieldQuery => {}
            // PduType::MinefieldData => {}
            // PduType::MinefieldResponseNACK => {}
            // PduType::EnvironmentalProcess => {}
            // PduType::GriddedData => {}
            // PduType::PointObjectState => {}
            // PduType::LinearObjectState => {}
            // PduType::ArealObjectState => {}
            // PduType::TSPI => {}
            // PduType::Appearance => {}
            // PduType::ArticulatedParts => {}
            // PduType::LEFire => {}
            // PduType::LEDetonation => {}
            // PduType::CreateEntityR => {}
            // PduType::RemoveEntityR => {}
            // PduType::StartResumeR => {}
            // PduType::StopFreezeR => {}
            // PduType::AcknowledgeR => {}
            // PduType::ActionRequestR => {}
            // PduType::ActionResponseR => {}
            // PduType::DataQueryR => {}
            // PduType::SetDataR => {}
            // PduType::DataR => {}
            // PduType::EventReportR => {}
            // PduType::CommentR => {}
            // PduType::RecordR => {}
            // PduType::SetRecordR => {}
            // PduType::RecordQueryR => {}
            // PduType::CollisionElastic => {}
            // PduType::EntityStateUpdate => {}
            // PduType::DirectedEnergyFire => {}
            // PduType::EntityDamageStatus => {}
            // PduType::InformationOperationsAction => {}
            // PduType::InformationOperationsReport => {}
            // PduType::Attribute => {}
        };
        // TODO handle result of pdu variable
        Ok((input, body))
    }
}

#[allow(dead_code)]
pub fn parse_peek_protocol_version(input: &[u8]) -> Result<ProtocolVersion,DisError> {
    let parse_result = peek_protocol_version(input);
    match parse_result {
        Ok((_, protocol_version)) => Ok(ProtocolVersion::from(protocol_version)),
        Err(_err) => Err(DisError::ParseError),
    }
}

/// Function tries to peek the protocol version field of the DIS header
/// and return the raw value when successful.
#[allow(dead_code)]
fn peek_protocol_version(input: &[u8]) -> IResult<&[u8], u8> {
    let (input, protocol_version) = peek(be_u8)(input)?;
    Ok((input, protocol_version))
}

pub fn protocol_version(input: &[u8]) -> IResult<&[u8], ProtocolVersion> {
    let (input, protocol_version) = be_u8(input)?;
    let protocol_version = ProtocolVersion::from(protocol_version);
    Ok((input, protocol_version))
}

pub fn pdu_type(input: &[u8]) -> IResult<&[u8], PduType> {
    let (input, pdu_type) = be_u8(input)?;
    let pdu_type = PduType::from(pdu_type);
    Ok((input, pdu_type))
}

pub fn protocol_family(input: &[u8]) -> IResult<&[u8], ProtocolFamily> {
    let (input, protocol_family) = be_u8(input)?;
    let protocol_family = ProtocolFamily::from(protocol_family);
    Ok((input, protocol_family))
}

#[allow(dead_code)]
pub fn skip_body(total_bytes: u16) -> impl Fn(&[u8]) -> IResult<&[u8], &[u8]> {
    let bytes_to_skip = total_bytes as usize - PDU_HEADER_LEN_BYTES;
    move |input| {
        take(bytes_to_skip)(input)
    }
}

pub fn entity_id(input: &[u8]) -> IResult<&[u8], EntityId> {
    let (input, site_id) = be_u16(input)?;
    let (input, application_id) = be_u16(input)?;
    let (input, entity_id) = be_u16(input)?;
    Ok((input, EntityId {
        simulation_address: SimulationAddress {
            site_id,
            application_id,
        },
        entity_id,
    }))
}

pub fn entity_type(input: &[u8]) -> IResult<&[u8], EntityType> {
    let (input, kind) = kind(input)?;
    let (input, domain) = domain(input)?;
    let (input, country) = country(input)?;
    let (input, category) = be_u8(input)?;
    let (input, subcategory) = be_u8(input)?;
    let (input, specific) = be_u8(input)?;
    let (input, extra) = be_u8(input)?;
    Ok((input, EntityType {
        kind,
        domain,
        country,
        category,
        subcategory,
        specific,
        extra,
    }))
}

fn kind(input: &[u8]) -> IResult<&[u8], EntityKind> {
    let (input, kind) = be_u8(input)?;
    let kind = EntityKind::from(kind);
    Ok((input, kind))
}

fn domain(input: &[u8]) -> IResult<&[u8], PlatformDomain> {
    let (input, domain) = be_u8(input)?;
    let domain = PlatformDomain::from(domain);
    Ok((input, domain))
}

fn country(input: &[u8]) -> IResult<&[u8], Country> {
    let (input, country) = be_u16(input)?;
    let country = Country::from(country);
    Ok((input, country))
}

pub fn vec3_f32(input: &[u8]) -> IResult<&[u8], VectorF32> {
    let (input, elements) = count(be_f32, 3)(input)?;
    Ok((input, VectorF32 {
        first_vector_component: *elements.get(0).expect("Value supposed to be parsed successfully"),
        second_vector_component: *elements.get(1).expect("Value supposed to be parsed successfully"),
        third_vector_component: *elements.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

pub fn location(input: &[u8]) -> IResult<&[u8], Location> {
    let (input, locations) = count(be_f64, 3)(input)?;
    Ok((input, Location {
        x_coordinate: *locations.get(0).expect("Value supposed to be parsed successfully"),
        y_coordinate: *locations.get(1).expect("Value supposed to be parsed successfully"),
        z_coordinate: *locations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

pub fn orientation(input: &[u8]) -> IResult<&[u8], Orientation> {
    let (input, orientations) = count(be_f32, 3)(input)?;
    Ok((input, Orientation {
        psi: *orientations.get(0).expect("Value supposed to be parsed successfully"),
        theta: *orientations.get(1).expect("Value supposed to be parsed successfully"),
        phi: *orientations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

pub fn event_id(input: &[u8]) -> IResult<&[u8], EventId> {
    let (input, site_id) = be_u16(input)?;
    let (input, application_id) = be_u16(input)?;
    let (input, event_id) = be_u16(input)?;
    Ok((input, EventId {
        simulation_address: SimulationAddress {
            site_id,
            application_id,
        },
        event_id,
    }))
}

#[cfg(test)]
mod tests {
    use crate::common::model::{EntityType, PduBody, ProtocolVersion};
    use crate::common::errors::DisError;
    use crate::common::entity_state::model::{Afterburner, AirPlatformsRecord, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityLights, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityTrailingEffect, FrozenStatus, GeneralAppearance, ParameterTypeVariant, PowerPlantStatus, SpecificAppearance, State};
    use crate::common::parser::{parse_multiple_header, parse_pdu};
    use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::{EntityKind, ForceId, Country, PduType, ProtocolFamily, ArticulatedPartsTypeMetric, ArticulatedPartsTypeClass, DeadReckoningAlgorithm, VariableParameterRecordType, PlatformDomain};

    #[test]
    fn parse_header() {
        let bytes : [u8;12] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00];

        let header = crate::common::parser::parse_header(&bytes);
        assert!(header.is_ok());
        let header = header.unwrap();
        assert_eq!(header.protocol_version, ProtocolVersion::Ieee1278_1a_1998);
        assert_eq!(header.exercise_id, 1);
        assert_eq!(header.pdu_type, PduType::EntityState);
        assert_eq!(header.protocol_family, ProtocolFamily::EntityInformationInteraction);
        assert_eq!(header.time_stamp, 1323973472);
        assert_eq!(header.pdu_length, PDU_HEADER_LEN_BYTES as u16); // only the header, 0-bytes pdu body
    }

    #[test]
    fn parse_header_too_short() {
        let bytes : [u8;10] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x60];

        let header = crate::common::parser::parse_header(&bytes);
        assert!(header.is_err());
        let error = header.expect_err("Should be Err");
        assert_eq!(error, DisError::InsufficientHeaderLength(10));
    }

    #[test]
    fn parse_header_unspecified_version() {
        let bytes : [u8;12] = [0x1F, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00];

        let header = crate::common::parser::parse_header(&bytes);
        assert!(header.is_ok());
        let header = header.unwrap();
        assert_eq!(header.protocol_version, ProtocolVersion::Other);
        assert_eq!(header.exercise_id, 1);
        assert_eq!(header.pdu_type, PduType::EntityState);
        assert_eq!(header.protocol_family, ProtocolFamily::EntityInformationInteraction);
        assert_eq!(header.time_stamp, 1323973472);
        assert_eq!(header.pdu_length, PDU_HEADER_LEN_BYTES as u16); // only the header, 0-bytes pdu body
    }

    #[test]
    fn parse_header_body_too_short() {
        // PDU with header that states that the total length is 208 bytes, but only contains a 2 bytes body;
        let bytes: [u8; 14] =
            [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0xd0, 0x00, 0x00, 0x01, 0xf4];

        let header = crate::common::parser::parse_header(&bytes);
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
        assert_eq!(pdu.header.pdu_type, PduType::EntityState);
        assert_eq!(pdu.header.pdu_length, 208u16);
        if let PduBody::EntityState(pdu) = pdu.body {
            assert_eq!(pdu.entity_id.simulation_address.site_id, 500u16);
            assert_eq!(pdu.entity_id.simulation_address.application_id, 900u16);
            assert_eq!(pdu.entity_id.entity_id, 14u16);
            assert_eq!(pdu.force_id, ForceId::Friendly);
            assert!(pdu.articulation_parameter.is_some());
            let articulation_parameters = pdu.articulation_parameter.unwrap();
            assert_eq!(articulation_parameters.len(), 4usize);
            assert_eq!(pdu.entity_type, EntityType {
                kind: EntityKind::Platform,
                domain: PlatformDomain::Air,
                country: Country::Netherlands_NLD_,
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
            assert_eq!(pdu.dead_reckoning_parameters.algorithm, DeadReckoningAlgorithm::DRM_RVW_HighSpeedorManeuveringEntitywithExtrapolationofOrientation);
            assert_eq!(pdu.entity_marking.marking_string, String::from("EYE 10"));
            assert_eq!(pdu.entity_capabilities, EntityCapabilities {
                ammunition_supply: false,
                fuel_supply: false,
                recovery: false,
                repair: false,
            });
            assert_eq!(articulation_parameters.len(), 4);
            let parameter_1 = articulation_parameters.get(0).unwrap();
            assert_eq!(parameter_1.parameter_type_designator, VariableParameterRecordType::ArticulatedPart);
            if let ParameterTypeVariant::Articulated(articulated_part) = &parameter_1.parameter_type_variant {
                assert_eq!(articulated_part.type_metric, ArticulatedPartsTypeMetric::Position);
                assert_eq!(articulated_part.type_class, ArticulatedPartsTypeClass::LandingGear); // landing gear
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
