use dis_rs::entity_state::model::{DrParameters, EntityAppearance, EntityMarking};
use dis_rs::enumerations::{DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId, PlatformDomain};
use dis_rs::model::{EntityType as DisEntityType, Location as DisLocation, Orientation as DisOrientation};
use std::time::Instant;
use crate::codec::{Codec, CodecOptimizeMode, CodecOptions, CodecStateResult, CodecUpdateMode};
use crate::entity_state::model::{CdisDRParametersOther, CdisEntityCapabilities, EntityState};
use crate::records::codec::{decode_world_coordinates, encode_world_coordinates};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisVariableParameter, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, Units};
use crate::types::model::{UVINT32, UVINT8};

type Counterpart = dis_rs::entity_state::model::EntityState;

#[derive(Debug)]
pub struct EncoderStateEntityState {
    pub last_send: Instant,
}

impl EncoderStateEntityState {
    pub fn new() -> Self {
        Self {
            last_send: Instant::now()
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateEntityState {
    pub last_received: Instant,
    pub force_id: ForceId,
    pub entity_type: DisEntityType,
    pub alt_entity_type: DisEntityType,
    pub entity_location: DisLocation,
    pub entity_orientation: DisOrientation,
    pub entity_appearance: EntityAppearance,
}

impl DecoderStateEntityState {
    pub fn new(pdu: &dis_rs::entity_state::model::EntityState) -> Self {
        Self {
            last_received: Instant::now(),
            force_id: pdu.force_id,
            entity_type: pdu.entity_type,
            alt_entity_type: pdu.alternative_entity_type,
            entity_location: pdu.entity_location,
            entity_orientation: pdu.entity_orientation,
            entity_appearance: pdu.entity_appearance,
        }
    }
}

impl EntityState {
    pub fn encode(item: &Counterpart, state: Option<&EncoderStateEntityState>, options: &CodecOptions) -> (Self, CodecStateResult) {
        let entity_linear_velocity = encode_ent_linear_velocity(item);
        let dr_params_other = encode_dr_params_other(item);
        let dr_params_entity_linear_acceleration = encode_dr_linear_acceleration(item);
        let dr_params_entity_angular_velocity = encode_dr_angular_velocity(item);
        let capabilities = encode_entity_capabilities(item, options);

        let (
            units,
            full_update_flag,
            force_id,
            entity_type,
            alternate_entity_type,
            entity_location,
            entity_orientation,
            entity_appearance,
            entity_marking,
            state_result
        ) = if options.update_mode == CodecUpdateMode::PartialUpdate
            && state.is_some()
            && !evaluate_timeout_for_entity_type(&item.entity_type, state.unwrap(), options) {
            // Do not update stateful fields when a full update is not required
            ( Units::Dekameter, false, None, None, None, None, None, None, None, CodecStateResult::StateUnaffected )
        } else {
            let (entity_location, units) = encode_world_coordinates(&item.entity_location);
            let alternate_entity_type = if options.use_guise {
                Some(EntityType::encode(&item.alternative_entity_type))
            } else { None };
            // full update mode, or partial with a (state) timeout on the entity
            (
                units,
                true,
                Some(UVINT8::from(u8::from(item.force_id))),
                Some(EntityType::encode(&item.entity_type)),
                alternate_entity_type,
                Some(entity_location),
                Some(Orientation::encode(&item.entity_orientation)),
                Some((&item.entity_appearance).into()),
                Some(CdisEntityMarking::new(item.entity_marking.marking_string.clone())),
                CodecStateResult::StateUpdateEntityState
            )
        };

        (Self {
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
            variable_parameters: item.variable_parameters.iter()
                .map(CdisVariableParameter::encode )
                .collect(),
        }, state_result)
    }

    pub fn decode(&self, state: &DecoderStateEntityState, options: &CodecOptions) -> (Counterpart, CodecStateResult) {
        let (
            force_id,
            entity_type,
            alternate_entity_type,
            entity_location,
            entity_orientation,
            entity_appearance,
            entity_marking,
            state_result) =
            match options.update_mode {
                CodecUpdateMode::FullUpdate => {
                    let entity_type = self.entity_type.unwrap_or_default().decode();
                    (
                        ForceId::from(self.force_id.unwrap_or_default().value),
                        entity_type,
                        self.alternate_entity_type.unwrap_or_default(),
                        self.entity_location.unwrap_or_default(),
                        self.entity_location
                            .map(| world_coordinates | decode_world_coordinates(&world_coordinates, self.units) )
                            .unwrap_or_default(),
                        self.entity_appearance.as_ref()
                            .map(|cdis| EntityAppearance::from_bytes(cdis.0, &entity_type))
                            .unwrap_or_default(),
                        EntityMarking::new(self.entity_marking.clone().unwrap_or_default().marking, EntityMarkingCharacterSet::ASCII),
                        CodecStateResult::StateUpdateEntityState
                    )
                }
                CodecUpdateMode::PartialUpdate => {
                    if self.full_update_flag {

                    } else {

                    }
                }
            };

        (Counterpart::builder()
            .with_entity_id(self.entity_id.decode())
            .with_force_id(ForceId::from(self.force_id.unwrap_or_default().value))
            .with_entity_type(entity_type)
            .with_alternative_entity_type(self.alternate_entity_type.unwrap_or_default().decode())
            .with_velocity(self.entity_linear_velocity.unwrap_or_default().decode())
            .with_location(self.entity_location
                .map(| world_coordinates | decode_world_coordinates(&world_coordinates, self.units) )
                .unwrap_or_default())
            .with_orientation(self.entity_orientation.unwrap_or_default().decode())
            .with_appearance(self.entity_appearance.as_ref()
                .map(|cdis| EntityAppearance::from_bytes(cdis.0, &entity_type))
                .unwrap_or_default())
            .with_dead_reckoning_parameters(DrParameters::default()
                .with_algorithm(self.dr_algorithm)
                .with_parameters(self.dr_params_other.clone().unwrap_or_default().decode(self.dr_algorithm))
                .with_linear_acceleration(self.dr_params_entity_linear_acceleration.unwrap_or_default().decode())
                .with_angular_velocity(self.dr_params_entity_angular_velocity.unwrap_or_default().decode()))
            .with_marking(EntityMarking::new(self.entity_marking.clone().unwrap_or_default().marking, EntityMarkingCharacterSet::ASCII))
            .with_capabilities(dis_rs::entity_capabilities_from_bytes(self.capabilities.clone().unwrap_or_default().0.value, &entity_type))
            .with_variable_parameters(self.variable_parameters.iter()
                .map(|vp| vp.decode() )
                .collect())
            .build(), state_result)
    }
}

/// Encodes the Entity Linear Velocity field when the Dead Reckoning Algorithm requires it (no 2 through 9).
fn encode_ent_linear_velocity(item: &Counterpart) -> Option<LinearVelocity> {
    match item.dead_reckoning_parameters.algorithm {
        DeadReckoningAlgorithm::Other |
        DeadReckoningAlgorithm::StaticNonmovingEntity => { None }
        _ => { Some(LinearVelocity::encode(&item.entity_linear_velocity)) }
    }
}

/// Encodes the Entity State Capabilities field, when optimizing for completeness and the field is non-zero (i.e., has capabilities).
fn encode_entity_capabilities(item: &Counterpart, options: &CodecOptions) -> Option<CdisEntityCapabilities> {
    match options.optimize_mode {
        CodecOptimizeMode::Bandwidth => { None }
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
fn encode_dr_params_other(item: &Counterpart) -> Option<CdisDRParametersOther> {
    let other_params = CdisDRParametersOther::from(&item.dead_reckoning_parameters.other_parameters);
    if item.dead_reckoning_parameters.algorithm != DeadReckoningAlgorithm::Other
        && other_params.0 != 0 {
        Some(other_params)
    } else {
        None
    }
}

/// Encodes the Dead Reckoning Linear Acceleration field when the Dead Reckoning Algorithm requires it (no 4, 5, 8 and 9).
fn encode_dr_linear_acceleration(item: &Counterpart) -> Option<LinearAcceleration> {
    match item.dead_reckoning_parameters.algorithm {
        DeadReckoningAlgorithm::DRM_RVW_HighSpeedorManeuveringEntitywithExtrapolationofOrientation |
        DeadReckoningAlgorithm::DRM_FVW_HighSpeedorManeuveringEntity |
        DeadReckoningAlgorithm::DRM_RVB_SimilartoRVWexceptinBodyCoordinates |
        DeadReckoningAlgorithm::DRM_FVB_SimilartoFVWexceptinBodyCoordinates => {
            Some(LinearAcceleration::encode(&item.dead_reckoning_parameters.linear_acceleration))
        }
        _ => { None }
    }
}

/// Encodes the Dead Reckoning Angular Velocity field when the Dead Reckoning Algorithm requires it (no 3, 4, 7 and 8).
fn encode_dr_angular_velocity(item: &Counterpart) -> Option<AngularVelocity> {
    match item.dead_reckoning_parameters.algorithm {
        DeadReckoningAlgorithm::DRM_RPW_ConstantVelocityLowAccelerationLinearMotionEntitywithExtrapolationofOrientation |
        DeadReckoningAlgorithm::DRM_RVW_HighSpeedorManeuveringEntitywithExtrapolationofOrientation |
        DeadReckoningAlgorithm::DRM_RPB_SimilartoRPWexceptinBodyCoordinates |
        DeadReckoningAlgorithm::DRM_RVB_SimilartoRVWexceptinBodyCoordinates => {
            Some(AngularVelocity::encode(&item.dead_reckoning_parameters.angular_velocity))
        }
        _ => { None }
    }
}

/// Evaluate if a heartbeat timeout has occurred, given the `entity_type`, `state` of the encoder, and federation agreement settings.
/// Returns `true` when a timeout has occurred, `false` otherwise.
fn evaluate_timeout_for_entity_type(entity_type: &DisEntityType, state: &EncoderStateEntityState, config: &CodecOptions) -> bool {
    let elapsed = state.last_send.elapsed().as_secs_f32();
    let hbt_timeout = match (entity_type.kind, entity_type.domain) {
        (EntityKind::Culturalfeature, _) => { config.federation_parameters.HBT_ESPDU_KIND_CULTURAL_FEATURE }
        (EntityKind::Environmental, _) => { config.federation_parameters.HBT_ESPDU_KIND_ENVIRONMENTAL }
        (EntityKind::Expendable, _) => { config.federation_parameters.HBT_ESPDU_KIND_EXPENDABLE }
        (EntityKind::Lifeform, _) => { config.federation_parameters.HBT_ESPDU_KIND_LIFE_FORM }
        (EntityKind::Munition, _) => { config.federation_parameters.HBT_ESPDU_KIND_MUNITION }
        (EntityKind::Radio, _) => { config.federation_parameters.HBT_ESPDU_KIND_RADIO }
        (EntityKind::SensorEmitter, _) => { config.federation_parameters.HBT_ESPDU_KIND_SENSOR }
        (EntityKind::Supply, _) => { config.federation_parameters.HBT_ESPDU_KIND_SUPPLY }
        (EntityKind::Platform, PlatformDomain::Air) => { config.federation_parameters.HBT_ESPDU_PLATFORM_AIR }
        (EntityKind::Platform, PlatformDomain::Land) => { config.federation_parameters.HBT_ESPDU_PLATFORM_LAND }
        (EntityKind::Platform, PlatformDomain::Space) => { config.federation_parameters.HBT_ESPDU_PLATFORM_SPACE }
        (EntityKind::Platform, PlatformDomain::Subsurface) => { config.federation_parameters.HBT_ESPDU_PLATFORM_SUBSURFACE }
        (EntityKind::Platform, PlatformDomain::Surface) => { config.federation_parameters.HBT_ESPDU_PLATFORM_SURFACE }
        (EntityKind::Platform, _) => { config.federation_parameters.HBT_ESPDU_PLATFORM_AIR } // Air domain is takes as the default for any other domain...
        (_, _) => { config.federation_parameters.HBT_ESPDU_PLATFORM_AIR } // ...And also for anything other/unspecified.
    };

    elapsed > (hbt_timeout * config.hbt_cdis_full_update_mplier)
}
