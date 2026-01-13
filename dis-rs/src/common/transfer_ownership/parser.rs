use crate::common::parser::{entity_id, record_specification};
use crate::enumerations::{RequiredReliabilityService, TransferControlTransferType};
use crate::model::PduBody;
use crate::transfer_ownership::model::TransferOwnership;
use crate::BodyRaw;
use nom::number::complete::{be_u32, be_u8};
use nom::IResult;

pub(crate) fn transfer_ownership_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, required_reliability) = be_u8(input)?;
    let required_reliability = RequiredReliabilityService::from(required_reliability);
    let (input, transfer_type) = be_u8(input)?;
    let transfer_type = TransferControlTransferType::from(transfer_type);
    let (input, transfer_entity_id) = entity_id(input)?;
    let (input, record_specification) = record_specification(input)?;

    Ok((
        input,
        TransferOwnership::builder()
            .with_originating_id(originating_id)
            .with_receiving_id(receiving_id)
            .with_request_id(request_id)
            .with_required_reliability_service(required_reliability)
            .with_transfer_type(transfer_type)
            .with_transfer_entity_id(transfer_entity_id)
            .with_record_specification(record_specification)
            .build()
            .into_pdu_body(),
    ))
}
