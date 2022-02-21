use bytes::{BufMut, BytesMut};
use crate::dis::common::Serialize;
use crate::dis::v7::entity_state::model::{AirPlatformsRecord, Appearance, ArticulationParameter, DrParameters, EntityCapabilities, EntityId, EntityMarking, EntityState, EntityType, EnvironmentalsRecord, ForceId, GeneralAppearance, GuidedMunitionsRecord, LandPlatformsRecord, LifeFormsRecord, Location, Orientation, ParameterTypeVariant, SimulationAddress, SpacePlatformsRecord, SpecificAppearance, SubsurfacePlatformsRecord, SurfacePlatformRecord, VectorF32};

impl Serialize for EntityState {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let header_bytes = self.header.serialize(buf);
        let entity_id_bytes = self.entity_id.serialize(buf);
        let force_id_bytes = self.force_id.serialize(buf);
        buf.put_u8(self.articulated_parts_no);

        let entity_type_bytes = self.entity_type.serialize(buf);
        let alt_entity_type_bytes = self.alternative_entity_type.serialize(buf);

        let linear_velocity_bytes = self.entity_linear_velocity.serialize(buf);
        let location_bytes = self.entity_location.serialize(buf);
        let orientation_bytes = self.entity_orientation.serialize(buf);

        let appearance_bytes = self.entity_appearance.serialize(buf);
        let dr_params_bytes = self.dead_reckoning_parameters.serialize(buf);

        let marking_bytes = self.entity_marking.serialize(buf);
        let capabilities_bytes = self.entity_capabilities.serialize(buf);

        let art_params_bytes = if let Some(params) = &self.articulation_parameter {
            let mut num_bytes = 0;
            for param in params {
                num_bytes += param.serialize(buf);
            }
            num_bytes
        } else { 0 };

        header_bytes + entity_id_bytes + force_id_bytes + 1 + entity_type_bytes
            + alt_entity_type_bytes + linear_velocity_bytes + location_bytes
            + orientation_bytes + appearance_bytes + dr_params_bytes + capabilities_bytes + 40 + marking_bytes + 4 + art_params_bytes
    }
}

impl Serialize for ArticulationParameter {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.parameter_type_designator.into());
        buf.put_u8(self.parameter_change_indicator);
        buf.put_u16(self.articulation_attachment_id);
        match &self.parameter_type_variant {
            ParameterTypeVariant::AttachedParts(attached) => { buf.put_u32(*attached); }
            ParameterTypeVariant::ArticulatedParts(articulated) => {
                let on_wire_value = articulated.type_class + &articulated.type_metric.into();
                buf.put_u32(on_wire_value);
            }
        }
        buf.put_f32(self.articulation_parameter_value);
        buf.put_u32(0u32); // 4-byte padding
        16
    }
}

impl Serialize for EntityId {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let num_bytes = self.simulation_address.serialize(buf);
        buf.put_u16(self.entity_id);
        num_bytes + 2
    }
}

impl Serialize for SimulationAddress {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u16(self.site_id);
        buf.put_u16(self.application_id);
        4
    }
}

impl Serialize for ForceId {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let force_id = *self;
        buf.put_u8(force_id.into());
        1
    }
}

impl Serialize for EntityType {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.kind.into());
        buf.put_u8(self.domain);
        buf.put_u16(self.country.into()); // TODO: country Into<u16>;
        buf.put_u8(self.category);
        buf.put_u8(self.subcategory);
        buf.put_u8(self.specific);
        buf.put_u8(self.extra);
        8
    }
}

impl Serialize for VectorF32 {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32(self.first_vector_component);
        buf.put_f32(self.second_vector_component);
        buf.put_f32(self.third_vector_component);
        12
    }
}

impl Serialize for Location {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f64(self.x_coordinate);
        buf.put_f64(self.y_coordinate);
        buf.put_f64(self.z_coordinate);
        24
    }
}

impl Serialize for Orientation {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32(self.psi);
        buf.put_f32(self.theta);
        buf.put_f32(self.phi);
        12
    }
}

impl Serialize for EntityMarking {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.marking_character_set.into());
        let num_pad = 11 - self.marking_string.len();
        let marking = self.marking_string.clone(); // TODO is this clone necessary?
// for b in marking.as_bytes() {
//     write!(buf, "{:02x}", b);
// }
        buf.put_slice(&marking.into_bytes()[..]);
        (0..num_pad).for_each( |_i| buf.put_u8(0x20) );
        12
    }
}

