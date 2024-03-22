use bytes::BytesMut;
use dis_rs::Serialize;
use crate::types::model::UVINT8;

impl Serialize for UVINT8 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}