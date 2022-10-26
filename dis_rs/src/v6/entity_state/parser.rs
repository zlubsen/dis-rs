use nom::{bits, IResult};
use nom::error::Error;
use nom::sequence::tuple;
use nom::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use nom::multi::count;
use nom::number::complete::be_u8;
use crate::v6::entity_state::model::EntityCapabilities;
use crate::{EntityState, PduBody};
use crate::common::entity_state::parser::{articulation_record, entity_appearance, entity_marking};
use crate::common::parser;

pub fn entity_state_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, entity_id_val) = parser::entity_id(input)?;
    let (input, force_id_val) = crate::common::entity_state::parser::force_id(input)?;
    let (input, articulated_parts_no) = be_u8(input)?;
    let (input, entity_type_val) = parser::entity_type(input)?;
    let (input, alternative_entity_type) = parser::entity_type(input)?;
    let (input, entity_linear_velocity) = parser::vec3_f32(input)?;
    let (input, entity_location) = parser::location(input)?;
    let (input, entity_orientation) = parser::orientation(input)?;
    let (input, entity_appearance) = entity_appearance(entity_type_val)(input)?;
    let (input, dead_reckoning_parameters) = crate::common::entity_state::parser::dr_parameters(input)?;
    let (input, entity_marking) = entity_marking(input)?;
    let (input, entity_capabilities) = entity_capabilities(input)?;
    let (input, articulation_parameter) = if articulated_parts_no > 0 {
        let (input, params) = count(articulation_record, articulated_parts_no as usize)(input)?;
        (input, Some(params))
    } else { (input, None) };

    // TODO replace custom builder with buildstructor
    let builder = EntityState::builder()
        // .header(header)
        .entity_id(entity_id_val)
        .force_id(force_id_val)
        .entity_type(entity_type_val)
        .alt_entity_type(alternative_entity_type)
        .linear_velocity(entity_linear_velocity)
        .location(entity_location)
        .orientation(entity_orientation)
        .appearance(entity_appearance)
        .dead_reckoning(dead_reckoning_parameters)
        .marking(entity_marking)
        .capabilities(entity_capabilities);
    let builder = if let Some(params) = articulation_parameter {
        builder.add_articulation_parameters_vec(params)
    } else { builder };
    let body = builder.build();

    Ok((input, body.unwrap()))
}

pub fn entity_capabilities(input: &[u8]) -> IResult<&[u8], EntityCapabilities> {
    let (input,
        (ammunition_supply,
            fuel_supply,
            recovery,
            repair,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(3usize))))(input)?;
    let (input, _pad_3_bytes) = take_bytes(3usize)(input)?;

    Ok((input, EntityCapabilities {
        ammunition_supply: ammunition_supply != 0,
        fuel_supply: fuel_supply != 0,
        recovery: recovery != 0,
        repair: repair != 0,
    }))
}
