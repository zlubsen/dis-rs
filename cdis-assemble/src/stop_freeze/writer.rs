use crate::constants::{FOUR_BITS, ONE_BIT};
use crate::stop_freeze::model::StopFreeze;
use crate::types::writer::serialize_clock_time;
use crate::writing::{write_value_unsigned, SerializeCdis};
use crate::{BitBuffer, SerializeCdisPdu};
use dis_rs::enumerations::StopFreezeFrozenBehavior;

impl SerializeCdisPdu for StopFreeze {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.originating_id.serialize(buf, cursor);
        let cursor = self.receiving_id.serialize(buf, cursor);
        let cursor = serialize_clock_time(buf, cursor, self.real_world_time);

        let cursor = write_value_unsigned::<u8>(buf, cursor, FOUR_BITS, self.reason.into());
        let cursor = serialize_frozen_behavior(buf, cursor, self.frozen_behavior);

        let cursor = self.request_id.serialize(buf, cursor);

        cursor
    }
}

/// Writes the `StopFreezeFrozenBehavior` bit field to the `BitBuffer`
#[allow(clippy::let_and_return)]
fn serialize_frozen_behavior(
    buf: &mut BitBuffer,
    cursor: usize,
    frozen_behavior: StopFreezeFrozenBehavior,
) -> usize {
    // C-DIS v1.0 spec states 2-bit field; StopFreezeFrozenBehavior expects 3 bit flags.
    // Decision here is to use 3 bits for StopFreezeFrozenBehavior
    let cursor = write_value_unsigned::<u8>(
        buf,
        cursor,
        ONE_BIT,
        frozen_behavior.run_simulation_clock.into(),
    );
    let cursor = write_value_unsigned::<u8>(
        buf,
        cursor,
        ONE_BIT,
        frozen_behavior.transmit_updates.into(),
    );
    let cursor =
        write_value_unsigned::<u8>(buf, cursor, ONE_BIT, frozen_behavior.process_updates.into());

    cursor
}
