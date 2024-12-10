use crate::common::create_entity::model::CreateEntity;
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for CreateEntity {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id);

        originating_id_bytes + receiving_id_bytes + 4
    }
}
