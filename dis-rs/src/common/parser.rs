use nom::combinator::peek;
use nom::Err;
use nom::IResult;
use nom::number::complete::{be_f32, be_f64, be_i32, be_u16, be_u32, be_u64, be_u8};
use nom::bytes::complete::take;
use nom::error::ErrorKind::Eof;
use nom::multi::{count, many1};
use nom::sequence::tuple;
use crate::common::entity_state::parser::entity_state_body;
use crate::constants::{EIGHT_OCTETS, FIVE_LEAST_SIGNIFICANT_BITS, ONE_BYTE_IN_BITS, PDU_HEADER_LEN_BYTES};
use crate::common::errors::DisError;
use crate::common::other::parser::other_body;
use crate::common::model::{ArticulatedPart, AttachedPart, BeamData, ClockTime, DatumSpecification, DescriptorRecord, EntityAssociationParameter, EntityId, EntityType, EntityTypeParameter, EventId, FixedDatum, length_padded_to_num, Location, MunitionDescriptor, Orientation, Pdu, PduBody, PduHeader, SeparationParameter, SimulationAddress, VariableDatum, VariableParameter, VectorF32};
use crate::common::acknowledge::parser::acknowledge_body;
use crate::common::action_request::parser::action_request_body;
use crate::common::action_response::parser::action_response_body;
use crate::common::attribute::parser::attribute_body;
use crate::common::collision::parser::collision_body;
use crate::common::collision_elastic::parser::collision_elastic_body;
use crate::common::comment::parser::comment_body;
use crate::common::create_entity::parser::create_entity_body;
use crate::common::data::parser::data_body;
use crate::common::data_query::parser::data_query_body;
use crate::common::designator::parser::designator_body;
use crate::common::detonation::parser::detonation_body;
use crate::common::electromagnetic_emission::parser::emission_body;
use crate::common::entity_state_update::parser::entity_state_update_body;
use crate::common::event_report::parser::event_report_body;
use crate::common::fire::parser::fire_body;
use crate::common::receiver::parser::receiver_body;
use crate::common::remove_entity::parser::remove_entity_body;
use crate::common::set_data::parser::set_data_body;
use crate::common::signal::parser::signal_body;
use crate::common::start_resume::parser::start_resume_body;
use crate::common::stop_freeze::parser::stop_freeze_body;
use crate::common::transmitter::parser::transmitter_body;
use crate::v7::parser::parse_pdu_status;
use crate::enumerations::{Country, DetonationTypeIndicator, EntityKind, ExplosiveMaterialCategories, FireTypeIndicator, MunitionDescriptorFuse, MunitionDescriptorWarhead, PduType, PlatformDomain, ProtocolFamily, ProtocolVersion, VariableRecordType};
use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedPartDetachedIndicator, AttachedParts, ChangeIndicator, EntityAssociationAssociationStatus, EntityAssociationGroupMemberType, EntityAssociationPhysicalAssociationType, EntityAssociationPhysicalConnectionType, SeparationPreEntityIndicator, SeparationReasonForSeparation, StationName, VariableParameterRecordType};
use crate::common::iff::parser::iff_body;
use crate::model::SupplyQuantity;
use crate::repair_complete::parser::repair_complete_body;
use crate::repair_response::parser::repair_response_body;
use crate::resupply_cancel::parser::resupply_cancel_body;
use crate::resupply_offer::parser::resupply_offer_body;
use crate::resupply_received::parser::resupply_received_body;
use crate::service_request::parser::service_request_body;

