use crate::{EntityId, Location, PduBody, PduType, VectorF32};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{DesignatorSystemName, DesignatorCode, DeadReckoningAlgorithm};

pub const DESIGNATOR_BODY_LENGTH : u16 = 76;

#[derive(Debug, PartialEq)]
pub struct Designator {
    pub designating_entity_id: EntityId,
    pub system_name: DesignatorSystemName,
    pub designated_entity_id: EntityId,
    pub code: DesignatorCode,
    pub power: f32,
    pub wavelength: f32,
    pub spot_wrt_designated_entity: VectorF32,
    pub spot_location: Location,
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub linear_acceleration: VectorF32,
}

impl Default for Designator {
    fn default() -> Self {
        Self::new()
    }
}

impl Designator {
    pub fn new() -> Self {
        Self {
            designating_entity_id: Default::default(),
            system_name: Default::default(),
            designated_entity_id: Default::default(),
            code: Default::default(),
            power: 0.0,
            wavelength: 0.0,
            spot_wrt_designated_entity: Default::default(),
            spot_location: Default::default(),
            dead_reckoning_algorithm: Default::default(),
            linear_acceleration: Default::default(),
        }
    }

    pub fn with_designating_entity_id(mut self, designating_entity_id: EntityId) -> Self {
        self.designating_entity_id = designating_entity_id;
        self
    }

    pub fn with_system_name(mut self, system_name: DesignatorSystemName) -> Self {
        self.system_name = system_name;
        self
    }

    pub fn with_designated_entity_id(mut self, designated_entity_id: EntityId) -> Self {
        self.designated_entity_id = designated_entity_id;
        self
    }

    pub fn with_code(mut self, code: DesignatorCode) -> Self {
        self.code = code;
        self
    }

    pub fn with_power(mut self, power: f32) -> Self {
        self.power = power;
        self
    }

    pub fn with_wavelength(mut self, wavelength: f32) -> Self {
        self.wavelength = wavelength;
        self
    }

    pub fn with_spot_wrt_designated_entity(mut self, spot_wrt_designated_entity: VectorF32) -> Self {
        self.spot_wrt_designated_entity = spot_wrt_designated_entity;
        self
    }

    pub fn with_spot_location(mut self, spot_location: Location) -> Self {
        self.spot_location = spot_location;
        self
    }

    pub fn with_dead_reckoning_algorithm(mut self, dead_reckoning_algorithm: DeadReckoningAlgorithm) -> Self {
        self.dead_reckoning_algorithm = dead_reckoning_algorithm;
        self
    }

    pub fn with_linear_acceleration(mut self, linear_acceleration: VectorF32) -> Self {
        self.linear_acceleration = linear_acceleration;
        self
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Designator(self)
    }
}

impl BodyInfo for Designator {
    fn body_length(&self) -> u16 {
        DESIGNATOR_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Designator
    }
}

impl Interaction for Designator {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.designating_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.designated_entity_id)
    }
}