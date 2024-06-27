use crate::detonation::model::Detonation;
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
use crate::constants::{EIGHT_BITS, TWO_BITS};
use crate::writing::{serialize_when_present, SerializeCdis, write_value_unsigned};

impl SerializeCdisPdu for Detonation {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();

        let cursor = write_value_unsigned(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_value_unsigned::<u8>(buf, cursor, TWO_BITS, self.units.into());

        let cursor = self.source_entity_id.serialize(buf, cursor);
        let cursor = self.target_entity_id.serialize(buf, cursor);
        let cursor = self.exploding_entity_id.serialize(buf, cursor);
        let cursor = self.event_id.serialize(buf, cursor);

        let cursor = self.entity_linear_velocity.serialize(buf, cursor);
        let cursor = self.location_in_world_coordinates.serialize(buf, cursor);

        let cursor = self.descriptor_entity_type.serialize(buf, cursor),
        let cursor = serialize_when_present(&self.descriptor_warhead, buf, cursor);
        let cursor = serialize_when_present(&self.descriptor_fuze, buf, cursor);

        let cursor = serialize_when_present(&self.descriptor_quantity, buf, cursor);
        let cursor = serialize_when_present(&self.descriptor_rate, buf, cursor);

        let cursor = self.location_in_entity_coordinates.serialize(buf, cursor);
        let cursor = self.detonation_results.serialize(buf, cursor);

        let cursor = if !self.variable_parameters.is_empty() {
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.variable_parameters.len() as u8)
        } else { cursor };
        let cursor = self.variable_parameters.iter()
            .fold(cursor, |cursor, vp| vp.serialize(buf, cursor) );

        cursor
    }
}