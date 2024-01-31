use nom::IResult;
use nom::number::complete::{be_f32, be_u8};
use crate::common::collision::model::Collision;
use crate::common::parser::{entity_id, event_id, vec3_f32};
use crate::enumerations::CollisionType;
use crate::common::model::PduBody;

pub fn collision_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, issuing_entity_id) = entity_id(input)?;
    let (input, colliding_entity_id) = entity_id(input)?;
    let (input, event_id) = event_id(input)?;
    let (input, collision_type) = be_u8(input)?;
    let collision_type = CollisionType::from(collision_type);
    let (input, _padding) = be_u8(input)?;
    let (input, velocity) = vec3_f32(input)?;
    let (input, mass) = be_f32(input)?;
    let (input, location) = vec3_f32(input)?;

    let body = Collision::new()
        .with_issuing_entity_id(issuing_entity_id)
        .with_colliding_entity_id(colliding_entity_id)
        .with_event_id(event_id)
        .with_collision_type(collision_type)
        .with_velocity(velocity)
        .with_mass(mass)
        .with_location(location);

    Ok((input, body.into_pdu_body()))
}