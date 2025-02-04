use crate::acknowledge::model::Acknowledge;
use crate::constants::{THREE_BITS, TWO_BITS};
use crate::parsing::BitInput;
use crate::records::parser::entity_identification;
use crate::types::parser::uvint32;
use crate::{BodyProperties, CdisBody};
use dis_rs::enumerations::{AcknowledgeFlag, ResponseFlag};
use nom::complete::take;
use nom::IResult;

pub(crate) fn acknowledge_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, originating_id) = entity_identification(input)?;
    let (input, receiving_id) = entity_identification(input)?;

    let (input, acknowledge_flag): (BitInput, u16) = take(THREE_BITS)(input)?;
    let acknowledge_flag = AcknowledgeFlag::from(acknowledge_flag);
    let (input, response_flag): (BitInput, u16) = take(TWO_BITS)(input)?;
    let response_flag = ResponseFlag::from(response_flag);

    let (input, request_id) = uvint32(input)?;

    Ok((
        input,
        Acknowledge {
            originating_id,
            receiving_id,
            acknowledge_flag,
            response_flag,
            request_id,
        }
        .into_cdis_body(),
    ))
}
