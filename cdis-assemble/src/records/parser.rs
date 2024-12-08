use crate::constants::{
    EIGHT_BITS, ELEVEN_BITS, FIVE_BITS, FOURTEEN_BITS, FOUR_BITS, NINE_BITS, ONE_BIT, SIXTEEN_BITS,
    SIX_BITS, TEN_BITS, THIRTEEN_BITS, THIRTY_ONE_BITS, THIRTY_TWO_BITS, THREE_BITS, TWELVE_BITS,
    TWENTY_SIX_BITS, TWO_BITS,
};
use crate::parsing::take_signed;
use crate::parsing::BitInput;
use crate::records::model::{
    AngularVelocity, BeamAntennaPattern, BeamData, CdisArticulatedPartVP, CdisAttachedPartVP,
    CdisEntityAssociationVP, CdisEntityMarking, CdisEntitySeparationVP, CdisEntityTypeVP,
    CdisHeader, CdisMarkingCharEncoding, CdisProtocolVersion, CdisVariableParameter,
    EncodingScheme, EntityCoordinateVector, EntityId, EntityType, LayerHeader, LinearAcceleration,
    LinearVelocity, Orientation, ParameterValueFloat, WorldCoordinates,
};
use crate::types::model::{CdisFloat, SVINT24, UVINT16, UVINT8};
use crate::types::parser::{svint12, svint13, svint14, svint16, svint24, uvint16, uvint8};
use bitvec::macros::internal::funty::Floating;
use dis_rs::enumerations::{
    ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedPartDetachedIndicator,
    AttachedParts, ChangeIndicator, EntityAssociationAssociationStatus,
    EntityAssociationGroupMemberType, EntityAssociationPhysicalAssociationType,
    EntityAssociationPhysicalConnectionType, PduType, SeparationPreEntityIndicator,
    SeparationReasonForSeparation, SignalEncodingClass, SignalEncodingType, StationName,
    TransmitterAntennaPatternReferenceSystem, VariableParameterRecordType, VariableRecordType,
};
use dis_rs::model::{FixedDatum, TimeStamp, VariableDatum};
use dis_rs::parse_pdu_status_fields;
use nom::bits::complete::take;
use nom::multi::count;
use nom::IResult;
use num::Integer;

const FIVE_LEAST_SIGNIFICANT_BITS: u32 = 0x1f;

pub(crate) fn cdis_header(input: BitInput) -> IResult<BitInput, CdisHeader> {
    let (input, protocol_version): (BitInput, u8) = take(TWO_BITS)(input)?;
    let (input, exercise_id) = uvint8(input)?;
    let (input, pdu_type): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, timestamp): (BitInput, u32) = take(TWENTY_SIX_BITS)(input)?;
    let (input, length): (BitInput, u16) = take(FOURTEEN_BITS)(input)?;
    let (input, pdu_status): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let pdu_status = parse_pdu_status_fields(pdu_type, pdu_status);

    Ok((
        input,
        CdisHeader {
            protocol_version: CdisProtocolVersion::from(protocol_version),
            exercise_id,
            pdu_type: PduType::from(pdu_type),
            timestamp: TimeStamp::from(timestamp),
            length,
            pdu_status,
        },
    ))
}

pub(crate) fn angular_velocity(input: BitInput) -> IResult<BitInput, AngularVelocity> {
    let (input, x_component) = svint12(input)?;
    let (input, y_component) = svint12(input)?;
    let (input, z_component) = svint12(input)?;

    Ok((
        input,
        AngularVelocity::new(x_component, y_component, z_component),
    ))
}

pub(crate) fn entity_coordinate_vector(
    input: BitInput,
) -> IResult<BitInput, EntityCoordinateVector> {
    let (input, x_component) = svint16(input)?;
    let (input, y_component) = svint16(input)?;
    let (input, z_component) = svint16(input)?;

    Ok((
        input,
        EntityCoordinateVector::new(x_component, y_component, z_component),
    ))
}

pub(crate) fn entity_identification(input: BitInput) -> IResult<BitInput, EntityId> {
    let (input, site) = uvint16(input)?;
    let (input, application) = uvint16(input)?;
    let (input, entity) = uvint16(input)?;

    Ok((input, EntityId::new(site, application, entity)))
}

