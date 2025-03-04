use crate::common::parser::{entity_id, event_id, vec3_f32};
use crate::constants::LEAST_SIGNIFICANT_BIT;
use crate::enumerations::{
    APAStatus, UAAcousticEmitterSystemFunction, UAAcousticSystemName,
    UAActiveEmissionParameterIndex, UAAdditionalPassiveActivityParameterIndex,
    UAPassiveParameterIndex, UAPropulsionPlantConfiguration, UAScanPattern,
    UAStateChangeUpdateIndicator,
};
use crate::model::PduBody;
use crate::underwater_acoustic::model::{
    AcousticEmitterSystem, PropulsionPlantConfiguration, Shaft, UABeam, UAEmitterSystem,
    UAFundamentalParameterData, UnderwaterAcoustic, APA,
};
use nom::multi::count;
use nom::number::complete::{be_f32, be_i16, be_i32, be_u16, be_u8};
use nom::{IResult, Parser};

pub(crate) fn underwater_acoustic_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, emitting_entity_id) = entity_id(input)?;
    let (input, event_id) = event_id(input)?;
    let (input, state_change_update_indicator) = be_u8(input)?;
    let state_change_update_indicator =
        UAStateChangeUpdateIndicator::from(state_change_update_indicator);
    let (input, _padding) = be_u8(input)?;
    let (input, passive_parameter_index) = be_u16(input)?;
    let passive_parameter_index = UAPassiveParameterIndex::from(passive_parameter_index);
    let (input, propulsion_plant_configuration) = propulsion_plant_configuration(input)?;

    let (input, number_of_shafts) = be_u8(input)?;
    let (input, number_of_apas) = be_u8(input)?;
    let (input, number_of_emitter_systems) = be_u8(input)?;

    let (input, shafts) = count(shaft, number_of_shafts as usize).parse(input)?;
    let (input, apas) = count(apa, number_of_apas as usize).parse(input)?;
    let (input, emitter_systems) =
        count(ua_emitter_system, number_of_emitter_systems as usize).parse(input)?;

    Ok((
        input,
        UnderwaterAcoustic::builder()
            .with_emitting_entity_id(emitting_entity_id)
            .with_event_id(event_id)
            .with_state_change_update_indicator(state_change_update_indicator)
            .with_passive_parameter_index(passive_parameter_index)
            .with_propulsion_plant_configuration(propulsion_plant_configuration)
            .with_shafts(shafts)
            .with_apas(apas)
            .with_emitter_systems(emitter_systems)
            .build()
            .into_pdu_body(),
    ))
}

fn propulsion_plant_configuration(input: &[u8]) -> IResult<&[u8], PropulsionPlantConfiguration> {
    let (input, field) = be_u8(input)?;
    let configuration = field >> 1;
    let configuration = UAPropulsionPlantConfiguration::from(configuration);
    let hull_mounted_masker_on = field & LEAST_SIGNIFICANT_BIT as u8 != 0;

    Ok((
        input,
        PropulsionPlantConfiguration::default()
            .with_configuration(configuration)
            .with_hull_mounted_masker(hull_mounted_masker_on),
    ))
}

fn shaft(input: &[u8]) -> IResult<&[u8], Shaft> {
    let (input, current_rpm) = be_i16(input)?;
    let (input, ordered_rpm) = be_i16(input)?;
    let (input, rpm_rate_of_change) = be_i32(input)?;

    Ok((
        input,
        Shaft::default()
            .with_current_rpm(current_rpm)
            .with_ordered_rpm(ordered_rpm)
            .with_rpm_rate_of_change(rpm_rate_of_change),
    ))
}

fn apa(input: &[u8]) -> IResult<&[u8], APA> {
    const LAST_TWO_BITS_MASK: u16 = 0x0003;
    let (input, parameter_index) = be_u16(input)?;
    let parameter = UAAdditionalPassiveActivityParameterIndex::from(parameter_index >> 2);
    let parameter_status = APAStatus::from((parameter_index & LAST_TWO_BITS_MASK) as u8);
    let (input, value) = be_i16(input)?;

    Ok((
        input,
        APA::default()
            .with_parameter(parameter)
            .with_status(parameter_status)
            .with_value(value),
    ))
}

fn ua_emitter_system(input: &[u8]) -> IResult<&[u8], UAEmitterSystem> {
    let (input, _data_length) = be_u8(input)?;
    let (input, number_of_beams) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, acoustic_emitter_system) = acoustic_emitter_system(input)?;
    let (input, location) = vec3_f32(input)?;
    let (input, beams) = count(ua_beam, number_of_beams as usize).parse(input)?;

    Ok((
        input,
        UAEmitterSystem::default()
            .with_acoustic_emitter_system(acoustic_emitter_system)
            .with_location(location)
            .with_beams(beams),
    ))
}

fn acoustic_emitter_system(input: &[u8]) -> IResult<&[u8], AcousticEmitterSystem> {
    let (input, acoustic_system_name) = be_u16(input)?;
    let acoustic_system_name = UAAcousticSystemName::from(acoustic_system_name);
    let (input, function) = be_u8(input)?;
    let function = UAAcousticEmitterSystemFunction::from(function);
    let (input, acoustic_id_number) = be_u8(input)?;

    Ok((
        input,
        AcousticEmitterSystem::default()
            .with_acoustic_system_name(acoustic_system_name)
            .with_function(function)
            .with_acoustic_id_number(acoustic_id_number),
    ))
}

fn ua_beam(input: &[u8]) -> IResult<&[u8], UABeam> {
    let (input, beam_data_length) = be_u8(input)?;
    let (input, beam_id_number) = be_u8(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, fundamental_parameters) = ua_fundamental_parameter_data(input)?;

    Ok((
        input,
        UABeam::default()
            .with_beam_data_length(beam_data_length)
            .with_beam_id_number(beam_id_number)
            .with_fundamental_parameters(fundamental_parameters),
    ))
}

fn ua_fundamental_parameter_data(input: &[u8]) -> IResult<&[u8], UAFundamentalParameterData> {
    let (input, active_emission_parameter_index) = be_u16(input)?;
    let active_emission_parameter_index =
        UAActiveEmissionParameterIndex::from(active_emission_parameter_index);
    let (input, scan_pattern) = be_u16(input)?;
    let scan_pattern = UAScanPattern::from(scan_pattern);
    let (input, beam_center_azimuth) = be_f32(input)?;
    let (input, azimuthal_beamwidth) = be_f32(input)?;
    let (input, beam_center_depression_elevation) = be_f32(input)?;
    let (input, depression_elevation_beamwidth) = be_f32(input)?;

    Ok((
        input,
        UAFundamentalParameterData::default()
            .with_active_emission_parameter_index(active_emission_parameter_index)
            .with_scan_pattern(scan_pattern)
            .with_beam_center_azimuth(beam_center_azimuth)
            .with_azimuthal_beamwidth(azimuthal_beamwidth)
            .with_beam_center_depression_elevation(beam_center_depression_elevation)
            .with_depression_elevation_beamwidth(depression_elevation_beamwidth),
    ))
}
