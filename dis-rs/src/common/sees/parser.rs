use crate::common::parser::entity_id;
use crate::model::PduBody;
use crate::sees::model::{PropulsionSystemData, VectoringNozzleSystemData, SEES};
use crate::BodyRaw;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16};
use nom::{IResult, Parser};

pub(crate) fn sees_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_entity_id) = entity_id(input)?;
    let (input, ir_signature) = be_u16(input)?;
    let (input, acoustic_signature) = be_u16(input)?;
    let (input, rcs_signature) = be_u16(input)?;
    let (input, nr_of_propulsion_systems) = be_u16(input)?;
    let (input, nr_of_vectoring_nozzle_systems) = be_u16(input)?;

    let (input, propulsion_systems) =
        count(propulsion_system_data, nr_of_propulsion_systems.into()).parse(input)?;
    let (input, vectoring_nozzle_systems) = count(
        vectoring_nozzle_system_data,
        nr_of_vectoring_nozzle_systems.into(),
    )
    .parse(input)?;

    let body = SEES::builder()
        .with_originating_entity_id(originating_entity_id)
        .with_infrared_signature_representation_index(ir_signature)
        .with_acoustic_signature_representation_index(acoustic_signature)
        .with_radar_cross_section_representation_index(rcs_signature)
        .with_propulsion_systems(propulsion_systems)
        .with_vectoring_nozzle_systems(vectoring_nozzle_systems)
        .build();

    Ok((input, body.into_pdu_body()))
}

fn propulsion_system_data(input: &[u8]) -> IResult<&[u8], PropulsionSystemData> {
    let (input, power_setting) = be_f32(input)?;
    let (input, engine_rpm) = be_f32(input)?;

    Ok((
        input,
        PropulsionSystemData::default()
            .with_power_setting(power_setting)
            .with_engine_rpm(engine_rpm),
    ))
}

fn vectoring_nozzle_system_data(input: &[u8]) -> IResult<&[u8], VectoringNozzleSystemData> {
    let (input, horizontal) = be_f32(input)?;
    let (input, vertical) = be_f32(input)?;

    Ok((
        input,
        VectoringNozzleSystemData::default()
            .with_horizontal_deflection_angle(horizontal)
            .with_vertical_deflection_angle(vertical),
    ))
}
