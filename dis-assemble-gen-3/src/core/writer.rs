use bytes::BytesMut;

#[allow(
    unused,
    reason = "Used by generated code, lints and the compiler don't see the usage."
)]
pub trait Serialize {
    fn serialize(&self, buf: &mut BytesMut) -> u16;
}
