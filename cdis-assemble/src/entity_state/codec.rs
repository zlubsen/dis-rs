use crate::codec::{
    Codec, CodecOptimizeMode, CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState,
    EncoderState,
};
use crate::entity_state::model::{CdisDRParametersOther, CdisEntityCapabilities, EntityState};
use crate::records::codec::{decode_world_coordinates, encode_world_coordinates};
use crate::records::model::{
    AngularVelocity, CdisEntityMarking, CdisVariableParameter, EntityId, EntityType,
    LinearAcceleration, LinearVelocity, Orientation, UnitsDekameters,
};
use crate::types::model::{UVINT32, UVINT8};
use crate::{BodyProperties, CdisBody};
use dis_rs::entity_state::model::{DrParameters, EntityAppearance, EntityMarking};
use dis_rs::enumerations::{
    DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId, PlatformDomain,
};
use dis_rs::model::{
    EntityType as DisEntityType, Location as DisLocation, Orientation as DisOrientation, PduBody,
    VectorF32,
};
use std::time::Instant;

type Counterpart = dis_rs::entity_state::model::EntityState;

pub(crate) fn encode_entity_state_body_and_update_state(
    dis_body: &Counterpart,
    state: &mut EncoderState,
    options: &CodecOptions,
) -> (CdisBody, CodecStateResult) {
    let state_for_id = state.entity_state.get(&dis_body.entity_id);

    let (cdis_body, state_result) = EntityState::encode(dis_body, state_for_id, options);

    if state_result == CodecStateResult::StateUpdateEntityState {
        state
            .entity_state
            .entry(dis_body.entity_id)
            .and_modify(|es| es.heartbeat = Instant::now())
            .or_default();
    }

    (cdis_body.into_cdis_body(), state_result)
}

pub(crate) fn decode_entity_state_body_and_update_state(
    cdis_body: &EntityState,
    state: &mut DecoderState,
    options: &CodecOptions,
) -> (PduBody, CodecStateResult) {
    let state_for_id = state
        .entity_state
        .get(&dis_rs::model::EntityId::from(&cdis_body.entity_id));
    let (dis_body, state_result) = cdis_body.decode(state_for_id, options);

    if state_result == CodecStateResult::StateUpdateEntityState {
        state
            .entity_state
            .entry(dis_rs::model::EntityId::from(&cdis_body.entity_id))
            .and_modify(|es| {
                es.heartbeat = Instant::now();
                es.force_id = dis_body.force_id;
                es.entity_type = dis_body.entity_type;
                es.entity_location = dis_body.entity_location;
                es.entity_orientation = dis_body.entity_orientation;
                es.entity_appearance = dis_body.entity_appearance;
            })
            .or_insert(DecoderStateEntityState::new(&dis_body));
    }

    (dis_body.into_pdu_body(), state_result)
}

#[derive(Debug)]
pub struct EncoderStateEntityState {
    pub heartbeat: Instant,
}

impl Default for EncoderStateEntityState {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateEntityState {
    pub heartbeat: Instant,
    pub force_id: ForceId,
    pub entity_type: DisEntityType,
    pub entity_location: DisLocation,
    pub entity_orientation: DisOrientation,
    pub entity_appearance: EntityAppearance,
    pub entity_marking: EntityMarking,
}

impl DecoderStateEntityState {
    #[must_use]
    pub fn new(pdu: &Counterpart) -> Self {
        Self {
            heartbeat: Instant::now(),
            force_id: pdu.force_id,
            entity_type: pdu.entity_type,
            entity_location: pdu.entity_location,
            entity_orientation: pdu.entity_orientation,
            entity_appearance: pdu.entity_appearance,
            entity_marking: pdu.entity_marking.clone(),
        }
    }
}

impl Default for DecoderStateEntityState {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
            force_id: ForceId::default(),
            entity_type: DisEntityType::default(),
            entity_location: DisLocation::default(),
            entity_orientation: DisOrientation::default(),
            entity_appearance: EntityAppearance::default(),
            entity_marking: EntityMarking::default(),
        }
    }
}

