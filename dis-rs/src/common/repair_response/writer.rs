use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::repair_response::model::RepairResponse;

impl SerializePdu for RepairResponse {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        let repairing_id_bytes = self.repairing_id.serialize(buf);
        buf.put_u8(self.repair_result.into());
        buf.put_u8(0u8);
        buf.put_u16(0u16);

        receiving_id_bytes + repairing_id_bytes + 4
    }
}