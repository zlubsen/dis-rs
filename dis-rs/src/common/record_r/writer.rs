use bytes::{BufMut, BytesMut};
use crate::record_r::model::{RecordR, RecordSet, RecordSpecification};
use crate::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::{EIGHT_OCTETS, FOUR_OCTETS, ONE_BYTE_IN_BITS};
use crate::model::length_padded_to_num;

impl SerializePdu for RecordR {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let origination_id_bytes = self.originating_id.serialize(buf);
        let receiving_id_bytes = self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id.into());
        buf.put_u8(self.required_reliability_service.into());
        buf.put_u8(0u8);
        buf.put_u32(self.event_type.into());
        buf.put_u32(self.response_serial_number);
        let record_specification_bytes = self.record_specification.serialize(buf);

        origination_id_bytes + receiving_id_bytes + 14 + record_specification_bytes
    }
}

impl Serialize for RecordSpecification {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u32(self.record_sets.len() as u32);
        let record_sets_bytes : u16 = self.record_sets.iter()
            .map(|record_set| record_set.serialize(buf) )
            .sum();

        FOUR_OCTETS as u16 + record_sets_bytes
    }
}

impl Serialize for RecordSet {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u32(self.record_id.into());
        buf.put_u32(self.record_serial_number);
        buf.put_u32(0u32);

        buf.put_u16(self.record_length_bytes * ONE_BYTE_IN_BITS as u16); // record length in bits
        buf.put_u16(self.records.len() as u16); // record count
        let records_bytes = self.records.iter()
            .map(|record| { buf.put(record.as_slice()); record.len() })
            .sum::<usize>() as u16;
        let padded_record = length_padded_to_num(records_bytes as usize, EIGHT_OCTETS);
        buf.put_bytes(0u8, padded_record.padding_length);

        16 + records_bytes + padded_record.padding_length as u16
    }
}