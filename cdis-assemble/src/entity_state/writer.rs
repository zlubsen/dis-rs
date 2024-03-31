use crate::entity_state::model::EntityState;
use crate::{BitBuffer, BodyProperties, SerializeCdis, SerializeCdisPdu};
use crate::constants::{ONE_BIT};
use crate::parser_utils::write_value_with_length;
use crate::types::model::UVINT8;

impl SerializeCdisPdu for EntityState {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();
        let cursor = write_value_with_length(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_value_with_length(buf, cursor, ONE_BIT, u8::from(self.units));
        let cursor = write_value_with_length(buf, cursor, ONE_BIT, u8::from(self.full_update_flag));
        let cursor = self.entity_id.serialize(buf, cursor);
        let cursor = if let Some(force_id) = self.force_id { force_id.serialize(buf, cursor) } else { cursor };
        let cursor = if !self.variable_parameters.is_empty() {
            UVINT8::from(self.variable_parameters.len() as u8 ).serialize(buf, cursor)
        } else { cursor };
        let cursor = if let Some(entity_type) = self.entity_type { entity_type.serialize(buf, cursor) } else { cursor };
        let cursor = if let Some(entity_type) = self.alternate_entity_type { entity_type.serialize(buf, cursor) } else { cursor };

        let cursor = if let Some(velocity) = self.entity_linear_velocity { velocity.serialize(buf, cursor) } else { cursor };
        let cursor = if let Some(location) = self.entity_location { location.serialize(buf, cursor) } else { cursor };
        let cursor = if let Some(orientation) = self.entity_orientation { orientation.serialize(buf, cursor) } else { cursor };
        // TODO rest of the fields

        cursor
    }
}