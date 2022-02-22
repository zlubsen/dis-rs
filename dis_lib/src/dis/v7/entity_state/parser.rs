use nom::{bits, IResult};
use nom::multi::count;
use nom::number::complete::{u8, be_u16, be_u8, be_f32, be_u32};
use nom::bits::complete::take as take_bits;
use nom::bytes::complete::take as take_bytes;
use nom::error::{Error};
use nom::sequence::tuple;
use crate::dis::common::entity_state::model::{EntityKind, EntityType};
use crate::dis::common::entity_state::parser;
use crate::dis::v7::entity_state::model::{ActivityState, Afterburner, AirPlatformsRecord, ApTypeMetric, Appearance, ApTypeDesignator, ArticulatedParts, ArticulationParameter, Camouflage, Concealed, Density, DrAlgorithm, DrParameters, EntityCapabilities, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityLights, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityState, EntityTrailingEffect, EnvironmentalsRecord, FrozenStatus, GeneralAppearance, GuidedMunitionsRecord, LandPlatformsRecord, Launcher, LaunchFlash, LifeFormsRecord, LifeFormsState, ParameterTypeVariant, PowerPlantStatus, Ramp, SpacePlatformsRecord, SpecificAppearance, State, SubsurfacePlatformsRecord, SurfacePlatformRecord, Tent, Weapon};
use crate::dis::v7::model::{Pdu, PduHeader};

pub fn entity_state_body(header: PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], Pdu> {
    move |input: &[u8]| {
        let (input, entity_id_val) = parser::entity_id(input)?;
        let (input, force_id_val) = parser::force_id(input)?;
        let (input, articulated_parts_no) = u8(input)?;
        let (input, entity_type_val) = parser::entity_type(input)?;
        let (input, alternative_entity_type) = parser::entity_type(input)?;
        let (input, entity_linear_velocity) = parser::vec3_f32(input)?;
        let (input, entity_location) = parser::location(input)?;
        let (input, entity_orientation) = parser::orientation(input)?;
        let (input, entity_appearance) = appearance(entity_type_val.clone())(input)?;
        let (input, dead_reckoning_parameters) = dr_parameters(input)?;

        // FIXME before the marking two few bytes are read;
        let (input, entity_marking) = parser::entity_marking(input)?;
        let (input, entity_capabilities) = entity_capabilities(input)?;
        let (input, articulation_parameter) = if articulated_parts_no > 0 {
            let (input, params) = count(articulation_record, articulated_parts_no as usize)(input)?;
            (input, Some(params))
        } else { (input, None) };

        let builder = EntityState::builder()
            .header(header)
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
        let pdu = builder.build();

        Ok((input, Pdu::EntityState(pdu.unwrap())))
    }
}

// TODO adapt to v7 appearance record (UID 31-43)
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
        // domain codes are defined as part of the Entity Type Database > v29.
        let (input, appearance) = match (entity_type.kind, entity_type.domain) {
            (EntityKind::Platform, 1u8) => { // land
                let (input, record) = land_platform_record(input)?;
                (input, SpecificAppearance::LandPlatform(record))
            }
            (EntityKind::Platform, 2u8) => { // air
                let (input, record) = air_platform_record(input)?;
                (input, SpecificAppearance::AirPlatform(record))
            }
            (EntityKind::Platform, 3u8) => { // surface
                let (input, record) = surface_platform_record(input)?;
                (input, SpecificAppearance::SurfacePlatform(record))
            }
            (EntityKind::Platform, 4u8) => { // subsurface
                let (input, record) = subsurface_platforms_record(input)?;
                (input, SpecificAppearance::SubsurfacePlatform(record))
            }
            (EntityKind::Platform, 5u8) => { // space
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
            (EntityKind::LifeForm, 1u8) => { // lifeform
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

// TODO adapt to v7 spec, other parameters field is changed from v6
fn dr_parameters(input: &[u8]) -> IResult<&[u8], DrParameters> {
    let (input, algorithm) = be_u8(input)?;
    let (input, other_parameters) = take_bytes(15usize)(input)?;
    let (input, acceleration) = parser::vec3_f32(input)?;
    let (input, velocity) = parser::vec3_f32(input)?;

    let other_parameters = other_parameters.try_into().unwrap();

    Ok((input, DrParameters {
        algorithm: DrAlgorithm::from(algorithm),
        other_parameters,
        linear_acceleration: acceleration,
        angular_velocity: velocity,
    }))
}

// TODO adapt to v7 enumerations, specific for different type of entities
fn entity_capabilities(input: &[u8]) -> IResult<&[u8], EntityCapabilities> {
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

fn articulation_record(input: &[u8]) -> IResult<&[u8], ArticulationParameter> {
    let (input, parameter_type_designator) = be_u8(input)?;
    let (input, parameter_change_indicator) = be_u8(input)?;
    let (input, articulation_attachment_id) = be_u16(input)?;
    let parameter_type_designator : ApTypeDesignator = ApTypeDesignator::from(parameter_type_designator);
    let (input, parameter_type_variant) = match parameter_type_designator {
        ApTypeDesignator::Articulated => { articulated_part(input)? }
        ApTypeDesignator::Attached => { attached_part(input)? }
    };
    let (input, articulation_parameter_value) = be_f32(input)?;
    let (input, _pad_out) = take_bytes(4usize)(input)?;

    Ok((input, ArticulationParameter {
        parameter_type_designator,
        parameter_change_indicator,
        articulation_attachment_id,
        parameter_type_variant,
        articulation_parameter_value,
    }))
}

fn attached_part(input: &[u8]) -> IResult<&[u8], ParameterTypeVariant> {
    let (input, attached_part) = be_u32(input)?;
    Ok((input, ParameterTypeVariant::AttachedParts(attached_part)))
}

fn articulated_part(input: &[u8]) -> IResult<&[u8], ParameterTypeVariant> {
    let (input, type_varient) = be_u32(input)?;
    let type_metric = (type_varient & 0x1f) as u32;  // 5 least significant bits (0x1f) are the type metric
    let type_class = type_varient - (type_metric as u32);   // rest of the bits (minus type metric value) are the type class

    Ok((input, ParameterTypeVariant::ArticulatedParts(ArticulatedParts {
        type_metric: ApTypeMetric::from(type_metric),
        type_class,
    })))
}