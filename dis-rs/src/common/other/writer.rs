use crate::common::other::model::Other;
use crate::common::{SerializePdu, SupportedVersion};
use bytes::{BufMut, BytesMut};

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
    use crate::common::model::{Pdu, PduHeader};
    use crate::enumerations::PduType;
    use crate::other::model::Other;
    use bytes::BytesMut;

    #[test]
    fn serialize_other_pdu() {
        let header = PduHeader::new_v6(1, PduType::Other);
        let body = Other::builder()
            .with_body(vec![0x01, 0x02, 0x03])
            .build()
            .into_pdu_body();
        let pdu = Pdu::finalize_from_parts(header, body, 10);
        let pdu_length = pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        let wire_size = pdu.serialize(&mut buf).unwrap();
        assert_eq!(wire_size, pdu_length);

        let expected: [u8; 15] = [
            0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0f, 0x00, 0x00, 0x01, 0x02,
            0x03,
        ];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}
