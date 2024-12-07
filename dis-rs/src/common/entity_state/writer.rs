use crate::common::entity_state::model::{
    DrEulerAngles, DrOtherParameters, DrWorldOrientationQuaternion, EntityAppearance,
};
use crate::common::entity_state::model::{DrParameters, EntityMarking, EntityState};
use crate::common::model::EntityType;
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::enumerations::{DrParametersType, ForceId};
use crate::v6::entity_state::model::EntityCapabilities;
use bytes::{BufMut, BytesMut};

impl SerializePdu for EntityState {
    fn serialize_pdu(&self, version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let entity_id_bytes = self.entity_id.serialize(buf);
        let force_id_bytes = self.force_id.serialize(buf);
        buf.put_u8(self.variable_parameters.len() as u8);

        let entity_type_bytes = self.entity_type.serialize(buf);
        let alt_entity_type_bytes = self.alternative_entity_type.serialize(buf);

        let linear_velocity_bytes = self.entity_linear_velocity.serialize(buf);
        let location_bytes = self.entity_location.serialize(buf);
        let orientation_bytes = self.entity_orientation.serialize(buf);

        let appearance_bytes = self.entity_appearance.serialize(buf);
        let dr_params_bytes = self.dead_reckoning_parameters.serialize(buf);

        let marking_bytes = self.entity_marking.serialize(buf);
        let capabilities_bytes = match version {
            SupportedVersion::V6 => {
                let capabilities: EntityCapabilities = self.entity_capabilities.into();
                capabilities.serialize(buf)
            }
            SupportedVersion::V7 => {
                buf.put_u32(self.entity_capabilities.into());
                4
            }
            SupportedVersion::Unsupported => {
                buf.put_u32(0u32);
                4
            }
        };

        let variable_params_bytes: u16 = self
            .variable_parameters
            .iter()
            .map(|param| param.serialize(buf))
            .sum();

        entity_id_bytes
            + force_id_bytes
            + 1
            + entity_type_bytes
            + alt_entity_type_bytes
            + linear_velocity_bytes
            + location_bytes
            + orientation_bytes
            + appearance_bytes
            + dr_params_bytes
            + capabilities_bytes
            + 40
            + marking_bytes
            + 4
            + variable_params_bytes
    }
}

impl Serialize for EntityAppearance {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let appearance: u32 = u32::from(self);
        buf.put_u32(appearance);
        4
    }
}

impl Serialize for DrParameters {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.algorithm.into());
        let other_parameters_bytes = self.other_parameters.serialize(buf);
        let lin_acc_bytes = self.linear_acceleration.serialize(buf);
        let ang_vel_bytes = self.angular_velocity.serialize(buf);
        1 + other_parameters_bytes + lin_acc_bytes + ang_vel_bytes
    }
}

impl Serialize for DrOtherParameters {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        match self {
            DrOtherParameters::None(bytes) => {
                for x in bytes {
                    buf.put_u8(*x);
                }
                15
            }
            DrOtherParameters::LocalEulerAngles(params) => params.serialize(buf),
            DrOtherParameters::WorldOrientationQuaternion(params) => params.serialize(buf),
        }
    }
}

impl Serialize for DrEulerAngles {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(DrParametersType::LocalEulerAngles_Yaw_Pitch_Roll_.into());
        buf.put_u16(0u16);
        buf.put_f32(self.local_yaw);
        buf.put_f32(self.local_pitch);
        buf.put_f32(self.local_roll);
        15
    }
}

impl Serialize for DrWorldOrientationQuaternion {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(DrParametersType::WorldOrientationQuaternion.into());
        buf.put_u16(self.nil);
        buf.put_f32(self.x);
        buf.put_f32(self.y);
        buf.put_f32(self.z);
        15
    }
}

impl Serialize for ForceId {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let force_id = *self;
        buf.put_u8(force_id.into());
        1
    }
}

impl Serialize for EntityType {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.kind.into());
        buf.put_u8(self.domain.into());
        buf.put_u16(self.country.into());
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory);
        buf.put_u8(self.specific);
        buf.put_u8(self.extra);
        8
    }
}

impl Serialize for EntityMarking {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.marking_character_set.into());
        let num_pad = 11 - self.marking_string.len();
        let marking = self.marking_string.clone(); // clone necessary because into_bytes consumes self.

        buf.put_slice(&marking.into_bytes()[..]);
        (0..num_pad).for_each(|_i| buf.put_u8(0x20));
        12
    }
}

#[cfg(test)]
mod tests {
    use crate::common::entity_state::model::{
        DrOtherParameters, DrParameters, EntityAppearance, EntityMarking, EntityState,
    };
    use crate::common::model::{
        ArticulatedPart, EntityId, EntityType, Location, Orientation, Pdu, PduHeader,
        VariableParameter, VectorF32,
    };
    use crate::common::Serialize;
    use crate::enumerations::{
        AirPlatformAppearance, AppearanceAntiCollisionDayNight, AppearanceCanopy, AppearanceDamage,
        AppearanceEntityOrObjectState, AppearanceNVGMode, AppearanceNavigationPositionBrightness,
        AppearancePaintScheme, AppearanceTrailingEffects,
    };
    use crate::enumerations::{
        ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, ChangeIndicator, Country,
        DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId, PduType,
        PlatformDomain,
    };
    use bytes::BytesMut;

    #[test]
    fn entity_marking() {
        let marking = EntityMarking {
            marking_character_set: EntityMarkingCharacterSet::ASCII,
            marking_string: "EYE 10".to_string(),
        };
        let mut buf = BytesMut::with_capacity(11);

        marking.serialize(&mut buf);

        let expected: [u8; 12] = [
            0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20,
        ];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }

    #[test]
    fn articulated_part() {
        let articulated_part = VariableParameter::Articulated(ArticulatedPart {
            change_indicator: ChangeIndicator::from(0u8),
            attachment_id: 0,
            type_class: ArticulatedPartsTypeClass::LandingGear,
            type_metric: ArticulatedPartsTypeMetric::Position,
            parameter_value: 1.0,
        });

        let mut buf = BytesMut::with_capacity(11);

        articulated_part.serialize(&mut buf);

        let expected: [u8; 16] = [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x01, 0x3f, 0x80, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }

    #[test]
    fn entity_state_pdu() {
        let header = PduHeader::new_v6(1, PduType::EntityState);

        let body = EntityState::builder()
            .with_entity_id(EntityId::new(500, 900, 14))
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
                other_parameters: DrOtherParameters::None([0u8;15]),
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
        let pdu = Pdu::finalize_from_parts(header, body, 0);

        let mut buf = BytesMut::with_capacity(208);

        pdu.serialize(&mut buf).unwrap();

        let expected: [u8; 208] = [
            0x06, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0x00, 0x00, 0x01, 0xf4,
            0x03, 0x84, 0x00, 0x0e, 0x01, 0x04, 0x01, 0x02, 0x00, 0x99, 0x32, 0x04, 0x04, 0x00,
            0x01, 0x02, 0x00, 0x99, 0x32, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x01, 0x3f, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0b,
            0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x10, 0x0c, 0x40, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x11, 0x4d, 0x40, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];

        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}
