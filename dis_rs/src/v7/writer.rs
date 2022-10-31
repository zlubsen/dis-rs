use bytes::{BufMut, BytesMut};
use crate::common::Serialize;
use crate::v7::model::PduStatus;

impl Serialize for PduStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let tei : u8 = if let Some(tei) = self.transferred_entity_indicator {
            u8::from(tei)
        } else {0u8};
        let lvc : u8 = if let Some(lvc) = self.transferred_entity_indicator {
            u8::from(lvc) << 1
        } else {0u8};
        let cei : u8 = if let Some(cei) = self.transferred_entity_indicator {
            u8::from(cei) << 3
        } else {0u8};
        let fti : u8 = if let Some(fti) = self.transferred_entity_indicator {
            u8::from(fti) << 4
        } else {0u8};
        let dti : u8 = if let Some(dti) = self.transferred_entity_indicator {
            u8::from(dti) << 4
        } else {0u8};
        let rai : u8 = if let Some(rai) = self.transferred_entity_indicator {
            u8::from(rai) << 4
        } else {0u8};
        let iai : u8 = if let Some(iai) = self.transferred_entity_indicator {
            u8::from(iai) << 4
        } else {0u8};
        let ism : u8 = if let Some(ism) = self.transferred_entity_indicator {
            u8::from(ism) << 4
        } else {0u8};
        let aii : u8 = if let Some(aii) = self.transferred_entity_indicator {
            u8::from(aii) << 5
        } else {0u8};

        let status_field_byte = tei | lvc | cei | fti | dti | rai | iai | ism | aii;
        buf.put_u8(status_field_byte);
        1
    }
}