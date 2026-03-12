use crate::set_record_r::model::SetRecordR;
use crate::{Serialize, SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

impl SerializePdu for SetRecordR {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let origination_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u8(self.required_reliability_service.into());
        buf.put_u8(0u8);
        buf.put_u16(0u16);
        buf.put_u32(0u32);
        let record_specification_bytes = self.record_specification.serialize(buf);

        origination_id_bytes + receiving_id_bytes + 12 + record_specification_bytes
    }
}
