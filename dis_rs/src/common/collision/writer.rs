use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::common::collision::model::Collision;

impl SerializePdu for Collision {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let issuing_id_bytes = self.issuing_entity_id.serialize(buf);
        let colliding_id_bytes = self.colliding_entity_id.serialize(buf);
        let event_id_bytes = self.event_id.serialize(buf);
        buf.put_u8(self.collision_type.into());
        buf.put_u8(0u8);
        let velocity_bytes = self.velocity.serialize(buf);
        buf.put_f32(self.mass);
        let location_bytes = self.location.serialize(buf);

        issuing_id_bytes + colliding_id_bytes + event_id_bytes
            + 2 + velocity_bytes
            + 4 + location_bytes
    }
}