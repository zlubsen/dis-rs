use crate::core::model::Serialize;
use crate::other_pdu::model::Other;
use bytes::{BufMut, BytesMut};

impl Serialize for Other {
    #[expect(
        clippy::cast_possible_truncation,
        reason = "MTU of PDUs and Records is well within u16::MAX"
    )]
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put(self.body.as_slice());
        self.body.len() as u16
    }
}
