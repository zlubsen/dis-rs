use nom::IResult;
use nom::number::complete::{be_f32, be_f64, be_u16, be_u8};
use nom::multi::count;
use crate::dis::common::entity_state::model::{Country, EntityId, EntityKind, EntityMarking, EntityMarkingCharacterSet, EntityType, ForceId, Location, Orientation, SimulationAddress, VectorF32};

pub fn entity_id(input: &[u8]) -> IResult<&[u8], EntityId> {
    let (input, site_id) = be_u16(input)?;
    let (input, application_id) = be_u16(input)?;
    let (input, entity_id) = be_u16(input)?;
    Ok((input, EntityId {
        simulation_address: SimulationAddress {
            site_id,
            application_id,
        },
        entity_id,
    }))
}

pub fn force_id(input: &[u8]) -> IResult<&[u8], ForceId> {
    let (input, force_id) = be_u8(input)?;
    Ok((input, ForceId::from(force_id)))
}

pub fn entity_type(input: &[u8]) -> IResult<&[u8], EntityType> {
    let (input, kind) = kind(input)?;
    let (input, domain) = be_u8(input)?;
    let (input, country) = country(input)?;
    let (input, category) = be_u8(input)?;
    let (input, subcategory) = be_u8(input)?;
    let (input, specific) = be_u8(input)?;
    let (input, extra) = be_u8(input)?;
    Ok((input, EntityType {
        kind,
        domain,
        country,
        category,
        subcategory,
        specific,
        extra,
    }))
}

fn kind(input: &[u8]) -> IResult<&[u8], EntityKind> {
    let (input, kind) = be_u8(input)?;
    let kind = EntityKind::from(kind);
    Ok((input, kind))
}

fn country(input: &[u8]) -> IResult<&[u8], Country> {
    let (input, country) = be_u16(input)?;
    let country = Country::from(country);
    Ok((input, country))
}

pub fn vec3_f32(input: &[u8]) -> IResult<&[u8], VectorF32> {
    let (input, elements) = count(be_f32, 3)(input)?;
    Ok((input, VectorF32 {
        first_vector_component: *elements.get(0).expect("Value supposed to be parsed successfully"),
        second_vector_component: *elements.get(1).expect("Value supposed to be parsed successfully"),
        third_vector_component: *elements.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

pub fn location(input: &[u8]) -> IResult<&[u8], Location> {
    let (input, locations) = count(be_f64, 3)(input)?;
    Ok((input, Location {
        x_coordinate: *locations.get(0).expect("Value supposed to be parsed successfully"),
        y_coordinate: *locations.get(1).expect("Value supposed to be parsed successfully"),
        z_coordinate: *locations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

pub fn orientation(input: &[u8]) -> IResult<&[u8], Orientation> {
    let (input, orientations) = count(be_f32, 3)(input)?;
    Ok((input, Orientation {
        psi: *orientations.get(0).expect("Value supposed to be parsed successfully"),
        theta: *orientations.get(1).expect("Value supposed to be parsed successfully"),
        phi: *orientations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

// TODO review if this is an efficient way to read the string and trim trailing whitespace
pub fn entity_marking(input: &[u8]) -> IResult<&[u8], EntityMarking> {
    let mut buf : [u8;11] = [0;11];
    let (input, character_set) = be_u8(input)?;
    let (input, _) = nom::multi::fill(be_u8, &mut buf)(input)?;

    let mut marking = String::from_utf8_lossy(&buf[..]).into_owned();
    marking.truncate(marking.trim_end().len());

    Ok((input, EntityMarking{
        marking_character_set: EntityMarkingCharacterSet::from(character_set),
        marking_string: marking,
    }))
}