impl Serialize for Appearance {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let general_bytes = self.general_appearance.serialize(buf);
        let specific_bytes = self.specific_appearance.serialize(buf);
        general_bytes + specific_bytes
    }
}

impl Serialize for GeneralAppearance {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let entity_paint_scheme : u16 = self.entity_paint_scheme.into();
        let entity_paint_scheme = entity_paint_scheme << 15;
        let entity_mobility_kill : u16 = self.entity_mobility_kill.into();
        let entity_mobility_kill = entity_mobility_kill << 14;
        let entity_fire_power : u16 = self.entity_fire_power.into();
        let entity_fire_power = entity_fire_power << 13;
        let entity_damage : u16 = self.entity_damage.into();
        let entity_damage = entity_damage << 11;
        let entity_smoke : u16 = self.entity_smoke.into();
        let entity_smoke = entity_smoke << 9;
        let entity_trailing_effect : u16 = self.entity_trailing_effect.into();
        let entity_trailing_effect = entity_trailing_effect << 7;
        let entity_hatch_state : u16 = self.entity_hatch_state.into();
        let entity_hatch_state = entity_hatch_state << 4;
        let entity_lights : u16 = self.entity_lights.into();
        let entity_lights = entity_lights << 1;
        let entity_flaming_effect : u16 = self.entity_flaming_effect.into();

        let general_appearance : u16 = 0u16 | entity_paint_scheme | entity_mobility_kill
            | entity_fire_power | entity_damage | entity_smoke | entity_trailing_effect
            | entity_hatch_state | entity_lights | entity_flaming_effect;
        buf.put_u16(general_appearance);
        2
    }
}

impl Serialize for SpecificAppearance {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        match self {
            SpecificAppearance::LandPlatform(record) => { record.serialize(buf) }
            SpecificAppearance::AirPlatform(record) => { record.serialize(buf) }
            SpecificAppearance::SurfacePlatform(record) => { record.serialize(buf) }
            SpecificAppearance::SubsurfacePlatform(record) => { record.serialize(buf) }
            SpecificAppearance::SpacePlatform(record) => { record.serialize(buf) }
            SpecificAppearance::GuidedMunition(record) => { record.serialize(buf) }
            SpecificAppearance::LifeForm(record) => { record.serialize(buf) }
            SpecificAppearance::Environmental(record) => { record.serialize(buf) }
            SpecificAppearance::Other(bytes) => { buf.put_slice(bytes); 2 }
        }
    }
}

impl Serialize for LandPlatformsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let launcher : u16 = self.launcher.into();
        let launcher = launcher << 15;
        let camouflage : u16 = self.camouflage_type.into();
        let camouflage = camouflage << 13;
        let concealed : u16 = self.concealed.into();
        let concealed = concealed << 12;
        let frozen_status : u16 = self.frozen_status.into();
        let frozen_status = frozen_status << 10;
        let power_plant_status : u16 = self.power_plant_status.into();
        let power_plant_status = power_plant_status << 9;
        let state : u16 = self.state.into();
        let state = state << 8;
        let tent : u16 = self.tent.into();
        let tent = tent << 7;
        let ramp : u16 = self.ramp.into();
        let ramp = ramp << 6;

        let land_appearance = 0u16 | launcher | camouflage | concealed
            | frozen_status | power_plant_status | state | tent | ramp;
        buf.put_u16(land_appearance);
        2
    }
}

impl Serialize for AirPlatformsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let afterburner : u16 = self.afterburner.into();
        let afterburner = afterburner << 15;
        let frozen_status : u16 = self.frozen_status.into();
        let frozen_status = frozen_status << 10;
        let power_plant_status : u16 = self.power_plant_status.into();
        let power_plant_status = power_plant_status << 9;
        let state : u16 = self.state.into();
        let state = state << 8;

        let air_appearance : u16 = 0u16 | afterburner | frozen_status | power_plant_status | state;
        buf.put_u16(air_appearance);
        2
    }
}

impl Serialize for SurfacePlatformRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let frozen_status : u16 = self.frozen_status.into();
        let frozen_status = frozen_status << 10;
        let power_plant_status : u16 = self.power_plant_status.into();
        let power_plant_status = power_plant_status << 9;
        let state : u16 = self.state.into();
        let state = state << 8;

        let surface_appearance = 0u16 | frozen_status | power_plant_status | state;
        buf.put_u16(surface_appearance);
        2
    }
}

