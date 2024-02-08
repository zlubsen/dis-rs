use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::repair_complete::model::RepairComplete;

impl SerializePdu for RepairComplete {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        let repairing_id_bytes = self.repairing_id.serialize(buf);
        buf.put_u16(self.repair.into());
        buf.put_u16(0u16);

        receiving_id_bytes + repairing_id_bytes + 4
    }
}