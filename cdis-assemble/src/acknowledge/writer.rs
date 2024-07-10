use crate::{BitBuffer, SerializeCdisPdu};
use crate::acknowledge::model::Acknowledge;
use crate::constants::{THREE_BITS, TWO_BITS};
use crate::writing::{SerializeCdis, write_value_unsigned};

impl SerializeCdisPdu for Acknowledge {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);

        let cursor = write_value_unsigned::<u16>(buf, cursor, THREE_BITS, self.acknowledge_flag.into());
        let cursor = write_value_unsigned::<u16>(buf, cursor, TWO_BITS, self.response_flag.into());

        let cursor = self.request_id.serialize(buf, cursor);

        cursor
    }
}