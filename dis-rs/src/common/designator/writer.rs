use bytes::{BufMut, BytesMut};
use crate::common::designator::model::Designator;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for Designator {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let designating_id_bytes = self.designating_entity_id.serialize(buf);
        buf.put_u16(self.system_name.into());
        let designated_id_bytes = self.designated_entity_id.serialize(buf);
        buf.put_u16(self.code.into());
        buf.put_f32(self.power);
        buf.put_f32(self.wavelength);
        let spot_wrt_bytes = self.spot_wrt_designated_entity.serialize(buf);
        let spot_location_bytes = self.spot_location.serialize(buf);
        buf.put_u8(self.dead_reckoning_algorithm.into());
        buf.put_u8(0u8);
        buf.put_u16(0u16);
        let linear_acceleration_bytes = self.linear_acceleration.serialize(buf);

        designating_id_bytes + 2
            + designated_id_bytes + 10
            + spot_wrt_bytes + spot_location_bytes
            + 4 + linear_acceleration_bytes
    }
}