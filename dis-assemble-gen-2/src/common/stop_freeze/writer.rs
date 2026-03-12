use crate::common::stop_freeze::model::StopFreeze;
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for StopFreeze {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        let real_world_bytes = self.real_world_time.serialize(buf);
        buf.put_u8(self.reason.into());
        buf.put_u8(self.frozen_behavior.into());
        buf.put_u16(0u16);
        buf.put_u32(self.request_id);

        originating_id_bytes + receiving_id_bytes + real_world_bytes + 1 + 1 + 2 + 4
    }
}
