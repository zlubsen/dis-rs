use crate::common::designator::model::Designator;
use crate::common::model::{EntityId, Location, VectorF32};
use crate::enumerations::{DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName};

pub struct DesignatorBuilder(Designator);

impl Default for DesignatorBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DesignatorBuilder {
    #[must_use]
    pub fn new() -> Self {
        DesignatorBuilder(Designator::default())
    }

    #[must_use]
    pub fn new_from_body(body: Designator) -> Self {
        DesignatorBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> Designator {
        self.0
    }

    #[must_use]
    pub fn with_designating_entity_id(mut self, designating_entity_id: EntityId) -> Self {
        self.0.designating_entity_id = designating_entity_id;
        self
    }

    #[must_use]
    pub fn with_system_name(mut self, system_name: DesignatorSystemName) -> Self {
        self.0.system_name = system_name;
        self
    }

    #[must_use]
    pub fn with_designated_entity_id(mut self, designated_entity_id: EntityId) -> Self {
        self.0.designated_entity_id = designated_entity_id;
        self
    }

    #[must_use]
    pub fn with_code(mut self, code: DesignatorCode) -> Self {
        self.0.code = code;
        self
    }

    #[must_use]
    pub fn with_power(mut self, power: f32) -> Self {
        self.0.power = power;
        self
    }

    #[must_use]
    pub fn with_wavelength(mut self, wavelength: f32) -> Self {
        self.0.wavelength = wavelength;
        self
    }

    #[must_use]
    pub fn with_spot_wrt_designated_entity(
        mut self,
        spot_wrt_designated_entity: VectorF32,
    ) -> Self {
        self.0.spot_wrt_designated_entity = spot_wrt_designated_entity;
        self
    }

    #[must_use]
    pub fn with_spot_location(mut self, spot_location: Location) -> Self {
        self.0.spot_location = spot_location;
        self
    }

    #[must_use]
    pub fn with_dead_reckoning_algorithm(
        mut self,
        dead_reckoning_algorithm: DeadReckoningAlgorithm,
    ) -> Self {
        self.0.dead_reckoning_algorithm = dead_reckoning_algorithm;
        self
    }

    #[must_use]
    pub fn with_linear_acceleration(mut self, linear_acceleration: VectorF32) -> Self {
        self.0.linear_acceleration = linear_acceleration;
        self
    }
}
