use bytes::{BufMut, BytesMut};
use crate::common::detonation::model::Detonation;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for Detonation {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let source_entity_id_bytes = self.source_entity_id.serialize(buf);
        let target_entity_id_bytes = self.target_entity_id.serialize(buf);
        let exploding_entity_id_bytes = self.exploding_entity_id.serialize(buf);
        let event_id_bytes = self.event_id.serialize(buf);
        let velocity_bytes = self.velocity.serialize(buf);
        let world_location_bytes = self.location_in_world_coordinates.serialize(buf);
        let descriptor_bytes = self.descriptor.serialize(buf);
        let entity_location_bytes = self.location_in_entity_coordinates.serialize(buf);
        buf.put_u8(self.detonation_result.into());
        buf.put_u8(self.variable_parameters.len() as u8);
        buf.put_u16(0u16);
        let variable_params_bytes : u16 = self.variable_parameters.iter()
            .map(|param| param.serialize(buf))
            .sum();

        source_entity_id_bytes + target_entity_id_bytes + exploding_entity_id_bytes
            + event_id_bytes + velocity_bytes + world_location_bytes + descriptor_bytes
            + entity_location_bytes + 4 + variable_params_bytes
    }
}