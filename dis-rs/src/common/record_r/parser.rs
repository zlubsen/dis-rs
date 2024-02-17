use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u32, be_u8};
use crate::common::model::PduBody;
use crate::common::parser::entity_id;
use crate::constants::{EIGHT_OCTETS, ONE_BYTE_IN_BITS};
use crate::enumerations::{EventType, RequiredReliabilityService, VariableRecordType};
use crate::model::length_padded_to_num;
use crate::record_r::model::{RecordR, RecordSet, RecordSpecification};

pub fn record_r_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_id) = entity_id(input)?;
    let (input, receiving_id) = entity_id(input)?;
    let (input, request_id) = be_u32(input)?;
    let (input, required_reliability_service) = be_u8(input)?;
    let required_reliability_service = RequiredReliabilityService::from(required_reliability_service);
    let (input, _padding) = be_u8(input)?;
    let (input, event_type) = be_u32(input)?;
    let event_type = EventType::from(event_type);
    let (input, response_serial_number) = be_u32(input)?;
    let (input, record_specification) = record_specification(input)?;

    let body = RecordR::builder()
        .with_origination_id(originating_id)
        .with_receiving_id(receiving_id)
        .with_request_id(request_id)
        .with_required_reliability_service(required_reliability_service)
        .with_event_type(event_type)
        .with_response_serial_number(response_serial_number)
        .with_record_specification(record_specification)
        .build();

    Ok((input, body.into_pdu_body()))
}

/// Parses the RecordSpecification record (6.2.73)
pub fn record_specification(input: &[u8]) -> IResult<&[u8], RecordSpecification> {
    let (input, number_of_records) = be_u32(input)?;
    let (input, record_sets) = count(record_set, number_of_records as usize)(input)?;

    Ok((input, RecordSpecification::default().with_record_sets(record_sets)))
}

/// Parses a Record Set as part of a RecordSpecification record (6.2.73).
///
/// Parsing will always consider record values to be byte-aligned.
/// Record length is defined in bits, but this function always rounds up to the next full byte.
/// This is compensated for in the padding.
pub fn record_set(input: &[u8]) -> IResult<&[u8], RecordSet> {
    let (input, record_id) = be_u32(input)?;
    let record_id = VariableRecordType::from(record_id);
    let (input, serial_number) = be_u32(input)?;
    let (input, _padding) = be_u32(input)?;
    let (input, record_length_bits) = be_u16(input)?;
    let record_length_bytes = ceil_bits_to_bytes(record_length_bits);
    let (input, record_count) = be_u16(input)?;
    let (input, record_values) : (&[u8], Vec<&[u8]>) =
        count(take(record_length_bytes), record_count as usize)(input)?;
    let record_values = record_values.iter()
        .map(|values| values.to_vec() )
        .collect();
    let padded_record_length = length_padded_to_num(
        (record_length_bytes * record_count) as usize,
        EIGHT_OCTETS);
    let (input, _padding) = take(padded_record_length.padding_length)(input)?;

    Ok((input, RecordSet::default()
        .with_record_id(record_id)
        .with_record_serial_number(serial_number)
        .with_records(record_values)))
}

/// Round upward a given number of bits to the next amount of full bytes
///
/// E.g., 7 bits become 1 byte (8 bits), 12 bits become 2 bytes (16 bits)
fn ceil_bits_to_bytes(bits: u16) -> u16 {
    bits.div_ceil(ONE_BYTE_IN_BITS as u16)
}