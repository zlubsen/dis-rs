use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::resupply_cancel::model::ResupplyCancel;
use bytes::BytesMut;

impl SerializePdu for ResupplyCancel {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let requesting_id_bytes = self.requesting_id.serialize(buf);
        let servicing_id_bytes = self.servicing_id.serialize(buf);

        requesting_id_bytes + servicing_id_bytes
    }
}
