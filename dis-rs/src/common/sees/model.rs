use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::model::{EntityId, PduBody};
use crate::sees::builder::SeesBuilder;

const BASE_SEES_BODY_LENGTH: u16 = 16;
const BASE_SYSTEM_DATA_LENGTH: u16 = 8;

/// 5.7.7 Supplemental Emission/Entity State (SEES) PDU
///
/// 7.6.6 Supplemental Emission/Entity State (SEES) PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SEES {
    pub originating_entity_id: EntityId,
    pub infrared_signature_representation_index: u16,
    pub acoustic_signature_representation_index: u16,
    pub radar_cross_section_representation_index: u16,
    pub propulsion_systems: Vec<PropulsionSystemData>,
    pub vectoring_nozzle_systems: Vec<VectoringNozzleSystemData>,
}

impl SEES {
    pub fn builder() -> SeesBuilder {
        SeesBuilder::new()
    }

    pub fn into_builder(self) -> SeesBuilder {
        SeesBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::SupplementalEmissionEntityState(self)
    }
}

impl BodyInfo for SEES {
    fn body_length(&self) -> u16 {
        BASE_SEES_BODY_LENGTH +
            (BASE_SYSTEM_DATA_LENGTH * self.propulsion_systems.len() as u16) +
            (BASE_SYSTEM_DATA_LENGTH * self.vectoring_nozzle_systems.len() as u16)
    }

    fn body_type(&self) -> PduType {
        PduType::SupplementalEmissionEntityState
    }
}

impl Interaction for SEES {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

/// 6.2.68 Propulsion System Data record
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PropulsionSystemData {
    pub power_setting: f32,
    pub engine_rpm: f32,
}

impl PropulsionSystemData {
    pub fn with_power_setting(mut self, power_setting: f32) -> Self {
        self.power_setting = power_setting;
        self
    }

    pub fn with_engine_rpm(mut self, engine_rpm: f32) -> Self {
        self.engine_rpm = engine_rpm;
        self
    }
}

/// 6.2.97 Vectoring Nozzle System Data record
#[derive(Clone, Debug, Default, PartialEq)]
pub struct VectoringNozzleSystemData {
    pub horizontal_deflection_angle: f32,
    pub vertical_deflection_angle: f32,
}

impl VectoringNozzleSystemData {
    pub fn with_horizontal_deflection_angle(mut self, horizontal_deflection_angle: f32) -> Self {
        self.horizontal_deflection_angle = horizontal_deflection_angle;
        self
    }

    pub fn with_vertical_deflection_angle(mut self, vertical_deflection_angle: f32) -> Self {
        self.vertical_deflection_angle = vertical_deflection_angle;
        self
    }
}