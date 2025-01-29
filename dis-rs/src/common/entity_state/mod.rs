pub mod builder;
mod compatibility;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::entity_state::model::{
        DrOtherParameters, DrParameters, DrWorldOrientationQuaternion, EntityAppearance,
        EntityMarking, EntityState,
    };
    use crate::common::model::{
        ArticulatedPart, EntityId, EntityType, Location, Orientation, Pdu, PduHeader,
        SimulationAddress, VariableParameter, VectorF32,
    };
    use crate::common::parser::parse_pdu;
    use crate::enumerations::*;
    use bytes::BytesMut;

    #[test]
    fn entity_state_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::EntityState);

        let body = EntityState::builder()
            .with_entity_id(EntityId {
            simulation_address: SimulationAddress {site_id: 500, application_id: 900 },
            entity_id: 14
        })
            .with_force_id(ForceId::Friendly)
            .with_entity_type(EntityType {
            kind: EntityKind::Platform, domain: PlatformDomain::Air, country: Country::Netherlands_NLD_, category: 50, subcategory: 4, specific: 4, extra: 0
        })
            .with_alternative_entity_type(EntityType {
                kind: EntityKind::Platform, domain: PlatformDomain::Air, country: Country::Netherlands_NLD_, category: 50, subcategory: 4, specific: 4, extra: 0
            })
            .with_velocity(VectorF32 {
                first_vector_component: 0f32, second_vector_component: 0f32, third_vector_component: 0f32
            })
            .with_location(Location {
                x_coordinate: 0f64, y_coordinate : 0f64, z_coordinate: 0f64
            })
            .with_orientation(Orientation {
                psi: 0f32, theta: 0f32, phi: 0f32
            })
            .with_appearance(EntityAppearance::AirPlatform(AirPlatformAppearance {
                paint_scheme: AppearancePaintScheme::UniformColor,
                propulsion_killed: false,
                nvg_mode: AppearanceNVGMode::default(),
                damage: AppearanceDamage::default(),
                is_smoke_emanating: true,
                is_engine_emitting_smoke: false,
                trailing_effects: AppearanceTrailingEffects::None,
                canopy_troop_door: AppearanceCanopy::NotApplicable,
                landing_lights_on: false,
                navigation_lights_on: false,
                anticollision_lights_on: false,
                is_flaming: false,
                afterburner_on: false,
                lower_anticollision_light_on: false,
                upper_anticollision_light_on: false,
                anticollision_light_day_night: AppearanceAntiCollisionDayNight::Day,
                is_blinking: false,
                is_frozen: false,
                power_plant_on: false,
                state: AppearanceEntityOrObjectState::Active,
                formation_lights_on: false,
                landing_gear_extended: false,
                cargo_doors_opened: false,
                navigation_position_brightness: AppearanceNavigationPositionBrightness::Dim,
                spot_search_light_1_on: false,
                interior_lights_on: false,
                reverse_thrust_engaged: false,
                weightonwheels: false,
            }))
            .with_dead_reckoning_parameters(DrParameters {
                algorithm: DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation,
                other_parameters: DrOtherParameters::WorldOrientationQuaternion(
                    DrWorldOrientationQuaternion::default()),
                linear_acceleration: VectorF32 {
                    first_vector_component: 0f32, second_vector_component: 0f32, third_vector_component: 0f32
                },
                angular_velocity: VectorF32 {
                    first_vector_component: 0f32, second_vector_component: 0f32, third_vector_component: 0f32
                }
            })
            .with_marking(EntityMarking {
                marking_character_set: EntityMarkingCharacterSet::ASCII,
                marking_string: "EYE 10".to_string()
            })
            .with_capabilities_flags(false, false, false, false)
            .with_variable_parameter(VariableParameter::Articulated(ArticulatedPart {
                change_indicator: ChangeIndicator::from(0u8),
                attachment_id: 0,
                type_class: ArticulatedPartsTypeClass::LandingGear,
                type_metric: ArticulatedPartsTypeMetric::Position,
                parameter_value: 1.0
            }))
            .with_variable_parameter(VariableParameter::Articulated(ArticulatedPart {
                change_indicator: ChangeIndicator::from(0u8),
                attachment_id: 0,
                type_class: ArticulatedPartsTypeClass::PrimaryTurretNumber1,
                type_metric: ArticulatedPartsTypeMetric::Azimuth,
                parameter_value: 2.0
            }))
            .with_variable_parameter(VariableParameter::Articulated(ArticulatedPart {
                change_indicator: ChangeIndicator::from(0u8),
                attachment_id: 0,
                type_class: ArticulatedPartsTypeClass::PrimaryTurretNumber1,
                type_metric: ArticulatedPartsTypeMetric::AzimuthRate,
                parameter_value: 3.0
            }))
            .with_variable_parameter(VariableParameter::Articulated(ArticulatedPart {
                change_indicator: ChangeIndicator::from(0u8),
                attachment_id: 0,
                type_class: ArticulatedPartsTypeClass::PrimaryGunNumber1,
                type_metric: ArticulatedPartsTypeMetric::Elevation,
                parameter_value: 4.0
            }))
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, 0);
        let pdu_length = original_pdu.header.pdu_length;
        let original_length = original_pdu.pdu_length();

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        let serialized_length = original_pdu.serialize(&mut buf).unwrap();

        assert_eq!(original_length, serialized_length);

        let parsed = parse_pdu(&buf);
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, pdu);
            }
            Err(ref _err) => {
                assert!(false);
            }
        }
    }
}
