use crate::common::collision_elastic::model::CollisionElastic;
use crate::common::model::PduBody;
use crate::common::parser::{entity_id, event_id, vec3_f32};
use crate::BodyRaw;
use nom::number::complete::{be_f32, be_u16};
use nom::IResult;

#[allow(clippy::similar_names)]
pub(crate) fn collision_elastic_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, issuing_entity_id) = entity_id(input)?;
    let (input, colliding_entity_id) = entity_id(input)?;
    let (input, event_id) = event_id(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, velocity) = vec3_f32(input)?;
    let (input, mass) = be_f32(input)?;
    let (input, location) = vec3_f32(input)?;
    let (input, intermediate_result_xx) = be_f32(input)?;
    let (input, intermediate_result_xy) = be_f32(input)?;
    let (input, intermediate_result_xz) = be_f32(input)?;
    let (input, intermediate_result_yy) = be_f32(input)?;
    let (input, intermediate_result_yz) = be_f32(input)?;
    let (input, intermediate_result_zz) = be_f32(input)?;
    let (input, unit_surface_normal) = vec3_f32(input)?;
    let (input, coefficient_of_restitution) = be_f32(input)?;

    let body = CollisionElastic::builder()
        .with_issuing_entity_id(issuing_entity_id)
        .with_colliding_entity_id(colliding_entity_id)
        .with_event_id(event_id)
        .with_velocity(velocity)
        .with_mass(mass)
        .with_location(location)
        .with_intermediate_result_xx(intermediate_result_xx)
        .with_intermediate_result_xy(intermediate_result_xy)
        .with_intermediate_result_xz(intermediate_result_xz)
        .with_intermediate_result_yy(intermediate_result_yy)
        .with_intermediate_result_yz(intermediate_result_yz)
        .with_intermediate_result_zz(intermediate_result_zz)
        .with_unit_surface_normal(unit_surface_normal)
        .with_coefficient_of_restitution(coefficient_of_restitution)
        .build();

    Ok((input, body.into_pdu_body()))
}
