use nom::IResult;
use nom::number::complete::{be_u16, be_u32, be_u8};
use crate::common::parser::{clock_time, entity_id};
use crate::common::stop_freeze::model::StopFreeze;
use crate::common::model::PduBody;
use crate::enumerations::{StopFreezeFrozenBehavior, StopFreezeReason};

pub fn stop_freeze_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, real_world_time) = clock_time(input)?;
    let (input, reason) = be_u8(input)?;
    let (input, behavior) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, request_id) = be_u32(input)?;

    let reason = StopFreezeReason::from(reason);
    let behavior = StopFreezeFrozenBehavior::from(behavior);

    let body = StopFreeze::new()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_real_world_time(real_world_time)
        .with_reason(reason)
        .with_frozen_behavior(behavior)
        .with_request_id(request_id);

    Ok((input, body.into_pdu_body()))
}