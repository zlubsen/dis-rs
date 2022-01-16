use nom::{bits, IResult};
use nom::multi::count;
use nom::number::complete::{u8, be_u16, be_u8, be_f32, be_f64};
use nom::bits::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use nom::error::{Error};
use nom::sequence::tuple;
use crate::dis::v6::entity_state::model::{ActivityState, Afterburner, AirPlatformsRecord, Appearance, ArticulationParameter, Camouflage, Concealed, Country, Density, DrParameters, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityId, EntityKind, EntityLights, EntityMarking, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityState, EntityTrailingEffect, EntityType, EnvironmentalsRecord, ForceId, FrozenStatus, GeneralAppearance, GuidedMunitionsRecord, LandPlatformsRecord, Launcher, LaunchFlash, LifeFormsRecord, LifeFormsState, Location, Orientation, PowerPlantStatus, Ramp, SimulationAddress, SpacePlatformsRecord, SpecificAppearance, State, SubsurfacePlatformsRecord, SurfacePlatformRecord, Tent, VectorF32, Weapon};
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
        let (input, entity_appearance) = appearance(entity_type_val.clone())(input)?; // struct
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

fn appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], Appearance> {
    move | input: &[u8] | {
        let (input, general_appearance) = general_appearance(input)?;
        let (input, specific_appearance) = specific_appearance(entity_type.clone())(input)?;
        Ok((input, Appearance {
            general_appearance,
            specific_appearance,
        }))
    }
}

fn general_appearance(input: &[u8]) -> IResult<&[u8], GeneralAppearance> {
    let (input, (
        entity_paint_scheme,
        entity_mobility_kill,
        entity_fire_power,
        entity_damage,
        entity_smoke,
        entity_trailing_effect,
        entity_hatch_state,
        entity_lights,
        entity_flaming_effect)) : (&[u8], (u8,u8,u8,u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(
        tuple(
            (take_bits(1usize),
             take_bits(1usize),
             take_bits(1usize),
             take_bits(2usize),
             take_bits(2usize),
             take_bits(2usize),
             take_bits(3usize),
             take_bits(3usize),
             take_bits(1usize))))(input)?;

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

fn specific_appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], SpecificAppearance> {
    move |input: &[u8]| {
        // TODO how to match is TBD, domain is part of the Entity Type Database.
        let appearance = match (entity_type.kind, entity_type.domain) {
            (EntityKind::Platform, _) => { SpecificAppearance::LandPlatform(land_platform_record(input)?.1) } // land
            // TODO distinguish domains
            // (EntityKind::Platform, _) => { SpecificAppearance::AirPlatform(air_platform_record(input)?.1) } // air
            // (EntityKind::Platform, _) => { SpecificAppearance::SurfacePlatform(surface_platform_record(input)?.1) } // surface
            // (EntityKind::Platform, _) => { SpecificAppearance::SubsurfacePlatform(subsurface_platforms_record(input)?.1) } // subsurface
            // (EntityKind::Platform, _) => { SpecificAppearance::SpacePlatform(space_platforms_record(input)?.1) } // space
            (EntityKind::Munition, _) => { SpecificAppearance::GuidedMunition(guided_munitions_record(input)?.1) } // guided munition
            (EntityKind::LifeForm, _) => { SpecificAppearance::LifeForm(life_forms_record(input)?.1) } // lifeform
            (EntityKind::Environmental, _) => { SpecificAppearance::Environmental(environmentals_record(input)?.1) } // environmental
            (_, _) => { SpecificAppearance::Other(other_specific_appearance(input)?.1) }
        };
        Ok((input, appearance))
    }
}

fn other_specific_appearance(input: &[u8]) -> IResult<&[u8], [u8;2]> {
    if let Ok((input,slice)) = take_bytes(2usize)(input) {
        let two_bytes : [u8;2] = [slice[0], slice[1]];
        Ok((input, two_bytes))
    } else {
        Ok((input, [ 0, 0 ]))
    }
}

fn land_platform_record(input: &[u8]) -> IResult<&[u8], LandPlatformsRecord> {
    let (input,
        (launcher,
            camouflage,
            concealed,
            frozen_status,
            power_plant_status,
            state,
            tent,
            ramp,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(2usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(6usize))))(input)?;

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
    let (input,
        (afterburner,
            _unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(4usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, AirPlatformsRecord {
        afterburner: Afterburner::from(afterburner),
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn surface_platform_record(input: &[u8]) -> IResult<&[u8], SurfacePlatformRecord> {
    let (input,
        (_unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(5usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, SurfacePlatformRecord {
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn subsurface_platforms_record(input: &[u8]) -> IResult<&[u8], SubsurfacePlatformsRecord> {
    let (input,
        (_unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(5usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, SubsurfacePlatformsRecord {
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn space_platforms_record(input: &[u8]) -> IResult<&[u8], SpacePlatformsRecord> {
    let (input,
        (_unused,
            frozen_status,
            power_plant_status,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(5usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, SpacePlatformsRecord {
        frozen_status: FrozenStatus::from(frozen_status),
        power_plant_status: PowerPlantStatus::from(power_plant_status),
        state: State::from(state),
    }))
}

fn guided_munitions_record(input: &[u8]) -> IResult<&[u8], GuidedMunitionsRecord> {
    let (input,
        (launch_flash,
            _unused_1,
            frozen_status,
            _unused_2,
            state,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(1usize),
         take_bits(4usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(8usize))))(input)?;

    Ok((input, GuidedMunitionsRecord {
        launch_flash: LaunchFlash::from(launch_flash),
        frozen_status: FrozenStatus::from(frozen_status),
        state: State::from(state),
    }))
}

fn life_forms_record(input: &[u8]) -> IResult<&[u8], LifeFormsRecord> {
    let (input,
        (life_form_state,
            _unused_1,
            frozen_status,
            _unused_2,
            activity_state,
            weapon_1,
            weapon_2,
            _pad_out)) : (&[u8], (u8,u8,u8,u8,u8,u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(4usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(1usize),
         take_bits(2usize),
         take_bits(2usize),
         take_bits(4usize))))(input)?;

    Ok((input, LifeFormsRecord {
        life_form_state: LifeFormsState::from(life_form_state),
        frozen_status: FrozenStatus::from(frozen_status),
        activity_state: ActivityState::from(activity_state),
        weapon_1: Weapon::from(weapon_1),
        weapon_2: Weapon::from(weapon_2),
    }))
}

fn environmentals_record(input: &[u8]) -> IResult<&[u8], EnvironmentalsRecord> {
    let (input,
        (density,
            _unused,
            _pad_out)) : (&[u8], (u8,u8,u8)) = bits::<_,_,Error<(&[u8], usize)>,_,_>(tuple(
        (take_bits(4usize),
         take_bits(4usize),
         take_bits(8usize))))(input)?;

    Ok((input, EnvironmentalsRecord {
        density: Density::from(density),
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

