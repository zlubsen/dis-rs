use bytes::{BufMut, BytesMut};
use crate::common::Serialize;
use crate::common::other::model::Other;

impl Serialize for Other {
    /// Serializes the Other PDU into a buffer.
    /// Assumes there is enough free space in the buffer and relies on the buffer's
    /// behaviour for what happens if this is not the case (probably panics - BytesMut does)
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put(self.body.as_slice());
        self.body.len()
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::builder::PduHeaderBuilder;
    use crate::common::model::{PduType, ProtocolFamily, ProtocolVersion};
    use crate::common::other::builder::OtherBuilder;
    use crate::common::Serialize;
    use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;

    #[test]
    fn serialize_other_pdu() {
        let pdu_length = PDU_HEADER_LEN_BYTES + 3;
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::Ieee1278_1a_1998)
            .exercise_id(1)
            .pdu_type(PduType::OtherPdu)
            .protocol_family(ProtocolFamily::Other)
            .time_stamp(10)
            .pdu_length(pdu_length as u16)
            .build().expect("Should be Ok");
        let pdu = OtherBuilder::new()
            .body( vec![0x01, 0x02, 0x03] )
            .build_with_header(header).expect("Should be Ok");

        let mut buf = BytesMut::with_capacity(pdu_length);

        pdu.serialize(&mut buf);

        let expected : [u8;15] = [0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0f, 0x00, 0x00,
            0x01, 0x02, 0x03];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}