pub(crate) fn entity_identification_uncompressed(input: BitInput) -> IResult<BitInput, EntityId> {
    let (input, site): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, application): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, entity): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((
        input,
        EntityId::new(
            UVINT16::from(site),
            UVINT16::from(application),
            UVINT16::from(entity),
        ),
    ))
}

pub(crate) fn entity_type(input: BitInput) -> IResult<BitInput, EntityType> {
    let (input, kind): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, domain): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, country): (BitInput, u16) = take(NINE_BITS)(input)?;
    let (input, category) = uvint8(input)?;
    let (input, subcategory) = uvint8(input)?;
    let (input, specific) = uvint8(input)?;
    let (input, extra) = uvint8(input)?;

    Ok((
        input,
        EntityType::new(
            kind,
            domain,
            country,
            category,
            subcategory,
            specific,
            extra,
        ),
    ))
}

pub(crate) fn entity_type_uncompressed(input: BitInput) -> IResult<BitInput, EntityType> {
    let (input, kind): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, domain): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, country): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, category): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, subcategory): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, specific): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, extra): (BitInput, u8) = take(EIGHT_BITS)(input)?;

    let entity_type = EntityType {
        kind,
        domain,
        country,
        category: UVINT8::from(category),
        subcategory: UVINT8::from(subcategory),
        specific: UVINT8::from(specific),
        extra: UVINT8::from(extra),
    };

    Ok((input, entity_type))
}

pub(crate) fn linear_velocity(input: BitInput) -> IResult<BitInput, LinearVelocity> {
    let (input, x_component) = svint16(input)?;
    let (input, y_component) = svint16(input)?;
    let (input, z_component) = svint16(input)?;

    Ok((
        input,
        LinearVelocity::new(x_component, y_component, z_component),
    ))
}

pub(crate) fn linear_acceleration(input: BitInput) -> IResult<BitInput, LinearAcceleration> {
    let (input, x_component) = svint14(input)?;
    let (input, y_component) = svint14(input)?;
    let (input, z_component) = svint14(input)?;

    Ok((
        input,
        LinearAcceleration::new(x_component, y_component, z_component),
    ))
}

#[allow(clippy::similar_names)]
#[allow(clippy::cast_possible_truncation)]
pub(crate) fn orientation(input: BitInput) -> IResult<BitInput, Orientation> {
    let (input, psi): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;
    let (input, theta): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;
    let (input, phi): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;

    Ok((
        input,
        Orientation::new(psi as i16, theta as i16, phi as i16),
    ))
}

pub(crate) fn entity_marking(input: BitInput) -> IResult<BitInput, CdisEntityMarking> {
    let (input, length): (BitInput, usize) = take(FOUR_BITS)(input)?;
    let (input, char_bit_size): (BitInput, u8) = take(ONE_BIT)(input)?;
    let encoding = CdisMarkingCharEncoding::new(char_bit_size);
    let (input, chars): (BitInput, Vec<u8>) = count(take(encoding.bit_size()), length)(input)?;

    let marking = CdisEntityMarking::from((chars.as_slice(), encoding));

    Ok((input, marking))
}

#[allow(clippy::cast_precision_loss)]
const WORLD_COORDINATES_LAT_SCALE: f32 = ((2 ^ 30) - 1) as f32 / (f32::PI / 2.0);
const WORLD_COORDINATES_LON_SCALE: f32 = ((2 ^ 31) - 1) as f32 / (f32::PI);

#[allow(clippy::cast_precision_loss)]
pub(crate) fn world_coordinates(input: BitInput) -> IResult<BitInput, WorldCoordinates> {
    let (input, latitude): (BitInput, isize) = take_signed(THIRTY_ONE_BITS)(input)?;
    let latitude = latitude as f32 / WORLD_COORDINATES_LAT_SCALE;
    let (input, longitude): (BitInput, isize) = take_signed(THIRTY_TWO_BITS)(input)?;
    let longitude = longitude as f32 / WORLD_COORDINATES_LON_SCALE;
    let (input, altitude_msl): (BitInput, SVINT24) = svint24(input)?;

    Ok((
        input,
        WorldCoordinates {
            latitude,
            longitude,
            altitude_msl,
        },
    ))
}