impl EntityState {
    pub fn encode(
        item: &Counterpart,
        state: Option<&EncoderStateEntityState>,
        options: &CodecOptions,
    ) -> (Self, CodecStateResult) {
        let alternate_entity_type = if item.alternative_entity_type != DisEntityType::default() {
            Some(EntityType::encode(&item.alternative_entity_type))
        } else {
            None
        };
        let entity_linear_velocity = encode_ent_linear_velocity(item);
        let dr_params_other = encode_dr_params_other(item);
        let dr_params_entity_linear_acceleration = encode_dr_linear_acceleration(
            item.dead_reckoning_parameters.algorithm,
            &item.dead_reckoning_parameters.linear_acceleration,
        );
        let dr_params_entity_angular_velocity = encode_dr_angular_velocity(item);
        let capabilities = encode_entity_capabilities(item, options);

        let (
            units,
            full_update_flag,
            force_id,
            entity_type,
            entity_location,
            entity_orientation,
            entity_appearance,
            entity_marking,
            state_result,
        ) = if options.update_mode == CodecUpdateMode::PartialUpdate
            && state.is_some_and(|state| {
                !evaluate_timeout_for_entity_type(&item.entity_type, &state.heartbeat, options)
            }) {
            // Do not update stateful fields when a full update is not required
            (
                UnitsDekameters::Dekameter,
                false,
                None,
                None,
                None,
                None,
                None,
                None,
                CodecStateResult::StateUnaffected,
            )
        } else {
            // full update mode, or partial with a (state) timeout on the entity
            let (entity_location, units) = encode_world_coordinates(&item.entity_location);
            (
                units,
                true,
                Some(UVINT8::from(u8::from(item.force_id))),
                Some(EntityType::encode(&item.entity_type)),
                Some(entity_location),
                Some(Orientation::encode(&item.entity_orientation)),
                Some((&item.entity_appearance).into()),
                Some(CdisEntityMarking::new(
                    item.entity_marking.marking_string.clone(),
                )),
                if options.update_mode == CodecUpdateMode::PartialUpdate {
                    CodecStateResult::StateUpdateEntityState
                } else {
                    CodecStateResult::StateUnaffected
                },
            )
        };

        (
            Self {
                units,
                full_update_flag,
                entity_id: EntityId::encode(&item.entity_id),
                force_id,
                entity_type,
                alternate_entity_type,
                entity_linear_velocity,
                entity_location,
                entity_orientation,
                entity_appearance,
                dr_algorithm: item.dead_reckoning_parameters.algorithm,
                dr_params_other,
                dr_params_entity_linear_acceleration,
                dr_params_entity_angular_velocity,
                entity_marking,
                capabilities,
                variable_parameters: item
                    .variable_parameters
                    .iter()
                    .map(CdisVariableParameter::encode)
                    .collect(),
            },
            state_result,
        )
    }

