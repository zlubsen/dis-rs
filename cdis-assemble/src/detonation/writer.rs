use crate::constants::{EIGHT_BITS, ONE_BIT};
use crate::detonation::model::Detonation;
use crate::types::model::CdisFloat;
use crate::writing::{serialize_when_present, write_value_unsigned, SerializeCdis};
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};

impl SerializeCdisPdu for Detonation {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();

        let cursor =
            write_value_unsigned(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            ONE_BIT,
            self.units.world_location_altitude.into(),
        );
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            ONE_BIT,
            self.units.location_entity_coordinates.into(),
        );

        let cursor = self.source_entity_id.serialize(buf, cursor);
        let cursor = self.target_entity_id.serialize(buf, cursor);
        let cursor = self.exploding_entity_id.serialize(buf, cursor);
        let cursor = self.event_id.serialize(buf, cursor);

        let cursor = self.entity_linear_velocity.serialize(buf, cursor);
        let cursor = self.location_in_world_coordinates.serialize(buf, cursor);

        let cursor = self.descriptor_entity_type.serialize(buf, cursor);
        let cursor = serialize_when_present(&self.descriptor_warhead, buf, cursor);
        let cursor = serialize_when_present(&self.descriptor_fuze, buf, cursor);

        let cursor = serialize_when_present(&self.descriptor_quantity, buf, cursor);
        let cursor = serialize_when_present(&self.descriptor_rate, buf, cursor);

        let cursor = serialize_when_present(&self.descriptor_explosive_material, buf, cursor);
        let cursor = if let Some(explosive_force) = &self.descriptor_explosive_force {
            explosive_force.serialize(buf, cursor)
        } else {
            cursor
        };

        let cursor = self.location_in_entity_coordinates.serialize(buf, cursor);
        let cursor = self.detonation_results.serialize(buf, cursor);

        let cursor = if !self.variable_parameters.is_empty() {
            write_value_unsigned::<u8>(
                buf,
                cursor,
                EIGHT_BITS,
                self.variable_parameters.len() as u8,
            )
        } else {
            cursor
        };
        let cursor = self
            .variable_parameters
            .iter()
            .fold(cursor, |cursor, vp| vp.serialize(buf, cursor));

        cursor
    }
}

#[cfg(test)]
mod tests {
    use crate::detonation::model::{Detonation, DetonationUnits};
    use crate::records::model::{
        EntityCoordinateVector, EntityId, EntityType, LinearVelocity, UnitsDekameters, UnitsMeters,
        WorldCoordinates,
    };
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT8};
    use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
    use bitvec::prelude::BitArray;
    use dis_rs::enumerations::DetonationResult;

    #[test]
    fn serialize_detonation_no_fields_present() {
        let cdis_body = Detonation {
            units: DetonationUnits {
                world_location_altitude: UnitsDekameters::Dekameter,
                location_entity_coordinates: UnitsMeters::Centimeter,
            },
            source_entity_id: EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1)),
            target_entity_id: EntityId::new(UVINT16::from(2), UVINT16::from(2), UVINT16::from(2)),
            exploding_entity_id: EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(2),
            ),
            event_id: EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(3)),
            entity_linear_velocity: LinearVelocity::new(
                SVINT16::from(1),
                SVINT16::from(1),
                SVINT16::from(1),
            ),
            location_in_world_coordinates: WorldCoordinates {
                latitude: 0.0,
                longitude: 0.0,
                altitude_msl: SVINT24::from(1),
            },
            descriptor_entity_type: EntityType::new(
                2,
                2,
                0,
                UVINT8::from(0),
                UVINT8::from(0),
                UVINT8::from(0),
                UVINT8::from(0),
            ),
            descriptor_warhead: None,
            descriptor_fuze: None,
            descriptor_quantity: None,
            descriptor_rate: None,
            descriptor_explosive_material: None,
            descriptor_explosive_force: None,
            location_in_entity_coordinates: EntityCoordinateVector::new(
                SVINT16::from(0),
                SVINT16::from(0),
                SVINT16::from(0),
            ),
            detonation_results: UVINT8::from(u8::from(DetonationResult::Detonation)),
            variable_parameters: vec![],
        }
        .into_cdis_body();

        let mut buf: BitBuffer = BitArray::ZERO;
        let cursor = cdis_body.serialize(&mut buf, 0);

        assert_eq!(cursor, cdis_body.body_length());

        #[rustfmt::skip]
        #[allow(clippy::unusual_byte_groupings)]
        #[allow(clippy::unreadable_literal)]
        let expected: [u8; 39] = [
            0b0000_10_00,
            0b00000001_,
            0b00000000,
            0b01_000000,
            0b0001_0000,
            0b000010_00,
            0b00000010_,
            0b00000000,
            0b10_000000,
            0b0001_0000,
            0b000001_00,
            0b00000010_,
            0b00000000,
            0b01_000000,
            0b0001_0000,
            0b000011_00,
            0b00000001_,
            0b00000000,
            0b01_000000,
            0b0001_0000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b000_00000,
            0b00000000,
            0b00000000,
            0b00000000,
            0b000_00000,
            0b00000000,
            0b00001_001,
            0b0_0010_000,
            0b000000_00,
            0b000_00000_,
            0b00000_000,
            0b00_000000,
            0b0000_0000,
            0b000000_00,
            0b00000000_,
            0b00101_000,
        ];
        // fields                    ^fl ^u ^ entityid                                       ^ entityid                                       ^ entityid                                   ^ eventid                                        ^ velocity 1,1,1                                 ^ world location                                                                                                            ^ entity type                                                   ^ entity location                            ^ results ^ remainder
        // bits                      ^4  ^2 ^ 3x 10                                          ^ 3x 10                                          ^ 3x 10                                      ^ 3x 10                                          ^ 3x 10                                          ^ 31,32,18                                                                                                                  ^ 4,4,9,5,5,5,5                                                 ^ 3x 10                                      ^ 5       ^
        // values                    ^0  ^1 ^ 1,1,1                                          ^ 2,2,2                                          ^ 1,1,2                                      ^ 1,1,3                                          ^ 1,1,1                                          ^ 0 0 0                                                                                                                     ^ 2,2,0,0,0,0,0                                                 ^ 0 0 0                                      ^ 5       ^

        assert_eq!(buf.data[..39], expected);
    }
}