pub(crate) fn variable_parameter(input: BitInput) -> IResult<BitInput, CdisVariableParameter> {
    let (input, compressed_flag): (BitInput, u8) = take(ONE_BIT)(input)?;
    let compressed_flag = compressed_flag != 0;
    let record_type_bits = if compressed_flag {
        THREE_BITS
    } else {
        EIGHT_BITS
    };
    let (input, record_type): (BitInput, u8) = take(record_type_bits)(input)?;
    let record_type = VariableParameterRecordType::from(record_type);

    let (input, variable_parameter) = match (record_type, compressed_flag) {
        (VariableParameterRecordType::ArticulatedPart, true) => {
            let (input, vp) = articulated_part_vp_compressed(input)?;
            (input, CdisVariableParameter::ArticulatedPart(vp))
        }
        (VariableParameterRecordType::ArticulatedPart, false) => {
            let (input, vp) = articulated_part_vp(input)?;
            (input, CdisVariableParameter::ArticulatedPart(vp))
        }
        (VariableParameterRecordType::AttachedPart, true) => {
            let (input, vp) = attached_part_vp_compressed(input)?;
            (input, CdisVariableParameter::AttachedPart(vp))
        }
        (VariableParameterRecordType::AttachedPart, false) => {
            let (input, vp) = attached_part_vp(input)?;
            (input, CdisVariableParameter::AttachedPart(vp))
        }
        (VariableParameterRecordType::Separation, true) => {
            let (input, vp) = entity_separation_vp_compressed(input)?;
            (input, CdisVariableParameter::EntitySeparation(vp))
        }
        (VariableParameterRecordType::Separation, false) => {
            let (input, vp) = entity_separation_vp(input)?;
            (input, CdisVariableParameter::EntitySeparation(vp))
        }
        (VariableParameterRecordType::EntityType, true) => {
            let (input, vp) = entity_type_vp_compressed(input)?;
            (input, CdisVariableParameter::EntityType(vp))
        }
        (VariableParameterRecordType::EntityType, false) => {
            let (input, vp) = entity_type_vp(input)?;
            (input, CdisVariableParameter::EntityType(vp))
        }
        (VariableParameterRecordType::EntityAssociation, true) => {
            let (input, vp) = entity_association_vp_compressed(input)?;
            (input, CdisVariableParameter::EntityAssociation(vp))
        }
        (VariableParameterRecordType::EntityAssociation, false) => {
            let (input, vp) = entity_association_vp(input)?;
            (input, CdisVariableParameter::EntityAssociation(vp))
        }
        (_, _) => (input, CdisVariableParameter::Unspecified),
    };
    Ok((input, variable_parameter))
}

pub(crate) fn articulated_part_vp_compressed(
    input: BitInput,
) -> IResult<BitInput, CdisArticulatedPartVP> {
    let (input, change_indicator): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, attachment_id): (BitInput, u16) = take(TEN_BITS)(input)?;
    let (input, parameter_type): (BitInput, u32) = take(FOURTEEN_BITS)(input)?;
    let type_metric: u32 = parameter_type & FIVE_LEAST_SIGNIFICANT_BITS; // 5 least significant bits are the Type Metric
    let type_class: u32 = parameter_type - type_metric; // Rest of the bits (Param Type minus Type Metric) are the Type Class
    let type_metric = ArticulatedPartsTypeMetric::from(type_metric);
    let type_class = ArticulatedPartsTypeClass::from(type_class);

    let (input, parameter_value) = ParameterValueFloat::parse(input)?;

    Ok((
        input,
        CdisArticulatedPartVP {
            change_indicator,
            attachment_id,
            type_class,
            type_metric,
            parameter_value,
        },
    ))
}

