use bytes::{BufMut, BytesMut};
use crate::dis::common::Serialize;
use crate::dis::v7::other::model::Other;

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