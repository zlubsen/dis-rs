use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u32, be_u8};
use crate::{AttachedPart, DrEulerAngles, DrWorldOrientationQuaternion, EntityAppearance, EntityAssociationParameter, EntityState, EntityType, EntityTypeParameter, PduHeader, SeparationParameter};
use crate::common::entity_state::model::{ArticulatedPart, DrOtherParameters, DrParameters, EntityMarking, VariableParameter};
use crate::common::model::PduBody;
use crate::common::parser;
use crate::common::parser::{entity_id, entity_type, vec3_f32};
use crate::enumerations::*;
use crate::v6::entity_state::parser::entity_capabilities;

pub fn entity_state_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input: &[u8]| {
        let (input, entity_id_val) = entity_id(input)?;
        let (input, force_id_val) = force_id(input)?;
        let (input, variable_parameters_no) = be_u8(input)?;
        let (input, entity_type_val) = entity_type(input)?;
        let (input, alternative_entity_type) = entity_type(input)?;
        let (input, entity_linear_velocity) = vec3_f32(input)?;
        let (input, entity_location) = parser::location(input)?;
        let (input, entity_orientation) = parser::orientation(input)?;
        let (input, entity_appearance) = entity_appearance(entity_type_val)(input)?;
        let (input, dead_reckoning_parameters) = dr_parameters(input)?;
        let (input, entity_marking) = entity_marking(input)?;
        let (input, entity_capabilities) = match header.protocol_version {
            ProtocolVersion::IEEE1278_12012 => {
                crate::v7::entity_state::parser::entity_capabilities(entity_type_val)(input)?
            }
            ProtocolVersion::IEEE1278_1A1998 | _ => {
                let (input, entity_capabilities) = entity_capabilities(input)?;
                (input, crate::enumerations::EntityCapabilities::from(entity_capabilities))
            }
        };
        let (input, variable_parameters) = if variable_parameters_no > 0 {
            count(variable_parameter, variable_parameters_no as usize)(input)?
        } else { (input, vec![]) };

        let entity_state = EntityState::new(entity_id_val, force_id_val, entity_type_val)
            .with_alternative_entity_type(alternative_entity_type)
            .with_velocity(entity_linear_velocity)
            .with_location(entity_location)
            .with_orientation(entity_orientation)
            .with_appearance(entity_appearance)
            .with_dead_reckoning_parameters(dead_reckoning_parameters)
            .with_marking(entity_marking)
            .with_capabilities(entity_capabilities)
            .with_variable_parameters(variable_parameters);

        Ok((input, entity_state.as_pdu_body()))
    }
}

pub fn force_id(input: &[u8]) -> IResult<&[u8], ForceId> {
    let (input, force_id) = be_u8(input)?;
    Ok((input, ForceId::from(force_id)))
}

