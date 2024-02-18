use bytes::{BufMut, BytesMut};
use crate::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::FOUR_OCTETS;
use crate::record_query_r::model::{RecordQueryR, RecordQuerySpecification};

impl SerializePdu for RecordQueryR {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let origination_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id.into());
        buf.put_u8(self.required_reliability_service.into());
        buf.put_u8(0u8);
        buf.put_u16(self.event_type.into());
        buf.put_u32(self.time.raw_timestamp);
        let record_query_specification = self.record_query_specification.serialize(buf);

        origination_id_bytes + receiving_id_bytes + 12 + record_query_specification
    }
}

impl Serialize for RecordQuerySpecification {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u32(self.record_ids.len() as u32);
        let record_bytes = self.record_ids.iter()
            .map(|record| {
                buf.put_u32(u32::from(*record));
                FOUR_OCTETS as u16
        }).sum::<u16>();

        FOUR_OCTETS as u16 + record_bytes
    }
}