use bitvec::macros::internal::funty::Floating;
use nom::IResult;
use nom::bits::complete::take;
use nom::multi::count;
use dis_rs::enumerations::PduType;
use dis_rs::model::TimeStamp;
use dis_rs::parse_pdu_status_fields;
use crate::constants::{EIGHT_BITS, FOUR_BITS, FOURTEEN_BITS, NINE_BITS, ONE_BIT, THIRTEEN_BITS, THIRTY_BITS, THIRTY_TWO_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisHeader, CdisMarkingCharEncoding, CdisProtocolVersion, EntityCoordinateVector, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, WorldCoordinates};
use crate::types::model::SVINT24;
use crate::types::parser::{svint12, svint14, svint16, svint24, uvint16, uvint8};

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

pub(crate) fn world_coordinates(input: (&[u8], usize)) -> IResult<(&[u8], usize), WorldCoordinates> {
    let (input, lat_sign_bit) : ((&[u8], usize), i32) = take(ONE_BIT)(input)?;
    let (input, lat_value_bits) : ((&[u8], usize), i32) = take(THIRTY_BITS)(input)?;
    let latitude = (lat_sign_bit << 31) | lat_value_bits;
    let latitude = latitude as f32 / WORLD_COORDINATES_LAT_SCALE;
    let (input, longitude) : ((&[u8], usize), i32) = take(THIRTY_TWO_BITS)(input)?;
    let longitude = longitude as f32 / WORLD_COORDINATES_LON_SCALE;
    let (input, altitude_msl) : ((&[u8], usize), SVINT24) = svint24(input)?;

    Ok((input, WorldCoordinates {
        latitude,
        longitude,
        altitude_msl,
    }))
}