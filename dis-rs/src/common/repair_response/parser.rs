use nom::IResult;
use nom::number::complete::{be_u16, be_u8};
use crate::common::parser::{entity_id};
use crate::common::model::PduBody;
use crate::enumerations::RepairResponseRepairResult;
use crate::repair_response::model::RepairResponse;

pub(crate) fn repair_response_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, receiving_id) = entity_id(input)?;
    let (input, repairing_id) = entity_id(input)?;
    let (input, repair_result) = be_u8(input)?;
    let repair_result = RepairResponseRepairResult::from(repair_result);
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;

    let body = RepairResponse::builder()
        .with_receiving_id(receiving_id)
        .with_repairing_id(repairing_id)
        .with_repair_result(repair_result)
        .build();

    Ok((input, body.into_pdu_body()))
}