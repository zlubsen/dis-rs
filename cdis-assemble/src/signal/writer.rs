use crate::constants::{EIGHT_BITS, FOURTEEN_BITS};
use crate::signal::model::Signal;
use crate::writing::{serialize_when_present, write_value_unsigned, SerializeCdis};
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};

impl SerializeCdisPdu for Signal {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();

        let cursor =
            write_value_unsigned(buf, cursor, self.fields_present_length(), fields_present);

        let cursor = self.radio_reference_id.serialize(buf, cursor);
        let cursor = self.radio_number.serialize(buf, cursor);

        let cursor = self.encoding_scheme.serialize(buf, cursor);
        let tdl_type: u16 = self.tdl_type.into();
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, tdl_type);

        let cursor = serialize_when_present(&self.sample_rate, buf, cursor);
        let cursor = write_value_unsigned(buf, cursor, FOURTEEN_BITS, self.data.len() * EIGHT_BITS);
        let cursor = serialize_when_present(&self.samples, buf, cursor);

        let cursor = self.data.iter().fold(cursor, |cursor, &byte| {
            write_value_unsigned(buf, cursor, EIGHT_BITS, byte)
        });

        cursor
    }
}