    #[must_use]
    #[allow(clippy::too_many_lines)]
    pub fn decode(
        &self,
        state: Option<&DecoderStateEntityState>,
        options: &CodecOptions,
    ) -> (Counterpart, CodecStateResult) {
        let (
            force_id,
            entity_type,
            entity_location,
            entity_orientation,
            entity_appearance,
            entity_marking,
            state_result,
        ) = match options.update_mode {
            CodecUpdateMode::FullUpdate => {
                // if in full-update-mode, fill all the fields with what is in the cdis-pdu; initialize to zeroes when not provided. State Unaffected.
                let entity_type = self
                    .entity_type
                    .map(|record| record.decode())
                    .unwrap_or_default();
                (
                    ForceId::from(self.force_id.map(|record| record.value).unwrap_or_default()),
                    entity_type,
                    self.entity_location
                        .map(|world_coordinates| {
                            decode_world_coordinates(&world_coordinates, self.units)
                        })
                        .unwrap_or_default(),
                    self.entity_orientation
                        .map(|record| record.decode())
                        .unwrap_or_default(),
                    self.entity_appearance
                        .as_ref()
                        .map(|cdis| EntityAppearance::from_bytes(cdis.0, &entity_type))
                        .unwrap_or_default(),
                    self.entity_marking
                        .as_ref()
                        .map(|record| {
                            EntityMarking::new(&record.marking, EntityMarkingCharacterSet::ASCII)
                        })
                        .unwrap_or_default(),
                    CodecStateResult::StateUnaffected,
                )
            }
            CodecUpdateMode::PartialUpdate => {
                if self.full_update_flag {
                    // if in partial-update-mode, and receiving a full update: fill all fields and store state. Optional fields are zeroed.
                    let entity_type = self
                        .entity_type
                        .map(|record| record.decode())
                        .unwrap_or_default();
                    (
                        ForceId::from(self.force_id.unwrap_or_default().value),
                        entity_type,
                        self.entity_location
                            .map(|world_coordinates| {
                                decode_world_coordinates(&world_coordinates, self.units)
                            })
                            .unwrap_or_default(),
                        self.entity_orientation
                            .map(|record| record.decode())
                            .unwrap_or_default(),
                        self.entity_appearance
                            .as_ref()
                            .map(|cdis| EntityAppearance::from_bytes(cdis.0, &entity_type))
                            .unwrap_or_default(),
                        self.entity_marking
                            .clone()
                            .map(|record| {
                                EntityMarking::new(record.marking, EntityMarkingCharacterSet::ASCII)
                            })
                            .unwrap_or_default(),
                        CodecStateResult::StateUpdateEntityState,
                    )
                } else {
                    // if in partial-update-mode, and receiving a partial update: fill present fields, else stateful fields from state cache, zeroed otherwise.
                    // TODO when no full update is yet received and no state has been build yet, discard? or init to zeroes?
                    let entity_type = self.entity_type.map_or_else(
                        || {
                            if let Some(state) = state {
                                state.entity_type
                            } else {
                                DisEntityType::default()
                            }
                        },
                        |record| record.decode(),
                    );
                    (
                        self.force_id.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.force_id
                                } else {
                                    ForceId::default()
                                }
                            },
                            |record| ForceId::from(record.value),
                        ),
                        entity_type,
                        self.entity_location.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.entity_location
                                } else {
                                    DisLocation::default()
                                }
                            },
                            |world_coordinates| {
                                decode_world_coordinates(&world_coordinates, self.units)
                            },
                        ),
                        self.entity_orientation.map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.entity_orientation
                                } else {
                                    DisOrientation::default()
                                }
                            },
                            |record| record.decode(),
                        ),
                        self.entity_appearance.as_ref().map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.entity_appearance
                                } else {
                                    EntityAppearance::default()
                                }
                            },
                            |cdis| EntityAppearance::from_bytes(cdis.0, &entity_type),
                        ),
                        self.entity_marking.clone().map_or_else(
                            || {
                                if let Some(state) = state {
                                    state.entity_marking.clone()
                                } else {
                                    EntityMarking::default()
                                }
                            },
                            |record| {
                                EntityMarking::new(record.marking, EntityMarkingCharacterSet::ASCII)
                            },
                        ),
                        CodecStateResult::StateUnaffected,
                    )
                }
            }
        };

        let alternate_entity_type = self
            .alternate_entity_type
            .map(|record| record.decode())
            .unwrap_or_default();

        (
            Counterpart::builder()
                .with_entity_id(self.entity_id.decode())
                .with_force_id(force_id)
                .with_entity_type(entity_type)
                .with_alternative_entity_type(alternate_entity_type)
                .with_velocity(
                    self.entity_linear_velocity
                        .map(|velocity| velocity.decode())
                        .unwrap_or_default(),
                )
                .with_location(entity_location)
                .with_orientation(entity_orientation)
                .with_appearance(entity_appearance)
                .with_dead_reckoning_parameters(
                    DrParameters::default()
                        .with_algorithm(self.dr_algorithm)
                        .with_parameters(
                            self.dr_params_other
                                .map(|other| other.decode(self.dr_algorithm))
                                .unwrap_or_default(),
                        )
                        .with_linear_acceleration(
                            self.dr_params_entity_linear_acceleration
                                .map(|param| param.decode())
                                .unwrap_or_default(),
                        )
                        .with_angular_velocity(
                            self.dr_params_entity_angular_velocity
                                .map(|param| param.decode())
                                .unwrap_or_default(),
                        ),
                )
                .with_marking(entity_marking)
                .with_capabilities(dis_rs::entity_capabilities_from_bytes(
                    self.capabilities
                        .clone()
                        .map(|capes| capes.0.value)
                        .unwrap_or_default(),
                    &entity_type,
                ))
                .with_variable_parameters(
                    self.variable_parameters
                        .iter()
                        .map(crate::codec::Codec::decode)
                        .collect(),
                )
                .build(),
            state_result,
        )
    }
}

/// Encodes the Entity Linear Velocity field when the Dead Reckoning Algorithm requires it (no 2 through 9).
fn encode_ent_linear_velocity(item: &Counterpart) -> Option<LinearVelocity> {
    match item.dead_reckoning_parameters.algorithm {
        DeadReckoningAlgorithm::Other | DeadReckoningAlgorithm::StaticNonmovingEntity => None,
        _ => Some(LinearVelocity::encode(&item.entity_linear_velocity)),
    }
}

/// Encodes the Entity State Capabilities field, when optimizing for completeness and the field is non-zero (i.e., has capabilities).
fn encode_entity_capabilities(
    item: &Counterpart,
    options: &CodecOptions,
) -> Option<CdisEntityCapabilities> {
    match options.optimize_mode {
        CodecOptimizeMode::Bandwidth => None,
        CodecOptimizeMode::Completeness => {
            let capabilities = u32::from(item.entity_capabilities);
            if capabilities != 0 {
                Some(CdisEntityCapabilities(UVINT32::from(capabilities)))
            } else {
                None
            }
        }
    }
}

