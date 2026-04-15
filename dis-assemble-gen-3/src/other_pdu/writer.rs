use crate::core::writer::Serialize;
use crate::model::Other;
use bytes::{BufMut, BytesMut};

impl Serialize for Other {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put(self.body.as_slice());
        self.body.len() as u16
    }
}
