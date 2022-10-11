use bytes::{BufMut, BytesMut};
use crate::common::fire::model::Fire;
use crate::common::model::BurstDescriptor;
use crate::common::Serialize;

impl Serialize for Fire {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let firing_entity_id_bytes = self.firing_entity_id.serialize(buf);
        let target_entity_id_bytes = self.target_entity_id.serialize(buf);
        let munition_id_bytes = self.munition_id.serialize(buf);
        let event_id_bytes = self.event_id.serialize(buf);
        buf.put_u32(self.fire_mission_index);
        let location_in_world_bytes = self.location_in_world.serialize(buf);
        let burst_descriptor_bytes = self.burst_descriptor.serialize(buf);
        let velocity_bytes = self.velocity.serialize(buf);
        buf.put_f32(self.range);

        firing_entity_id_bytes + target_entity_id_bytes + munition_id_bytes
            + event_id_bytes + 4 + location_in_world_bytes
            + burst_descriptor_bytes + velocity_bytes + 4
    }
}

impl Serialize for BurstDescriptor {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let munition_bytes = self.munition.serialize(buf);
        buf.put_u16(self.warhead.into());
        buf.put_u16(self.fuse.into());
        buf.put_u16(self.quantity);
        buf.put_u16(self.rate);
        munition_bytes + 8
    }
}