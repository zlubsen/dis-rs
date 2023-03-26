use bytes::{BufMut, BytesMut};
use crate::common::action_request::model::ActionRequest;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for ActionRequest {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_bytes = self.originating_id.serialize(buf);
        let receiving_bytes = self.receiving_id.serialize(buf);
        // buf.put_u16(self.acknowledge_flag.into());
        // buf.put_u16(self.response_flag.into());
        buf.put_u32(self.request_id);

        originating_bytes + receiving_bytes + 8
    }
}