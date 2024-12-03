use crate::aggregate_state::model::{
    aggregate_state_intermediate_length_padding, AggregateMarking, AggregateState, AggregateType,
    SilentAggregateSystem, SilentEntitySystem,
};
use crate::common::parser::{
    entity_id, entity_type, location, orientation, sanitize_marking, variable_datum, vec3_f32,
};
use crate::entity_state::parser::{entity_appearance, force_id};
use crate::enumerations::{
    AggregateStateAggregateKind, AggregateStateAggregateState, AggregateStateFormation,
    AggregateStateSpecific, AggregateStateSubcategory, Country, EntityMarkingCharacterSet,
    PlatformDomain,
};
use crate::model::PduBody;
use nom::bytes::complete::take;
use nom::multi::count;
use nom::number::complete::{be_u16, be_u32, be_u8};
use nom::IResult;

pub(crate) fn aggregate_state_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, aggregate_id) = entity_id(input)?;
    let (input, force_id) = force_id(input)?;
    let (input, aggregate_state) = be_u8(input)?;
    let aggregate_state = AggregateStateAggregateState::from(aggregate_state);
    let (input, aggregate_type) = aggregate_type(input)?;
    let (input, formation) = be_u32(input)?;
    let formation = AggregateStateFormation::from(formation);
    let (input, aggregate_marking) = aggregate_marking(input)?;
    let (input, dimensions) = vec3_f32(input)?;
    let (input, orientation) = orientation(input)?;
    let (input, center_of_mass) = location(input)?;
    let (input, velocity) = vec3_f32(input)?;

    let (input, number_of_aggregates) = be_u16(input)?;
    let (input, number_of_entities) = be_u16(input)?;
    let (input, number_of_silent_aggregates) = be_u16(input)?;
    let (input, number_of_silent_entities) = be_u16(input)?;

    let (input, aggregates) = count(entity_id, number_of_aggregates.into())(input)?;
    let (input, entities) = count(entity_id, number_of_entities.into())(input)?;

    let (_intermediate_length, padding_length) =
        aggregate_state_intermediate_length_padding(&aggregates, &entities);

    let (input, _padding) = take(padding_length)(input)?;

    let (input, silent_aggregate_systems) =
        count(silent_aggregate_system, number_of_silent_aggregates.into())(input)?;
    let (input, silent_entity_systems) =
        count(silent_entity_system, number_of_silent_entities.into())(input)?;

    let (input, number_of_variable_datums) = be_u32(input)?;
    let (input, variable_datums) =
        count(variable_datum, number_of_variable_datums as usize)(input)?;

    Ok((
        input,
        AggregateState::builder()
            .with_aggregate_id(aggregate_id)
            .with_force_id(force_id)
            .with_aggregate_state(aggregate_state)
            .with_aggregate_type(aggregate_type)
            .with_formation(formation)
            .with_aggregate_marking(aggregate_marking)
            .with_dimensions(dimensions)
            .with_orientation(orientation)
            .with_center_of_mass(center_of_mass)
            .with_velocity(velocity)
            .with_aggregates(aggregates)
            .with_entities(entities)
            .with_silent_aggregate_systems(silent_aggregate_systems)
            .with_silent_entity_systems(silent_entity_systems)
            .with_variable_datums(variable_datums)
            .build()
            .into_pdu_body(),
    ))
}

fn aggregate_type(input: &[u8]) -> IResult<&[u8], AggregateType> {
    let (input, aggregate_kind) = be_u8(input)?;
    let aggregate_kind = AggregateStateAggregateKind::from(aggregate_kind);
    let (input, domain) = be_u8(input)?;
    let domain = PlatformDomain::from(domain);
    let (input, country) = be_u16(input)?;
    let country = Country::from(country);
    let (input, category) = be_u8(input)?;
    let (input, subcategory) = be_u8(input)?;
    let subcategory = AggregateStateSubcategory::from(subcategory);
    let (input, specific) = be_u8(input)?;
    let specific = AggregateStateSpecific::from(specific);
    let (input, extra) = be_u8(input)?;

    Ok((
        input,
        AggregateType {
            aggregate_kind,
            domain,
            country,
            category,
            subcategory,
            specific,
            extra,
        },
    ))
}

fn aggregate_marking(input: &[u8]) -> IResult<&[u8], AggregateMarking> {
    let mut buf: [u8; 31] = [0; 31];
    let (input, marking_character_set) = be_u8(input)?;
    let marking_character_set = EntityMarkingCharacterSet::from(marking_character_set);
    let (input, ()) = nom::multi::fill(be_u8, &mut buf)(input)?;

    let marking_string = sanitize_marking(&buf[..]);

    Ok((
        input,
        AggregateMarking {
            marking_character_set,
            marking_string,
        },
    ))
}

fn silent_aggregate_system(input: &[u8]) -> IResult<&[u8], SilentAggregateSystem> {
    let (input, number_of_aggregates) = be_u16(input)?;
    let (input, _padding) = be_u16(input)?;
    let (input, aggregate_type) = aggregate_type(input)?;

    Ok((
        input,
        SilentAggregateSystem::default()
            .with_number_of_aggregates(number_of_aggregates)
            .with_aggregate_type(aggregate_type),
    ))
}

fn silent_entity_system(input: &[u8]) -> IResult<&[u8], SilentEntitySystem> {
    let (input, number_of_entities) = be_u16(input)?;
    let (input, number_of_appearance_records) = be_u16(input)?;
    let (input, entity_type) = entity_type(input)?;
    let (input, appearances) = count(
        entity_appearance(entity_type),
        number_of_appearance_records.into(),
    )(input)?;

    Ok((
        input,
        SilentEntitySystem::default()
            .with_number_of_entities(number_of_entities)
            .with_entity_type(entity_type)
            .with_appearances(appearances),
    ))
}
