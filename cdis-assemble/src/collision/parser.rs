use crate::collision::model::{Collision, CollisionUnits};
use crate::constants::{ONE_BIT, TWO_BITS};
use crate::parsing::BitInput;
use crate::records::parser::{entity_coordinate_vector, entity_identification, linear_velocity};
use crate::types::parser::uvint32;
use crate::{BodyProperties, CdisBody};
use dis_rs::enumerations::CollisionType;
use nom::complete::take;
use nom::IResult;

pub(crate) fn collision_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, units): (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = CollisionUnits::from(units);

    let (input, issuing_entity_id) = entity_identification(input)?;
    let (input, colliding_entity_id) = entity_identification(input)?;
    let (input, event_id) = entity_identification(input)?;

    let (input, collision_type): (BitInput, u8) = take(ONE_BIT)(input)?;
    let collision_type = CollisionType::from(collision_type);

    let (input, velocity) = linear_velocity(input)?;
    let (input, mass) = uvint32(input)?;
    let (input, location) = entity_coordinate_vector(input)?;

    Ok((
        input,
        Collision {
            units,
            issuing_entity_id,
            colliding_entity_id,
            event_id,
            collision_type,
            velocity,
            mass,
            location,
        }
        .into_cdis_body(),
    ))
}

#[cfg(test)]
mod tests {
    use crate::collision::parser::collision_body;
    use crate::records::model::{EntityId, UnitsMass, UnitsMeters};
    use crate::types::model::{SVINT16, UVINT16, UVINT32};
    use crate::CdisBody;
    use dis_rs::enumerations::CollisionType;

    #[test]
    fn parse_collision() {
        let input = [
            0b00_000000,
            0b0001_0000,
            0b000001_00,
            0b00000001,
            0b_00000000,
            0b10_000000,
            0b0010_0000,
            0b000010_00,
            0b00000001,
            0b_00000000,
            0b01_000000,
            0b0011_0_000,
            0b0000001_0,
            0b00000000,
            0b1_0000000,
            0b001_00011,
            0b00100_000,
            0b0000001_0,
            0b00000000,
            0b1_0000000,
            0b001_00000,
        ];
        // fields               ^u^ entityid                                       ^ entityid                                   ^ eventid                                        ^c^ velocity 1,1,1                                 ^ mass         ^ entity location                                ^ no remainder
        // bits                 ^2^ 3x 10                                          ^ 3x 10                                      ^ 3x 10                                          ^1^ 3x 10                                          ^ 10           ^ 3x 10                                          ^
        // values               ^0^ 1,1,1                                          ^ 2,2,2                                      ^ 1,1,3                                          ^0^ 1,1,1                                          ^ 100          ^ 1 1 1                                          ^

        let ((_input, cursor), body) = collision_body((&input, 0)).unwrap();

        assert_eq!(cursor, 3); // cursor position in last byte of input
        if let CdisBody::Collision(collision) = body {
            assert_eq!(collision.units.mass, UnitsMass::Grams);
            assert_eq!(
                collision.units.location_entity_coordinates,
                UnitsMeters::Centimeter
            );
            assert_eq!(
                collision.issuing_entity_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1))
            );
            assert_eq!(
                collision.colliding_entity_id,
                EntityId::new(UVINT16::from(2), UVINT16::from(2), UVINT16::from(2))
            );
            assert_eq!(
                collision.event_id,
                EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(3))
            );
            assert_eq!(collision.collision_type, CollisionType::Inelastic);
            assert_eq!(collision.location.x, SVINT16::from(1));
            assert_eq!(collision.location.y, SVINT16::from(1));
            assert_eq!(collision.location.z, SVINT16::from(1));
            assert_eq!(collision.mass, UVINT32::from(100));
        } else {
            assert!(false)
        }
    }
}