impl Serialize for SubsurfacePlatformsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let frozen_status : u16 = self.frozen_status.into();
        let frozen_status = frozen_status << 10;
        let power_plant_status : u16 = self.power_plant_status.into();
        let power_plant_status = power_plant_status << 9;
        let state : u16 = self.state.into();
        let state = state << 8;

        let subsurface_appearance = 0u16 | frozen_status | power_plant_status | state;
        buf.put_u16(subsurface_appearance);
        2
    }
}

impl Serialize for SpacePlatformsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let frozen_status : u16 = self.frozen_status.into();
        let frozen_status = frozen_status << 10;
        let power_plant_status : u16 = self.power_plant_status.into();
        let power_plant_status = power_plant_status << 9;
        let state : u16 = self.state.into();
        let state = state << 8;

        let space_appearance = 0u16 | frozen_status | power_plant_status | state;
        buf.put_u16(space_appearance);
        2
    }
}

impl Serialize for GuidedMunitionsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let launch_flash : u16 = self.launch_flash.into();
        let frozen_status : u16 = self.frozen_status.into();
        let state : u16 = self.state.into();

        let guided_appearance = 0u16
            | (launch_flash << 15)
            | (frozen_status << 10)
            | (state << 8);
        buf.put_u16(guided_appearance);
        2
    }
}

impl Serialize for LifeFormsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let life_form_state : u16 = self.life_form_state.into();
        let frozen_status : u16 = self.frozen_status.into();
        let activity_state : u16 = self.activity_state.into();
        let weapon_1 : u16 = self.weapon_1.into();
        let weapon_2 : u16 = self.weapon_2.into();

        let life_form_appearance = 0u16
            | (life_form_state << 12)
            | (frozen_status << 10)
            | (activity_state << 8)
            | (weapon_1 << 6)
            | (weapon_2 << 4);
        buf.put_u16(life_form_appearance);
        2
    }
}

impl Serialize for EnvironmentalsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let density : u16 = self.density.into();

        let env_appearance = 0u16 | (density << 12);
        buf.put_u16(env_appearance);
        2
    }
}

impl Serialize for DrParameters {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.algorithm.into());
        buf.put_bytes(0u8, 15);
        let lin_acc_bytes = self.linear_acceleration.serialize(buf);
        let ang_vel_bytes = self.angular_velocity.serialize(buf);
        1 + 15 + lin_acc_bytes + ang_vel_bytes
    }
}

impl Serialize for EntityCapabilities {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let ammunition_supply = if self.ammunition_supply { 1u32 } else { 0u32 } << 31;
        let fuel_supply = if self.fuel_supply { 1u32 } else { 0u32 } << 30;
        let recovery = if self.recovery { 1u32 } else { 0u32 } << 29;
        let repair = if self.repair { 1u32 } else { 0u32 } << 28;
        let capabilities = 0u32 | ammunition_supply | fuel_supply | recovery | repair;
        buf.put_u32(capabilities);
        4
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::dis::common::Serialize;
    use crate::dis::common::model::{PduType, ProtocolFamily, ProtocolVersion};
    use crate::dis::v7::entity_state::builder::GeneralAppearanceBuilder;
    use crate::dis::v7::entity_state::model::{Afterburner, AirPlatformsRecord, Appearance, ApTypeDesignator, ApTypeMetric, ArticulatedParts, ArticulationParameter, Country, DrAlgorithm, DrParameters, EntityDamage, EntityFirePower, EntityFlamingEffect, EntityHatchState, EntityId, EntityKind, EntityLights, EntityMarking, EntityMarkingCharacterSet, EntityMobilityKill, EntityPaintScheme, EntitySmoke, EntityState, EntityTrailingEffect, EntityType, ForceId, FrozenStatus, Location, Orientation, ParameterTypeVariant, PowerPlantStatus, SimulationAddress, SpecificAppearance, State, VectorF32};
    use crate::dis::v7::model::{Pdu, PduHeader};

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
        let articulated_part = ArticulationParameter {
            parameter_type_designator: ApTypeDesignator::Articulated,
            parameter_change_indicator: 0,
            articulation_attachment_id: 0,
            parameter_type_variant: ParameterTypeVariant::ArticulatedParts(ArticulatedParts {
                type_class: 3072,
                type_metric: ApTypeMetric::Position
            }),
            articulation_parameter_value: 1.0
        };
        let mut buf = BytesMut::with_capacity(11);

