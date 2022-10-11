use bytes::{BufMut, BytesMut};
use crate::common::Serialize;
use crate::common::other::model::Other;

impl Serialize for Other {
    /// Serializes the Other PDU into a buffer.
    /// Assumes there is enough free space in the buffer and relies on the buffer's
    /// behaviour for what happens if this is not the case (probably panics - BytesMut does)
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put(self.body.as_slice());
        self.body.len() as u16
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::other::builder::OtherBuilder;
    use crate::common::{Body, Serialize};
    use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::{PduType};
    use crate::{Pdu, PduHeader};

    #[test]
    fn serialize_other_pdu() {
        let pdu_length = PDU_HEADER_LEN_BYTES + 3;
        let mut header = PduHeader::v6_builder()
            .exercise_id(1)
            .pdu_type(PduType::Other)
            .build();
        let body = OtherBuilder::new()
            .body( vec![0x01, 0x02, 0x03] ).build().expect("Should be Ok");
        let pdu = Pdu::finalize_from_parts(header, body, 10);

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        let wire_size = pdu.serialize(&mut buf);
        assert_eq!(wire_size, pdu.body.body_length() + PDU_HEADER_LEN_BYTES);

        let expected : [u8;15] = [0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0f, 0x00, 0x00,
            0x01, 0x02, 0x03];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}