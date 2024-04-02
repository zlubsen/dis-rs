use bitvec::macros::internal::funty::Floating;
use nom::IResult;
use nom::bits::complete::take;
use nom::multi::count;
use dis_rs::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedPartDetachedIndicator, AttachedParts, ChangeIndicator, EntityAssociationAssociationStatus, EntityAssociationGroupMemberType, EntityAssociationPhysicalAssociationType, EntityAssociationPhysicalConnectionType, PduType, SeparationPreEntityIndicator, SeparationReasonForSeparation, StationName, VariableParameterRecordType};
use dis_rs::model::TimeStamp;
use dis_rs::parse_pdu_status_fields;
use crate::constants::{EIGHT_BITS, ELEVEN_BITS, FIVE_BITS, FOUR_BITS, FOURTEEN_BITS, NINE_BITS, ONE_BIT, SIX_BITS, SIXTEEN_BITS, TEN_BITS, THIRTEEN_BITS, THIRTY_ONE_BITS, THIRTY_TWO_BITS, THREE_BITS, TWELVE_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::parsing::BitInput;
use crate::parsing::take_signed;
use crate::records::model::{AngularVelocity, CdisArticulatedPartVP, CdisAttachedPartVP, CdisEntityAssociationVP, CdisEntityMarking, CdisEntitySeparationVP, CdisEntityTypeVP, CdisHeader, CdisMarkingCharEncoding, CdisProtocolVersion, CdisVariableParameter, EntityCoordinateVector, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, ParameterValueFloat, WorldCoordinates};
use crate::types::model::{CdisFloat, SVINT24, UVINT16, UVINT8};
use crate::types::parser::{cdis_float, svint12, svint14, svint16, svint24, uvint16, uvint8};

const FIVE_LEAST_SIGNIFICANT_BITS : u32 = 0x1f;

pub(crate) fn cdis_header(input: BitInput) -> IResult<BitInput, CdisHeader> {
    let (input, protocol_version) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let (input, exercise_id) = uvint8(input)?;
    let (input, pdu_type) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, timestamp) : (BitInput, u32) = take(TWENTY_SIX_BITS)(input)?;
    let (input, length) : (BitInput, u16) = take(FOURTEEN_BITS)(input)?;
    let (input, pdu_status) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let pdu_status = parse_pdu_status_fields(pdu_type, pdu_status);

    Ok((input, CdisHeader {
        protocol_version: CdisProtocolVersion::from(protocol_version),
        exercise_id,
        pdu_type: PduType::from(pdu_type),
        timestamp: TimeStamp::from(timestamp),
        length,
        pdu_status,
    }))
}