/// Encodes the Dead Reckoning Parameters Other field, when the DR Algorithm requires it , and the DIS on-wire are non-zero.
#[must_use]
pub fn encode_dr_params_other(item: &Counterpart) -> Option<CdisDRParametersOther> {
    let other_params =
        CdisDRParametersOther::from(&item.dead_reckoning_parameters.other_parameters);
    if item.dead_reckoning_parameters.algorithm != DeadReckoningAlgorithm::Other
        && other_params.0 != 0
    {
        Some(other_params)
    } else {
        None
    }
}

/// Encodes the Dead Reckoning Linear Acceleration field when the Dead Reckoning Algorithm requires it (no 4, 5, 8 and 9).
pub(crate) fn encode_dr_linear_acceleration(
    algorithm: DeadReckoningAlgorithm,
    linear_acceleration: &VectorF32,
) -> Option<LinearAcceleration> {
    match algorithm {
        DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation |
        DeadReckoningAlgorithm::DRM_FVW_HighSpeedOrManeuveringEntity |
        DeadReckoningAlgorithm::DRM_RVB_SimilarToRVWExceptInBodyCoordinates |
        DeadReckoningAlgorithm::DRM_FVB_SimilarToFVWExceptInBodyCoordinates => {
            Some(LinearAcceleration::encode(linear_acceleration))
        }
        _ => { None }
    }
}

/// Encodes the Dead Reckoning Angular Velocity field when the Dead Reckoning Algorithm requires it (no 3, 4, 7 and 8).
fn encode_dr_angular_velocity(item: &Counterpart) -> Option<AngularVelocity> {
    match item.dead_reckoning_parameters.algorithm {
        DeadReckoningAlgorithm::DRM_RPW_ConstantVelocityLowAccelerationLinearMotionEntityWithExtrapolationOfOrientation |
        DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation |
        DeadReckoningAlgorithm::DRM_RPB_SimilarToRPWExceptInBodyCoordinates |
        DeadReckoningAlgorithm::DRM_RVB_SimilarToRVWExceptInBodyCoordinates => {
            Some(AngularVelocity::encode(&item.dead_reckoning_parameters.angular_velocity))
        }
        _ => { None }
    }
}

/// Evaluate if a heartbeat timeout has occurred, given the `entity_type`, `state` of the encoder, and federation agreement settings.
/// Returns `true` when a timeout has occurred, `false` otherwise.
fn evaluate_timeout_for_entity_type(
    entity_type: &DisEntityType,
    heartbeat: &Instant,
    options: &CodecOptions,
) -> bool {
    let elapsed = heartbeat.elapsed().as_secs_f32();
    #[allow(clippy::match_same_arms)]
    let hbt_timeout = match (entity_type.kind, entity_type.domain) {
        (EntityKind::CulturalFeature, _) => {
            options
                .federation_parameters
                .HBT_ESPDU_KIND_CULTURAL_FEATURE
        }
        (EntityKind::Environmental, _) => {
            options.federation_parameters.HBT_ESPDU_KIND_ENVIRONMENTAL
        }
        (EntityKind::Expendable, _) => options.federation_parameters.HBT_ESPDU_KIND_EXPENDABLE,
        (EntityKind::LifeForm, _) => options.federation_parameters.HBT_ESPDU_KIND_LIFE_FORM,
        (EntityKind::Munition, _) => options.federation_parameters.HBT_ESPDU_KIND_MUNITION,
        (EntityKind::Radio, _) => options.federation_parameters.HBT_ESPDU_KIND_RADIO,
        (EntityKind::SensorEmitter, _) => options.federation_parameters.HBT_ESPDU_KIND_SENSOR,
        (EntityKind::Supply, _) => options.federation_parameters.HBT_ESPDU_KIND_SUPPLY,
        (EntityKind::Platform, PlatformDomain::Air) => {
            options.federation_parameters.HBT_ESPDU_PLATFORM_AIR
        }
        (EntityKind::Platform, PlatformDomain::Land) => {
            options.federation_parameters.HBT_ESPDU_PLATFORM_LAND
        }
        (EntityKind::Platform, PlatformDomain::Space) => {
            options.federation_parameters.HBT_ESPDU_PLATFORM_SPACE
        }
        (EntityKind::Platform, PlatformDomain::Subsurface) => {
            options.federation_parameters.HBT_ESPDU_PLATFORM_SUBSURFACE
        }
        (EntityKind::Platform, PlatformDomain::Surface) => {
            options.federation_parameters.HBT_ESPDU_PLATFORM_SURFACE
        }
        (EntityKind::Platform, _) => options.federation_parameters.HBT_ESPDU_PLATFORM_AIR, // Air domain is takes as the default for any other domain...
        (_, _) => options.federation_parameters.HBT_ESPDU_PLATFORM_AIR, // ...And also for anything other/unspecified.
    };

    elapsed > (hbt_timeout * options.hbt_cdis_full_update_mplier)
}