pub(crate) fn articulated_part_vp(input: BitInput) -> IResult<BitInput, CdisArticulatedPartVP> {
    let (input, change_indicator): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, attachment_id): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, parameter_type): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let type_metric: u32 = parameter_type & FIVE_LEAST_SIGNIFICANT_BITS; // 5 least significant bits are the Type Metric
    let type_class: u32 = parameter_type - type_metric; // Rest of the bits (Param Type minus Type Metric) are the Type Class
    let type_metric = ArticulatedPartsTypeMetric::from(type_metric);
    let type_class = ArticulatedPartsTypeClass::from(type_class);
    let (input, parameter_value): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let parameter_value = f32::from_bits(parameter_value);
    let parameter_value = ParameterValueFloat::new_uncompressed(parameter_value);

    Ok((
        input,
        CdisArticulatedPartVP {
            change_indicator,
            attachment_id,
            type_class,
            type_metric,
            parameter_value,
        },
    ))
}

pub(crate) fn attached_part_vp_compressed(
    input: BitInput,
) -> IResult<BitInput, CdisAttachedPartVP> {
    let (input, detached_indicator): (BitInput, u8) = take(ONE_BIT)(input)?;
    let detached_indicator = AttachedPartDetachedIndicator::from(detached_indicator);
    let (input, attachment_id): (BitInput, u16) = take(TEN_BITS)(input)?;
    let (input, parameter_type): (BitInput, u32) = take(ELEVEN_BITS)(input)?;
    let parameter_type = AttachedParts::from(parameter_type);
    let (input, attached_part_type) = entity_type(input)?;

    Ok((
        input,
        CdisAttachedPartVP {
            detached_indicator,
            attachment_id,
            parameter_type,
            attached_part_type,
        },
    ))
}

pub(crate) fn attached_part_vp(input: BitInput) -> IResult<BitInput, CdisAttachedPartVP> {
    let (input, detached_indicator): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let detached_indicator = AttachedPartDetachedIndicator::from(detached_indicator);
    let (input, attachment_id): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, parameter_type): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let parameter_type = AttachedParts::from(parameter_type);

    let (input, attached_part_type) = entity_type_uncompressed(input)?;

    Ok((
        input,
        CdisAttachedPartVP {
            detached_indicator,
            attachment_id,
            parameter_type,
            attached_part_type,
        },
    ))
}

pub(crate) fn entity_separation_vp_compressed(
    input: BitInput,
) -> IResult<BitInput, CdisEntitySeparationVP> {
    let (input, reason_for_separation): (BitInput, u8) = take(THREE_BITS)(input)?;
    let reason_for_separation = SeparationReasonForSeparation::from(reason_for_separation);
    let (input, pre_entity_indicator): (BitInput, u8) = take(THREE_BITS)(input)?;
    let pre_entity_indicator = SeparationPreEntityIndicator::from(pre_entity_indicator);
    let (input, parent_entity_id) = entity_identification(input)?;

    let (input, station_name): (BitInput, u16) = take(SIX_BITS)(input)?;
    let station_name = StationName::from(station_name);
    let (input, station_number): (BitInput, u16) = take(TWELVE_BITS)(input)?;

    Ok((
        input,
        CdisEntitySeparationVP {
            reason_for_separation,
            pre_entity_indicator,
            parent_entity_id,
            station_name,
            station_number,
        },
    ))
}

pub(crate) fn entity_separation_vp(input: BitInput) -> IResult<BitInput, CdisEntitySeparationVP> {
    let (input, reason_for_separation): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let reason_for_separation = SeparationReasonForSeparation::from(reason_for_separation);
    let (input, pre_entity_indicator): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let pre_entity_indicator = SeparationPreEntityIndicator::from(pre_entity_indicator);
    let (input, _padding): (BitInput, u8) = take(EIGHT_BITS)(input)?;

    let (input, parent_entity_id) = entity_identification_uncompressed(input)?;

    let (input, _padding): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    let (input, station_name): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let station_name = StationName::from(station_name);
    let (input, station_number): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((
        input,
        CdisEntitySeparationVP {
            reason_for_separation,
            pre_entity_indicator,
            parent_entity_id,
            station_name,
            station_number,
        },
    ))
}

pub(crate) fn entity_type_vp_compressed(input: BitInput) -> IResult<BitInput, CdisEntityTypeVP> {
    let (input, change_indicator): (BitInput, u8) = take(ONE_BIT)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, attached_part_type) = entity_type(input)?;

    Ok((
        input,
        CdisEntityTypeVP {
            change_indicator,
            attached_part_type,
        },
    ))
}

