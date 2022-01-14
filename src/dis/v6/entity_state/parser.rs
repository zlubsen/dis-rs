use nom::IResult;
use nom::multi::count;
use nom::number::complete::{u8, be_u16, be_u8, be_f32, be_f64};
use nom::bits::complete::take;
use crate::dis::v6::entity_state::model::{AirPlatformsRecord, Appearance, ArticulationParameter, Camouflage, Country, DrParameters, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityId, EntityKind, EntityLights, EntityMarking, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityState, EntityTrailingEffect, EntityType, EnvironmentalsRecord, ForceId, FrozenStatus, GeneralAppearance, GuidedMunitionsRecord, LandPlatformsRecord, Launcher, LifeFormsRecord, Location, Orientation, PowerPlantStatus, Ramp, SimulationAddress, SpacePlatformsRecord, SpecificAppearance, State, SubsurfacePlatformsRecord, SurfacePlatformRecord, Tent, VectorF32};
use crate::dis::v6::entity_state::model::Concealed::Concealed;
use crate::dis::v6::model::{Pdu, PduHeader};

pub fn entity_state_body(header: PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], Pdu> {
    move |input: &[u8]| {
        let (input, entity_id_val) = entity_id(input)?;
        let (input, force_id_val) = force_id(input)?;
        let (input, articulated_parts_no) = u8(input)?;
        let (input, entity_type_val) = entity_type(input)?;
        let (input, alternative_entity_type) = entity_type(input)?;
        let (input, entity_linear_velocity) = velocity(input)?;
        let (input, entity_location) = location(input)?; // struct - 3x f64 be
        let (input, entity_orientation) = orientation(input)?; // struct - 3x f32 be
        let (input, entity_appearance) = appearance(&entity_type_val)(input)?; // struct
        let (input, dead_reckoning_parameters) = dr_parameters(input)?; // struct
        let (input, entity_marking) = entity_marking(input)?; // struct
        let (input, entity_capabilities) = entity_capabilities(input)?; // struct
        let (input, articulation_parameter) = if articulated_parts_no > 0 {
            let (input, params) = count(articulation_record, articulated_parts_no as usize)(input)?;
            (input, Some(params))
        } else { (input, None) };

        todo!();
        let pdu = EntityState::builder()
            .header(header)
            // TODO insert all fields
            .build();
        Ok((input, Pdu::EntityState(pdu.unwrap())))
    }
}

fn entity_id(input: &[u8]) -> IResult<&[u8], EntityId> {
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
    let (input, general_appearance) = general_appearance(input);
    let (input, specific_appearance) = specific_appearance(input);
    Ok((input, Appearance {
        general_appearance,
        specific_appearance,
    }))
}

fn general_appearance(input: &[u8]) -> IResult<&[u8], GeneralAppearance> {
    let offset = 0usize;
    let ((input, offset), entity_paint_scheme) = take(1)((input, offset));
    let ((input, offset), entity_mobility_kill) = take(1)((input, offset));
    let ((input, offset), entity_fire_power) = take(1)((input, offset));
    let ((input, offset), entity_damage) = take(2)((input, offset));
    let ((input, offset), entity_smoke) = take(2)((input, offset));
    let ((input, offset), entity_trailing_effect) = take(2)((input, offset));
    let ((input, offset), entity_hatch_state) = take(3)((input, offset));
    let ((input, offset), entity_lights) = take(3)((input, offset));
    let ((input, _offset), entity_flaming_effect) = take(1)((input, offset));

    Ok((input, GeneralAppearance{
        entity_paint_scheme : EntityPaintScheme::from(entity_paint_scheme),
        entity_mobility_kill : EntityMobilityKill::from(entity_mobility_kill),
        entity_fire_power : EntityFirePower::from(entity_fire_power),
        entity_damage : EntityDamage::from(entity_damage),
        entity_smoke : EntitySmoke::from(entity_smoke),
        entity_trailing_effect : EntityTrailingEffect::from(entity_trailing_effect),
        entity_hatch_state : EntityHatchState::from(entity_hatch_state),
        entity_lights : EntityLights::from(entity_lights),
        entity_flaming_effect : EntityFlamingEffect::from(entity_flaming_effect),
    }))
}

fn specific_appearance(entity_type: &EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], SpecificAppearance> {
    todo!();
    match entity_type.kind {
        // LandPlatform
        // AirPlatform
        // SurfacePlatform
        // SubsurfacePlatform
        // SpacePlatform
        // GuidedMunition
        // LifeForm
        // Environmental
    }
}

fn land_platform_record(input: &[u8]) -> IResult<&[u8], LandPlatformsRecord> {
    let offset = 0usize;
    let ((input, offset), launcher) = take(1)((input, offset));
    let ((input, offset), camouflage) = take(2)((input, offset));
    let ((input, offset), concealed) = take(1)((input, offset));
    let ((input, offset), frozen_status) = take(1)((input, offset));
    let ((input, offset), power_plant_status) = take(1)((input, offset));
    let ((input, offset), state) = take(1)((input, offset));
    let ((input, offset), tent) = take(1)((input, offset));
    let ((input, offset), ramp) = take(1)((input, offset));
    let ((input, offset), _) = take(6)((input, offset)); // entity specific field / unused

    Ok((input, LandPlatformsRecord {
        launcher: Launcher::from(launcher),
        camouflage_type: Camouflage::from(camouflage),
        concealed: Concealed::from(concealed),
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
        tent: Tent::from(tent),
        ramp: Ramp::from(ramp),
    }))
}

fn air_platform_record(input: &[u8]) -> IResult<&[u8], AirPlatformsRecord> {
    todo!()
}

fn surface_platform_record(input: &[u8]) -> IResult<&[u8], SurfacePlatformRecord> {
    todo!()
}

fn subsurface_platforms_record(input: &[u8]) -> IResult<&[u8], SubsurfacePlatformsRecord> {
    todo!()
}

fn space_platforms_record(input: &[u8]) -> IResult<&[u8], SpacePlatformsRecord> {
    todo!()
}

fn guided_munitions_record(input: &[u8]) -> IResult<&[u8], GuidedMunitionsRecord> {
    todo!()
}

fn life_forms_record(input: &[u8]) -> IResult<&[u8], LifeFormsRecord> {
    todo!()
}

fn environmentals_record(input: &[u8]) -> IResult<&[u8], EnvironmentalsRecord> {
    todo!()
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

