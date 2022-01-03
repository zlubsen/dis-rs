use nom::IResult;
use nom::multi::count;
use nom::number::complete::{u8, f32, be_u16, be_u8, be_f32, be_f64};
use nom::number::Endianness::Big;
use crate::dis::v6::entity_state::model::{Appearance, ArticulationParameter, Country, DrParameters, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityId, EntityKind, EntityLights, EntityMarking, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityState, EntityTrailingEffect, EntityType, ForceId, GeneralAppearance, Location, Orientation, SimulationAddress, VectorF32};
use crate::dis::v6::model::{Pdu, PduHeader};

pub fn entity_state_body(input: &[u8], header: PduHeader) -> IResult<&[u8], Pdu> {
    let (input, entity_id) = entity_id(input)?;
    let (input, force_id) = force_id(input)?;
    let (input, articulated_parts_no) = u8(input)?;
    let (input, entity_type) = entity_type(input)?;
    let (input, alternative_entity_type) = entity_type(input)?;
    let (input, entity_linear_velocity) = velocity(input)?;
    let (input, entity_location) = location(input)?; // struct - 3x f64 be
    let (input, entity_orientation) = orientation(input)?; // struct - 3x f32 be
    let (input, entity_appearance) = appearance(input)?; // struct
    let (input, dead_reckoning_parameters) = dr_parameters(input)?; // struct
    let (input, entity_marking) = entity_marking(input)?; // struct
    let (input, entity_capabilities) = entity_capabilities(input)?; // struct
    let (input, articulation_parameter) =  match articulated_parts_no {
        0 => (input, None),
        num_records => Some(articulation_records(input, num_records)),
    };

    Ok((input, Pdu::EntityState(
        EntityState::builder()
            // TODO insert all fields
            .build())))
}

fn entity_id(input: &[u8]) -> IResult<&[u8], EntityId> {
    let (input, site_id) = be_u16(input)?;
    let (input, application_id) = be_u16(input)?;
    let (input, entity_id) = be_u16(input)?;
    Ok((input, EntityId {
        simulation_address : SimulationAddress {
            site_id, application_id },
        entity_id }))
}

fn force_id(input: &[u8]) -> IResult<&[u8], ForceId> {
    let (input, force_id) = be_u8(input)?;
    Ok((input, ForceId::from(force_id)))
}

fn entity_type(input: &[u8]) -> IResult<&[u8], EntityType> {
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
        extra
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

fn velocity(input: &[u8]) -> IResult<&[u8], VectorF32> {
    let (input, velocities) = count(be_f32, 3)(input)?;
    Ok((input, VectorF32 {
        first_vector_component: *velocities.get(0).expect("Value supposed to be parsed successfully"),
        second_vector_component: *velocities.get(1).expect("Value supposed to be parsed successfully"),
        third_vector_component: *velocities.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

fn location(input: &[u8]) -> IResult<&[u8], Location> {
    let (input, locations) = count(be_f64, 3)(input)?;
    Ok((input, Location {
        x_coordinate: *locations.get(0).expect("Value supposed to be parsed successfully"),
        y_coordinate: *locations.get(1).expect("Value supposed to be parsed successfully"),
        z_coordinate: *locations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

fn orientation(input: &[u8]) -> IResult<&[u8], Orientation> {
    let (input, orientations) = count(be_f32, 3)(input)?;
    Ok((input, Orientation {
        psi: *orientations.get(0).expect("Value supposed to be parsed successfully"),
        theta: *orientations.get(1).expect("Value supposed to be parsed successfully"),
        phi: *orientations.get(2).expect("Value supposed to be parsed successfully"),
    }))
}

fn appearance(input: &[u8]) -> IResult<&[u8], Appearance> {
    todo!();
    Ok((input, Appearance {
        general_appearance: GeneralAppearance {
            entity_paint_scheme: EntityPaintScheme::UniformColor,
            entity_mobility_kill: EntityMobilityKill::NoMobilityKill,
            entity_fire_power: EntityFirePower::NoFirePowerKill,
            entity_damage: EntityDamage::NoDamage,
            entity_smoke: EntitySmoke::NotSmoking,
            entity_trailing_effect: EntityTrailingEffect::None,
            entity_hatch_state: EntityHatchState::NotApplicable,
            entity_lights: EntityLights::None,
            entity_flaming_effect: EntityFlamingEffect::None
        },
        specific_appearance: ()
    }))
}

fn dr_parameters(input: &[u8]) -> IResult<&[u8], DrParameters> {
    todo!()
}

fn entity_marking(input: &[u8]) -> IResult<&[u8], EntityMarking> {
    todo!()
}

fn entity_capabilities(input: &[u8]) -> IResult<&[u8], EntityCapabilities> {
    todo!()
}

fn articulation_records(input: &[u8], num_records: u8) -> IResult<&[u8], Vec<ArticulationParameter>> {
    let (input, records) =
        count(articulation_record, num_records as usize)(input)?;
    Ok((input, records))
}

fn articulation_record(input: &[u8]) -> IResult<&[u8], ArticulationParameter> {
    todo!()
}