pub(crate) fn entity_type_vp(input: BitInput) -> IResult<BitInput, CdisEntityTypeVP> {
    let (input, change_indicator): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, attached_part_type) = entity_type_uncompressed(input)?;

    let (input, _padding): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, _padding): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;

    Ok((
        input,
        CdisEntityTypeVP {
            change_indicator,
            attached_part_type,
        },
    ))
}

pub(crate) fn entity_association_vp_compressed(
    input: BitInput,
) -> IResult<BitInput, CdisEntityAssociationVP> {
    let (input, change_indicator): (BitInput, u8) = take(ONE_BIT)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, association_status): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let association_status = EntityAssociationAssociationStatus::from(association_status);
    let (input, association_type): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let association_type = EntityAssociationPhysicalAssociationType::from(association_type);
    let (input, entity_id) = entity_identification(input)?;
    let (input, own_station_location): (BitInput, u16) = take(SIX_BITS)(input)?;
    let own_station_location = StationName::from(own_station_location);
    let (input, physical_connection_type): (BitInput, u8) = take(FIVE_BITS)(input)?;
    let physical_connection_type =
        EntityAssociationPhysicalConnectionType::from(physical_connection_type);
    let (input, group_member_type): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let group_member_type = EntityAssociationGroupMemberType::from(group_member_type);
    let (input, group_number): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((
        input,
        CdisEntityAssociationVP {
            change_indicator,
            association_status,
            association_type,
            entity_id,
            own_station_location,
            physical_connection_type,
            group_member_type,
            group_number,
        },
    ))
}

pub(crate) fn entity_association_vp(input: BitInput) -> IResult<BitInput, CdisEntityAssociationVP> {
    let (input, change_indicator): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, association_status): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let association_status = EntityAssociationAssociationStatus::from(association_status);
    let (input, association_type): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let association_type = EntityAssociationPhysicalAssociationType::from(association_type);

    let (input, entity_id) = entity_identification_uncompressed(input)?;
    let (input, own_station_location): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let own_station_location = StationName::from(own_station_location);

    let (input, physical_connection_type): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let physical_connection_type =
        EntityAssociationPhysicalConnectionType::from(physical_connection_type);
    let (input, group_member_type): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let group_member_type = EntityAssociationGroupMemberType::from(group_member_type);
    let (input, group_number): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((
        input,
        CdisEntityAssociationVP {
            change_indicator,
            association_status,
            association_type,
            entity_id,
            own_station_location,
            physical_connection_type,
            group_member_type,
            group_number,
        },
    ))
}

pub(crate) fn fixed_datum(input: BitInput) -> IResult<BitInput, FixedDatum> {
    let (input, datum_id): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let datum_id = VariableRecordType::from(datum_id);

    let (input, datum_value): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;

    Ok((input, FixedDatum::new(datum_id, datum_value)))
}

