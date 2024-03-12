use bytes::{BufMut, BytesMut};
use crate::acknowledge_r::model::AcknowledgeR;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for AcknowledgeR {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        buf.put_u16(self.acknowledge_flag.into());
        buf.put_u16(self.response_flag.into());
        buf.put_u32(self.request_id);

        originating_id_bytes + receiving_id_bytes + 2 + 2 + 4
    }
}