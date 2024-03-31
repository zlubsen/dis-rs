use bitvec::macros::internal::funty::Floating;
use nom::IResult;
use nom::bits::complete::take;
use nom::multi::count;
use dis_rs::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, PduType, VariableParameterRecordType};
use dis_rs::model::TimeStamp;
use dis_rs::parse_pdu_status_fields;
use crate::constants::{EIGHT_BITS, FIFTEEN_BITS, FOUR_BITS, FOURTEEN_BITS, NINE_BITS, ONE_BIT, SIX_BITS, TEN_BITS, THIRTEEN_BITS, THIRTY_BITS, THIRTY_ONE_BITS, THIRTY_TWO_BITS, THREE_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::parser_utils::{BitInput, take_signed};
use crate::records::model::{AngularVelocity, CdisArticulatedPartVP, CdisAttachedPartVP, CdisEntityAssociationVP, CdisEntityMarking, CdisEntitySeparationVP, CdisEntityTypeVP, CdisHeader, CdisMarkingCharEncoding, CdisProtocolVersion, CdisVariableParameter, EntityCoordinateVector, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, WorldCoordinates};
use crate::types::model::{CdisFloat, SVINT24};
use crate::types::parser::{svint12, svint14, svint16, svint24, uvint16, uvint8};

const FIVE_LEAST_SIGNIFICANT_BITS : u32 = 0x1f;

pub(crate) fn cdis_header(input: (&[u8], usize)) -> IResult<(&[u8], usize), CdisHeader> {
    let (input, protocol_version) : ((&[u8], usize), u8) = take(TWO_BITS)(input)?;
    let (input, exercise_id) = uvint8(input)?;
    let (input, pdu_type) : ((&[u8], usize), u8) = take(EIGHT_BITS)(input)?;
    let (input, timestamp) : ((&[u8], usize), u32) = take(TWENTY_SIX_BITS)(input)?;
    let (input, length) : ((&[u8], usize), u16) = take(FOURTEEN_BITS)(input)?;
    let (input, pdu_status) : ((&[u8], usize), u8) = take(EIGHT_BITS)(input)?;
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

pub(crate) fn angular_velocity(input: (&[u8], usize)) -> IResult<(&[u8], usize), AngularVelocity> {
    let (input, x_component) = svint12(input)?;
    let (input, y_component) = svint12(input)?;
    let (input, z_component) = svint12(input)?;

    Ok((input, AngularVelocity::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn entity_coordinate_vector(input: (&[u8], usize)) -> IResult<(&[u8], usize), EntityCoordinateVector> {
    let (input, x_component) = svint16(input)?;
    let (input, y_component) = svint16(input)?;
    let (input, z_component) = svint16(input)?;

    Ok((input, EntityCoordinateVector::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn entity_identification(input: (&[u8], usize)) -> IResult<(&[u8], usize), EntityId> {
    let (input, site) = uvint16(input)?;
    let (input, application) = uvint16(input)?;
    let (input, entity) = uvint16(input)?;

    Ok((input, EntityId::new(
        site,
        application,
        entity)))
}

pub(crate) fn entity_type(input: (&[u8], usize)) -> IResult<(&[u8], usize), EntityType> {
    let (input, kind) : ((&[u8], usize), u8) = take(FOUR_BITS)(input)?;
    let (input, domain) : ((&[u8], usize), u8) = take(FOUR_BITS)(input)?;
    let (input, country) : ((&[u8], usize), u16) = take(NINE_BITS)(input)?;
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

pub(crate) fn linear_velocity(input: (&[u8], usize)) -> IResult<(&[u8], usize), LinearVelocity> {
    let (input, x_component) = svint16(input)?;
    let (input, y_component) = svint16(input)?;
    let (input, z_component) = svint16(input)?;

    Ok((input, LinearVelocity::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn linear_acceleration(input: (&[u8], usize)) -> IResult<(&[u8], usize), LinearAcceleration> {
    let (input, x_component) = svint14(input)?;
    let (input, y_component) = svint14(input)?;
    let (input, z_component) = svint14(input)?;

    Ok((input, LinearAcceleration::new(
        x_component,
        y_component,
        z_component)))
}

pub(crate) fn orientation(input: (&[u8], usize)) -> IResult<(&[u8], usize), Orientation> {
    let (input, psi) : ((&[u8], usize), u16) = take(THIRTEEN_BITS)(input)?;
    let (input, theta) : ((&[u8], usize), u16) = take(THIRTEEN_BITS)(input)?;
    let (input, phi) : ((&[u8], usize), u16) = take(THIRTEEN_BITS)(input)?;

    Ok((input, Orientation::new(
        psi,
        theta,
        phi)))
}

pub(crate) fn entity_marking(input: (&[u8], usize)) -> IResult<(&[u8], usize), CdisEntityMarking> {
    let (input, length) : ((&[u8], usize), usize) = take(FOUR_BITS)(input)?;
    let (input, char_bit_size) : ((&[u8], usize), u8) = take(ONE_BIT)(input)?;
    let encoding = CdisMarkingCharEncoding::new(char_bit_size);
    let (input, chars) : ((&[u8], usize), Vec<u8>) = count(take(encoding.bit_size()), length)(input)?;

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
    let (input, parameter_value_mantissa) : (BitInput, isize) = take_signed(FIFTEEN_BITS)(input)?;
    let parameter_value_mantissa = parameter_value_mantissa as i32;
    let (input, parameter_value_exponent) : (BitInput, isize) = take_signed(THREE_BITS)(input)?;
    let parameter_value_exponent = parameter_value_exponent as i8;
    let parameter_value = CdisFloat::new(parameter_value_mantissa, parameter_value_exponent);

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
    let (input, attachment_id) : (BitInput, u16) = take(SIX_BITS)(input)?;
    let (input, parameter_type) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let type_metric : u32 = parameter_type & FIVE_LEAST_SIGNIFICANT_BITS;   // 5 least significant bits are the Type Metric
    let type_class : u32 = parameter_type - type_metric;                    // Rest of the bits (Param Type minus Type Metric) are the Type Class
    let type_metric = ArticulatedPartsTypeMetric::from(type_metric);
    let type_class = ArticulatedPartsTypeClass::from(type_class);
    let (input, parameter_value) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let parameter_value = f32::from_bits(parameter_value);
    let parameter_value = CdisFloat::from_f64(parameter_value as f64);

    Ok((input, CdisArticulatedPartVP {
        change_indicator,
        attachment_id,
        type_class,
        type_metric,
        parameter_value,
    }))
}

pub(crate) fn attached_part_vp_compressed(input: BitInput) -> IResult<BitInput, CdisAttachedPartVP> {
    todo!()
}

pub(crate) fn attached_part_vp(input: BitInput) -> IResult<BitInput, CdisAttachedPartVP> {
    todo!()
}

pub(crate) fn entity_separation_vp_compressed(input: BitInput) -> IResult<BitInput, CdisEntitySeparationVP> {
    todo!()
}

pub(crate) fn entity_separation_vp(input: BitInput) -> IResult<BitInput, CdisEntitySeparationVP> {
    todo!()
}

pub(crate) fn entity_type_vp_compressed(input: BitInput) -> IResult<BitInput, CdisEntityTypeVP> {
    todo!()
}

pub(crate) fn entity_type_vp(input: BitInput) -> IResult<BitInput, CdisEntityTypeVP> {
    todo!()
}

pub(crate) fn entity_association_vp_compressed(input: BitInput) -> IResult<BitInput, CdisEntityAssociationVP> {
    todo!()
}

pub(crate) fn entity_association_vp(input: BitInput) -> IResult<BitInput, CdisEntityAssociationVP> {
    todo!()
}