pub fn entity_appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], EntityAppearance> {
    move |input: &[u8]| {
        let (input, appearance) = be_u32(input)?;
        let appearance = match (entity_type.kind, entity_type.domain) {
            (EntityKind::Other, _) => EntityAppearance::Unspecified(appearance),
            (EntityKind::Platform, PlatformDomain::Land) => EntityAppearance::LandPlatform(LandPlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Air) => EntityAppearance::AirPlatform(AirPlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Surface) => EntityAppearance::SurfacePlatform(SurfacePlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Subsurface) => EntityAppearance::SubsurfacePlatform(SubsurfacePlatformAppearance::from(appearance)),
            (EntityKind::Platform, PlatformDomain::Space) => EntityAppearance::SpacePlatform(SpacePlatformAppearance::from(appearance)),
            (EntityKind::Munition, _) => EntityAppearance::Munition(MunitionAppearance::from(appearance)),
            (EntityKind::Lifeform, _) => EntityAppearance::LifeForms(LifeFormsAppearance::from(appearance)),
            (EntityKind::Environmental, _) => EntityAppearance::Environmental(EnvironmentalAppearance::from(appearance)),
            (EntityKind::Culturalfeature, _) => EntityAppearance::CulturalFeature(CulturalFeatureAppearance::from(appearance)),
            (EntityKind::Supply, _) => EntityAppearance::Supply(SupplyAppearance::from(appearance)),
            (EntityKind::Radio, _) => EntityAppearance::Radio(RadioAppearance::from(appearance)),
            (EntityKind::Expendable, _) => EntityAppearance::Expendable(ExpendableAppearance::from(appearance)),
            (EntityKind::SensorEmitter, _) => EntityAppearance::SensorEmitter(SensorEmitterAppearance::from(appearance)),
            (_, _) => EntityAppearance::Unspecified(appearance)
        };

        Ok((input, appearance))
    }
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

pub fn dr_parameters(input: &[u8]) -> IResult<&[u8], DrParameters> {
    let (input, algorithm) = be_u8(input)?;
    let algorithm = DeadReckoningAlgorithm::from(algorithm);

    // This match statement basically determines the value of the DrParametersType field for Euler and Quaternion variants
    let (input, other_parameters) = match algorithm {
        DeadReckoningAlgorithm::StaticNonmovingEntity |
            DeadReckoningAlgorithm::DRM_FPW_ConstantVelocityLowAccelerationLinearMotionEntity |
            DeadReckoningAlgorithm::DRM_FVW_HighSpeedorManeuveringEntity |
            DeadReckoningAlgorithm::DRM_FPB_SimilartoFPWexceptinBodyCoordinates |
            DeadReckoningAlgorithm::DRM_FVB_SimilartoFVWexceptinBodyCoordinates => {
            dr_other_parameters_euler(input)?
        }
        DeadReckoningAlgorithm::DRM_RPW_ConstantVelocityLowAccelerationLinearMotionEntitywithExtrapolationofOrientation |
            DeadReckoningAlgorithm::DRM_RVW_HighSpeedorManeuveringEntitywithExtrapolationofOrientation |
            DeadReckoningAlgorithm::DRM_RPB_SimilartoRPWexceptinBodyCoordinates |
            DeadReckoningAlgorithm::DRM_RVB_SimilartoRVWexceptinBodyCoordinates => {
            dr_other_parameters_quaternion(input)?
        }
        DeadReckoningAlgorithm::Other | _ => {
            dr_other_parameters_none(input)?
        }
    };

    let (input, acceleration) = vec3_f32(input)?;
    let (input, velocity) = vec3_f32(input)?;

    Ok((input, DrParameters {
        algorithm,
        other_parameters,
        linear_acceleration: acceleration,
        angular_velocity: velocity,
    }))
}

pub fn dr_other_parameters_none(input: &[u8]) -> IResult<&[u8], DrOtherParameters> {
    let (input, params) = take(15usize)(input)?;
    Ok((input, DrOtherParameters::None(params.try_into().unwrap())))
}

pub fn dr_other_parameters_euler(input: &[u8]) -> IResult<&[u8], DrOtherParameters> {
    let (input, _param_type) = be_u8(input)?;
    let (input, _unused) = be_u16(input)?;
    let (input, local_yaw) = be_f32(input)?;
    let (input, local_pitch) = be_f32(input)?;
    let (input, local_roll) = be_f32(input)?;
    Ok((input, DrOtherParameters::LocalEulerAngles(DrEulerAngles {
        local_yaw,
        local_pitch,
        local_roll,
    })))
}

pub fn dr_other_parameters_quaternion(input: &[u8]) -> IResult<&[u8], DrOtherParameters> {
    let (input, _param_type) = be_u8(input)?;
    let (input, nil) = be_u16(input)?;
    let (input, x) = be_f32(input)?;
    let (input, y) = be_f32(input)?;
    let (input, z) = be_f32(input)?;
    Ok((input, DrOtherParameters::WorldOrientationQuaternion(DrWorldOrientationQuaternion {
        nil,
        x,
        y,
        z,
    })))
}

pub fn variable_parameter(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, parameter_type_designator) = be_u8(input)?;
    let parameter_type = VariableParameterRecordType::from(parameter_type_designator);
    let (input, variable_parameter) = match parameter_type {
        VariableParameterRecordType::ArticulatedPart => { articulated_part(input)? }
        VariableParameterRecordType::AttachedPart => { attached_part(input)? }
        VariableParameterRecordType::Separation => { separation(input)? }
        VariableParameterRecordType::EntityType => { entity_type_variable_parameter(input)? }
        VariableParameterRecordType::EntityAssociation => { entity_association(input)? }
        VariableParameterRecordType::Unspecified(_) => {
            let (input, bytes) = take(15usize)(input)?;
            (input, VariableParameter::Unspecified(parameter_type_designator, <[u8; 15]>::try_from(bytes).unwrap()))
        } // TODO sensible error
    };

    Ok((input, variable_parameter))
}

fn articulated_part(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, change_indicator) = be_u8(input)?;
    let change_indicator = ChangeIndicator::from(change_indicator);
    let (input, attachment_id) = be_u16(input)?;
    let (input, type_variant) = be_u32(input)?;
    let type_metric : u32 = type_variant & 0x1f;  // 5 least significant bits (0x1f) are the type metric
    let type_class : u32 = type_variant - type_metric;   // rest of the bits (minus type metric value) are the type class
    let (input, value) = be_f32(input)?;
    let (input, _pad_out) = be_u32(input)?;

    Ok((input, VariableParameter::Articulated(ArticulatedPart {
        change_indicator,
        attachment_id,
        type_metric: ArticulatedPartsTypeMetric::from(type_metric),
        type_class: ArticulatedPartsTypeClass::from(type_class),
        parameter_value: value,
    })))
}

