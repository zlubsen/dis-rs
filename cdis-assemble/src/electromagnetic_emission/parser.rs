use nom::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::enumerations::{ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator, EmitterName, EmitterSystemFunction, HighDensityTrackJam};
use crate::{BodyProperties, CdisBody};
use crate::constants::{EIGHT_BITS, FIVE_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, TEN_BITS};
use crate::electromagnetic_emission::model::{BeamData, ElectromagneticEmission, EmitterBeam, EmitterSystem, FrequencyFloat, FundamentalParameter, PulseWidthFloat, SiteAppPair, TrackJam};
use crate::parsing::BitInput;
use crate::records::parser::{entity_coordinate_vector, entity_identification};
use crate::types::parser::{cdis_float, svint13, uvint16, uvint8};

#[allow(clippy::redundant_closure)]
pub(crate) fn electromagnetic_emission_body(input: BitInput) -> IResult<BitInput, CdisBody> {
    let (input, full_update_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let full_update_flag = full_update_flag != 0;

    let (input, number_of_fundamental_params): (BitInput, usize) = take(FIVE_BITS)(input)?;
    let (input, number_of_beam_params): (BitInput, usize) = take(FIVE_BITS)(input)?;
    let (input, number_of_site_app_pairs): (BitInput, usize) = take(FIVE_BITS)(input)?;

    let (input, emitting_id) = entity_identification(input)?;
    let (input, event_id) = entity_identification(input)?;

    let (input, state_update_indicator) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let state_update_indicator = ElectromagneticEmissionStateUpdateIndicator::from(state_update_indicator);

    let (input, number_of_systems) = uvint8(input)?;

    let (input, fundamental_params) = count(fundamental_parameter, number_of_fundamental_params)(input)?;
    let (input, beam_data) = count(beam_data, number_of_beam_params)(input)?;
    let (input, site_app_pairs) = count(site_app_pair, number_of_site_app_pairs)(input)?;

    let (input, emitter_systems) = count(emitter_system, number_of_systems.value as usize)(input)?;

    Ok((input, ElectromagneticEmission {
        full_update_flag,
        emitting_id,
        event_id,
        state_update_indicator,
        fundamental_params,
        beam_data,
        site_app_pairs,
        emitter_systems,
    }.into_cdis_body()))
}

fn fundamental_parameter(input: BitInput) -> IResult<BitInput, FundamentalParameter> {
    let (input, frequency) = cdis_float::<FrequencyFloat>(input)?;
    let (input, frequency_range) = cdis_float::<FrequencyFloat>(input)?;
    let (input, erp) : (BitInput, u8) = take(EIGHT_BITS)(input)?;
    let (input, prf) = uvint16(input)?;
    let (input, pulse_width) = cdis_float::<PulseWidthFloat>(input)?;

    Ok((input, FundamentalParameter {
        frequency,
        frequency_range,
        erp,
        prf,
        pulse_width,
    }))
}

fn beam_data(input: BitInput) -> IResult<BitInput, BeamData> {
    let (input, az_center) = svint13(input)?;
    let (input, az_sweep) = svint13(input)?;
    let (input, el_center) = svint13(input)?;
    let (input, el_sweep) = svint13(input)?;
    let (input, sweep_sync): (BitInput, u16) = take(TEN_BITS)(input)?;

    Ok((input, BeamData {
        az_center,
        az_sweep,
        el_center,
        el_sweep,
        sweep_sync,
    }))
}

fn site_app_pair(input: BitInput) -> IResult<BitInput, SiteAppPair> {
    let (input, site) = uvint16(input)?;
    let (input, application) = uvint16(input)?;

    Ok((input, SiteAppPair {
        site,
        application,
    }))
}

fn emitter_system(input: BitInput) -> IResult<BitInput, EmitterSystem> {
    let (input, emitter_system_details_present_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let emitter_system_details_present_flag = emitter_system_details_present_flag != 0;
    let (input, location_present_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let location_present_flag = location_present_flag != 0;

    let (input, number_of_beams) : (BitInput, usize) = take(FIVE_BITS)(input)?;

    let (input, name, function)  = if emitter_system_details_present_flag {
        let (input, emitter_name): (BitInput, u16) = take(SIXTEEN_BITS)(input)?;
        let emitter_name = EmitterName::from(emitter_name);
        let (input, emitter_function): (BitInput, u8) = take(EIGHT_BITS)(input)?;
        let emitter_function = EmitterSystemFunction::from(emitter_function);
        (input, Some(emitter_name), Some(emitter_function))
    } else {
        (input, None, None)
    };

    let (input, number) = uvint8(input)?;

    let (input, location_with_respect_to_entity) = if location_present_flag {
        let (input, location_with_respect_to_entity) = entity_coordinate_vector(input)?;
        (input, Some(location_with_respect_to_entity))
    } else {
        (input, None)
    };

    let (input, emitter_beams) = count(emitter_beam, number_of_beams)(input)?;

    Ok((input, EmitterSystem {
        name,
        function,
        number,
        location_with_respect_to_entity,
        emitter_beams,
    }))
}

fn emitter_beam(input: BitInput) -> IResult<BitInput, EmitterBeam> {
    let (input, fundamental_params_present_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let fundamental_params_present_flag = fundamental_params_present_flag != 0;
    let (input, beam_data_details_present_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let beam_data_details_present_flag = beam_data_details_present_flag != 0;
    let (input, jamming_technique_present_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let jamming_technique_present_flag = jamming_technique_present_flag != 0;
    let (input, jamming_track_present_flag) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let jamming_track_present_flag = jamming_track_present_flag != 0;

    let (input, beam_id) = uvint8(input)?;
    let (input, beam_parameter_index) : (BitInput, u16) = take(SIXTEEN_BITS)(input)?;

    let (input, fundamental_params_index) = if fundamental_params_present_flag {
        let (input, index) : (BitInput, u8) = take(FIVE_BITS)(input)?;
        (input, Some(index))
    } else {
        (input, None)
    };
    let (input, beam_data_index) = if beam_data_details_present_flag {
        let (input, index) : (BitInput, u8) = take(FIVE_BITS)(input)?;
        (input, Some(index))
    } else {
        (input, None)
    };

    let (input, beam_function) : (BitInput, u8) = take(FIVE_BITS)(input)?;
    let beam_function = ElectromagneticEmissionBeamFunction::from(beam_function);

    let (input, number_of_targets) : (BitInput, usize) = take(FOUR_BITS)(input)?;
    let (input, high_density_track_jam) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let high_density_track_jam = HighDensityTrackJam::from(high_density_track_jam);
    let (input, beam_status) : (BitInput, u8) = take(ONE_BIT)(input)?;
    let beam_status = beam_status != 0;

    let (input,
        jamming_technique_kind,
        jamming_technique_category,
        jamming_technique_subcategory,
        jamming_technique_specific) = if jamming_technique_present_flag {
        let (input, jamming_technique_kind) = uvint8(input)?;
        let (input, jamming_technique_category) = uvint8(input)?;
        let (input, jamming_technique_subcategory) = uvint8(input)?;
        let (input, jamming_technique_specific) = uvint8(input)?;
        (input, Some(jamming_technique_kind), Some(jamming_technique_category), Some(jamming_technique_subcategory), Some(jamming_technique_specific))
    } else {
        (input, None, None, None, None)
    };

    let (input, track_jam) = count(track_jam(jamming_track_present_flag), number_of_targets)(input)?;

    Ok((input, EmitterBeam {
        beam_id,
        beam_parameter_index,
        fundamental_params_index,
        beam_data_index,
        beam_function,
        high_density_track_jam,
        beam_status,
        jamming_technique_kind,
        jamming_technique_category,
        jamming_technique_subcategory,
        jamming_technique_specific,
        track_jam,
    }))
}

fn track_jam(jamming_track_present_flag: bool) -> impl Fn(BitInput) -> IResult<BitInput, TrackJam> {
    move |input: BitInput| {
        let (input, site_app_pair_index) : (BitInput, u8) = take(FOUR_BITS)(input)?;
        let (input, entity_id) = uvint16(input)?;
        let (input, emitter_number, beam_number) = if jamming_track_present_flag {
            let (input, emitter_number) = uvint8(input)?;
            let (input, beam_number) = uvint8(input)?;
            (input, Some(emitter_number), Some(beam_number))
        } else {
            (input, None, None)
        };

        Ok((input, TrackJam {
            site_app_pair_index,
            entity_id,
            emitter_number,
            beam_number,
        }))
    }
}