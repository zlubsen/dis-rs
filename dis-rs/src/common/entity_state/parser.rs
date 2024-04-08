use nom::bytes::complete::take;
use nom::IResult;
use nom::multi::count;
use nom::number::complete::{be_f32, be_u16, be_u32, be_u8};
use crate::common::entity_state::model::{EntityState, DrOtherParameters, DrParameters, EntityMarking, DrEulerAngles, DrWorldOrientationQuaternion, EntityAppearance};
use crate::common::model::{EntityType, PduBody, PduHeader};
use crate::common::parser;
use crate::common::parser::{entity_id, entity_type, sanitize_marking, vec3_f32};
use crate::enumerations::*;
use crate::v6::entity_state::parser::entity_capabilities;

pub(crate) fn entity_state_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
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
        #[allow(clippy::wildcard_in_or_patterns)]
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
            count(parser::variable_parameter, variable_parameters_no as usize)(input)?
        } else { (input, vec![]) };

        let body = EntityState::builder()
            .with_entity_id(entity_id_val)
            .with_force_id(force_id_val)
            .with_entity_type(entity_type_val)
            .with_alternative_entity_type(alternative_entity_type)
            .with_velocity(entity_linear_velocity)
            .with_location(entity_location)
            .with_orientation(entity_orientation)
            .with_appearance(entity_appearance)
            .with_dead_reckoning_parameters(dead_reckoning_parameters)
            .with_marking(entity_marking)
            .with_capabilities(entity_capabilities)
            .with_variable_parameters(variable_parameters)
            .build();

        Ok((input, body.into_pdu_body()))
    }
}

pub(crate) fn force_id(input: &[u8]) -> IResult<&[u8], ForceId> {
    let (input, force_id) = be_u8(input)?;
    Ok((input, ForceId::from(force_id)))
}

pub(crate) fn entity_appearance(entity_type: EntityType) -> impl Fn(&[u8]) -> IResult<&[u8], EntityAppearance> {
    move |input: &[u8]| {
        let (input, appearance) = be_u32(input)?;

        Ok((input, EntityAppearance::from_bytes(appearance, &entity_type)))
    }
}

/// Parses the marking portion of an EntityState PDU into an EntityMarking struct.
/// It will convert the parsed bytes (always 11 bytes are present in the PDU) to UTF-8, and
/// strip trailing whitespace and any trailing non-alphanumeric characters. In case the marking is less
/// than 11 characters, the trailing bytes are typically 0x00 in the PDU, which in UTF-8 is a control character.
pub(crate) fn entity_marking(input: &[u8]) -> IResult<&[u8], EntityMarking> {
    let mut buf : [u8;11] = [0;11];
    let (input, marking_character_set) = be_u8(input)?;
    let (input, _) = nom::multi::fill(be_u8, &mut buf)(input)?;

    let marking_character_set = EntityMarkingCharacterSet::from(marking_character_set);
    let marking_string = sanitize_marking(&buf[..]);

    Ok((input, EntityMarking{
        marking_character_set,
        marking_string,
    }))
}

pub(crate) fn dr_parameters(input: &[u8]) -> IResult<&[u8], DrParameters> {
    let (input, algorithm) = be_u8(input)?;
    let algorithm = DeadReckoningAlgorithm::from(algorithm);

    let (input, other_parameters) = dr_other_parameters(input, algorithm)?;

    // // This match statement basically determines the value of the DrParametersType field for Euler and Quaternion variants
    // let (input, other_parameters) = match algorithm {
    //     DeadReckoningAlgorithm::StaticNonmovingEntity |
    //         DeadReckoningAlgorithm::DRM_FPW_ConstantVelocityLowAccelerationLinearMotionEntity |
    //         DeadReckoningAlgorithm::DRM_FVW_HighSpeedorManeuveringEntity |
    //         DeadReckoningAlgorithm::DRM_FPB_SimilartoFPWexceptinBodyCoordinates |
    //         DeadReckoningAlgorithm::DRM_FVB_SimilartoFVWexceptinBodyCoordinates => {
    //         dr_other_parameters_euler(input)?
    //     }
    //     DeadReckoningAlgorithm::DRM_RPW_ConstantVelocityLowAccelerationLinearMotionEntitywithExtrapolationofOrientation |
    //         DeadReckoningAlgorithm::DRM_RVW_HighSpeedorManeuveringEntitywithExtrapolationofOrientation |
    //         DeadReckoningAlgorithm::DRM_RPB_SimilartoRPWexceptinBodyCoordinates |
    //         DeadReckoningAlgorithm::DRM_RVB_SimilartoRVWexceptinBodyCoordinates => {
    //         dr_other_parameters_quaternion(input)?
    //     }
    //     DeadReckoningAlgorithm::Other => {
    //         dr_other_parameters_none(input)?
    //     }
    //     _ => {
    //         dr_other_parameters_none(input)?
    //     }
    // };

    let (input, acceleration) = vec3_f32(input)?;
    let (input, velocity) = vec3_f32(input)?;

    Ok((input, DrParameters {
        algorithm,
        other_parameters,
        linear_acceleration: acceleration,
        angular_velocity: velocity,
    }))
}

