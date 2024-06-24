use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
use crate::constants::ONE_BIT;
use crate::fire::model::Fire;
use crate::writing::{serialize_when_present, SerializeCdis, write_value_unsigned};

impl SerializeCdisPdu for Fire {
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
    use crate::records::model::Units;

    #[test]
    fn serialize_fire_no_fields_present() {
        let cdis_body = Fire {
            units: Units::Dekameter,
            firing_entity_id: Default::default(),
            target_entity_id: Default::default(),
            munition_expandable_entity_id: Default::default(),
            event_id: Default::default(),
            fire_mission_index: None,
            location_world_coordinates: Default::default(),
            descriptor_entity_type: Default::default(),
            descriptor_warhead: None,
            descriptor_fuze: None,
            descriptor_quantity: None,
            descriptor_rate: None,
            velocity: Default::default(),
            range: None,
        }.into_cdis_body();

        let mut buf: BitBuffer = BitArray::ZERO;
        let cursor = cdis_body.serialize(&mut buf, 0);

        assert_eq!(cursor, cdis_body.body_length());
        assert!(false);
        // assert_eq!(buf.data[..5], [0b1010_1110, 0b0001_1100, 0b0000_0101, 0b0000_0001, 0b0100_0000]);
    }
}