fn attached_part(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, detached_indicator) = be_u8(input)?;
    let detached_indicator = AttachedPartDetachedIndicator::from(detached_indicator);
    let (input, attachment_id) = be_u16(input)?;
    let (input, attached_part) = be_u32(input)?;
    let (input, entity_type) = entity_type(input)?;
    Ok((input, VariableParameter::Attached(AttachedPart {
        detached_indicator,
        attachment_id,
        parameter_type: AttachedParts::from(attached_part),
        attached_part_type: entity_type
    })))
}

fn entity_association(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, change_indicator) = be_u8(input)?;
    let (input, association_status) = be_u8(input)?;
    let (input, association_type) = be_u8(input)?;
    let (input, entity_id) = entity_id(input)?;
    let (input, own_station_location) = be_u16(input)?;
    let (input, physical_connection_type) = be_u8(input)?;
    let (input, group_member_type) = be_u8(input)?;
    let (input, group_number) = be_u16(input)?;

    Ok((input, VariableParameter::EntityAssociation(EntityAssociationParameter {
        change_indicator: ChangeIndicator::from(change_indicator),
        association_status: EntityAssociationAssociationStatus::from(association_status),
        association_type: EntityAssociationPhysicalAssociationType::from(association_type),
        entity_id,
        own_station_location: StationName::from(own_station_location),
        physical_connection_type: EntityAssociationPhysicalConnectionType::from(physical_connection_type),
        group_member_type: EntityAssociationGroupMemberType::from(group_member_type),
        group_number,
    })))
}

fn entity_type_variable_parameter(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, change_indicator) = be_u8(input)?;
    let (input, entity_type) = entity_type(input)?;
    let (input, _pad_out_16) = be_u16(input)?;
    let (input, _pad_out_32) = be_u32(input)?;

    Ok((input, VariableParameter::EntityType(EntityTypeParameter {
        change_indicator: ChangeIndicator::from(change_indicator),
        entity_type,
    })))
}

fn separation(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, reason) = be_u8(input)?;
    let (input, pre_entity_indicator) = be_u8(input)?;
    let (input, parent_entity_id) = entity_id(input)?;
    let (input, _pad_16) = be_u16(input)?;
    let (input, station_name) = be_u16(input)?;
    let (input, station_number) = be_u16(input)?;

    Ok((input, VariableParameter::Separation(SeparationParameter {
        reason: SeparationReasonForSeparation::from(reason),
        pre_entity_indicator: SeparationPreEntityIndicator::from(pre_entity_indicator),
        parent_entity_id,
        station_name: StationName::from(station_name),
        station_number,
    })))
}

