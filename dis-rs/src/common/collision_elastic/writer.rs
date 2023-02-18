use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::common::collision_elastic::model::CollisionElastic;

impl SerializePdu for CollisionElastic {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let issuing_id_bytes = self.issuing_entity_id.serialize(buf);
        let colliding_id_bytes = self.colliding_entity_id.serialize(buf);
        let event_id_bytes = self.event_id.serialize(buf);
        buf.put_u16(0u16);
        let velocity_bytes = self.velocity.serialize(buf);
        buf.put_f32(self.mass);
        let location_bytes = self.location.serialize(buf);
        buf.put_f32(self.intermediate_result_xx);
        buf.put_f32(self.intermediate_result_xy);
        buf.put_f32(self.intermediate_result_xz);
        buf.put_f32(self.intermediate_result_yy);
        buf.put_f32(self.intermediate_result_yz);
        buf.put_f32(self.intermediate_result_zz);
        let unit_surface_normal_bytes = self.unit_surface_normal.serialize(buf);
        buf.put_f32(self.coefficient_of_restitution);

        issuing_id_bytes + colliding_id_bytes + event_id_bytes
            + 2 + velocity_bytes
            + 4 + location_bytes
            + (6 * 4)
            + unit_surface_normal_bytes + 4
    }
}