#[cfg(test)]
mod tests {
    use crate::codec::{CodecOptions, CodecStateResult, DecoderState, EncoderState};
    use crate::entity_state::codec::DecoderStateEntityState;
    use crate::entity_state::model::{CdisDRParametersOther, CdisEntityAppearance, EntityState};
    use crate::records::model::{
        AngularVelocity, CdisArticulatedPartVP, CdisEntityMarking, CdisVariableParameter, EntityId,
        EntityType, LinearAcceleration, LinearVelocity, Orientation, ParameterValueFloat,
        UnitsDekameters, WorldCoordinates,
    };
    use crate::types::model::{CdisFloat, SVINT12, SVINT14, SVINT16, SVINT24, UVINT16, UVINT8};
    use crate::{BodyProperties, CdisBody};
    use dis_rs::entity_state::builder::EntityStateBuilder;
    use dis_rs::entity_state::model::{
        DrEulerAngles, DrOtherParameters, DrParameters, EntityAppearance, EntityMarking,
        EntityState as DisEntityState,
    };
    use dis_rs::enumerations::EntityMarkingCharacterSet::ASCII;
    use dis_rs::enumerations::{
        AirPlatformAppearance, ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, Country,
        DeadReckoningAlgorithm, EntityKind, ForceId, PlatformDomain,
    };
    use dis_rs::model::{
        ArticulatedPart, EntityId as DisEntityId, EntityType as DisEntityType, Location,
        Orientation as DisOrientation, PduBody, VariableParameter,
    };
    use std::str::FromStr;
    use std::time::Instant;

    fn create_basic_dis_entity_state_body() -> EntityStateBuilder {
        DisEntityState::builder()
            .with_entity_id(DisEntityId::new(1, 1, 1))
            .with_force_id(ForceId::Friendly)
            .with_entity_type(
                DisEntityType::default()
                    .with_domain(PlatformDomain::Air)
                    .with_country(Country::Netherlands_NLD_)
                    .with_kind(EntityKind::Platform),
            )
            .with_marking(EntityMarking::new("Encode01", ASCII))
            .with_appearance(EntityAppearance::AirPlatform(
                AirPlatformAppearance::default(),
            ))
            .with_orientation(DisOrientation::new(10.0, 20.0, 30.0))
    }

    #[test]
    fn entity_state_body_encode_full_update() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_full_update();

