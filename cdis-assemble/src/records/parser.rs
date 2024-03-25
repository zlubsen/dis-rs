use nom::IResult;
use nom::bits::complete::take;
use dis_rs::enumerations::PduType;
use dis_rs::model::TimeStamp;
use dis_rs::parse_pdu_status_fields;
use crate::constants::{EIGHT_BITS, FOUR_BITS, FOURTEEN_BITS, NINE_BITS, THIRTEEN_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::records::model::{AngularVelocity, CdisHeader, CdisProtocolVersion, EntityCoordinateVector, EntityId, EntityType, LinearVelocity, Orientation};
use crate::types::parser::{svint12, svint16, uvint16, uvint8};

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

pub(crate) fn orientation(input: (&[u8], usize)) -> IResult<(&[u8], usize), Orientation> {
    let (input, psi) : ((&[u8], usize), u16) = take(THIRTEEN_BITS)(input)?;
    let (input, theta) : ((&[u8], usize), u16) = take(THIRTEEN_BITS)(input)?;
    let (input, phi) : ((&[u8], usize), u16) = take(THIRTEEN_BITS)(input)?;

    Ok((input, Orientation::new(
        psi,
        theta,
        phi)))
}