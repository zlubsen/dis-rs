use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::set_data_r::model::SetDataR;
use bytes::{BufMut, BytesMut};

impl SerializePdu for SetDataR {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_bytes = self.originating_id.serialize(buf);
        let receiving_bytes = self.receiving_id.serialize(buf);
        buf.put_u8(self.required_reliability_service.into());
        buf.put_u8(0u8);
        buf.put_u16(0u16);
        buf.put_u32(self.request_id);
        buf.put_u32(self.fixed_datum_records.len() as u32);
        buf.put_u32(self.variable_datum_records.len() as u32);
        let fixed_datum_bytes = self
            .fixed_datum_records
            .iter()
            .map(|datum| datum.serialize(buf))
            .sum::<u16>();
        let variable_datum_bytes = self
            .variable_datum_records
            .iter()
            .map(|datum| datum.serialize(buf))
            .sum::<u16>();

        originating_bytes + receiving_bytes + 16 + fixed_datum_bytes + variable_datum_bytes
    }
}
