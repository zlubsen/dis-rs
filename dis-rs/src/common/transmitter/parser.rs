use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u32, be_u64, be_u8};
use crate::common::model::{PduBody, PduHeader};
use crate::common::parser::{entity_id, entity_type, location, orientation, vec3_f32};
use crate::common::transmitter::model::{BASE_VTP_RECORD_LENGTH, BeamAntennaPattern, CryptoKeyId, ModulationType, SpreadSpectrum, Transmitter, VariableTransmitterParameter};
use crate::enumerations::{TransmitterAntennaPatternType, TransmitterInputSource, TransmitterTransmitState, ProtocolVersion, TransmitterAntennaPatternReferenceSystem, TransmitterCryptoSystem, TransmitterDetailAmplitudeAngleModulation, TransmitterDetailAmplitudeModulation, TransmitterDetailAngleModulation, TransmitterDetailCarrierPhaseShiftModulation, TransmitterDetailCombinationModulation, TransmitterDetailPulseModulation, TransmitterDetailSATCOMModulation, TransmitterDetailUnmodulatedModulation, TransmitterMajorModulation, TransmitterModulationTypeSystem, VariableRecordType};

pub(crate) fn transmitter_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input: &[u8]| {
        let (input, radio_reference_id) = entity_id(input)?;
        let (input, radio_number) = be_u16(input)?;
        let (input, radio_type) = entity_type(input)?;
        let (input, transmit_state) = be_u8(input)?;
        let transmit_state = TransmitterTransmitState::from(transmit_state);
        let (input, input_source) = be_u8(input)?;
        let input_source = TransmitterInputSource::from(input_source);
        #[allow(clippy::wildcard_in_or_patterns)]
        let (input, number_of_vtp) = {
            let (input, number_of_vtp) = be_u16(input)?;
            match header.protocol_version {
                ProtocolVersion::IEEE1278_12012 => { (input, number_of_vtp) }
                ProtocolVersion::IEEE1278_1A1998 | _ => {
                    (input, 0u16)  // set to zeroed padding to prevent parsing of not present Variable Transmitter Parameters
                }
            }
        };
        let (input, antenna_location) = location(input)?;
        let (input, relative_antenna_location) = vec3_f32(input)?;
        let (input, antenna_pattern_type) = be_u16(input)?;
        let antenna_pattern_type = TransmitterAntennaPatternType::from(antenna_pattern_type);
        let (input, antenna_pattern_length) = be_u16(input)?;
        let (input, frequency) = be_u64(input)?;
        let (input, transmit_frequency_bandwidth) = be_f32(input)?;
        let (input, power) = be_f32(input)?;
        let (input, modulation_type) = modulation_type(input)?;
        let (input, crypto_system) = be_u16(input)?;
        let crypto_system = TransmitterCryptoSystem::from(crypto_system);
        let (input, crypto_key_id) = crypto_key_id(input)?;
        let (input, length_of_modulation_parameters) = be_u8(input)?;
        let (input, _padding) = be_u8(input)?;
        let (input, _padding) = be_u16(input)?;

        let (input, modulation_parameters) = if length_of_modulation_parameters > 0 {
            let (input, params) = take(length_of_modulation_parameters)(input)?;
            (input, Some(params))
        } else { (input, None) };
        let (input, antenna_pattern) = if antenna_pattern_length > 0 {
            let (input, pattern) = beam_antenna_pattern(input)?;
            (input, Some(pattern))
        } else { (input, None) };

        let (input, vt_params) =
            count(variable_transmitter_parameter, number_of_vtp.into())(input)?;

        let body = Transmitter::builder()
            .with_radio_reference_id(radio_reference_id)
            .with_radio_number(radio_number)
            .with_radio_type(radio_type)
            .with_transmit_state(transmit_state)
            .with_input_source(input_source)
            .with_antenna_location(antenna_location)
            .with_relative_antenna_location(relative_antenna_location)
            .with_antenna_pattern_type(antenna_pattern_type)
            .with_frequency(frequency)
            .with_transmit_frequency_bandwidth(transmit_frequency_bandwidth)
            .with_power(power)
            .with_modulation_type(modulation_type)
            .with_crypto_system(crypto_system)
            .with_crypto_key_id(crypto_key_id)
            .with_variable_transmitter_parameters(vt_params);
        let body = if let Some(antenna_pattern) = antenna_pattern {
            body.with_antenna_pattern(antenna_pattern)
        } else { body };
        let body = if let Some(modulation_parameters) = modulation_parameters {
            body.with_modulation_parameters(modulation_parameters.to_vec())
        } else { body };
        let body = body.build();

        Ok((input, body.into_pdu_body()))
    }
}

