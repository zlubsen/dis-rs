use nom::IResult;
use nom::number::complete::be_u16;
use crate::common::parser::{entity_id};
use crate::common::model::PduBody;
use crate::enumerations::RepairCompleteRepair;
use crate::repair_complete::model::RepairComplete;

pub fn repair_complete_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, receiving_id) = entity_id(input)?;
    let (input, repairing_id) = entity_id(input)?;
    let (input, repair) = be_u16(input)?;
    let repair = RepairCompleteRepair::from(repair);
    let (input, _padding) = be_u16(input)?;

    let body = RepairComplete::builder()
        .with_receiving_id(receiving_id)
        .with_repairing_id(repairing_id)
        .with_repair(repair)
        .build();

    Ok((input, body.into_pdu_body()))
}