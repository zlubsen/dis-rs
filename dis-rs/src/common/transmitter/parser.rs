use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u32, be_u64, be_u8};
use crate::common::model::PduBody;
use crate::common::parser::{entity_id, entity_type, location, vec3_f32};
use crate::common::transmitter::model::{BeamAntennaPattern, CryptoKeyId, CryptoMode, ModulationType, SpreadSpectrum, Transmitter, VariableTransmitterParameter};
use crate::enumerations::{TransmitterAntennaPatternType, TransmitterInputSource, TransmitterTransmitState};
use crate::{TransmitterCryptoSystem, TransmitterDetailAmplitudeAngleModulation, TransmitterDetailAmplitudeModulation, TransmitterDetailAngleModulation, TransmitterDetailCarrierPhaseShiftModulation, TransmitterDetailCombinationModulation, TransmitterDetailPulseModulation, TransmitterDetailSATCOMModulation, TransmitterDetailUnmodulatedModulation, TransmitterMajorModulation, TransmitterModulationTypeSystem};

pub fn transmitter_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, radio_reference_id) = entity_id(input)?;
    let (input, radio_number) = be_u16(input)?;
    let (input, radio_type) = entity_type(input)?;
    let (input, transmit_state) = be_u8(input)?;
    let transmit_state = TransmitterTransmitState::from(transmit_state);
    let (input, input_source) = be_u8(input)?;
    let input_source = TransmitterInputSource::from(input_source);
    let (input, number_of_vtp) = be_u16(input)?;
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
    let (input, modulation_parameters) = if length_of_modulation_parameters > 0 {
        let params = take(length_of_modulation_parameters)(input)?;
        Some(params)
    } else { (input, None) };
    let (input, antenna_pattern) = if antenna_pattern_length > 0 {
        // TODO
        BeamAntennaPattern::new()
    } else { (input, None) };
    let (input, vt_params) =
        count(variable_transmitter_parameter, number_of_vtp)(input)?;

    let body = Transmitter::new()
        // TODO finish
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
    let frequency_hopping = (spread_spectrum_values >> 15) & 0x0001;
    let pseudo_noise = (spread_spectrum_values >> 14) & 0x0001;
    let time_hopping = (spread_spectrum_values >> 13) & 0x0001;

    Ok((input, SpreadSpectrum::new_with_values(frequency_hopping, pseudo_noise, time_hopping)))
}

fn crypto_key_id(input: &[u8]) -> IResult<&[u8], CryptoKeyId> {
    let (input, value) = be_u16(input)?;
    let pseudo_crypto_key = value >> 1;
    let crypto_mode = (value & 0x0001) as bool;
    let crypto_mode = CryptoMode::from(crypto_mode);

    Ok((input, CryptoKeyId {
        pseudo_crypto_key,
        crypto_mode,
    }))
}

fn variable_transmitter_parameter(input: &[u8]) -> IResult<&[u8], VariableTransmitterParameter> {
    // TODO
}