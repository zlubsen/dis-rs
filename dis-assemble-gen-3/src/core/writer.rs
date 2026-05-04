use crate::Pdu;
use bytes::BytesMut;

#[allow(
    unused,
    reason = "Used by generated code, lints and the compiler don't see the usage."
)]
pub trait Serialize {
    fn serialize(&self, buf: &mut BytesMut) -> u16;
}

impl Serialize for Pdu {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.header.serialize(buf);
        self.body.serialize(buf);

        self.pdu_length()
    }
}
