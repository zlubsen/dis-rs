use bytes::{BufMut, BytesMut};
use crate::common::{SerializePdu, SupportedVersion};
use crate::common::other::model::Other;

impl SerializePdu for Other {
    /// Serializes the Other PDU into a buffer.
    /// Assumes there is enough free space in the buffer and relies on the buffer's
    /// behaviour for what happens if this is not the case (probably panics - BytesMut does)
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        buf.put(self.body.as_slice());
        self.body.len() as u16
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::other::builder::OtherBuilder;
    use crate::common::{BodyInfo, Serialize};
    use crate::constants::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::{PduType};
    use crate::common::model::{Pdu, PduHeader};

    #[test]
    fn serialize_other_pdu() {
        let body_length = 3;
        let header = PduHeader::new_v6(1, PduType::Other);
        let body = OtherBuilder::new()
            .body( vec![0x01, 0x02, 0x03] ).build().expect("Should be Ok");
        let pdu = Pdu::finalize_from_parts(header, body, 10);

        let mut buf = BytesMut::with_capacity(body_length as usize);

        let wire_size = pdu.serialize(&mut buf);
        assert_eq!(wire_size, pdu.body.body_length() + PDU_HEADER_LEN_BYTES);

        let expected : [u8;15] = [0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0f, 0x00, 0x00,
            0x01, 0x02, 0x03];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}