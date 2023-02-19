use bytes::{BufMut, BytesMut};
use crate::common::remove_entity::model::RemoveEntity;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for RemoveEntity {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id);

        originating_id_bytes + receiving_id_bytes + 4
    }
}