pub fn dr_other_parameters(input: &[u8], algorithm: DeadReckoningAlgorithm) -> IResult<&[u8], DrOtherParameters> {
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
        DeadReckoningAlgorithm::Other => {
            dr_other_parameters_none(input)?
        }
        _ => {
            dr_other_parameters_none(input)?
        }
    };

    Ok((input, other_parameters))
}

pub(crate) fn dr_other_parameters_none(input: &[u8]) -> IResult<&[u8], DrOtherParameters> {
    let (input, params) = take(15usize)(input)?;
    Ok((input, DrOtherParameters::None(params.try_into().unwrap())))
}

pub(crate) fn dr_other_parameters_euler(input: &[u8]) -> IResult<&[u8], DrOtherParameters> {
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

pub(crate) fn dr_other_parameters_quaternion(input: &[u8]) -> IResult<&[u8], DrOtherParameters> {
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

#[cfg(test)]
mod tests {
    use crate::common::entity_state::parser::{entity_appearance, entity_marking};
    use crate::common::parser::{location, parse_pdu, variable_parameter};
    use crate::common::entity_state::model::EntityAppearance;
    use crate::enumerations::{*};
    use crate::common::model::{EntityType, PduBody};
    use crate::common::model::VariableParameter;
    use crate::v6::entity_state::parser::entity_capabilities;

    #[test]
    fn parse_pdu_entity_state() {
        let bytes : [u8;208] =
            [0x06, 0x01, 0x01, 0x01, 0x4e, 0xea, 0x3b, 0x60, 0x00, 0xd0, 0x00, 0x00, 0x01, 0xf4, 0x03, 0x84,
                0x00, 0x0e, 0x01, 0x04, 0x01, 0x02, 0x00, 0x99, 0x32, 0x04, 0x04, 0x00, 0x01, 0x02, 0x00, 0x99,
                0x32, 0x04, 0x04, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x41, 0x50, 0xc4, 0x1a, 0xde, 0xa4, 0xbe, 0xcc, 0x41, 0x50, 0xc9, 0xfa, 0x13, 0x3c, 0xf0, 0x5d,
                0x41, 0x35, 0x79, 0x16, 0x9e, 0x7a, 0x16, 0x78, 0xbf, 0x3e, 0xdd, 0xfa, 0x3e, 0x2e, 0x36, 0xdd,
                0x3f, 0xe6, 0x27, 0xc9, 0x00, 0x40, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x01, 0x3f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x4d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        let pdu = parse_pdu(&bytes);
        assert!(pdu.is_ok());
        let pdu = pdu.unwrap();
        assert_eq!(pdu.header.pdu_type, PduType::EntityState);
        assert_eq!(pdu.header.pdu_length, 208u16);
        if let PduBody::EntityState(pdu) = pdu.body {
            assert_eq!(pdu.entity_id.simulation_address.site_id, 500u16);
            assert_eq!(pdu.entity_id.simulation_address.application_id, 900u16);
            assert_eq!(pdu.entity_id.entity_id, 14u16);
            assert_eq!(pdu.force_id, ForceId::Friendly);
            assert!(!pdu.variable_parameters.is_empty());
            assert_eq!(pdu.variable_parameters.len(), 4usize);
            assert_eq!(pdu.entity_type, EntityType {
                kind: EntityKind::Platform,
                domain: PlatformDomain::Air,
                country: Country::Netherlands_NLD_,
                category: 50,
                subcategory: 4,
                specific: 4,
                extra: 0
            });

            if let EntityAppearance::AirPlatform(appearance) = pdu.entity_appearance {
                assert_eq!(appearance.paint_scheme, AppearancePaintScheme::UniformColor);
                assert_eq!(appearance.propulsion_killed, false);
                assert_eq!(appearance.damage, AppearanceDamage::NoDamage);
                assert_eq!(appearance.is_smoke_emanating, false);
                assert_eq!(appearance.is_engine_emitting_smoke, false);
                assert_eq!(appearance.trailing_effects, AppearanceTrailingEffects::None);
                assert_eq!(appearance.canopy_troop_door, AppearanceCanopy::SingleCanopySingleTroopDoorOpen);
                assert_eq!(appearance.landing_lights_on, false);
                assert_eq!(appearance.navigation_lights_on, false);
                assert_eq!(appearance.anticollision_lights_on, false);
                assert_eq!(appearance.is_flaming, false);
                assert_eq!(appearance.afterburner_on, false);
                assert_eq!(appearance.is_frozen, false);
                assert_eq!(appearance.power_plant_on, false);
                assert_eq!(appearance.state, AppearanceEntityorObjectState::Active);
            } else {
                assert!(false)
            }

            assert_eq!(pdu.dead_reckoning_parameters.algorithm, DeadReckoningAlgorithm::DRM_RVW_HighSpeedorManeuveringEntitywithExtrapolationofOrientation);
            assert_eq!(pdu.entity_marking.marking_string, String::from("EYE 10"));
            let capabilities : EntityCapabilities = pdu.entity_capabilities.into();
            if let EntityCapabilities::AirPlatformEntityCapabilities(capabilities) = capabilities {
                assert_eq!(capabilities.ammunition_supply, false);
                assert_eq!(capabilities.fuel_supply, false);
                assert_eq!(capabilities.recovery, false);
                assert_eq!(capabilities.repair, false);
            }
            assert_eq!(pdu.variable_parameters.len(), 4);
            let parameter_1 = pdu.variable_parameters.get(0).unwrap();
            if let VariableParameter::Articulated(part) = parameter_1 {
                assert_eq!(part.change_indicator, ChangeIndicator::from(0u8));
                assert_eq!(part.attachment_id, 0u16);
                assert_eq!(part.type_metric, ArticulatedPartsTypeMetric::Position);
                assert_eq!(part.type_class, ArticulatedPartsTypeClass::LandingGear); // landing gear
                assert_eq!(part.parameter_value, 1f32);
            } else {
                assert!(false);
            }
        } else { assert!(false) }
    }

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

    #[test]
    fn parse_marking_trailing_control_chars() {
        let bytes: [u8; 12] = [0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x00, 0x00, 0x00, 0x00, 0x00];

        let marking = entity_marking(&bytes);
        assert!(marking.is_ok());
        let (input, marking) = marking.unwrap();
        assert_eq!(marking.marking_character_set, EntityMarkingCharacterSet::ASCII);
        assert_eq!(marking.marking_string, "EYE 10");

        assert!(input.is_empty());
    }

    #[test]
    fn parse_appearance_none() {
        let input : [u8;4] = [0x00,0x00,0x00,0x00];
        let entity_type = EntityType::default().with_kind(EntityKind::Platform).with_domain(PlatformDomain::Air);

        let res = entity_appearance(entity_type)(&input);
        assert!(res.is_ok());
        let (input, appearance) = res.expect("value is Ok");

        if let EntityAppearance::AirPlatform(appearance) = appearance {
            assert_eq!(appearance.paint_scheme, AppearancePaintScheme::UniformColor);
            assert_eq!(appearance.propulsion_killed, false);
            assert_eq!(appearance.damage, AppearanceDamage::NoDamage);
            assert_eq!(appearance.is_smoke_emanating, false);
            assert_eq!(appearance.is_engine_emitting_smoke, false);
            assert_eq!(appearance.trailing_effects, AppearanceTrailingEffects::None);
            assert_eq!(appearance.canopy_troop_door , AppearanceCanopy::NotApplicable);
            assert_eq!(appearance.landing_lights_on, false);
            assert_eq!(appearance.is_flaming, false);
        } else {
            assert!(false);
        }
        assert!(input.is_empty());
    }

    #[test]
    fn parse_appearance_emitting_engine_smoke() {
        let input : [u8;4] = [0x06,0x00,0x00,0x00];
        let entity_type = EntityType::default().with_kind(EntityKind::Platform).with_domain(PlatformDomain::Air);

        let res = entity_appearance(entity_type)(&input);
        assert!(res.is_ok());
        let (input, appearance) = res.expect("value is Ok");

        if let EntityAppearance::AirPlatform(appearance) = appearance {
            assert_eq!(appearance.is_smoke_emanating, true);
            assert_eq!(appearance.is_engine_emitting_smoke, true);
        } else {
            assert!(false);
        }
        assert!(input.is_empty());
    }

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