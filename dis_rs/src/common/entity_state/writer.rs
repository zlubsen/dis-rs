use bytes::{BufMut, BytesMut};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::common::entity_state::model::{EntityMarking, EntityState, ParameterVariant, VariableParameter};
use crate::common::model::EntityType;
use crate::EntityAppearance;
use crate::enumerations::ForceId;
use crate::v6::entity_state::model::{DrParameters, EntityCapabilities};

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
                let capabilities : EntityCapabilities = self.entity_capabilities.into();
                capabilities.serialize(buf)
            }
            SupportedVersion::V7 => {
                buf.put_u32(self.entity_capabilities.into());
                4
            }
            // TODO should not be possible to construct such a PDU, but need to handle the case
            SupportedVersion::Unsupported => {
                buf.put_u32(0u32); 4
            }
        };

        let variable_params_bytes = {
            let mut num_bytes = 0;
            for param in &self.variable_parameters {
                num_bytes += param.serialize(buf);
            }
            num_bytes
        };

        entity_id_bytes + force_id_bytes + 1 + entity_type_bytes
            + alt_entity_type_bytes + linear_velocity_bytes + location_bytes
            + orientation_bytes + appearance_bytes + dr_params_bytes + capabilities_bytes + 40 + marking_bytes + 4 + variable_params_bytes
    }
}

impl Serialize for VariableParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.parameter_type_designator.into());
        buf.put_u8(self.changed_attached_indicator);
        buf.put_u16(self.articulation_attachment_id);
        match &self.parameter {
            ParameterVariant::Attached(attached) => {
                let parameter_type : u32 = attached.parameter_type.into();
                buf.put_u32(parameter_type);
                attached.attached_part_type.serialize(buf);
            }
            ParameterVariant::Articulated(articulated) => {
                let type_class : u32 = articulated.type_class.into();
                let type_metric : u32 = articulated.type_metric.into();
                let on_wire_value = type_class + type_metric;
                buf.put_u32(on_wire_value);
                buf.put_f32(articulated.parameter_value);
                buf.put_u32(0u32); // 4-byte padding
            }
        }
        16
    }
}

impl Serialize for EntityAppearance {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let appearance: u32 = match self {
            EntityAppearance::LandPlatform(appearance) => u32::from(*appearance),
            EntityAppearance::AirPlatform(appearance) => u32::from(*appearance),
            EntityAppearance::SurfacePlatform(appearance) => u32::from(*appearance),
            EntityAppearance::SubsurfacePlatform(appearance) => u32::from(*appearance),
            EntityAppearance::SpacePlatform(appearance) => u32::from(*appearance),
            EntityAppearance::Munition(appearance) => u32::from(*appearance),
            EntityAppearance::LifeForms(appearance) => u32::from(*appearance),
            EntityAppearance::Environmental(appearance) => u32::from(*appearance),
            EntityAppearance::CulturalFeature(appearance) => u32::from(*appearance),
            EntityAppearance::Supply(appearance) => u32::from(*appearance),
            EntityAppearance::Radio(appearance) => u32::from(*appearance),
            EntityAppearance::Expendable(appearance) => u32::from(*appearance),
            EntityAppearance::SensorEmitter(appearance) => u32::from(*appearance),
            EntityAppearance::Unspecified(appearance) => u32::from(*appearance),
        };
        buf.put_u32(appearance);
        4
    }
}