pub(crate) fn angular_velocity(input: BitInput) -> IResult<BitInput, AngularVelocity> {
    let (input, x_component) = svint12(input)?;
    let (input, y_component) = svint12(input)?;
    let (input, z_component) = svint12(input)?;

    Ok((input, AngularVelocity::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn entity_coordinate_vector(input: BitInput) -> IResult<BitInput, EntityCoordinateVector> {
    let (input, x_component) = svint16(input)?;
    let (input, y_component) = svint16(input)?;
    let (input, z_component) = svint16(input)?;

    Ok((input, EntityCoordinateVector::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn entity_identification(input: BitInput) -> IResult<BitInput, EntityId> {
    let (input, site) = uvint16(input)?;
    let (input, application) = uvint16(input)?;
    let (input, entity) = uvint16(input)?;

    Ok((input, EntityId::new(
        site,
        application,
        entity)))
}

pub(crate) fn entity_identification_uncompressed(input: BitInput) -> IResult<BitInput, EntityId> {
    let (input, site) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, application) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, entity) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((input, EntityId::new(
        UVINT16::from(site),
        UVINT16::from(application),
        UVINT16::from(entity))))
}

pub(crate) fn entity_type(input: BitInput) -> IResult<BitInput, EntityType> {
    let (input, kind) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, domain) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let (input, country) : (BitInput, u16) = take(NINE_BITS)(input)?;
    let (input, category) = uvint8(input)?;
    let (input, subcategory) = uvint8(input)?;
    let (input, specific) = uvint8(input)?;
    let (input, extra) = uvint8(input)?;

    Ok((input, EntityType::new(
        kind,
        domain,
        country,
        category,
        subcategory,
        specific,
        extra)))
}

pub(crate) fn entity_type_uncompressed(input: BitInput) -> IResult<BitInput, EntityType> {
    let (input, kind) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, domain) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, country) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, category) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, subcategory) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, specific) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, extra) : (BitInput, u8) = take(EIGHT_BITS)(input)?;

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

    Ok((input, LinearVelocity::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn linear_acceleration(input: BitInput) -> IResult<BitInput, LinearAcceleration> {
    let (input, x_component) = svint14(input)?;
    let (input, y_component) = svint14(input)?;
    let (input, z_component) = svint14(input)?;

    Ok((input, LinearAcceleration::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn orientation(input: BitInput) -> IResult<BitInput, Orientation> {
    let (input, psi) : (BitInput, u16) = take(THIRTEEN_BITS)(input)?;
    let (input, theta) : (BitInput, u16) = take(THIRTEEN_BITS)(input)?;
    let (input, phi) : (BitInput, u16) = take(THIRTEEN_BITS)(input)?;

    Ok((input, Orientation::new(
        psi,
        theta,
        phi)))
}

pub(crate) fn entity_marking(input: BitInput) -> IResult<BitInput, CdisEntityMarking> {
    let (input, length) : (BitInput, usize) = take(FOUR_BITS)(input)?;
    let (input, char_bit_size) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let encoding = CdisMarkingCharEncoding::new(char_bit_size);
    let (input, chars) : (BitInput, Vec<u8>) = count(take(encoding.bit_size()), length)(input)?;

    let marking = CdisEntityMarking::from((chars.as_slice(), encoding));

    Ok((input, marking))
}

const WORLD_COORDINATES_LAT_SCALE: f32 = (2^30 - 1) as f32 / (f32::PI / 2.0);
const WORLD_COORDINATES_LON_SCALE: f32 = (2^31 - 1) as f32 / (f32::PI);

pub(crate) fn world_coordinates(input: BitInput) -> IResult<BitInput, WorldCoordinates> {
    let (input, latitude) : (BitInput, isize) = take_signed(THIRTY_ONE_BITS)(input)?;
    let latitude = latitude as f32 / WORLD_COORDINATES_LAT_SCALE;
    let (input, longitude) : (BitInput, i32) = take(THIRTY_TWO_BITS)(input)?; // TODO does take(32) work for i32? Or should we use take_signed(32)
    let longitude = longitude as f32 / WORLD_COORDINATES_LON_SCALE;
    let (input, altitude_msl) : (BitInput, SVINT24) = svint24(input)?;

    Ok((input, WorldCoordinates {
        latitude,
        longitude,
        altitude_msl,
    }))
}

pub(crate) fn variable_parameter(input: BitInput) -> IResult<BitInput, CdisVariableParameter> {
    let (input, compressed_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let compressed_flag = compressed_flag != 0;
    let record_type_bits = if compressed_flag {
        THREE_BITS
    } else {
        EIGHT_BITS
    };
    let (input, record_type) : (BitInput, u8) = take(record_type_bits)(input)?;
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
        (_, _) => { (input, CdisVariableParameter::Unspecified) }
    };
    Ok((input, variable_parameter))
}

pub(crate) fn articulated_part_vp_compressed(input: BitInput) -> IResult<BitInput, CdisArticulatedPartVP> {
    let (input, change_indicator) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, attachment_id) : (BitInput, u16) = take(TEN_BITS)(input)?;
    let (input, parameter_type) : (BitInput, u32) = take(FOURTEEN_BITS)(input)?;
    let type_metric : u32 = parameter_type & FIVE_LEAST_SIGNIFICANT_BITS;   // 5 least significant bits are the Type Metric
    let type_class : u32 = parameter_type - type_metric;                    // Rest of the bits (Param Type minus Type Metric) are the Type Class
    let type_metric = ArticulatedPartsTypeMetric::from(type_metric);
    let type_class = ArticulatedPartsTypeClass::from(type_class);

    let (input, parameter_value) = cdis_float::<ParameterValueFloat>(input)?;

    Ok((input, CdisArticulatedPartVP {
        change_indicator,
        attachment_id,
        type_class,
        type_metric,
        parameter_value,
    }))
}

pub(crate) fn articulated_part_vp(input: BitInput) -> IResult<BitInput, CdisArticulatedPartVP> {
    let (input, change_indicator) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, attachment_id) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, parameter_type) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let type_metric : u32 = parameter_type & FIVE_LEAST_SIGNIFICANT_BITS;   // 5 least significant bits are the Type Metric
    let type_class : u32 = parameter_type - type_metric;                    // Rest of the bits (Param Type minus Type Metric) are the Type Class
    let type_metric = ArticulatedPartsTypeMetric::from(type_metric);
    let type_class = ArticulatedPartsTypeClass::from(type_class);
    let (input, parameter_value) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let parameter_value = f32::from_bits(parameter_value);
    let parameter_value = ParameterValueFloat::from_f64(parameter_value as f64);

    Ok((input, CdisArticulatedPartVP {
        change_indicator,
        attachment_id,
        type_class,
        type_metric,
        parameter_value,
    }))
}

pub(crate) fn attached_part_vp_compressed(input: BitInput) -> IResult<BitInput, CdisAttachedPartVP> {
    let (input, detached_indicator) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let detached_indicator = AttachedPartDetachedIndicator::from(detached_indicator);
    let (input, attachment_id) : (BitInput, u16) = take(TEN_BITS)(input)?;
    let (input, parameter_type) : (BitInput, u32) = take(ELEVEN_BITS)(input)?;
    let parameter_type = AttachedParts::from(parameter_type);
    let (input, attached_part_type) = entity_type(input)?;

    Ok((input, CdisAttachedPartVP {
        detached_indicator,
        attachment_id,
        parameter_type,
        attached_part_type,
    }))
}

pub(crate) fn attached_part_vp(input: BitInput) -> IResult<BitInput, CdisAttachedPartVP> {
    let (input, detached_indicator) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let detached_indicator = AttachedPartDetachedIndicator::from(detached_indicator);
    let (input, attachment_id) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, parameter_type) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let parameter_type = AttachedParts::from(parameter_type);

    let (input, attached_part_type) = entity_type_uncompressed(input)?;

    Ok((input, CdisAttachedPartVP {
        detached_indicator,
        attachment_id,
        parameter_type,
        attached_part_type,
    }))
}

pub(crate) fn entity_separation_vp_compressed(input: BitInput) -> IResult<BitInput, CdisEntitySeparationVP> {
    let (input, reason_for_separation) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let reason_for_separation = SeparationReasonForSeparation::from(reason_for_separation);
    let (input, pre_entity_indicator) : (BitInput, u8) = take(THREE_BITS)(input)?;
    let pre_entity_indicator = SeparationPreEntityIndicator::from(pre_entity_indicator);
    let (input, parent_entity_id) = entity_identification(input)?;

    let (input, station_name) : (BitInput, u16) = take(SIX_BITS)(input)?;
    let station_name = StationName::from(station_name);
    let (input, station_number) : (BitInput, u16) = take(TWELVE_BITS)(input)?;

    Ok((input, CdisEntitySeparationVP {
        reason_for_separation,
        pre_entity_indicator,
        parent_entity_id,
        station_name,
        station_number,
    }))
}

pub(crate) fn entity_separation_vp(input: BitInput) -> IResult<BitInput, CdisEntitySeparationVP> {
    let (input, reason_for_separation) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let reason_for_separation = SeparationReasonForSeparation::from(reason_for_separation);
    let (input, pre_entity_indicator) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let pre_entity_indicator = SeparationPreEntityIndicator::from(pre_entity_indicator);
    let (input, _padding) : (BitInput, u8) = take(EIGHT_BITS)(input)?;

    let (input, parent_entity_id) = entity_identification_uncompressed(input)?;

    let (input, _padding) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    let (input, station_name) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let station_name = StationName::from(station_name);
    let (input, station_number) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((input, CdisEntitySeparationVP {
        reason_for_separation,
        pre_entity_indicator,
        parent_entity_id,
        station_name,
        station_number,
    }))
}

pub(crate) fn entity_type_vp_compressed(input: BitInput) -> IResult<BitInput, CdisEntityTypeVP> {
    let (input, change_indicator) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, attached_part_type) = entity_type(input)?;

    Ok((input, CdisEntityTypeVP {
        change_indicator,
        attached_part_type,
    }))
}

