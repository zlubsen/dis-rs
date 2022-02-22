use bytes::{BufMut, BytesMut};
use crate::dis::common::model::PDU_HEADER_LEN_BYTES;
use crate::dis::common::Serialize;
use crate::dis::v7::model::{PduHeader, PduStatus};

impl Serialize for PduHeader {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.protocol_version.into());
        buf.put_u8(self.exercise_id);
        buf.put_u8(self.pdu_type.into());
        buf.put_u8(self.protocol_family.into());
        buf.put_u32(self.time_stamp);
        buf.put_u16(self.pdu_length);
        self.pdu_status.serialize(buf);
        buf.put_u8(0u8);

        PDU_HEADER_LEN_BYTES
    }
}

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