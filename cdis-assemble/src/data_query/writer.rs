use dis_rs::model::TimeStamp;
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
use crate::constants::{THIRTY_TWO_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::data_query::model::DataQuery;
use crate::types::model::UVINT8;
use crate::writing::{SerializeCdis, write_value_unsigned};

impl SerializeCdisPdu for DataQuery {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, TWO_BITS, self.fields_present_field());

        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);
        let cursor = self.request_id.serialize(buf, cursor);
        let time_interval = TimeStamp::from(self.time_interval.clone());
        let cursor = write_value_unsigned::<u32>(buf, cursor, TWENTY_SIX_BITS, time_interval.raw_timestamp);

        let cursor = if !self.fixed_datum_ids.is_empty() {
            UVINT8::from(self.fixed_datum_ids.len() as u8).serialize(buf, cursor)
        } else { cursor };
        let cursor = if !self.variable_datum_ids.is_empty() {
            UVINT8::from(self.variable_datum_ids.len() as u8).serialize(buf, cursor)
        } else { cursor };

        let cursor = self.fixed_datum_ids.iter()
            .fold(cursor, |cursor, id| write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, u32::from(*id)) );
        let cursor = self.variable_datum_ids.iter()
            .fold(cursor, |cursor, id| write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, u32::from(*id)) );

        cursor
    }
}