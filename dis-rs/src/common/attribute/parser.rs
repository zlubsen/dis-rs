use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u32, be_u8};
use crate::common::parser::{entity_id, pdu_type, protocol_version, simulation_address};
use crate::common::attribute::model::{Attribute, AttributeRecord, AttributeRecordSet, BASE_ATTRIBUTE_RECORD_LENGTH_OCTETS};
use crate::enumerations::{AttributeActionCode, VariableRecordType};
use crate::common::model::PduBody;

pub(crate) fn attribute_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, origination_simulation_address) = simulation_address(input)?;
    let (input, _padding) = be_u32(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, record_pdu_type) = pdu_type(input)?;
    let (input, record_protocol_version) = protocol_version(input)?;
    let (input, master_attribute_record_type) = be_u32(input)?;
    let master_attribute_record_type = VariableRecordType::from(master_attribute_record_type);
    let (input, action_code) = be_u8(input)?;
    let action_code = AttributeActionCode::from(action_code);
    let (input, _padding) = be_u8(input)?;
    let (input, number_of_record_sets) = be_u16(input)?;
    let (input, attribute_record_sets) =
        count(attribute_record_set, number_of_record_sets.into())(input)?;

    let body = Attribute::builder()
        .with_originating_simulation_address(origination_simulation_address)
        .with_record_pdu_type(record_pdu_type)
        .with_record_protocol_version(record_protocol_version)
        .with_master_attribute_record_type(master_attribute_record_type)
        .with_action_code(action_code)
        .with_attribute_record_sets(attribute_record_sets)
        .build();

    Ok((input, body.into_pdu_body()))
}

pub(crate) fn attribute_record_set(input: &[u8]) -> IResult<&[u8], AttributeRecordSet> {
    let (input, entity_id) = entity_id(input)?;
    let (input, number_of_records) = be_u16(input)?;
    let (input, attribute_records) = count(attribute_record, number_of_records.into())(input)?;

    Ok((input, AttributeRecordSet::new()
        .with_entity_id(entity_id)
        .with_attribute_records(attribute_records)))
}

pub(crate) fn attribute_record(input: &[u8]) -> IResult<&[u8], AttributeRecord> {
    let (input, record_type) = be_u32(input)?;
    let record_type = VariableRecordType::from(record_type);
    let (input, record_length_octets) = be_u16(input)?;
    let (input, fields) = take(record_length_octets.saturating_sub(BASE_ATTRIBUTE_RECORD_LENGTH_OCTETS))(input)?;

    Ok((input, AttributeRecord::new()
        .with_record_type(record_type)
        .with_specific_fields(fields.to_vec())))
}