use bytes::BytesMut;

pub trait Serialize {
    fn serialize(&self, buf: &mut BytesMut) -> u16;
}
