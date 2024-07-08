use nom::complete::take;
use nom::IResult;
use dis_rs::enumerations::CollisionType;
use crate::{BodyProperties, CdisBody};
use crate::collision::model::{Collision, CollisionUnits};
use crate::constants::{ONE_BIT, TWO_BITS};
use crate::parsing::BitInput;
use crate::records::parser::{entity_coordinate_vector, entity_identification, linear_velocity};
use crate::types::parser::uvint32;

pub(crate) fn collision_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, units) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = CollisionUnits::from(units);

    let (input, issuing_entity_id) = entity_identification(input)?;
    let (input, colliding_entity_id) = entity_identification(input)?;
    let (input, event_id) = entity_identification(input)?;

    let (input, collision_type) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let collision_type = CollisionType::from(collision_type);

    let (input, velocity) = linear_velocity(input)?;
    let (input, mass) = uvint32(input)?;
    let (input, location) = entity_coordinate_vector(input)?;

    Ok((input, Collision {
        units,
        issuing_entity_id,
        colliding_entity_id,
        event_id,
        collision_type,
        velocity,
        mass,
        location,
    }.into_cdis_body()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn parse_collision() {
        todo!()
    }
}