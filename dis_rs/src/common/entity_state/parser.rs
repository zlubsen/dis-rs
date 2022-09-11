use nom::bytes::complete::take;
use nom::IResult;
use nom::number::complete::{be_f32, be_u16, be_u32, be_u8};
use crate::AttachedPart;
use crate::common::entity_state::model::{ArticulatedPart, VariableParameter, EntityMarking, ParameterVariant};
use crate::common::model::PduBody;
use crate::common::parser;
use crate::common::parser::entity_type;
use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedParts, DeadReckoningAlgorithm, EntityMarkingCharacterSet, ForceId, ProtocolVersion, VariableParameterRecordType};
use crate::v6::entity_state::model::DrParameters;

pub fn entity_state_body(version: ProtocolVersion) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> {
    move |input: &[u8]| {
        // FIXME factor out this match construct to filter/branch on DIS versions
        let (input, body) = match u8::from(version) {
            legacy_version if legacy_version <= 5 => {
                unimplemented!("DIS Versions 1-5 are not supported, found {}", legacy_version);
            }
            6 => {
                // versions 6 and lower
                crate::v6::entity_state::parser::entity_state_body(input)?
            }
            7 => {
                // version 7
                crate::v7::entity_state::parser::entity_state_body(input)?
            }
            future_version => {
                unimplemented!("DIS 7 is the most recent DIS version at time of implementation, found {}.", future_version);
            }
        };
        Ok((input, body))
    }
}

pub fn force_id(input: &[u8]) -> IResult<&[u8], ForceId> {
    let (input, force_id) = be_u8(input)?;
    Ok((input, ForceId::from(force_id)))
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
    let (input, other_parameters) = take(15usize)(input)?;
    let (input, acceleration) = parser::vec3_f32(input)?;
    let (input, velocity) = parser::vec3_f32(input)?;

    let other_parameters = other_parameters.try_into().unwrap();

    Ok((input, DrParameters {
        algorithm: DeadReckoningAlgorithm::from(algorithm),
        other_parameters,
        linear_acceleration: acceleration,
        angular_velocity: velocity,
    }))
}

pub fn articulation_record(input: &[u8]) -> IResult<&[u8], VariableParameter> {
    let (input, parameter_type_designator) = be_u8(input)?;
    let (input, changed_attached_indicator) = be_u8(input)?;
    let (input, articulation_attachment_id) = be_u16(input)?;
    let parameter_type_designator : VariableParameterRecordType = VariableParameterRecordType::from(parameter_type_designator);
    let (input, parameter_type_variant) = match parameter_type_designator {
        VariableParameterRecordType::AttachedPart => { attached_part(input)? }
        VariableParameterRecordType::ArticulatedPart => { articulated_part(input)? }
        _ => { attached_part(input)? } // TODO impl other VariableParameterRecordType; now defaults to Unspecified AttachedPart
    };
    // // FIXME attached parts has an 64-bit EntityType record, articulated part a 32-bit float value + 32-bit padding
    // let (input, articulation_parameter_value) = be_f32(input)?;
    // let (input, _pad_out) = take(4usize)(input)?;

    Ok((input, VariableParameter {
        parameter_type_designator,
        changed_attached_indicator,
        articulation_attachment_id,
        parameter: parameter_type_variant,
    }))
}

fn attached_part(input: &[u8]) -> IResult<&[u8], ParameterVariant> {
    let (input, attached_part) = be_u32(input)?;
    let (input, entity_type) = entity_type(input)?;
    Ok((input, ParameterVariant::Attached(AttachedPart {
        parameter_type: AttachedParts::from(attached_part),
        attached_part_type: entity_type
    })))
}

fn articulated_part(input: &[u8]) -> IResult<&[u8], ParameterVariant> {
    let (input, type_variant) = be_u32(input)?;
    let type_metric : u32 = type_variant & 0x1f;  // 5 least significant bits (0x1f) are the type metric
    let type_class : u32 = type_variant - type_metric;   // rest of the bits (minus type metric value) are the type class
    let (input, value) = be_f32(input)?;
    let (input, _pad_out) = be_u32(input)?;

    Ok((input, ParameterVariant::Articulated(ArticulatedPart {
        type_metric: ArticulatedPartsTypeMetric::from(type_metric),
        type_class: ArticulatedPartsTypeClass::from(type_class),
        parameter_value: value,
    })))
}

