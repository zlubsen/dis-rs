use bytes::{BufMut, BytesMut};
use crate::common::set_data::model::SetData;
use crate::common::{Serialize, SerializePdu, SupportedVersion};

impl SerializePdu for SetData {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let originating_bytes = self.originating_id.serialize(buf);
        let receiving_bytes = self.receiving_id.serialize(buf);
        buf.put_u32(self.request_id);
        buf.put_u32(0u32);
        buf.put_u32(self.fixed_datum_records.len() as u32);
        buf.put_u32(self.variable_datum_records.len() as u32);
        let fixed_datum_bytes = self.fixed_datum_records.iter()
            .map(|datum| datum.serialize(buf)).sum::<u16>();
        let variable_datum_bytes = self.variable_datum_records.iter()
            .map(|datum| datum.serialize(buf)).sum::<u16>();

        originating_bytes + receiving_bytes + 16 + fixed_datum_bytes + variable_datum_bytes
    }
}