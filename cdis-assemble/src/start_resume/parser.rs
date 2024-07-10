use nom::IResult;
use crate::{BodyProperties, CdisBody};
use crate::parsing::BitInput;
use crate::records::parser::entity_identification;
use crate::start_resume::model::StartResume;
use crate::types::parser::{clock_time, uvint32};

pub(crate) fn start_resume_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, originating_id) = entity_identification(input)?;
    let (input, receiving_id) = entity_identification(input)?;

    let (input, real_world_time) = clock_time(input)?;
    let (input, simulation_time) = clock_time(input)?;

    let (input, request_id) = uvint32(input)?;

    Ok((input, StartResume {
        originating_id,
        receiving_id,
        real_world_time,
        simulation_time,
        request_id,
    }.into_cdis_body()))
}