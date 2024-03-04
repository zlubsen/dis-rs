use nom::IResult;
use nom::number::complete::be_u16;
use crate::common::parser::{entity_id, vec3_f32};
use crate::enumerations::{IsPartOfNature, IsPartOfPosition, StationName};
use crate::is_part_of::model::{IsPartOf, NamedLocationId, Relationship};
use crate::model::PduBody;

pub(crate) fn is_part_of_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, originating_sim_id) = entity_id(input)?;
    let (input, receiving_entity_id) = entity_id(input)?;
    let (input, relationship) = relationship(input)?;
    let (input, part_location) = vec3_f32(input)?;
    let (input, named_location_id) = named_location_id(input)?;

    Ok((input, IsPartOf::builder()
        .with_originating_simulation_id(originating_sim_id)
        .with_receiving_entity_id(receiving_entity_id)
        .with_relationship(relationship)
        .with_part_location(part_location)
        .with_named_location_id(named_location_id)
        .build()
        .into_pdu_body()))
}

fn relationship(input: &[u8]) -> IResult<&[u8], Relationship> {
    let (input, nature) = be_u16(input)?;
    let nature = IsPartOfNature::from(nature);
    let (input, position) = be_u16(input)?;
    let position = IsPartOfPosition::from(position);

    Ok((input, Relationship::default()
        .with_nature(nature)
        .with_position(position)))
}

fn named_location_id(input: &[u8]) -> IResult<&[u8], NamedLocationId> {
    let (input, name) = be_u16(input)?;
    let name = StationName::from(name);
    let (input, number) = be_u16(input)?;

    Ok((input, NamedLocationId::default()
        .with_station_name(name)
        .with_station_number(number)))
}