use nom::IResult;
use nom::number::complete::be_u32;
use crate::common::parser::entity_id;
use crate::common::remove_entity::model::RemoveEntity;
use crate::PduBody;

pub fn remove_entity_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;

    let body = RemoveEntity::new()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id);

    Ok((input, body.into_pdu_body()))
}