use crate::BitBuffer;
use crate::constants::{NINE_BITS, TWO_BITS};
use crate::receiver::model::Receiver;
use crate::writing::{write_value_signed, write_value_unsigned, SerializeCdis};

impl SerializeCdis for Receiver {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.radio_reference_id.serialize(buf, cursor);
        let cursor = self.radio_number.serialize(buf, cursor);

        let receiver_state: u16 = self.receiver_state.into();
        let cursor = write_value_unsigned(buf, cursor, TWO_BITS, receiver_state);
        let cursor = write_value_signed(buf, cursor, NINE_BITS, self.received_power);

        let cursor = self.transmitter_radio_reference_id.serialize(buf, cursor);
        let cursor = self.transmitter_radio_number.serialize(buf, cursor);

        cursor
    }
}