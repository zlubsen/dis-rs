use nom::IResult;
use crate::{BodyProperties, CdisBody};
use crate::parsing::BitInput;
use crate::records::parser::entity_identification;
use crate::remove_entity::model::RemoveEntity;
use crate::types::parser::uvint32;

pub(crate) fn remove_entity_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, originating_id) = entity_identification(input)?;
    let (input, receiving_id) = entity_identification(input)?;
    let (input, request_id) = uvint32(input)?;

    Ok((input, RemoveEntity {
        originating_id,
        receiving_id,
        request_id,
    }.into_cdis_body()))
}