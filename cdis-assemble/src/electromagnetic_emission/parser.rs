use nom::complete::take;
use nom::IResult;
use nom::multi::count;
use dis_rs::enumerations::ElectromagneticEmissionStateUpdateIndicator;
use crate::{CdisBody, parsing};
use crate::constants::{FIVE_BITS, ONE_BIT};
use crate::electromagnetic_emission::model::{BeamData, EmitterSystem, FundamentalParameter, SiteAppPair};
use crate::parsing::BitInput;
use crate::records::parser::entity_identification;
use crate::types::parser::{uvint16, uvint8};

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

    let (input, fundamental_params_list) = count(fundamental_parameter, number_of_fundamental_params)(input)?;
    let (input, beam_data_list) = count(beam_data, number_of_beam_params)(input)?;
    let (input, site_app_pairs_list) = count(site_app_pair, number_of_site_app_pairs)(input)?;

    let (input, emitter_system) = emitter_system(input)?;

}

fn fundamental_parameter(input: BitInput) -> IResult<BitInput, FundamentalParameter> {

}

fn beam_data(input: BitInput) -> IResult<BitInput, BeamData> {

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

}