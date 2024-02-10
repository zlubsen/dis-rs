use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::remove_entity_r::model::RemoveEntityR;

impl SerializePdu for RemoveEntityR {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        buf.put_u8(self.required_reliability_service.into());
        buf.put_u8(0u8);
        buf.put_u16(0u16);
        buf.put_u32(self.request_id);

        originating_id_bytes + receiving_id_bytes + 8
    }
}