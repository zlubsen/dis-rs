use crate::action_request::model::ActionRequest;
use crate::constants::TWO_BITS;
use crate::types::model::UVINT8;
use crate::writing::{write_value_unsigned, SerializeCdis};
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};

impl SerializeCdisPdu for ActionRequest {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, TWO_BITS, self.fields_present_field());

        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);
        let cursor = self.request_id.serialize(buf, cursor);
        let cursor = self.action_id.serialize(buf, cursor);

        let cursor = if !self.datum_specification.fixed_datum_records.is_empty() {
            UVINT8::from(self.datum_specification.fixed_datum_records.len() as u8)
                .serialize(buf, cursor)
        } else {
            cursor
        };
        let cursor = if !self.datum_specification.variable_datum_records.is_empty() {
            UVINT8::from(self.datum_specification.variable_datum_records.len() as u8)
                .serialize(buf, cursor)
        } else {
            cursor
        };

        let cursor = self
            .datum_specification
            .fixed_datum_records
            .iter()
            .fold(cursor, |cursor, vp| vp.serialize(buf, cursor));
        let cursor = self
            .datum_specification
            .variable_datum_records
            .iter()
            .fold(cursor, |cursor, vp| vp.serialize(buf, cursor));

        cursor
    }
}
