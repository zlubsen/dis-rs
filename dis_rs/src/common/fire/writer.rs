use bytes::{BufMut, BytesMut};
use crate::common::fire::model::Fire;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for Fire {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let firing_entity_id_bytes = self.firing_entity_id.serialize(buf);
        let target_entity_id_bytes = self.target_entity_id.serialize(buf);
        let munition_id_bytes = self.munition_id.serialize(buf);
        let event_id_bytes = self.event_id.serialize(buf);
        buf.put_u32(self.fire_mission_index);
        let location_in_world_bytes = self.location_in_world.serialize(buf);
        let descriptor_bytes = self.descriptor.serialize(buf);
        let velocity_bytes = self.velocity.serialize(buf);
        buf.put_f32(self.range);

        firing_entity_id_bytes + target_entity_id_bytes + munition_id_bytes
            + event_id_bytes + 4 + location_in_world_bytes
            + descriptor_bytes + velocity_bytes + 4
    }
}