use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u8};
use crate::common::model::{PduBody, PduHeader};
use crate::enumerations::{BeamStatusBeamState, ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, EmitterName, EmitterSystemFunction, HighDensityTrackJam};
use crate::common::electromagnetic_emission::model::{Beam, ElectromagneticEmission, EmitterSystem, FundamentalParameterData, JammingTechnique, TrackJam};
use crate::common::parser;
use crate::common::parser::{entity_id, event_id, vec3_f32};

pub(crate) fn emission_body(_header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input| {
        let (input, emitting_entity_id) = entity_id(input)?;
        let (input, event_id) = event_id(input)?;
        let (input, status_update_indicator) = be_u8(input)?;
        let (input, no_of_systems) = be_u8(input)?;
        let (input, _pad_16) = be_u16(input)?;

        let (input, mut emitter_systems) = count(emitter_system, no_of_systems as usize)(input)?;

        let body = ElectromagneticEmission::builder()
            .with_emitting_entity_id(emitting_entity_id)
            .with_event_id(event_id)
            .with_state_update_indicator(ElectromagneticEmissionStateUpdateIndicator::from(status_update_indicator))
            .with_emitter_systems(&mut emitter_systems)
            .build();

        Ok((input, body.into_pdu_body()))
    }
}

pub(crate) fn emitter_system(input: &[u8]) -> IResult<&[u8], EmitterSystem> {
    let (input, _system_data_length) = be_u8(input)?;
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

pub(crate) fn beam(input: &[u8]) -> IResult<&[u8], Beam> {
    let (input, _data_length) = be_u8(input)?;
    let (input, number) = be_u8(input)?;
    let (input, parameter_index) = be_u16(input)?;
    let (input, fundamental_parameter_data) = fundamental_parameter_data(input)?;
    let (input, beam_data) = parser::beam_data(input)?;
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

pub(crate) fn fundamental_parameter_data(input: &[u8]) -> IResult<&[u8], FundamentalParameterData> {
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

pub(crate) fn jamming_technique(input: &[u8]) -> IResult<&[u8], JammingTechnique> {
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

pub(crate) fn track_jam(input: &[u8]) -> IResult<&[u8], TrackJam> {
    let (input, entity_id) = entity_id(input)?;
    let (input, emitter_number) = be_u8(input)?;
    let (input, beam_number) = be_u8(input)?;

    let track = TrackJam::new()
        .with_entity_id(entity_id)
        .with_emitter(emitter_number)
        .with_beam(beam_number);

    Ok((input, track))
}

#[cfg(test)]
mod tests {
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{*};
    use crate::common::model::{PduBody, EntityId};

    #[test]
    fn parse_pdu_emission_basic() {
        let bytes: [u8; 48] =
            [0x06, 0x01, 0x17, 0x06, 0xaa, 0x58, 0xbe, 0xc1, 0x00, 0x30, 0x00, 0x00, 0x01, 0xf4, 0x2b, 0x67,
                0x00, 0x27, 0x01, 0xf4, 0x2b, 0x67, 0x01, 0xd2, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x00, 0x00,
                0x4d, 0x58, 0x02, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x41, 0x70, 0x00, 0x00];

        let pdu = parse_pdu(&bytes);
        assert!(pdu.is_ok());
        let pdu = pdu.unwrap();
        assert_eq!(pdu.header.pdu_type, PduType::ElectromagneticEmission);
        assert_eq!(pdu.header.pdu_length, 48u16);
        if let PduBody::ElectromagneticEmission(pdu) = pdu.body {
            assert_eq!(pdu.emitting_entity_id.simulation_address.site_id, 500u16);
            assert_eq!(pdu.emitting_entity_id.simulation_address.application_id, 11111u16);
            assert_eq!(pdu.emitting_entity_id.entity_id, 39u16);
            assert_eq!(pdu.emitter_systems.len(), 1);
            assert_eq!(pdu.emitter_systems.first().unwrap().name, EmitterName::Unnamed_19800);
            assert_eq!(pdu.emitter_systems.first().unwrap().function, EmitterSystemFunction::EarlyWarningSurveillance_2);
            assert_eq!(pdu.emitter_systems.first().unwrap().number, 1u8);
            assert_eq!(pdu.emitter_systems.first().unwrap().location.first_vector_component, 0f32);
            assert_eq!(pdu.emitter_systems.first().unwrap().location.second_vector_component, 0f32);
            assert_eq!(pdu.emitter_systems.first().unwrap().location.third_vector_component, 15f32);
        } else {
            assert!(false);
        }
    }

    #[test]
    fn parse_pdu_emission_with_tracks() {
        let bytes: [u8; 228] =
            [0x06, 0x01, 0x17, 0x06, 0xb4, 0x97, 0xa2, 0xe3, 0x00, 0xe4, 0x00, 0x00, 0x01, 0xf4, 0x2b, 0x67,
                0x00, 0x3e, 0x01, 0xf4, 0x2b, 0x67, 0x02, 0x41, 0x00, 0x01, 0x00, 0x00, 0x32, 0x03, 0x00, 0x00,
                0x7d, 0xaf, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0xa0, 0x00, 0x00,
                0x0f, 0x02, 0x00, 0x7a, 0x4f, 0xb2, 0xd0, 0x5e, 0x00, 0x00, 0x00, 0x00, 0x42, 0xf0, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x05, 0xd5, 0xff, 0x3c, 0x0e, 0xfa, 0x35,
                0x3f, 0x30, 0x95, 0x12, 0x3c, 0x0e, 0xfa, 0x35, 0x00, 0x00, 0x00, 0x00, 0x07, 0x01, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x01, 0xf4, 0x2b, 0x67, 0x00, 0x47, 0x00, 0x00, 0x0f, 0x01, 0x00, 0x64,
                0x4f, 0xb2, 0xd0, 0x5e, 0x00, 0x00, 0x00, 0x00, 0x42, 0xf0, 0x00, 0x00, 0x45, 0x9c, 0x40, 0x00,
                0x42, 0x48, 0x00, 0x00, 0x3e, 0x05, 0xd6, 0x14, 0x3c, 0x0e, 0xfa, 0x35, 0x3f, 0x30, 0x95, 0x54,
                0x3c, 0x0e, 0xfa, 0x35, 0x00, 0x00, 0x00, 0x00, 0x04, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0xf4, 0x2b, 0x67, 0x00, 0x47, 0x00, 0x00, 0x0f, 0x03, 0x00, 0xd4, 0x4f, 0xb2, 0xd0, 0x5e,
                0x00, 0x00, 0x00, 0x00, 0x42, 0xf0, 0x00, 0x00, 0x43, 0xbb, 0x8c, 0x01, 0x40, 0xa0, 0x00, 0x00,
                0x3e, 0x05, 0xd6, 0x14, 0x3c, 0x0e, 0xfa, 0x35, 0x3f, 0x30, 0x95, 0x54, 0x3c, 0x0e, 0xfa, 0x35,
                0x00, 0x00, 0x00, 0x00, 0x06, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0xf4, 0x2b, 0x67,
                0x00, 0x47, 0x00, 0x00];

        let pdu = parse_pdu(&bytes);
        assert!(pdu.is_ok());
        let pdu = pdu.unwrap();
        assert_eq!(pdu.header.pdu_type, PduType::ElectromagneticEmission);
        assert_eq!(pdu.header.pdu_length, 228u16);
        if let PduBody::ElectromagneticEmission(pdu) = pdu.body {
            assert_eq!(pdu.emitting_entity_id.simulation_address.site_id, 500u16);
            assert_eq!(pdu.emitting_entity_id.simulation_address.application_id, 11111u16);
            assert_eq!(pdu.emitting_entity_id.entity_id, 62u16);
            assert_eq!(pdu.emitter_systems.len(), 1);

            let emitter = pdu.emitter_systems.first().unwrap();
            assert_eq!(emitter.name, EmitterName::Unnamed_32175);
            assert_eq!(emitter.function, EmitterSystemFunction::Multifunction_1);
            assert_eq!(emitter.number, 1u8);
            assert_eq!(emitter.beams.len(), 3usize);
            assert_eq!(emitter.location.first_vector_component, 0f32);
            assert_eq!(emitter.location.second_vector_component, 0f32);
            assert_eq!(emitter.location.third_vector_component, 5f32);

            let beam = emitter.beams.first().unwrap();
            assert_eq!(beam.number, 2u8);
            assert_eq!(beam.parameter_index, 122u16);
            assert_eq!(beam.beam_function, ElectromagneticEmissionBeamFunction::Illumination);
            assert_eq!(beam.high_density_track_jam, HighDensityTrackJam::NotSelected);
            assert_eq!(beam.track_jam_data.len(), 1usize);

            let track = beam.track_jam_data.first().unwrap();
            assert_eq!(track.beam, 0u8);
            assert_eq!(track.emitter, 0u8);
            assert_eq!(track.entity_id, EntityId::new(500u16, 11111u16, 71u16));
        } else {
            assert!(false);
        }
    }
}