pub(crate) fn variable_datum(input: BitInput) -> IResult<BitInput, VariableDatum> {
    let (input, datum_id): (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let datum_id = VariableRecordType::from(datum_id);

    let (input, datum_length_bits): (BitInput, usize) = take(FOURTEEN_BITS)(input)?;
    let (num_full_bytes, partial_bytes_bits) = datum_length_bits.div_rem(&EIGHT_BITS);

    let (input, mut datum_value): (BitInput, Vec<u8>) =
        count(take(EIGHT_BITS), num_full_bytes)(input)?;
    let (input, datum_value) = if partial_bytes_bits != 0 {
        let (input, last_byte): (BitInput, u8) = take(partial_bytes_bits)(input)?;
        datum_value.push(last_byte << (EIGHT_BITS - partial_bytes_bits));
        (input, datum_value)
    } else {
        (input, datum_value)
    };

    Ok((input, VariableDatum::new(datum_id, datum_value)))
}

#[allow(clippy::match_same_arms)]
pub(crate) fn encoding_scheme(input: BitInput) -> IResult<BitInput, EncodingScheme> {
    let (input, encoding_scheme_class): (BitInput, u16) = take(TWO_BITS)(input)?;
    let encoding_scheme_class = SignalEncodingClass::from(encoding_scheme_class);
    let (input, encoding_scheme_type) = uvint8(input)?;

    let encoding_scheme = match encoding_scheme_class {
        SignalEncodingClass::EncodedAudio => EncodingScheme::EncodedAudio {
            encoding_class: encoding_scheme_class,
            encoding_type: SignalEncodingType::from(u16::from(encoding_scheme_type.value)),
        },
        SignalEncodingClass::RawBinaryData => EncodingScheme::RawBinaryData {
            encoding_class: encoding_scheme_class,
            nr_of_messages: encoding_scheme_type.value,
        },
        SignalEncodingClass::ApplicationSpecificData => EncodingScheme::Unspecified {
            encoding_class: encoding_scheme_class,
            encoding_type: encoding_scheme_type.value,
        },
        SignalEncodingClass::DatabaseIndex => EncodingScheme::Unspecified {
            encoding_class: encoding_scheme_class,
            encoding_type: encoding_scheme_type.value,
        },
        SignalEncodingClass::Unspecified(_) => EncodingScheme::Unspecified {
            encoding_class: encoding_scheme_class,
            encoding_type: encoding_scheme_type.value,
        },
    };

    Ok((input, encoding_scheme))
}

#[allow(clippy::similar_names)]
pub(crate) fn beam_antenna_pattern(input: BitInput) -> IResult<BitInput, BeamAntennaPattern> {
    let (input, beam_direction_psi): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;
    let (input, beam_direction_theta): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;
    let (input, beam_direction_phi): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;
    let (input, az_beamwidth): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;
    let (input, el_beamwidth): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;
    let (input, reference_system): (BitInput, u8) = take(TWO_BITS)(input)?;
    let reference_system = TransmitterAntennaPatternReferenceSystem::from(reference_system);
    let (input, e_z): (BitInput, isize) = take_signed(SIXTEEN_BITS)(input)?;
    let (input, e_y): (BitInput, isize) = take_signed(SIXTEEN_BITS)(input)?;
    let (input, phase): (BitInput, isize) = take_signed(THIRTEEN_BITS)(input)?;

    Ok((
        input,
        BeamAntennaPattern {
            beam_direction_psi: beam_direction_psi as i16,
            beam_direction_theta: beam_direction_theta as i16,
            beam_direction_phi: beam_direction_phi as i16,
            az_beamwidth: az_beamwidth as i16,
            el_beamwidth: el_beamwidth as i16,
            reference_system,
            e_z: e_z as i16,
            e_x: e_y as i16,
            phase: phase as i16,
        },
    ))
}

#[allow(clippy::missing_errors_doc)]
pub fn beam_data(input: BitInput) -> IResult<BitInput, BeamData> {
    let (input, az_center) = svint13(input)?;
    let (input, az_sweep) = svint13(input)?;
    let (input, el_center) = svint13(input)?;
    let (input, el_sweep) = svint13(input)?;
    let (input, sweep_sync): (BitInput, u16) = take(TEN_BITS)(input)?;

    Ok((
        input,
        BeamData {
            az_center,
            az_sweep,
            el_center,
            el_sweep,
            sweep_sync,
        },
    ))
}

#[allow(clippy::missing_errors_doc)]
pub fn layer_header(input: BitInput) -> IResult<BitInput, LayerHeader> {
    let (input, layer_number): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, layer_specific_information): (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, length): (BitInput, u16) = take(FOURTEEN_BITS)(input)?;

    Ok((
        input,
        LayerHeader {
            layer_number,
            layer_specific_information,
            length,
        },
    ))
}

#[cfg(test)]
mod tests {
    use crate::records::model::CdisProtocolVersion;
    use crate::records::parser::cdis_header;
    use crate::types::model::UVINT8;
    use dis_rs::enumerations::PduType;

    #[test]
    fn parse_cdis_header() {
        let input = [
            0b01001110, 0b00000010, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000,
        ];

        let (_input, header) = cdis_header((&input, 0)).unwrap();

        assert_eq!(header.protocol_version, CdisProtocolVersion::SISO_023_2023);
        assert_eq!(header.exercise_id, UVINT8::from(7));
        assert_eq!(header.pdu_type, PduType::EntityState);
        assert_eq!(
            header.timestamp,
            dis_rs::model::TimeStamp { raw_timestamp: 0 }
        );
        assert_eq!(header.length, 0);
    }
}
