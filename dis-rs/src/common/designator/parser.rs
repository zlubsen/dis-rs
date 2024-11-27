use crate::common::designator::model::Designator;
use crate::common::model::PduBody;
use crate::common::parser::{entity_id, location, vec3_f32};
use crate::enumerations::{DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName};
use nom::number::complete::{be_f32, be_u16, be_u8};
use nom::IResult;

pub(crate) fn designator_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, designating_entity_id) = entity_id(input)?;
    let (input, system_name) = be_u16(input)?;
    let system_name = DesignatorSystemName::from(system_name);
    let (input, designated_entity_id) = entity_id(input)?;
    let (input, code) = be_u16(input)?;
    let code = DesignatorCode::from(code);
    let (input, power) = be_f32(input)?;
    let (input, wavelength) = be_f32(input)?;
    let (input, spot_wrt_designated_entity) = vec3_f32(input)?;
    let (input, spot_location) = location(input)?;
    let (input, dead_reckoning_algorithm) = be_u8(input)?;
    let dead_reckoning_algorithm = DeadReckoningAlgorithm::from(dead_reckoning_algorithm);
    let (input, _padding_8) = be_u8(input)?;
    let (input, _padding_16) = be_u16(input)?;
    let (input, linear_acceleration) = vec3_f32(input)?;

    let body = Designator::builder()
        .with_designating_entity_id(designating_entity_id)
        .with_system_name(system_name)
        .with_designated_entity_id(designated_entity_id)
        .with_code(code)
        .with_power(power)
        .with_wavelength(wavelength)
        .with_spot_wrt_designated_entity(spot_wrt_designated_entity)
        .with_spot_location(spot_location)
        .with_dead_reckoning_algorithm(dead_reckoning_algorithm)
        .with_linear_acceleration(linear_acceleration)
        .build();

    Ok((input, body.into_pdu_body()))
}