impl Serialize for DrParameters {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.algorithm.into());
        buf.put_bytes(0u8, 15);
        let lin_acc_bytes = self.linear_acceleration.serialize(buf);
        let ang_vel_bytes = self.angular_velocity.serialize(buf);
        1 + 15 + lin_acc_bytes + ang_vel_bytes
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
        let marking = self.marking_string.clone(); // TODO is this clone necessary?

        buf.put_slice(&marking.into_bytes()[..]);
        (0..num_pad).for_each( |_i| buf.put_u8(0x20) );
        12
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::entity_state::model::{ArticulatedPart, EntityMarking, EntityState, ParameterVariant, VariableParameter};
    use crate::common::model::{EntityId, EntityType, Location, Orientation, Pdu, PduHeader, SimulationAddress, VectorF32};
    use crate::common::Serialize;
    use crate::EntityAppearance;
    use crate::enumerations::{ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, Country, DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId, PduType, PlatformDomain, VariableParameterRecordType};
    use crate::enumerations::{AirPlatformAppearance, AppearancePaintScheme, AppearanceNVGMode, AppearanceDamage, AppearanceTrailingEffects, AppearanceCanopy, AppearanceAntiCollisionDayNight, AppearanceEntityorObjectState, AppearanceNavigationPositionBrightness};
    use crate::v6::entity_state::model::{DrParameters};

    #[test]
    fn entity_marking() {
        let marking = EntityMarking {
            marking_character_set: EntityMarkingCharacterSet::ASCII,
            marking_string: "EYE 10".to_string(),
        };
        let mut buf = BytesMut::with_capacity(11);

        marking.serialize(&mut buf);

        let expected : [u8;12] = [0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20];
        assert_eq!(buf.as_ref(), expected.as_ref())
    }

    #[test]
    fn articulated_part() {
        let articulated_part = VariableParameter {
            parameter_type_designator: VariableParameterRecordType::ArticulatedPart,
            changed_attached_indicator: 0,
            articulation_attachment_id: 0,
            parameter: ParameterVariant::Articulated(ArticulatedPart {
                type_class: ArticulatedPartsTypeClass::LandingGear,
                type_metric: ArticulatedPartsTypeMetric::Position,
                parameter_value: 1.0,
            }),
        };
        let mut buf = BytesMut::with_capacity(11);

        articulated_part.serialize(&mut buf);

        let expected : [u8;16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x01, 0x3f, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(buf.as_ref(), expected.as_ref())
    }

    #[test]
    fn entity_state_pdu() {
        let mut header = PduHeader::v6_builder()
            .exercise_id(1)
            .pdu_type(PduType::EntityState)
            .build();
        // TODO replace custom builder with buildstructor
        let body = EntityState::builder()
            .entity_id(EntityId {
                simulation_address: SimulationAddress {site_id: 500, application_id: 900 },
                entity_id: 14
            })
            .force_id(ForceId::Friendly)
            .entity_type(EntityType {
                kind: EntityKind::Platform, domain: PlatformDomain::Air, country: Country::Netherlands_NLD_, category: 50, subcategory: 4, specific: 4, extra: 0
            })
            .alt_entity_type(EntityType {
                kind: EntityKind::Platform, domain: PlatformDomain::Air, country: Country::Netherlands_NLD_, category: 50, subcategory: 4, specific: 4, extra: 0
            })
            .linear_velocity(VectorF32 {
                first_vector_component: 0f32, second_vector_component: 0f32, third_vector_component: 0f32
            })
            .location(Location {
                x_coordinate: 0f64, y_coordinate : 0f64, z_coordinate: 0f64
            })
            .orientation(Orientation {
                psi: 0f32, theta: 0f32, phi: 0f32
            })
            .appearance(EntityAppearance::AirPlatform(AirPlatformAppearance {
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
                state: AppearanceEntityorObjectState::Active,
                formation_lights_on: false,
                landing_gear_extended: false,
                cargo_doors_opened: false,
                navigation_position_brightness: AppearanceNavigationPositionBrightness::Dim,
                spot_search_light_1_on: false,
                interior_lights_on: false,
                reverse_thrust_engaged: false,
                weightonwheels: false,
            }))
            .dead_reckoning(DrParameters {
                algorithm: DeadReckoningAlgorithm::DRM_RVW_HighSpeedorManeuveringEntitywithExtrapolationofOrientation,
                other_parameters: [0u8;15],
                linear_acceleration: VectorF32 {
                    first_vector_component: 0f32, second_vector_component: 0f32, third_vector_component: 0f32
                },
                angular_velocity: VectorF32 {
                    first_vector_component: 0f32, second_vector_component: 0f32, third_vector_component: 0f32
                }
            })
            .marking(EntityMarking {
                marking_character_set: EntityMarkingCharacterSet::ASCII,
                marking_string: "EYE 10".to_string()
            })
            .capabilities_flags(false, false, false, false)
            .add_articulation_parameter(VariableParameter {
                parameter_type_designator: VariableParameterRecordType::ArticulatedPart,
                changed_attached_indicator: 0,
                articulation_attachment_id: 0,
                parameter: ParameterVariant::Articulated(ArticulatedPart {
                    type_class: ArticulatedPartsTypeClass::LandingGear,
                    type_metric: ArticulatedPartsTypeMetric::Position,
                    parameter_value: 1.0,
                }),
            })
            .add_articulation_parameter(VariableParameter {
                parameter_type_designator: VariableParameterRecordType::ArticulatedPart,
                changed_attached_indicator: 0,
                articulation_attachment_id: 0,
                parameter: ParameterVariant::Articulated(ArticulatedPart {
                    type_class: ArticulatedPartsTypeClass::PrimaryTurretNumber1,
                    type_metric: ArticulatedPartsTypeMetric::Azimuth,
                    parameter_value: 0.0,
                }),
            })
            .add_articulation_parameter(VariableParameter {
                parameter_type_designator: VariableParameterRecordType::ArticulatedPart,
                changed_attached_indicator: 0,
                articulation_attachment_id: 0,
                parameter: ParameterVariant::Articulated(ArticulatedPart {
                    type_class: ArticulatedPartsTypeClass::PrimaryTurretNumber1,
                    type_metric: ArticulatedPartsTypeMetric::AzimuthRate,
                    parameter_value: 0.0,
                }),
            })
            .add_articulation_parameter(VariableParameter {
                parameter_type_designator: VariableParameterRecordType::ArticulatedPart,
                changed_attached_indicator: 0,
                articulation_attachment_id: 0,
                parameter: ParameterVariant::Articulated(ArticulatedPart {
                    type_class: ArticulatedPartsTypeClass::PrimaryGunNumber1,
                    type_metric: ArticulatedPartsTypeMetric::Elevation,
                    parameter_value: 0.0,
                }),
            })
            .build().expect("Should be Ok");
        let pdu = Pdu::finalize_from_parts(header, body, 0);

        let mut buf = BytesMut::with_capacity(208);

        pdu.serialize(&mut buf);

        let expected : [u8;208] =
            [0x06, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0xd0, 0x00, 0x00, 0x01, 0xf4, 0x03, 0x84,
                0x00, 0x0e, 0x01, 0x04, 0x01, 0x02, 0x00, 0x99, 0x32, 0x04, 0x04, 0x00, 0x01, 0x02, 0x00, 0x99,
                0x32, 0x04, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x01, 0x45, 0x59, 0x45, 0x20, 0x31, 0x30, 0x20, 0x20, 0x20, 0x20, 0x20, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x01, 0x3f, 0x80, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0b, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10, 0x0c, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x11, 0x4d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];

        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}