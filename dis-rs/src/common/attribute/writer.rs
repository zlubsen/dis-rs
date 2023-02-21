use bytes::{BufMut, BytesMut};
use crate::common::attribute::model::{Attribute, AttributeRecord, AttributeRecordSet, BASE_ATTRIBUTE_RECORD_LENGTH_OCTETS};
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for Attribute {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let sim_address_bytes = self.originating_simulation_address.serialize(buf);
        buf.put_u32(0u32);
        buf.put_u16(0u16);
        buf.put_u8(self.record_pdu_type.into());
        buf.put_u8(self.record_protocol_version.into());
        buf.put_u32(self.master_attribute_record_type.into());
        buf.put_u8(self.action_code.into());
        buf.put_u8(0u8);
        buf.put_u16(self.attribute_record_sets.len() as u16);
        let record_sets_bytes = self.attribute_record_sets.iter()
            .map(|record_set| record_set.serialize(buf))
            .sum::<u16>();

        sim_address_bytes + 16 + record_sets_bytes
    }
}

impl Serialize for AttributeRecordSet {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let entity_id_bytes = self.entity_id.serialize(buf);
        buf.put_u16(self.attribute_records.len() as u16);
        let records_bytes = self.attribute_records.iter()
            .map(|record| record.serialize(buf)).sum::<u16>();

        entity_id_bytes + 2 + records_bytes
    }
}

impl Serialize for AttributeRecord {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        const EIGHT_OCTETS : u16 = 8;
        const NO_REMAINDER : u16 = 0;
        let fields_bytes = BASE_ATTRIBUTE_RECORD_LENGTH_OCTETS + self.specific_fields.len() as u16;
        let num_of_padding_bytes = fields_bytes % EIGHT_OCTETS;
        let record_bytes = fields_bytes + num_of_padding_bytes;
        assert_eq!(record_bytes % EIGHT_OCTETS, NO_REMAINDER,
                   "The length for the attribute record is not aligned to 8 octets. Record length is {record_bytes} octets.");

        buf.put_u32(self.record_type.into());
        buf.put_u16(record_bytes);
        buf.put(&*self.specific_fields);
        buf.put_bytes(0u8, num_of_padding_bytes.into());

        record_bytes
    }
}