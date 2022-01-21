use bytes::{BufMut, BytesMut};
use crate::dis::common::Serialize;
use crate::dis::v6::other::model::Other;

impl Serialize for Other {
    /// Serializes the Other PDU into a buffer.
    /// Assumes there is enough free space in the buffer and relies on the buffer's
    /// behaviour for what happens if this is not the case (probably panics - BytesMut does)
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        self.header.serialize(buf);
        buf.put(self.body.as_slice());
        self.body.len()
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::dis::common::model::ProtocolVersion;
    use crate::dis::common::Serialize;
    use crate::dis::v6::builder::PduHeaderBuilder;
    use crate::dis::v6::model::{PDU_HEADER_LEN_BYTES, PduType, ProtocolFamily};
    use crate::dis::v6::other::builder::OtherBuilder;

    #[test]
    fn serialize_other_pdu() {
        let pdu_length = PDU_HEADER_LEN_BYTES + 3;
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::Ieee1278_1a1998)
            .exercise_id(1)
            .pdu_type(PduType::OtherPdu)
            .protocol_family(ProtocolFamily::Other)
            .time_stamp(10)
            .pdu_length(pdu_length as u16)
            .build().expect("Should be Ok");
        let pdu = OtherBuilder::new()
            .body( vec![0x01, 0x02, 0x03] )
            .header(header)
            .build().expect("Should be Ok");

        let mut buf = BytesMut::with_capacity(pdu_length);

        pdu.serialize(&mut buf);

        let expected : [u8;15] = [0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x63, 0x00, 0x00,
            0x01, 0x02, 0x03];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}