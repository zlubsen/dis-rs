use bytes::{BufMut, BytesMut};
use crate::common::Serialize;
use crate::v7::model::PduStatus;

impl Serialize for PduStatus {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let tei : u8 = if let Some(tei) = self.transferred_entity_indicator {
            tei as u8
        } else {0u8};
        let lvc : u8 = if let Some(lvc) = self.transferred_entity_indicator {
            (lvc as u8) << 1
        } else {0u8};
        let cei : u8 = if let Some(cei) = self.transferred_entity_indicator {
            (cei as u8) << 3
        } else {0u8};
        let fti : u8 = if let Some(fti) = self.transferred_entity_indicator {
            (fti as u8) << 4
        } else {0u8};
        let dti : u8 = if let Some(dti) = self.transferred_entity_indicator {
            (dti as u8) << 4
        } else {0u8};
        let rai : u8 = if let Some(rai) = self.transferred_entity_indicator {
            (rai as u8) << 4
        } else {0u8};
        let iai : u8 = if let Some(iai) = self.transferred_entity_indicator {
            (iai as u8) << 4
        } else {0u8};
        let ism : u8 = if let Some(ism) = self.transferred_entity_indicator {
            (ism as u8) << 4
        } else {0u8};
        let aii : u8 = if let Some(aii) = self.transferred_entity_indicator {
            (aii as u8) << 5
        } else {0u8};

        let status_field_byte = tei | lvc | cei | fti | dti | rai | iai | ism | aii;
        buf.put_u8(status_field_byte);
        1
    }
}