use nom::IResult;
use nom::number::complete::be_u32;
use crate::common::parser::{clock_time, entity_id};
use crate::common::start_resume::model::StartResume;
use crate::PduBody;

pub fn start_resume_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, real_world_time) = clock_time(input)?;
    let (input, simulation_time) = clock_time(input)?;
    let (input, request_id) = be_u32(input)?;

    let body = StartResume::new()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_real_world_time(real_world_time)
        .with_simulation_time(simulation_time)
        .with_request_id(request_id);

    Ok((input, PduBody::StartResume(body)))
}