        let dis_body = create_basic_dis_entity_state_body().build().into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::EntityState(es) = cdis_body {
            assert!(es.full_update_flag);
            assert_eq!(es.force_id.unwrap().value, u8::from(ForceId::Friendly));
            assert_eq!(
                es.entity_type.unwrap().domain,
                u8::from(PlatformDomain::Air)
            );
            assert_eq!(
                es.entity_type.unwrap().country,
                u16::from(Country::Netherlands_NLD_)
            );
            assert!(es.alternate_entity_type.is_none());
            assert!(es.entity_orientation.is_some());
            assert_eq!(es.entity_marking.unwrap().marking, "Encode01".to_string());
            assert_eq!(es.dr_algorithm, DeadReckoningAlgorithm::Other);
            assert!(es.dr_params_other.is_none());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_encode_partial_update_first_occurrence() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let entity_id = DisEntityId::new(1, 1, 1);
        let dis_body = create_basic_dis_entity_state_body().build().into_pdu_body();

        assert!(state.entity_state.get(&entity_id).is_none());

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUpdateEntityState);
        assert!(state.entity_state.get(&entity_id).is_some());
        if let CdisBody::EntityState(es) = cdis_body {
            assert!(es.full_update_flag);
            assert_eq!(es.force_id.unwrap().value, u8::from(ForceId::Friendly));
            assert_eq!(
                es.entity_type.unwrap().domain,
                u8::from(PlatformDomain::Air)
            );
            assert_eq!(
                es.entity_type.unwrap().country,
                u16::from(Country::Netherlands_NLD_)
            );
            assert!(es.alternate_entity_type.is_none());
            assert!(es.entity_orientation.is_some());
            assert_eq!(es.entity_marking.unwrap().marking, "Encode01".to_string());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_encode_partial_update_second_occurrence() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_entity_state_body().build().into_pdu_body();

        let (_, _) = CdisBody::encode(&dis_body, &mut state, &options);
        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::EntityState(es) = cdis_body {
            assert_eq!(es.entity_id.entity.value, 1);
            assert_eq!(es.entity_id.application.value, 1);
            assert_eq!(es.entity_id.site.value, 1);
            assert!(!es.full_update_flag);
            assert!(es.force_id.is_none());
            assert!(es.entity_type.is_none());
            assert!(es.alternate_entity_type.is_none());
            assert!(es.entity_orientation.is_none());
            assert!(es.entity_marking.is_none());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_encode_dr_non_static() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_full_update();

        let dis_body = create_basic_dis_entity_state_body()
            .with_dead_reckoning_parameters(
                DrParameters::default()
                    .with_algorithm(DeadReckoningAlgorithm::DRM_FVW_HighSpeedOrManeuveringEntity)
                    .with_parameters(DrOtherParameters::LocalEulerAngles(
                        DrEulerAngles::default()
                            .with_local_pitch(1.0)
                            .with_local_roll(1.0)
                            .with_local_yaw(1.0),
                    )),
            )
            .build()
            .into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::EntityState(es) = cdis_body {
            assert!(es.full_update_flag);
            assert_eq!(es.force_id.unwrap().value, u8::from(ForceId::Friendly));
            assert_eq!(
                es.entity_type.unwrap().domain,
                u8::from(PlatformDomain::Air)
            );
            assert_eq!(
                es.entity_type.unwrap().country,
                u16::from(Country::Netherlands_NLD_)
            );
            assert!(es.alternate_entity_type.is_none());
            assert!(es.entity_orientation.is_some());
            assert_eq!(es.entity_marking.unwrap().marking, "Encode01".to_string());
            assert_eq!(
                es.dr_algorithm,
                DeadReckoningAlgorithm::DRM_FVW_HighSpeedOrManeuveringEntity
            );
            assert!(es.dr_params_other.is_some());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_encode_use_guise() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_partial_update();

        let dis_body = create_basic_dis_entity_state_body()
            .with_alternative_entity_type(
                DisEntityType::default()
                    .with_domain(PlatformDomain::Land)
                    .with_country(Country::UnitedStatesOfAmerica_USA_)
                    .with_kind(EntityKind::LifeForm),
            )
            .with_appearance(EntityAppearance::AirPlatform(
                AirPlatformAppearance::default(),
            ))
            .build()
            .into_pdu_body();

        let (cdis_body, _state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        if let CdisBody::EntityState(es) = cdis_body {
            assert_eq!(
                es.alternate_entity_type.unwrap().domain,
                u8::from(PlatformDomain::Land)
            );
            assert_eq!(
                es.alternate_entity_type.unwrap().country,
                u16::from(Country::UnitedStatesOfAmerica_USA_)
            );
            assert_eq!(
                es.alternate_entity_type.unwrap().kind,
                u8::from(EntityKind::LifeForm)
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_encode_with_variable_parameters() {
        let mut state = EncoderState::new();
        let options = CodecOptions::new_full_update();

        let dis_body = create_basic_dis_entity_state_body()
            .with_variable_parameter(VariableParameter::Articulated(
                ArticulatedPart::default()
                    .with_attachment_id(1)
                    .with_type_class(ArticulatedPartsTypeClass::PrimaryTurretNumber1)
                    .with_type_metric(ArticulatedPartsTypeMetric::Azimuth)
                    .with_parameter_value(45.0),
            ))
            .build()
            .into_pdu_body();

        let (cdis_body, state_result) = CdisBody::encode(&dis_body, &mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        if let CdisBody::EntityState(es) = cdis_body {
            assert!(!es.variable_parameters.is_empty());
            if let CdisVariableParameter::ArticulatedPart(ap) =
                es.variable_parameters.first().unwrap()
            {
                assert_eq!(ap.change_indicator, 0);
                assert_eq!(ap.attachment_id, 1);
                assert_eq!(
                    ap.type_class,
                    ArticulatedPartsTypeClass::PrimaryTurretNumber1
                );
                assert_eq!(ap.type_metric, ArticulatedPartsTypeMetric::Azimuth);
                assert_eq!(ap.parameter_value.to_float(), 45.0f32);
            } else {
                assert!(false);
            }
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_decode_full_update() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = EntityState {
            units: UnitsDekameters::Dekameter,
            full_update_flag: true,
            entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            force_id: Some(UVINT8::from(1)),
            entity_type: Some(EntityType::new(1,1, 1, UVINT8::from(1), UVINT8::from(1), UVINT8::from(1), UVINT8::from(1))),
            alternate_entity_type: Some(EntityType::new(2,2, 2, UVINT8::from(2), UVINT8::from(2), UVINT8::from(2), UVINT8::from(2))),
            entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(10), SVINT16::from(10), SVINT16::from(10))),
            entity_location: Some(WorldCoordinates::new(52.0, 5.0, SVINT24::from(1000))),
            entity_orientation: Some(Orientation::new(1, 1, 1)),
            entity_appearance: Some(CdisEntityAppearance(0x1F00)),
            dr_algorithm: DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation,
            dr_params_other: Some(CdisDRParametersOther::from(0)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::new(SVINT14::from(10), SVINT14::from(10), SVINT14::from(10))),
            dr_params_entity_angular_velocity: Some(AngularVelocity::new(SVINT12::from(1), SVINT12::from(2), SVINT12::from(3))),
            entity_marking: Some(CdisEntityMarking::new("CDIS01".to_string())),
            capabilities: None,
            variable_parameters: vec![],
        }.into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

        if let PduBody::EntityState(es) = dis_body {
            assert_eq!(es.entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(
                es.entity_type,
                DisEntityType::from_str("1:1:1:1:1:1:1").unwrap()
            );
            assert_eq!(
                es.alternative_entity_type,
                DisEntityType::from_str("2:2:2:2:2:2:2").unwrap()
            );
            assert_eq!(es.force_id, ForceId::from(1));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_decode_with_variable_parameters() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = EntityState {
            units: UnitsDekameters::Dekameter,
            full_update_flag: true,
            entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            force_id: Some(UVINT8::from(1)),
            entity_type: Some(EntityType::new(1,1, 1, UVINT8::from(1), UVINT8::from(1), UVINT8::from(1), UVINT8::from(1))),
            alternate_entity_type: Some(EntityType::new(2,2, 2, UVINT8::from(2), UVINT8::from(2), UVINT8::from(2), UVINT8::from(2))),
            entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(10), SVINT16::from(10), SVINT16::from(10))),
            entity_location: Some(WorldCoordinates::new(52.0, 5.0, SVINT24::from(1000))),
            entity_orientation: Some(Orientation::new(1, 1, 1)),
            entity_appearance: Some(CdisEntityAppearance(0x1F00)),
            dr_algorithm: DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation,
            dr_params_other: Some(CdisDRParametersOther::from(0)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::new(SVINT14::from(10), SVINT14::from(10), SVINT14::from(10))),
            dr_params_entity_angular_velocity: Some(AngularVelocity::new(SVINT12::from(1), SVINT12::from(2), SVINT12::from(3))),
            entity_marking: Some(CdisEntityMarking::new("CDIS01".to_string())),
            capabilities: None,
            variable_parameters: vec![CdisVariableParameter::ArticulatedPart(CdisArticulatedPartVP {
                change_indicator: 0,
                attachment_id: 1,
                type_class: ArticulatedPartsTypeClass::PrimaryTurretNumber1,
                type_metric: ArticulatedPartsTypeMetric::Azimuth,
                parameter_value: ParameterValueFloat::from_float(45.0),
            })],
        }.into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

        if let PduBody::EntityState(es) = dis_body {
            assert!(es.variable_parameters.first().is_some());
            assert_eq!(
                *es.variable_parameters.first().unwrap(),
                VariableParameter::Articulated(
                    ArticulatedPart::default()
                        .with_attachment_id(1)
                        .with_type_class(ArticulatedPartsTypeClass::PrimaryTurretNumber1)
                        .with_type_metric(ArticulatedPartsTypeMetric::Azimuth)
                        .with_parameter_value(45.0)
                )
            );
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_decode_partial_update_full_flag() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_partial_update();

        let cdis_body = EntityState {
            units: UnitsDekameters::Dekameter,
            full_update_flag: true,
            entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            force_id: Some(UVINT8::from(1)),
            entity_type: Some(EntityType::new(1,1, 1, UVINT8::from(1), UVINT8::from(1), UVINT8::from(1), UVINT8::from(1))),
            alternate_entity_type: Some(EntityType::new(2,2, 2, UVINT8::from(2), UVINT8::from(2), UVINT8::from(2), UVINT8::from(2))),
            entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(10), SVINT16::from(10), SVINT16::from(10))),
            entity_location: Some(WorldCoordinates::new(52.0, 5.0, SVINT24::from(1000))),
            entity_orientation: Some(Orientation::new(1, 1, 1)),
            entity_appearance: Some(CdisEntityAppearance(0x1F00)),
            dr_algorithm: DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation,
            dr_params_other: Some(CdisDRParametersOther::from(0)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::new(SVINT14::from(10), SVINT14::from(10), SVINT14::from(10))),
            dr_params_entity_angular_velocity: Some(AngularVelocity::new(SVINT12::from(1), SVINT12::from(2), SVINT12::from(3))),
            entity_marking: Some(CdisEntityMarking::new("CDIS01".to_string())),
            capabilities: None,
            variable_parameters: vec![],
        }.into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUpdateEntityState);

        if let PduBody::EntityState(es) = dis_body {
            assert_eq!(es.entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(
                es.entity_type,
                DisEntityType::from_str("1:1:1:1:1:1:1").unwrap()
            );
            assert_eq!(
                es.alternative_entity_type,
                DisEntityType::from_str("2:2:2:2:2:2:2").unwrap()
            );
            assert_eq!(es.force_id, ForceId::from(1));
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_decode_partial_update_no_state() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_full_update();

        let cdis_body = EntityState {
            units: UnitsDekameters::Dekameter,
            full_update_flag: false,
            entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            force_id: None,
            entity_type: None,
            alternate_entity_type: None,
            entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(10), SVINT16::from(10), SVINT16::from(10))),
            entity_location: None,
            entity_orientation: None,
            entity_appearance: Some(CdisEntityAppearance(0x1F00)),
            dr_algorithm: DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation,
            dr_params_other: Some(CdisDRParametersOther::from(0)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::new(SVINT14::from(10), SVINT14::from(10), SVINT14::from(10))),
            dr_params_entity_angular_velocity: Some(AngularVelocity::new(SVINT12::from(1), SVINT12::from(2), SVINT12::from(3))),
            entity_marking: None,
            capabilities: None,
            variable_parameters: vec![],
        }.into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

        if let PduBody::EntityState(es) = dis_body {
            assert_eq!(es.entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(es.entity_type, DisEntityType::default());
            assert_eq!(es.alternative_entity_type, DisEntityType::default());
            assert_eq!(es.force_id, ForceId::default());
        } else {
            assert!(false);
        }
    }

    #[test]
    fn entity_state_body_decode_partial_update_with_state() {
        let mut state = DecoderState::new();
        let options = CodecOptions::new_partial_update();

        let decoder_state = DecoderStateEntityState {
            heartbeat: Instant::now(),
            force_id: ForceId::Friendly8,
            entity_type: DisEntityType::from_str("1:2:3:4:5:6:7").unwrap(),
            entity_location: Location::new(20000.0, 20000.0, 20000.0),
            entity_orientation: DisOrientation::new(10.0, 10.0, 10.0),
            entity_appearance: EntityAppearance::default(),
            entity_marking: EntityMarking::new("STATE15", ASCII),
        };
        state
            .entity_state
            .insert(DisEntityId::new(10, 10, 10), decoder_state);

        let cdis_body = EntityState {
            units: UnitsDekameters::Dekameter,
            full_update_flag: false,
            entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            force_id: None,
            entity_type: None,
            alternate_entity_type: None,
            entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(10), SVINT16::from(10), SVINT16::from(10))),
            entity_location: None,
            entity_orientation: None,
            entity_appearance: None,
            dr_algorithm: DeadReckoningAlgorithm::DRM_RVW_HighSpeedOrManeuveringEntityWithExtrapolationOfOrientation,
            dr_params_other: Some(CdisDRParametersOther::from(0)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::new(SVINT14::from(10), SVINT14::from(10), SVINT14::from(10))),
            dr_params_entity_angular_velocity: Some(AngularVelocity::new(SVINT12::from(1), SVINT12::from(2), SVINT12::from(3))),
            entity_marking: None,
            capabilities: None,
            variable_parameters: vec![],
        }.into_cdis_body();

        let (dis_body, state_result) = cdis_body.decode(&mut state, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

        if let PduBody::EntityState(es) = dis_body {
            assert_eq!(es.entity_id, DisEntityId::new(10, 10, 10));
            assert_eq!(
                es.entity_type,
                DisEntityType::from_str("1:2:3:4:5:6:7").unwrap()
            );
            assert_eq!(es.force_id, ForceId::Friendly8);
            assert_eq!(es.entity_marking.marking_string, "STATE15".to_string());
        } else {
            assert!(false);
        }
    }
}