#[cfg(test)]
mod tests {
    use crate::common::entity_state::parser::{variable_parameter, entity_marking};
    use crate::common::parser::location;
    use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, ChangeIndicator, EntityMarkingCharacterSet};
    use crate::v6::entity_state::parser::entity_capabilities;
    use crate::VariableParameter;

    #[test]
    fn parse_entity_location() {
        let bytes: [u8; 24] = [0x41, 0x50, 0xc4, 0x1a, 0xde, 0xa4, 0xbe, 0xcc, 0x41, 0x50,
            0xc9, 0xfa, 0x13, 0x3c, 0xf0, 0x5d, 0x41, 0x35, 0x79, 0x16, 0x9e, 0x7a, 0x16, 0x78];

        let location = location(&bytes);
        assert!(location.is_ok());
        let (input, location) = location.unwrap();
        assert_eq!(input.len(), 0);
        assert_eq!(location.x_coordinate, 4395115.478805255);
        assert_eq!(location.y_coordinate, 4401128.300594416);
        assert_eq!(location.z_coordinate, 1407254.6190504115);
    }

    #[test]
    fn parse_marking_ascii() {
        let bytes: [u8; 12] = [0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20];

        let marking = entity_marking(&bytes);
        assert!(marking.is_ok());
        let (input, marking) = marking.unwrap();
        assert_eq!(marking.marking_character_set, EntityMarkingCharacterSet::ASCII);
        assert_eq!(marking.marking_string, "EYE 10");

        assert!(input.is_empty());
    }

    // TODO change test to fit new appearance models
    // #[test]
    // fn parse_general_appearance_none() {
    //     let input : [u8;2] = [0x00,0x00];
    //
    //     let res = general_appearance(&input);
    //     assert!(res.is_ok());
    //     let (input, appearance) = res.expect("value is Ok");
    //     assert_eq!(appearance.entity_paint_scheme, EntityPaintScheme::UniformColor);
    //     assert_eq!(appearance.entity_mobility_kill, EntityMobilityKill::NoMobilityKill);
    //     assert_eq!(appearance.entity_fire_power, EntityFirePower::NoFirePowerKill);
    //     assert_eq!(appearance.entity_damage, EntityDamage::NoDamage);
    //     assert_eq!(appearance.entity_smoke, EntitySmoke::NotSmoking);
    //     assert_eq!(appearance.entity_trailing_effect, EntityTrailingEffect::None);
    //     assert_eq!(appearance.entity_hatch_state, EntityHatchState::NotApplicable);
    //     assert_eq!(appearance.entity_lights, EntityLights::None);
    //     assert_eq!(appearance.entity_flaming_effect, EntityFlamingEffect::None);
    //
    //     assert!(input.is_empty());
    // }
    //
    // #[test]
    // fn parse_general_appearance_emitting_engine_smoke() {
    //     let input : [u8;2] = [0x04,0x00];
    //
    //     let res = general_appearance(&input);
    //     assert!(res.is_ok());
    //     let (input, appearance) = res.expect("value is Ok");
    //     assert_eq!(appearance.entity_paint_scheme, EntityPaintScheme::UniformColor);
    //     assert_eq!(appearance.entity_mobility_kill, EntityMobilityKill::NoMobilityKill);
    //     assert_eq!(appearance.entity_fire_power, EntityFirePower::NoFirePowerKill);
    //     assert_eq!(appearance.entity_damage, EntityDamage::NoDamage);
    //     assert_eq!(appearance.entity_smoke, EntitySmoke::EmittingEngineSmoke);
    //     assert_eq!(appearance.entity_trailing_effect, EntityTrailingEffect::None);
    //     assert_eq!(appearance.entity_hatch_state, EntityHatchState::NotApplicable);
    //     assert_eq!(appearance.entity_lights, EntityLights::None);
    //     assert_eq!(appearance.entity_flaming_effect, EntityFlamingEffect::None);
    //
    //     assert!(input.is_empty());
    // }

    #[test]
    fn parse_entity_capabilities_none() {
        let input : [u8;4] = [0x00,0x00,0x00,0x00];

        let res = entity_capabilities(&input);
        assert!(res.is_ok());
        let (input, capabilities) = res.expect("value is Ok");
        assert!(!capabilities.ammunition_supply);
        assert!(!capabilities.fuel_supply);
        assert!(!capabilities.recovery);
        assert!(!capabilities.repair);

        assert!(input.is_empty());
    }

    #[test]
    fn parse_articulated_parameter_gun1_azimuth() {
        let input : [u8;16] =
            [0x00,  // u8; type articulated
                0x00,   // u8; no change
                0x00,0x00,  // u16; 0 value attachment id
                0x00,0x00,  // u32; type variant metric - 11 - azimuth
                0x10,0x0b,  // type variant high bits - 4096 - primary gun 1
                0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00]; // f64 - value 0

        let parameter = variable_parameter(&input);
        assert!(parameter.is_ok());
        let (input, parameter) = parameter.expect("should be Ok");

        if let VariableParameter::Articulated(articulated_part) = parameter {
            assert_eq!(articulated_part.change_indicator, ChangeIndicator::from(0u8));
            assert_eq!(articulated_part.attachment_id, 0);
            assert_eq!(articulated_part.type_class, ArticulatedPartsTypeClass::PrimaryTurretNumber1);
            assert_eq!(articulated_part.type_metric, ArticulatedPartsTypeMetric::Azimuth);
        } else { assert!(false) }

        assert!(input.is_empty());
    }

    #[test]
    fn parse_articulated_parameter_landing_gear_down() {
        let input : [u8;16] =
            [0x00,  // u8; type articulated
                0x00,   // u8; no change
                0x00,0x00,  // u16; 0 value attachment id
                0x00,0x00,  // u32; type variant metric - 11 - position
                0x0C,0x01,  // type variant high bits - 3072 - landing gear
                0x3F,0x80,0x00,0x00,0x00,0x00,0x00,0x00]; // f32 - value '1' and 4 bytes padding

        let parameter = variable_parameter(&input);
        assert!(parameter.is_ok());
        let (input, parameter) = parameter.expect("should be Ok");
        if let VariableParameter::Articulated(articulated_part) = parameter {
            assert_eq!(articulated_part.change_indicator, ChangeIndicator::from(0u8));
            assert_eq!(articulated_part.attachment_id, 0);
            assert_eq!(articulated_part.type_class, ArticulatedPartsTypeClass::LandingGear);
            assert_eq!(articulated_part.type_metric, ArticulatedPartsTypeMetric::Position);
            assert_eq!(articulated_part.parameter_value, 1f32);
        } else { assert!(false) }

        assert!(input.is_empty());
    }
}