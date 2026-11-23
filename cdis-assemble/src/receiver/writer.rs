use crate::BitBuffer;
use crate::constants::{NINE_BITS, TWO_BITS};
use crate::receiver::model::Receiver;
use crate::writing::{SerializeCdis, write_integer_bits};

impl SerializeCdis for Receiver {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.radio_reference_id.serialize(buf, cursor);
        let cursor = self.radio_number.serialize(buf, cursor);

        let cursor = write_integer_bits(buf, cursor, TWO_BITS, u16::from(self.receiver_state));
        let cursor = write_integer_bits(buf, cursor, NINE_BITS, self.received_power);

        let cursor = self.transmitter_radio_reference_id.serialize(buf, cursor);
        let cursor = self.transmitter_radio_number.serialize(buf, cursor);

        cursor
    }
}
