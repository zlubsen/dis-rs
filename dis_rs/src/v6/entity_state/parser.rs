use nom::{bits, IResult};
use nom::error::Error;
use nom::sequence::tuple;
use nom::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use crate::enumerations::{EntityKind, PlatformDomain};
use crate::EntityType;
use crate::v6::entity_state::model::{ActivityState, Afterburner, AirPlatformsRecord, Appearance, Camouflage, Concealed, Density, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityLights, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityTrailingEffect, EnvironmentalsRecord, FrozenStatus, GeneralAppearance, GuidedMunitionsRecord, LandPlatformsRecord, Launcher, LaunchFlash, LifeFormsRecord, LifeFormsState, PowerPlantStatus, Ramp, SpacePlatformsRecord, SpecificAppearance, State, SubsurfacePlatformsRecord, SurfacePlatformRecord, Tent, Weapon};

pub fn appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], Appearance> {
    move | input: &[u8] | {
        let (input, general_appearance) = general_appearance(input)?;
        let (input, specific_appearance) = specific_appearance(entity_type)(input)?;
        Ok((input, Appearance {
            general_appearance,
            specific_appearance,
        }))
    }
}

#[allow(clippy::type_complexity)]
pub fn general_appearance(input: &[u8]) -> IResult<&[u8], GeneralAppearance> {
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
        entity_paint_scheme : EntityPaintScheme::from(entity_paint_scheme as u16),
        entity_mobility_kill : EntityMobilityKill::from(entity_mobility_kill as u16),
        entity_fire_power : EntityFirePower::from(entity_fire_power as u16),
        entity_damage : EntityDamage::from(entity_damage as u16),
        entity_smoke : EntitySmoke::from(entity_smoke as u16),
        entity_trailing_effect : EntityTrailingEffect::from(entity_trailing_effect as u16),
        entity_hatch_state : EntityHatchState::from(entity_hatch_state as u16),
        entity_lights : EntityLights::from(entity_lights as u16),
        entity_flaming_effect : EntityFlamingEffect::from(entity_flaming_effect as u16),
    }))
}

fn specific_appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], SpecificAppearance> {
    move |input: &[u8]| {
        // FIXME it seems the bit-level parsers do not consume the bytes from the input.
        // domain codes are defined as part of the Entity Type Database.
        let (input, appearance) = match (entity_type.kind, entity_type.domain) {
            (EntityKind::Platform, PlatformDomain::Land) => { // land
                let (input, record) = land_platform_record(input)?;
                (input, SpecificAppearance::LandPlatform(record))
            }
            (EntityKind::Platform, PlatformDomain::Air) => { // air
                let (input, record) = air_platform_record(input)?;
                (input, SpecificAppearance::AirPlatform(record))
            }
            (EntityKind::Platform, PlatformDomain::Surface) => { // surface
                let (input, record) = surface_platform_record(input)?;
                (input, SpecificAppearance::SurfacePlatform(record))
            }
            (EntityKind::Platform, PlatformDomain::Subsurface) => { // subsurface
                let (input, record) = subsurface_platforms_record(input)?;
                (input, SpecificAppearance::SubsurfacePlatform(record))
            }
            (EntityKind::Platform, PlatformDomain::Space) => { // space
                let (input, record) = space_platforms_record(input)?;
                (input, SpecificAppearance::SpacePlatform(record))
            }
            (EntityKind::Platform, _) => { // other platform, 0 and unspecified
                let (input, record) = other_specific_appearance(input)?;
                (input, SpecificAppearance::Other(record))
            }
            (EntityKind::Munition, _) => { // guided munitions // FIXME more specific than just entity kind 'Munition'?
                let (input, record) = guided_munitions_record(input)?;
                (input, SpecificAppearance::GuidedMunition(record))
            }
            (EntityKind::Lifeform, PlatformDomain::Land) => { // lifeform
                let (input, record) = life_forms_record(input)?;
                (input, SpecificAppearance::LifeForm(record))
            }
            (EntityKind::Environmental, _) => { // environmental
                let (input, record) = environmentals_record(input)?;
                (input, SpecificAppearance::Environmental(record))
            }
            (_, _) => {
                let (input, record) = other_specific_appearance(input)?;
                (input, SpecificAppearance::Other(record))
            }
        };
        Ok((input, appearance))
    }
}

fn other_specific_appearance(input: &[u8]) -> IResult<&[u8], [u8;2]> {
    if let Ok((input,slice)) = take_bytes::<usize, &[u8], Error<&[u8]>>(2usize)(input) {
        let two_bytes : [u8;2] = slice.try_into().unwrap();
        Ok((input, two_bytes))
    } else {
        Ok((input, [ 0, 0 ]))
    }
}

#[allow(clippy::type_complexity)]
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
        launcher: Launcher::from(launcher as u16),
        camouflage_type: Camouflage::from(camouflage as u16),
        concealed: Concealed::from(concealed as u16),
        frozen_status: FrozenStatus::from(frozen_status as u16),
        power_plant_status: PowerPlantStatus::from(power_plant_status as u16),
        state: State::from(state as u16),
        tent: Tent::from(tent as u16),
        ramp: Ramp::from(ramp as u16),
    }))
}

#[allow(clippy::type_complexity)]
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
        afterburner: Afterburner::from(afterburner as u16),
        frozen_status: FrozenStatus::from(frozen_status as u16),
        power_plant_status: PowerPlantStatus::from(power_plant_status as u16),
        state: State::from(state as u16),
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
        frozen_status: FrozenStatus::from(frozen_status as u16),
        power_plant_status: PowerPlantStatus::from(power_plant_status as u16),
        state: State::from(state as u16),
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
        frozen_status: FrozenStatus::from(frozen_status as u16),
        power_plant_status: PowerPlantStatus::from(power_plant_status as u16),
        state: State::from(state as u16),
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
        frozen_status: FrozenStatus::from(frozen_status as u16),
        power_plant_status: PowerPlantStatus::from(power_plant_status as u16),
        state: State::from(state as u16),
    }))
}

#[allow(clippy::type_complexity)]
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
        launch_flash: LaunchFlash::from(launch_flash as u16),
        frozen_status: FrozenStatus::from(frozen_status as u16),
        state: State::from(state as u16),
    }))
}

#[allow(clippy::type_complexity)]
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
        life_form_state: LifeFormsState::from(life_form_state as u16),
        frozen_status: FrozenStatus::from(frozen_status as u16),
        activity_state: ActivityState::from(activity_state as u16),
        weapon_1: Weapon::from(weapon_1 as u16),
        weapon_2: Weapon::from(weapon_2 as u16),
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
        density: Density::from(density as u16),
    }))
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
