use nom::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::enumerations::{SignalTdlType};
use crate::{BodyProperties, CdisBody};
use crate::constants::{EIGHT_BITS, FOURTEEN_BITS, TWO_BITS};
use crate::parsing::{parse_field_when_present, BitInput};
use crate::records::parser::{encoding_scheme, entity_identification};
use crate::signal::model::{Signal, SignalFieldsPresent};
use crate::types::parser::{uvint16, uvint32};

pub(crate) fn signal_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u8) = take(TWO_BITS)(input)?;

    let (input, radio_reference_id) = entity_identification(input)?;
    let (input, radio_number) = uvint16(input)?;

    let (input, encoding_scheme) = encoding_scheme(input)?;

    let (input, tdl_type) : (BitInput, u16) = take(EIGHT_BITS)(input)?;
    let tdl_type = SignalTdlType::from(tdl_type);

    let (input, sample_rate) =
        parse_field_when_present(fields_present, SignalFieldsPresent::SAMPLE_RATE_BIT, uvint32)(input)?;

    let (input, data_length) : (BitInput, usize) = take(FOURTEEN_BITS)(input)?;

    let (input, samples) =
        parse_field_when_present(fields_present, SignalFieldsPresent::SAMPLES_BIT, uvint16)(input)?;

    let (input, data) = count(take(EIGHT_BITS), data_length / EIGHT_BITS)(input)?;

    Ok((input, Signal {
        radio_reference_id,
        radio_number,
        encoding_scheme,
        tdl_type,
        sample_rate,
        samples,
        data,
    }.into_cdis_body()))
}