fn modulation_type(input: &[u8]) -> IResult<&[u8], ModulationType> {
    let (input, spread_spectrum) = spread_spectrum(input)?;
    let (input, major_modulation) = be_u16(input)?;
    let major_modulation = TransmitterMajorModulation::from(major_modulation);
    let (input, detail) = be_u16(input)?;
    let major_modulation = match major_modulation {
        TransmitterMajorModulation::NoStatement =>
            { TransmitterMajorModulation::NoStatement }
        TransmitterMajorModulation::Amplitude(_) =>
            { TransmitterMajorModulation::Amplitude(TransmitterDetailAmplitudeModulation::from(detail)) }
        TransmitterMajorModulation::AmplitudeandAngle(_) =>
            { TransmitterMajorModulation::AmplitudeandAngle(TransmitterDetailAmplitudeAngleModulation::from(detail)) }
        TransmitterMajorModulation::Angle(_) =>
            { TransmitterMajorModulation::Angle(TransmitterDetailAngleModulation::from(detail)) }
        TransmitterMajorModulation::Combination(_) =>
            { TransmitterMajorModulation::Combination(TransmitterDetailCombinationModulation::from(detail)) }
        TransmitterMajorModulation::Pulse(_) =>
            { TransmitterMajorModulation::Pulse(TransmitterDetailPulseModulation::from(detail)) }
        TransmitterMajorModulation::Unmodulated(_) =>
            { TransmitterMajorModulation::Unmodulated(TransmitterDetailUnmodulatedModulation::from(detail)) }
        TransmitterMajorModulation::CarrierPhaseShiftModulation_CPSM_(_) =>
            { TransmitterMajorModulation::CarrierPhaseShiftModulation_CPSM_(TransmitterDetailCarrierPhaseShiftModulation::from(detail)) }
        TransmitterMajorModulation::SATCOM(_) =>
            { TransmitterMajorModulation::SATCOM(TransmitterDetailSATCOMModulation::from(detail)) }
        TransmitterMajorModulation::Unspecified(_) =>
            { TransmitterMajorModulation::Unspecified(detail) }
    };
    let (input, radio_system) = be_u16(input)?;
    let radio_system = TransmitterModulationTypeSystem::from(radio_system);

    Ok((input, ModulationType::new()
        .with_spread_spectrum(spread_spectrum)
        .with_major_modulation(major_modulation)
        .with_radio_system(radio_system)))
}

fn spread_spectrum(input: &[u8]) -> IResult<&[u8], SpreadSpectrum> {
    let (input, spread_spectrum_values) = be_u16(input)?;
    let frequency_hopping = ((spread_spectrum_values >> 15) & 0x0001) != 0;
    let pseudo_noise = ((spread_spectrum_values >> 14) & 0x0001) != 0;
    let time_hopping = ((spread_spectrum_values >> 13) & 0x0001) != 0;

    Ok((input, SpreadSpectrum::new_with_values(frequency_hopping, pseudo_noise, time_hopping)))
}

fn crypto_key_id(input: &[u8]) -> IResult<&[u8], CryptoKeyId> {
    let (input, value) = be_u16(input)?;

    Ok((input, CryptoKeyId::from(value)))
}

fn beam_antenna_pattern(input: &[u8]) -> IResult<&[u8], BeamAntennaPattern> {
    let (input, beam_direction) = orientation(input)?;
    let (input, azimuth_beamwidth) = be_f32(input)?;
    let (input, elevation_beamwidth) = be_f32(input)?;
    let (input, reference_system) = be_u8(input)?;
    let reference_system = TransmitterAntennaPatternReferenceSystem::from(reference_system);
    let (input, _padding) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, e_z) = be_f32(input)?;
    let (input, e_x) = be_f32(input)?;
    let (input, phase) = be_f32(input)?;
    let (input, _padding) = be_u32(input)?;

    Ok((input, BeamAntennaPattern::new()
        .with_beam_direction(beam_direction)
        .with_azimuth_beamwidth(azimuth_beamwidth)
        .with_elevation_beamwidth(elevation_beamwidth)
        .with_reference_system(reference_system)
        .with_e_z(e_z)
        .with_e_x(e_x)
        .with_phase(phase)))
}

fn variable_transmitter_parameter(input: &[u8]) -> IResult<&[u8], VariableTransmitterParameter> {
    let (input, record_type) = be_u32(input)?;
    let record_type = VariableRecordType::from(record_type);
    let (input, record_length) = be_u16(input)?;
    let fields_length_bytes = record_length - BASE_VTP_RECORD_LENGTH;
    let (input, specific_fields) = take(fields_length_bytes)(input)?;

    Ok((input, VariableTransmitterParameter::new()
        .with_record_type(record_type)
        .with_fields(specific_fields.to_vec())))
}