pub(crate) fn entity_type_vp(input: BitInput) -> IResult<BitInput, CdisEntityTypeVP> {
    let (input, change_indicator) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, attached_part_type) = entity_type_uncompressed(input)?;

    let (input, _padding) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let (input, _padding) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;

    Ok((input, CdisEntityTypeVP {
        change_indicator,
        attached_part_type,
    }))
}

pub(crate) fn entity_association_vp_compressed(input: BitInput) -> IResult<BitInput, CdisEntityAssociationVP> {
    let (input, change_indicator) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, association_status) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let association_status = EntityAssociationAssociationStatus::from(association_status);
    let (input, association_type) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let association_type = EntityAssociationPhysicalAssociationType::from(association_type);
    let (input, entity_id) = entity_identification(input)?;
    let (input, own_station_location) : (BitInput, u16) = take(SIX_BITS)(input)?;
    let own_station_location = StationName::from(own_station_location);
    let (input, physical_connection_type) : (BitInput, u8) = take(FIVE_BITS)(input)?;
    let physical_connection_type = EntityAssociationPhysicalConnectionType::from(physical_connection_type);
    let (input, group_member_type) : (BitInput, u8) = take(FOUR_BITS)(input)?;
    let group_member_type = EntityAssociationGroupMemberType::from(group_member_type);
    let (input, group_number) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((input, CdisEntityAssociationVP {
        change_indicator,
        association_status,
        association_type,
        entity_id,
        own_station_location,
        physical_connection_type,
        group_member_type,
        group_number,
    }))
}

pub(crate) fn entity_association_vp(input: BitInput) -> IResult<BitInput, CdisEntityAssociationVP> {
    let (input, change_indicator) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, association_status) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let association_status = EntityAssociationAssociationStatus::from(association_status);
    let (input, association_type) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let association_type = EntityAssociationPhysicalAssociationType::from(association_type);

    let (input, entity_id) = entity_identification_uncompressed(input)?;
    let (input, own_station_location) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
    let own_station_location = StationName::from(own_station_location);

    let (input, physical_connection_type) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let physical_connection_type = EntityAssociationPhysicalConnectionType::from(physical_connection_type);
    let (input, group_member_type) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let group_member_type = EntityAssociationGroupMemberType::from(group_member_type);
    let (input, group_number) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    Ok((input, CdisEntityAssociationVP {
        change_indicator,
        association_status,
        association_type,
        entity_id,
        own_station_location,
        physical_connection_type,
        group_member_type,
        group_number,
    }))
}