        articulated_part.serialize(&mut buf);

        let expected : [u8;16] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x0c, 0x01, 0x3f, 0x80,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        assert_eq!(buf.as_ref(), expected.as_ref())
    }

    #[test]
    fn entity_state_pdu() {
        let pdu = EntityState::builder().header(
            PduHeader::builder()
                .protocol_version(ProtocolVersion::Ieee1278_1a1998)
                .exercise_id(1)
                .pdu_type(PduType::EntityStatePdu)
                .protocol_family(ProtocolFamily::EntityInformationInteraction)
                .time_stamp(0)
                .pdu_length(208u16)
                .build().expect("Should be Ok"))
            .entity_id(EntityId {
                simulation_address: SimulationAddress {site_id: 500, application_id: 900 },
                entity_id: 14
            })
            .force_id(ForceId::Friendly)
            .entity_type(EntityType {
                kind: EntityKind::Platform, domain: 2, country: Country::Netherlands, category: 50, subcategory: 4, specific: 4, extra: 0
            })
            .alt_entity_type(EntityType {
                kind: EntityKind::Platform, domain: 2, country: Country::Netherlands, category: 50, subcategory: 4, specific: 4, extra: 0
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
            .appearance(Appearance {
                general_appearance: GeneralAppearanceBuilder::new()
                    .entity_paint_scheme(EntityPaintScheme::UniformColor)
                    .entity_mobility_kill(EntityMobilityKill::NoMobilityKill)
                    .entity_fire_power(EntityFirePower::NoFirePowerKill)
                    .entity_damage(EntityDamage::NoDamage)
                    .entity_smoke(EntitySmoke::EmittingEngineSmoke)
                    .entity_trailing_effect(EntityTrailingEffect::None)
                    .entity_hatch_state(EntityHatchState::NotApplicable)
                    .entity_lights(EntityLights::None)
                    .entity_flaming_effect(EntityFlamingEffect::None)
                    .build().expect("Should be Ok"),
                specific_appearance: SpecificAppearance::AirPlatform(AirPlatformsRecord {
                    afterburner: Afterburner::NotOn,
                    frozen_status: FrozenStatus::NotFrozen,
                    power_plant_status: PowerPlantStatus::Off,
                    state: State::Active,
                })
            })
            .dead_reckoning(DrParameters {
                algorithm: DrAlgorithm::DrmRVW,
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
            .add_articulation_parameter(ArticulationParameter {
                parameter_type_designator: ApTypeDesignator::Articulated,
                parameter_change_indicator: 0,
                articulation_attachment_id: 0,
                parameter_type_variant: ParameterTypeVariant::ArticulatedParts(ArticulatedParts {
                    type_class: 3072,
                    type_metric: ApTypeMetric::Position
                }),
                articulation_parameter_value: 1.0
            })
            .add_articulation_parameter(ArticulationParameter {
                parameter_type_designator: ApTypeDesignator::Articulated,
                parameter_change_indicator: 0,
                articulation_attachment_id: 0,
                parameter_type_variant: ParameterTypeVariant::ArticulatedParts(ArticulatedParts {
                    type_class: 4096,
                    type_metric: ApTypeMetric::Azimuth
                }),
                articulation_parameter_value: 0.0
            })
            .add_articulation_parameter(ArticulationParameter {
                parameter_type_designator: ApTypeDesignator::Articulated,
                parameter_change_indicator: 0,
                articulation_attachment_id: 0,
                parameter_type_variant: ParameterTypeVariant::ArticulatedParts(ArticulatedParts {
                    type_class: 4096,
                    type_metric: ApTypeMetric::AzimuthRate
                }),
                articulation_parameter_value: 0.0
            })
            .add_articulation_parameter(ArticulationParameter {
                parameter_type_designator: ApTypeDesignator::Articulated,
                parameter_change_indicator: 0,
                articulation_attachment_id: 0,
                parameter_type_variant: ParameterTypeVariant::ArticulatedParts(ArticulatedParts {
                    type_class: 4416,
                    type_metric: ApTypeMetric::Elevation
                }),
                articulation_parameter_value: 0.0
            })
            .build().expect("Should be Ok");
        let pdu = Pdu::EntityState(pdu);

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