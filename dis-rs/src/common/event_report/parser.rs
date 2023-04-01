use nom::IResult;
use nom::number::complete::be_u32;
use crate::common::parser::{datum_specification, entity_id};
use crate::enumerations::EventType;
use crate::common::event_report::model::EventReport;
use crate::common::model::PduBody;

pub fn event_report_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, event_type) = be_u32(input)?;
    let event_type = EventType::from(event_type);
    let (input, _padding) = be_u32(input)?;
    let (input, datums) = datum_specification(input)?;

    let body = EventReport::new()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_event_type(event_type)
        .with_fixed_datums(datums.fixed_datum_records)
        .with_variable_datums(datums.variable_datum_records);

    Ok((input, body.into_pdu_body()))
}