pub fn parse_multiple_pdu(input: &[u8]) -> Result<Vec<Pdu>, DisError> {
    match many1(pdu)(input) {
        Ok((_, pdus)) => { Ok(pdus) }
        Err(err) => { Err(DisError::ParseError(err.to_string())) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

#[allow(dead_code)]
pub fn parse_pdu(input: &[u8]) -> Result<Pdu, DisError> {
    match pdu(input) {
        Ok((_, pdu)) => { Ok(pdu) }
        Err(err) => { Err(DisError::ParseError(err.to_string())) } // TODO not very descriptive / error means we can not match any PDUs
    }
}

#[allow(dead_code)]
pub fn parse_multiple_header(input: &[u8]) -> Result<Vec<PduHeader>, DisError> {
    match many1(pdu_header_skip_body)(input) {
        Ok((_, headers)) => { Ok(headers) }
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
pub fn parse_header(input: &[u8]) -> Result<PduHeader, DisError> {
    match pdu_header(input) {
        Ok((input, header)) => {
            let skipped = skip_body(header.pdu_length)(input); // Discard the body
            if let Err(Err::Error(error)) = skipped {
                return if error.code == Eof {
                    Err(DisError::InsufficientPduLength(header.pdu_length - PDU_HEADER_LEN_BYTES, input.len() as u16))
                } else { Err(DisError::ParseError("ParseError while parsing a pdu header and skipping body.".to_string())) }
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
    let (input, pdu_status, padding) = match u8::from(protocol_version) {
        legacy_version if (1..=5).contains(&legacy_version) => {
            let (input, padding) = be_u16(input)?;
            (input, None, padding as u16)
        }
        6 => {
            let (input, padding) = be_u16(input)?;
            (input, None, padding as u16)
        }
        7 => {
            let (input, (status, padding)) = parse_pdu_status(pdu_type)(input)?;
            (input, Some(status), padding)
        }
        _future_version => {
            let (input, (status, padding)) = parse_pdu_status(pdu_type)(input)?;
            (input, Some(status), padding)
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
        // NOTE only processes supported PduTypes; process others as 'Other'
        let (input, body) = match header.pdu_type {
            PduType::Other => { other_body(header)(input)? }
            PduType::EntityState => { entity_state_body(header)(input)? }
            PduType::Fire => { fire_body(header)(input)? }
            PduType::Detonation => { detonation_body(header)(input)? }
            PduType::Collision => { collision_body(input)? }
            PduType::ServiceRequest => { service_request_body(input)? }
            PduType::ResupplyOffer => { resupply_offer_body(input)? }
            PduType::ResupplyReceived => { resupply_received_body(input)? }
            PduType::ResupplyCancel => { resupply_cancel_body(input)? }
            PduType::RepairComplete => { repair_complete_body(input)? }
            PduType::RepairResponse => { repair_response_body(input)? }
            PduType::CreateEntity => { create_entity_body(input)? }
            PduType::RemoveEntity => { remove_entity_body(input)? }
            PduType::StartResume => { start_resume_body(input)? }
            PduType::StopFreeze => { stop_freeze_body(input)? }
            PduType::Acknowledge => { acknowledge_body(input)? }
            PduType::ActionRequest => { action_request_body(input)? }
            PduType::ActionResponse => { action_response_body(input)? }
            PduType::DataQuery => { data_query_body(input)? }
            PduType::SetData => { set_data_body(input)? }
            PduType::Data => { data_body(input)? }
            PduType::EventReport => { event_report_body(input)? }
            PduType::Comment => { comment_body(input)? }
            PduType::ElectromagneticEmission => { emission_body(header)(input)? }
            PduType::Designator => { designator_body(input)? }
            PduType::Transmitter => { transmitter_body(header)(input)? }
            PduType::Signal => { signal_body(input)? }
            PduType::Receiver => { receiver_body(input)? }
            PduType::IFF => { iff_body(input)? }
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
            PduType::CollisionElastic => { collision_elastic_body(input)? }
            PduType::EntityStateUpdate => { entity_state_update_body(input)? }
            // PduType::DirectedEnergyFire => {}
            // PduType::EntityDamageStatus => {}
            // PduType::InformationOperationsAction => {}
            // PduType::InformationOperationsReport => {}
            PduType::Attribute => { attribute_body(input)? }
            PduType::Unspecified(_type_number) => { other_body(header)(input)? } // TODO Log unspecified type number?
            _ => { other_body(header)(input)? }
        };
        Ok((input, body))
    }
}

#[allow(dead_code)]
pub fn parse_peek_protocol_version(input: &[u8]) -> Result<ProtocolVersion,DisError> {
    let parse_result = peek_protocol_version(input);
    match parse_result {
        Ok((_, protocol_version)) => Ok(ProtocolVersion::from(protocol_version)),
        Err(err) => Err(DisError::ParseError(err.to_string())),
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
    let bytes_to_skip = total_bytes - PDU_HEADER_LEN_BYTES;
    move |input| {
        take(bytes_to_skip)(input)
    }
}

pub fn simulation_address(input: &[u8]) -> IResult<&[u8], SimulationAddress> {
    let (input, site_id) = be_u16(input)?;
    let (input, application_id) = be_u16(input)?;
    Ok((input, SimulationAddress::new(site_id, application_id)))
}

pub fn entity_id(input: &[u8]) -> IResult<&[u8], EntityId> {
    let (input, simulation_address) = simulation_address(input)?;
    let (input, entity_id) = be_u16(input)?;
    Ok((input, EntityId {
        simulation_address,
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
    #[allow(clippy::get_first)]
    Ok((input, VectorF32 {
        first_vector_component: *elements.get(0).expect("Value supposed to be parsed successfully"),
        second_vector_component: *elements.get(1).expect("Value supposed to be parsed successfully"),
        third_vector_component: *elements.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

pub fn location(input: &[u8]) -> IResult<&[u8], Location> {
    let (input, locations) = count(be_f64, 3)(input)?;
    #[allow(clippy::get_first)]
    Ok((input, Location {
        x_coordinate: *locations.get(0).expect("Value supposed to be parsed successfully"),
        y_coordinate: *locations.get(1).expect("Value supposed to be parsed successfully"),
        z_coordinate: *locations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

pub fn orientation(input: &[u8]) -> IResult<&[u8], Orientation> {
    let (input, orientations) = count(be_f32, 3)(input)?;
    #[allow(clippy::get_first)]
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

pub fn descriptor_record_fti(fire_type_indicator: FireTypeIndicator) -> impl Fn(&[u8]) -> IResult<&[u8], DescriptorRecord> {
    move |input: &[u8]| {
        let (input, entity_type) = entity_type(input)?;

        match fire_type_indicator {
            FireTypeIndicator::Munition => {
                let (input, munition) = munition_descriptor(input)?;

                Ok((input, DescriptorRecord::Munition {
                    entity_type,
                    munition,
                }))
            }
            FireTypeIndicator::Expendable => {
                let (input, _pad_out) = be_u64(input)?;

                Ok((input, DescriptorRecord::Expendable {
                    entity_type
                }))
            }
            FireTypeIndicator::Unspecified(_) => {
                // TODO should be an error; parse as Expendable, which has no data, for now
                let (input, _pad_out) = be_u64(input)?;

                Ok((input, DescriptorRecord::Expendable {
                    entity_type
                }))
            }
        }
    }
}

pub fn descriptor_record_dti(detonation_type_indicator: DetonationTypeIndicator) -> impl Fn(&[u8]) -> IResult<&[u8], DescriptorRecord> {
    move |input: &[u8]| {
        let (input, entity_type) = entity_type(input)?;
        match detonation_type_indicator {
            DetonationTypeIndicator::Munition => {
                let (input, munition) = munition_descriptor(input)?;

                Ok((input, DescriptorRecord::Munition {
                    entity_type,
                    munition,
                }))
            }
            DetonationTypeIndicator::Expendable => {
                let (input, _pad_out) = be_u64(input)?;
                Ok((input, DescriptorRecord::Expendable {
                    entity_type
                }))
            }
            DetonationTypeIndicator::NonmunitionExplosion => {
                let (input, explosive_material) = be_u16(input)?;
                let explosive_material = ExplosiveMaterialCategories::from(explosive_material);
                let (input, explosive_force) = be_f32(input)?;

                Ok((input, DescriptorRecord::Explosion {
                    entity_type,
                    explosive_material,
                    explosive_force
                }))
            }
            DetonationTypeIndicator::Unspecified(_) => {
                // TODO should be an error; parse as Expendable, which has no data, for now
                let (input, _pad_out) = be_u64(input)?;
                Ok((input, DescriptorRecord::Expendable {
                    entity_type
                }))
            }
        }
    }
}

pub fn munition_descriptor(input: &[u8]) -> IResult<&[u8], MunitionDescriptor> {
    let (input, warhead) = warhead(input)?;
    let (input, fuse) = fuse(input)?;
    let (input, quantity) = be_u16(input)?;
    let (input, rate) = be_u16(input)?;

    Ok((input, MunitionDescriptor {
        warhead,
        fuse,
        quantity,
        rate,
    }))
}

fn warhead(input: &[u8]) -> IResult<&[u8], MunitionDescriptorWarhead> {
    let (input, warhead) = be_u16(input)?;
    let warhead = MunitionDescriptorWarhead::from(warhead);
    Ok((input, warhead))
}

fn fuse(input: &[u8]) -> IResult<&[u8], MunitionDescriptorFuse> {
    let (input, fuse) = be_u16(input)?;
    let fuse = MunitionDescriptorFuse::from(fuse);
    Ok((input, fuse))
}

pub fn clock_time(input: &[u8]) -> IResult<&[u8], ClockTime> {
    let (input, hour) = be_i32(input)?;
    let (input, time_past_hour) = be_u32(input)?;
    let time = ClockTime::new(hour, time_past_hour);
    Ok((input, time))
}

pub fn datum_specification(input: &[u8]) -> IResult<&[u8], DatumSpecification> {
    let (input, num_fixed_datums) = be_u32(input)?;
    let (input, num_variable_datums) = be_u32(input)?;

    let (input, fixed_datums) = count(fixed_datum, num_fixed_datums as usize)(input)?;
    let (input, variable_datums) = count(variable_datum, num_variable_datums as usize)(input)?;

    let datums = DatumSpecification::new(fixed_datums, variable_datums);

    Ok((input, datums))
}

pub fn fixed_datum(input: &[u8]) -> IResult<&[u8], FixedDatum> {
    let (input, datum_id) = be_u32(input)?;
    let (input, datum_value) = be_u32(input)?;

    let datum_id = VariableRecordType::from(datum_id);
    let datum = FixedDatum::new(datum_id, datum_value);

    Ok((input, datum))
}

pub fn variable_datum(input: &[u8]) -> IResult<&[u8], VariableDatum> {
    let (input, datum_id) = be_u32(input)?;
    let datum_id = VariableRecordType::from(datum_id);
    let (input, datum_length_bits) = be_u32(input)?;

    // NOTE: The standard defines the data length and padding in bits.
    // However, we assume that one only puts in values that consists of whole bytes.
    // (As why would one put 11 bits in a datum, which then ends up in a Vec<u8>)
    let datum_length_bytes = datum_length_bits as usize / ONE_BYTE_IN_BITS;
    let padded_record = length_padded_to_num(
        datum_length_bytes,
        EIGHT_OCTETS);

    let (input, datum_value) = take(padded_record.data_length)(input)?;
    let (input, _datum_padding) = take(padded_record.padding_length)(input)?;

    let variable_datum = VariableDatum::new(datum_id, datum_value.to_vec());

    Ok((input, variable_datum))
}

#[cfg(test)]
mod tests {
    use crate::common::errors::DisError;
    use crate::common::parser::parse_multiple_header;
    use crate::constants::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::{PduType, ProtocolFamily, ProtocolVersion};

    #[test]
    fn parse_header() {
        let bytes : [u8;12] = [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0x0c, 0x00, 0x00];

        let header = crate::common::parser::parse_header(&bytes);
        assert!(header.is_ok());
        let header = header.unwrap();
        assert_eq!(header.protocol_version, ProtocolVersion::IEEE1278_1A1998);
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
        assert_eq!(header.protocol_version, ProtocolVersion::Unspecified(31));
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

pub fn variable_parameter(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, parameter_type_designator) = be_u8(input)?;
    let parameter_type = VariableParameterRecordType::from(parameter_type_designator);
    let (input, variable_parameter) = match parameter_type {
        VariableParameterRecordType::ArticulatedPart => { articulated_part(input)? }
        VariableParameterRecordType::AttachedPart => { attached_part(input)? }
        VariableParameterRecordType::Separation => { separation(input)? }
        VariableParameterRecordType::EntityType => { entity_type_variable_parameter(input)? }
        VariableParameterRecordType::EntityAssociation => { entity_association(input)? }
        VariableParameterRecordType::Unspecified(_) => {
            let (input, bytes) = take(15usize)(input)?;
            (input, VariableParameter::Unspecified(parameter_type_designator, <[u8; 15]>::try_from(bytes).unwrap()))
        } // TODO sensible error
    };

    Ok((input, variable_parameter))
}

/// I.2.2 Articulated parts
fn articulated_part(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, change_indicator) = be_u8(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, attachment_id) = be_u16(input)?;
    let (input, parameter_type) = be_u32(input)?;                    // Parameter Type = Type Class + Type Metric
    let type_metric : u32 = parameter_type & FIVE_LEAST_SIGNIFICANT_BITS;   // 5 least significant bits are the Type Metric
    let type_class : u32 = parameter_type - type_metric;                    // Rest of the bits (Param Type minus Type Metric) are the Type Class
    let (input, value) = be_f32(input)?;
    let (input, _pad_out) = be_u32(input)?;

    Ok((input, VariableParameter::Articulated(ArticulatedPart {
        change_indicator,
        attachment_id,
        type_metric: ArticulatedPartsTypeMetric::from(type_metric),
        type_class: ArticulatedPartsTypeClass::from(type_class),
        parameter_value: value,
    })))
}

fn attached_part(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, detached_indicator) = be_u8(input)?;
    let detached_indicator = AttachedPartDetachedIndicator::from(detached_indicator);
    let (input, attachment_id) = be_u16(input)?;
    let (input, attached_part) = be_u32(input)?;
    let (input, entity_type) = entity_type(input)?;

    Ok((input, VariableParameter::Attached(AttachedPart {
        detached_indicator,
        attachment_id,
        parameter_type: AttachedParts::from(attached_part),
        attached_part_type: entity_type
    })))
}

fn entity_association(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, change_indicator) = be_u8(input)?;
    let (input, association_status) = be_u8(input)?;
    let (input, association_type) = be_u8(input)?;
    let (input, entity_id) = entity_id(input)?;
    let (input, own_station_location) = be_u16(input)?;
    let (input, physical_connection_type) = be_u8(input)?;
    let (input, group_member_type) = be_u8(input)?;
    let (input, group_number) = be_u16(input)?;

    Ok((input, VariableParameter::EntityAssociation(EntityAssociationParameter {
        change_indicator: ChangeIndicator::from(change_indicator),
        association_status: EntityAssociationAssociationStatus::from(association_status),
        association_type: EntityAssociationPhysicalAssociationType::from(association_type),
        entity_id,
        own_station_location: StationName::from(own_station_location),
        physical_connection_type: EntityAssociationPhysicalConnectionType::from(physical_connection_type),
        group_member_type: EntityAssociationGroupMemberType::from(group_member_type),
        group_number,
    })))
}

fn entity_type_variable_parameter(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, change_indicator) = be_u8(input)?;
    let (input, entity_type) = entity_type(input)?;
    let (input, _pad_out_16) = be_u16(input)?;
    let (input, _pad_out_32) = be_u32(input)?;

    Ok((input, VariableParameter::EntityType(EntityTypeParameter {
        change_indicator: ChangeIndicator::from(change_indicator),
        entity_type,
    })))
}

fn separation(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, reason) = be_u8(input)?;
    let (input, pre_entity_indicator) = be_u8(input)?;
    let (input, parent_entity_id) = entity_id(input)?;
    let (input, _pad_16) = be_u16(input)?;
    let (input, station_name) = be_u16(input)?;
    let (input, station_number) = be_u16(input)?;

    Ok((input, VariableParameter::Separation(SeparationParameter {
        reason: SeparationReasonForSeparation::from(reason),
        pre_entity_indicator: SeparationPreEntityIndicator::from(pre_entity_indicator),
        parent_entity_id,
        station_name: StationName::from(station_name),
        station_number,
    })))
}

pub fn beam_data(input: &[u8]) -> IResult<&[u8], BeamData> {
    let (input, azimuth_center) = be_f32(input)?;
    let (input, azimuth_sweep) = be_f32(input)?;
    let (input, elevation_center) = be_f32(input)?;
    let (input, elevation_sweep) = be_f32(input)?;
    let (input, sweep_sync) = be_f32(input)?;

    let data = BeamData::new()
        .with_azimuth_center(azimuth_center)
        .with_azimuth_sweep(azimuth_sweep)
        .with_elevation_center(elevation_center)
        .with_elevation_sweep(elevation_sweep)
        .with_sweep_sync(sweep_sync);

    Ok((input, data))
}

pub fn supply_quantity(input: &[u8]) -> IResult<&[u8], SupplyQuantity> {
    let (input, supply_type) = entity_type(input)?;
    let (input, quantity) = be_f32(input)?;

    Ok((input, SupplyQuantity::default()
        .with_supply_type(supply_type)
        .with_quantity(quantity)))
}
