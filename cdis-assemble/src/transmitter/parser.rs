use nom::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::enumerations::{TransmitterAntennaPatternType, TransmitterCryptoSystem, TransmitterTransmitState, VariableRecordType};
use dis_rs::transmitter::model::VariableTransmitterParameter;
use crate::{BodyProperties, CdisBody};
use crate::constants::{EIGHT_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, TEN_BITS, THIRTY_TWO_BITS, THREE_BITS, TWO_BITS};
use crate::parsing::{field_present, parse_field_when_present, BitInput};
use crate::records::parser::{entity_coordinate_vector, entity_identification, entity_type, world_coordinates};
use crate::transmitter::model::{ModulationType, TransmitFrequencyBandwidthFloat, Transmitter, TransmitterFieldsPresent, TransmitterFrequencyFloat, TransmitterUnits};
use crate::types::parser::{uvint16, uvint8};
use crate::types::model::CdisFloat;

pub(crate) fn transmitter_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, fields_present) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, units) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let units = TransmitterUnits::from(units);
    let (input, full_update_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, radio_reference_id) = entity_identification(input)?;
    let (input, radio_number) = uvint16(input)?;

    let (input, radio_type) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::RADIO_TYPE_BIT, entity_type)(input)?;

    let (input, transmit_state) : (BitInput, u8) = take(TWO_BITS)(input)?;
    let transmit_state = TransmitterTransmitState::from(transmit_state);

    let (input, input_source) = uvint8(input)?;

    let (input, nr_of_variable_transmitter_parameters) = uvint8(input)?;

    let (input, antenna_location) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::ANTENNA_LOCATION_BIT, world_coordinates)(input)?;
    let (input, relative_antenna_location) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::RELATIVE_ANTENNA_LOCATION_BIT, entity_coordinate_vector)(input)?;

    let (input, antenna_pattern_type) : (BitInput, Option<u16>) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::ANTENNA_PATTERN_BIT, take(THREE_BITS))(input)?;
    let antenna_pattern_type = antenna_pattern_type.map(TransmitterAntennaPatternType::from);

    let (input, antenna_pattern_length) : (BitInput, u16) = take(TEN_BITS)(input)?;

    let (input, frequency) = if field_present(fields_present, TransmitterFieldsPresent::TRANSMITTER_DETAILS_BIT) {
        let (input, frequency) = TransmitterFrequencyFloat::parse(input)?;
        (input, Some(frequency))
    } else { (input, None) };
    let (input, transmit_frequency_bandwidth) = if field_present(fields_present, TransmitterFieldsPresent::TRANSMITTER_DETAILS_BIT) {
        let (input, transmit_frequency_bandwidth) = TransmitFrequencyBandwidthFloat::parse(input)?;
        (input, Some(transmit_frequency_bandwidth))
    } else { (input, None) };

    let (input, power) : (BitInput, Option<u8>) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::TRANSMITTER_DETAILS_BIT, take(EIGHT_BITS))(input)?;

    let (input, modulation_type) = if field_present(fields_present, TransmitterFieldsPresent::TRANSMITTER_DETAILS_BIT) {
        let (input, spread_spectrum) = take(FOUR_BITS)(input)?;
        let (input, major_modulation) = take(FOUR_BITS)(input)?;
        let (input, detail) = take(FOUR_BITS)(input)?;
        let (input, radio_system) = take(FOUR_BITS)(input)?;
        (input, Some(ModulationType {
            spread_spectrum,
            major_modulation,
            detail,
            radio_system,
        }))
    } else { (input, None) };

    let (input, crypto_system) : (BitInput, Option<u16>) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::CRYPTO_DETAILS_BIT, take(FOUR_BITS))(input)?;
    let crypto_system = crypto_system.map(TransmitterCryptoSystem::from);
    let (input, crypto_key_id) : (BitInput, Option<u16>) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::CRYPTO_DETAILS_BIT, take(SIXTEEN_BITS))(input)?;

    let (input, length_of_modulation_parameters) : (BitInput, Option<u16>) =
        parse_field_when_present(fields_present, TransmitterFieldsPresent::MODULATION_PARAMETERS_BIT, take(EIGHT_BITS))(input)?;

    let (input, modulation_parameters) = if field_present(fields_present, TransmitterFieldsPresent::MODULATION_PARAMETERS_BIT) {
        let (input, params) : (BitInput, Vec<u8>) = count(
            take(EIGHT_BITS), length_of_modulation_parameters.unwrap_or_default() as usize)(input)?;
        (input, params)
    } else {
        (input, vec![])
    };

    let (input, antenna_pattern) = if field_present(fields_present, TransmitterFieldsPresent::ANTENNA_PATTERN_BIT) {
        let (input, params) : (BitInput, Vec<u8>) = count(
            take(EIGHT_BITS), antenna_pattern_length as usize)(input)?;
        (input, params)
    } else {
        (input, vec![])
    };

    let (input, variable_transmitter_parameters) = if field_present(fields_present, TransmitterFieldsPresent::VARIABLE_PARAMETERS_BIT) {
        let (input, params) : (BitInput, Vec<VariableTransmitterParameter>) = count(
            variable_transmitter_parameter, nr_of_variable_transmitter_parameters.value as usize)(input)?;
        (input, params)
    } else {
        (input, vec![])
    };

    Ok((input, Transmitter {
        units,
        full_update_flag,
        radio_reference_id,
        radio_number,
        radio_type,
        transmit_state,
        input_source,
        antenna_location,
        relative_antenna_location,
        antenna_pattern_type,
        frequency,
        transmit_frequency_bandwidth,
        power,
        modulation_type,
        crypto_system,
        crypto_key_id,
        modulation_parameters,
        antenna_pattern,
        variable_transmitter_parameters,
    }.into_cdis_body()))
}

fn variable_transmitter_parameter(input: BitInput) -> IResult<BitInput, VariableTransmitterParameter> {
    const SIX_OCTETS: usize = 6;
    let (input, record_type) : (BitInput, u32) = take(THIRTY_TWO_BITS)(input)?;
    let (input, record_length) : (BitInput, usize) = take(SIXTEEN_BITS)(input)?;

    let nr_of_records = record_length.saturating_sub(SIX_OCTETS);
    let (input, record_specific_fields) : (BitInput, Vec<u8>) = count(take(EIGHT_BITS), nr_of_records)(input)?;

    Ok((input, VariableTransmitterParameter::default()
        .with_record_type(VariableRecordType::from(record_type))
        .with_fields(record_specific_fields)))
}