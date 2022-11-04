use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u8};
use crate::common::model::{PduBody, PduHeader};
use crate::enumerations::{BeamStatusBeamState, ElectromagneticEmissionStateUpdateIndicator, ElectromagneticEmissionBeamFunction, EmitterName, EmitterSystemFunction, HighDensityTrackJam};
use crate::common::electromagnetic_emission::model::{Beam, BeamData, ElectromagneticEmission, EmitterSystem, FundamentalParameterData, JammingTechnique, TrackJam};
use crate::common::parser::{entity_id, event_id, vec3_f32};

pub fn emission_body(_header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input| {
        let (input, emitting_entity_id) = entity_id(input)?;
        let (input, event_id) = event_id(input)?;
        let (input, status_update_indicator) = be_u8(input)?;
        let (input, no_of_systems) = be_u8(input)?;
        let (input, _pad_16) = be_u16(input)?;

        let (input, mut emitter_systems) = count(emitter_system, no_of_systems as usize)(input)?;

        let body = ElectromagneticEmission::new()
            .with_emitting_entity_id(emitting_entity_id)
            .with_event_id(event_id)
            .with_state_update_indicator(ElectromagneticEmissionStateUpdateIndicator::from(status_update_indicator))
            .with_emitter_systems(&mut emitter_systems);

        Ok((input, body.as_pdu_body()))
    }
}

pub fn emitter_system(input: &[u8]) -> IResult<&[u8], EmitterSystem> {
    let (input, _system_data_length) = be_u8(input)?; // TODO check available length of input
    let (input, no_of_beams) = be_u8(input)?;
    let (input, _pad_16) = be_u16(input)?;
    let (input, name) = be_u16(input)?;
    let (input, function) = be_u8(input)?;
    let (input, number) = be_u8(input)?;
    let (input, location) = vec3_f32(input)?;

    let (input, mut beams) = count(beam, no_of_beams as usize)(input)?;

    let system = EmitterSystem::new()
        .with_name(EmitterName::from(name))
        .with_function(EmitterSystemFunction::from(function))
        .with_number(number)
        .with_location(location)
        .with_beams(&mut beams);

    Ok((input, system))
}

pub fn beam(input: &[u8]) -> IResult<&[u8], Beam> {
    let (input, _data_length) = be_u8(input)?;
    let (input, number) = be_u8(input)?;
    let (input, parameter_index) = be_u16(input)?;
    let (input, fundamental_parameter_data) = fundamental_parameter_data(input)?;
    let (input, beam_data) = beam_data(input)?;
    let (input, function) = be_u8(input)?;
    let (input, no_of_targets) = be_u8(input)?;
    let (input, high_density_track_jam) = be_u8(input)?;
    let (input, status) = be_u8(input)?;
    let (input, jamming_technique) = jamming_technique(input)?;
    let (input, mut track_jams) = count(track_jam, no_of_targets as usize)(input)?;

    let beam = Beam::new()
        .with_number(number)
        .with_parameter_index(parameter_index)
        .with_parameter_data(fundamental_parameter_data)
        .with_beam_data(beam_data)
        .with_beam_function(ElectromagneticEmissionBeamFunction::from(function))
        .with_high_density_track_jam(HighDensityTrackJam::from(high_density_track_jam))
        .with_beam_status(BeamStatusBeamState::from(status))
        .with_jamming_technique(jamming_technique)
        .with_track_jams(&mut track_jams);

    Ok((input, beam))
}

pub fn fundamental_parameter_data(input: &[u8]) -> IResult<&[u8], FundamentalParameterData> {
    let (input, frequency) = be_f32(input)?;
    let (input, frequency_range) = be_f32(input)?;
    let (input, effective_power) = be_f32(input)?;
    let (input, pulse_repetition_frequency) = be_f32(input)?;
    let (input, pulse_width) = be_f32(input)?;

    let data = FundamentalParameterData::new()
        .with_frequency(frequency)
        .with_frequency_range(frequency_range)
        .with_effective_power(effective_power)
        .with_pulse_repetition_frequency(pulse_repetition_frequency)
        .with_pulse_width(pulse_width);

    Ok((input, data))
}

pub fn beam_data(input: &[u8]) -> IResult<&[u8], BeamData> {
    let (input, azimuth_center) = be_f32(input)?;
    let (input, azimuth_sweep) = be_f32(input)?;
    let (input, elevation_center) = be_f32(input)?;
    let (input, elevation_sweep) = be_f32(input)?;
    let (input, sweep_sync) = be_f32(input)?;

    let data = BeamData::new()
        .with_azimuth_center(azimuth_center)
        .with_azimuth_sweep(azimuth_sweep)
        .with_elevation_center(elevation_center)
        .with_elevation_sweep(elevation_sweep)
        .with_sweep_sync(sweep_sync);

    Ok((input, data))
}

pub fn jamming_technique(input: &[u8]) -> IResult<&[u8], JammingTechnique> {
    let (input, kind) = be_u8(input)?;
    let (input, category) = be_u8(input)?;
    let (input, subcategory) = be_u8(input)?;
    let (input, specific) = be_u8(input)?;

    let technique = JammingTechnique::new()
        .with_kind(kind)
        .with_category(category)
        .with_subcategory(subcategory)
        .with_specific(specific);

    Ok((input, technique))
}

pub fn track_jam(input: &[u8]) -> IResult<&[u8], TrackJam> {
    let (input, entity_id) = entity_id(input)?;
    let (input, emitter_number) = be_u8(input)?;
    let (input, beam_number) = be_u8(input)?;

    let track = TrackJam::new()
        .with_entity_id(entity_id)
        .with_emitter(emitter_number)
        .with_beam(beam_number);

    Ok((input, track))
}