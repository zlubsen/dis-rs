use crate::common::create_entity::model::CreateEntity;
use crate::common::model::PduBody;
use crate::common::parser::entity_id;
use crate::BodyRaw;
use nom::number::complete::be_u32;
use nom::IResult;

pub(crate) fn create_entity_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;

    let body = CreateEntity::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .build();

    Ok((input, body.into_pdu_body()))
}
