use crate::constants::{FOUR_BITS, ONE_BIT};
use crate::parsing::BitInput;
use crate::records::parser::entity_identification;
use crate::stop_freeze::model::StopFreeze;
use crate::types::parser::{clock_time, uvint32};
use crate::{BodyProperties, CdisBody};
use dis_rs::enumerations::{StopFreezeFrozenBehavior, StopFreezeReason};
use nom::bits::complete::take;
use nom::IResult;

pub(crate) fn stop_freeze_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, originating_id) = entity_identification(input)?;
    let (input, receiving_id) = entity_identification(input)?;

    let (input, real_world_time) = clock_time(input)?;
    let (input, reason): (BitInput, u8) = take(FOUR_BITS)(input)?;
    let reason = StopFreezeReason::from(reason);

    let (input, frozen_behavior) = frozen_behavior(input)?;

    let (input, request_id) = uvint32(input)?;

    Ok((
        input,
        StopFreeze {
            originating_id,
            receiving_id,
            real_world_time,
            reason,
            frozen_behavior,
            request_id,
        }
        .into_cdis_body(),
    ))
}

/// Parse a `StopFreezeFrozenBehavior` record from a C-DIS bitstream
fn frozen_behavior(input: BitInput) -> IResult<BitInput, StopFreezeFrozenBehavior> {
    // C-DIS v1.0 spec states 2-bit field; StopFreezeFrozenBehavior expects 3 bit flags.
    // Decision here is to use 3 bits for StopFreezeFrozenBehavior
    let (input, run_simulation_clock): (BitInput, u8) = take(ONE_BIT)(input)?;
    let (input, transmit_updates): (BitInput, u8) = take(ONE_BIT)(input)?;
    let (input, process_updates): (BitInput, u8) = take(ONE_BIT)(input)?;

    Ok((
        input,
        StopFreezeFrozenBehavior {
            run_simulation_clock: run_simulation_clock != 0,
            transmit_updates: transmit_updates != 0,
            process_updates: process_updates != 0,
        },
    ))
}
