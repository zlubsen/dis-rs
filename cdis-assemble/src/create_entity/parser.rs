use nom::IResult;
use crate::{BodyProperties, CdisBody};
use crate::create_entity::model::CreateEntity;
use crate::parsing::BitInput;
use crate::records::parser::entity_identification;
use crate::types::parser::uvint32;

pub(crate) fn create_entity_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, originating_id) = entity_identification(input)?;
    let (input, receiving_id) = entity_identification(input)?;
    let (input, request_id) = uvint32(input)?;

    Ok((input, CreateEntity {
        originating_id,
        receiving_id,
        request_id,
    }.into_cdis_body()))
}