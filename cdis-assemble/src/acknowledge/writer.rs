use crate::acknowledge::model::Acknowledge;
use crate::constants::{THREE_BITS, TWO_BITS};
use crate::writing::{BitBuffer, SerializeCdis, write_integer_bits};

impl SerializeCdis for Acknowledge {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);

        let cursor = write_integer_bits(buf, cursor, THREE_BITS, u16::from(self.acknowledge_flag));
        let cursor = write_integer_bits(buf, cursor, TWO_BITS, u16::from(self.response_flag));

        let cursor = self.request_id.serialize(buf, cursor);

        cursor
    }
}
