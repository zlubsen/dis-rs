use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::stop_freeze_r::model::StopFreezeR;

impl SerializePdu for StopFreezeR {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        let real_world_bytes = self.real_world_time.serialize(buf);
        buf.put_u8(self.reason.into());
        buf.put_u8(self.frozen_behavior.into());
        buf.put_u8(self.required_reliability_service.into());
        buf.put_u8(0u8);
        buf.put_u32(self.request_id);

        originating_id_bytes + receiving_id_bytes + real_world_bytes + 8
    }
}