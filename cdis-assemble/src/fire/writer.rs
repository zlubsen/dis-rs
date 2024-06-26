use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
use crate::constants::ONE_BIT;
use crate::fire::model::Fire;
use crate::writing::{serialize_when_present, SerializeCdis, write_value_unsigned};

impl SerializeCdisPdu for Fire {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();

        let cursor = write_value_unsigned(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.into());

        let cursor = self.firing_entity_id.serialize(buf, cursor);
        let cursor = self.target_entity_id.serialize(buf, cursor);
        let cursor = self.munition_expandable_entity_id.serialize(buf, cursor);
        let cursor = self.event_id.serialize(buf, cursor);

        let cursor = serialize_when_present(&self.fire_mission_index, buf, cursor);
        let cursor = self.location_world_coordinates.serialize(buf, cursor);
        let cursor = self.descriptor_entity_type.serialize(buf, cursor);

        let cursor = serialize_when_present(&self.descriptor_warhead, buf, cursor);
        let cursor = serialize_when_present(&self.descriptor_fuze, buf, cursor);

        let cursor = serialize_when_present(&self.descriptor_quantity, buf, cursor);
        let cursor = serialize_when_present(&self.descriptor_rate, buf, cursor);

        let cursor = self.velocity.serialize(buf, cursor);

        let cursor = serialize_when_present(&self.range, buf, cursor);

        cursor
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::BitArray;
    use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
    use crate::fire::model::Fire;
    use crate::records::model::{EntityId, EntityType, LinearVelocity, Units, WorldCoordinates};
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT8};

    #[test]
    fn serialize_fire_no_fields_present() {
        let cdis_body = Fire {
            units: Units::Dekameter,
            firing_entity_id: EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(1)),
            target_entity_id: EntityId::new(UVINT16::from(2), UVINT16::from(2), UVINT16::from(2)),
            munition_expandable_entity_id: EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(2)),
            event_id: EntityId::new(UVINT16::from(1), UVINT16::from(1), UVINT16::from(3)),
            fire_mission_index: None,
            location_world_coordinates: WorldCoordinates::new(0.0, 0.0, SVINT24::from(1)),
            descriptor_entity_type: EntityType::new(2, 2, 0,
                UVINT8::from(0), UVINT8::from(0), UVINT8::from(0), UVINT8::from(0)),
            descriptor_warhead: None,
            descriptor_fuze: None,
            descriptor_quantity: None,
            descriptor_rate: None,
            velocity: LinearVelocity::new(SVINT16::from(1), SVINT16::from(1), SVINT16::from(1)),
            range: None,
        }.into_cdis_body();

        let mut buf: BitBuffer = BitArray::ZERO;
        let cursor = cdis_body.serialize(&mut buf, 0);

        assert_eq!(cursor, cdis_body.body_length());

        let expected = [0b0000_1_000, 0b0000001_0, 0b00000000, 0b1_0000000, 0b001_00000, 0b00010_000, 0b0000010_0, 0b00000001, 0b0_0000000, 0b001_00000, 0b00001_000, 0b0000010_0, 0b00000000, 0b1_0000000, 0b001_00000, 0b00011_000, 0b00000000, 0b00000000, 0b00000000, 0b0000_0000, 0b00000000, 0b00000000, 0b00000000, 0b0000_0000, 0b00000000, 0b000001_00, 0b10_0010_00, 0b0000000_0, 0b0000_0000, 0b0_00000_00, 0b000_00000, 0b00001_000, 0b0000001_0, 0b00000000, 0b1_0000000];
        //                         ^ fl ^u^ entityid                                       ^ entityid                                       ^ entityid                                   ^ eventid                                        ^ location                                    ^31                                              ^32                          ^ entity_type                                                   ^ velocity                                       ^ remainder
        //                      flags 4; units 1; entity/event ids 12x ten bits; location: 31 + 32 + 18; entity_type 4 + 4 + 9 + 5 + 5 + 5 + 5; (no descriptor 16 + 16 + 8 + 8); velocity 10 + 10 + 10; (no range)
        //                      0       ; 1     ; 1, 1, 1, 2, 2, 2, 1, 1, 2, 1, 1, 3 ; 1, 1, 1               ; 2, 2, 0, 0, 0, 0, 0               ;                                   ; 1, 1, 1           ;
        assert_eq!(buf.data[..35], expected);
    }
}
