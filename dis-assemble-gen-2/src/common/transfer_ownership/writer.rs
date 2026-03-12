use crate::transfer_ownership::model::TransferOwnership;
use crate::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for TransferOwnership {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_bytes = self.originating_id.serialize(buf);
        let receiving_bytes = self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.required_reliability_service.into());
        buf.put_u8(self.transfer_type.into());
        let transfer_bytes = self.transfer_entity_id.serialize(buf);
        let record_spec_bytes = self.record_specification.serialize(buf);

        originating_bytes + receiving_bytes + 6 + transfer_bytes + record_spec_bytes
    }
}