#[cfg(test)]
mod tests {
    use crate::common::entity_state::model::ParameterVariant;
    use crate::common::entity_state::parser::{articulation_record, entity_marking};
    use crate::common::parser::location;
    use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, EntityMarkingCharacterSet, VariableParameterRecordType};
    use crate::v6::entity_state::model::{EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityLights, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityTrailingEffect};
    use crate::v6::entity_state::parser::{entity_capabilities, general_appearance};

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
    fn parse_general_appearance_none() {
        let input : [u8;2] = [0x00,0x00];

        let res = general_appearance(&input);
        assert!(res.is_ok());
        let (input, appearance) = res.expect("value is Ok");
        assert_eq!(appearance.entity_paint_scheme, EntityPaintScheme::UniformColor);
        assert_eq!(appearance.entity_mobility_kill, EntityMobilityKill::NoMobilityKill);
        assert_eq!(appearance.entity_fire_power, EntityFirePower::NoFirePowerKill);
        assert_eq!(appearance.entity_damage, EntityDamage::NoDamage);
        assert_eq!(appearance.entity_smoke, EntitySmoke::NotSmoking);
        assert_eq!(appearance.entity_trailing_effect, EntityTrailingEffect::None);
        assert_eq!(appearance.entity_hatch_state, EntityHatchState::NotApplicable);
        assert_eq!(appearance.entity_lights, EntityLights::None);
        assert_eq!(appearance.entity_flaming_effect, EntityFlamingEffect::None);

        assert!(input.is_empty());
    }

    #[test]
    fn parse_general_appearance_emitting_engine_smoke() {
        let input : [u8;2] = [0x04,0x00];

        let res = general_appearance(&input);
        assert!(res.is_ok());
        let (input, appearance) = res.expect("value is Ok");
        assert_eq!(appearance.entity_paint_scheme, EntityPaintScheme::UniformColor);
        assert_eq!(appearance.entity_mobility_kill, EntityMobilityKill::NoMobilityKill);
        assert_eq!(appearance.entity_fire_power, EntityFirePower::NoFirePowerKill);
        assert_eq!(appearance.entity_damage, EntityDamage::NoDamage);
        assert_eq!(appearance.entity_smoke, EntitySmoke::EmittingEngineSmoke);
        assert_eq!(appearance.entity_trailing_effect, EntityTrailingEffect::None);
        assert_eq!(appearance.entity_hatch_state, EntityHatchState::NotApplicable);
        assert_eq!(appearance.entity_lights, EntityLights::None);
        assert_eq!(appearance.entity_flaming_effect, EntityFlamingEffect::None);

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

        let parameter = articulation_record(&input);
        assert!(parameter.is_ok());
        let (input, parameter) = parameter.expect("should be Ok");
        assert_eq!(parameter.parameter_type_designator, VariableParameterRecordType::ArticulatedPart);
        assert_eq!(parameter.changed_attached_indicator, 0);
        assert_eq!(parameter.articulation_attachment_id, 0);
        if let ParameterVariant::Articulated(articulated_part) = parameter.parameter {
            assert_eq!(articulated_part.type_class, ArticulatedPartsTypeClass::PrimaryTurretNumber1);
            assert_eq!(articulated_part.type_metric, ArticulatedPartsTypeMetric::Azimuth);
        }

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

        let parameter = articulation_record(&input);
        assert!(parameter.is_ok());
        let (input, parameter) = parameter.expect("should be Ok");
        assert_eq!(parameter.parameter_type_designator, VariableParameterRecordType::ArticulatedPart);
        assert_eq!(parameter.changed_attached_indicator, 0);
        assert_eq!(parameter.articulation_attachment_id, 0);
        if let ParameterVariant::Articulated(type_variant) = parameter.parameter {
            assert_eq!(type_variant.type_class, ArticulatedPartsTypeClass::LandingGear);
            assert_eq!(type_variant.type_metric, ArticulatedPartsTypeMetric::Position);
            assert_eq!(type_variant.parameter_value, 1f32);
        }

        assert!(input.is_empty());
    }
}