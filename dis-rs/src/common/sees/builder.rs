use crate::model::EntityId;
use crate::sees::model::{PropulsionSystemData, VectoringNozzleSystemData, SEES};

pub struct SeesBuilder(SEES);

impl Default for SeesBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SeesBuilder {
    #[must_use]
    pub fn new() -> Self {
        SeesBuilder(SEES::default())
    }

    #[must_use]
    pub fn new_from_body(body: SEES) -> Self {
        SeesBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> SEES {
        self.0
    }

    #[must_use]
    pub fn with_originating_entity_id(mut self, originating_entity_id: EntityId) -> Self {
        self.0.originating_entity_id = originating_entity_id;
        self
    }

    #[must_use]
    pub fn with_infrared_signature_representation_index(
        mut self,
        infrared_signature_representation_index: u16,
    ) -> Self {
        self.0.infrared_signature_representation_index = infrared_signature_representation_index;
        self
    }

    #[must_use]
    pub fn with_acoustic_signature_representation_index(
        mut self,
        acoustic_signature_representation_index: u16,
    ) -> Self {
        self.0.acoustic_signature_representation_index = acoustic_signature_representation_index;
        self
    }

    #[must_use]
    pub fn with_radar_cross_section_representation_index(
        mut self,
        radar_cross_section_representation_index: u16,
    ) -> Self {
        self.0.radar_cross_section_representation_index = radar_cross_section_representation_index;
        self
    }

    #[must_use]
    pub fn with_propulsion_system(mut self, propulsion_system: PropulsionSystemData) -> Self {
        self.0.propulsion_systems.push(propulsion_system);
        self
    }

    #[must_use]
    pub fn with_propulsion_systems(
        mut self,
        propulsion_systems: Vec<PropulsionSystemData>,
    ) -> Self {
        self.0.propulsion_systems = propulsion_systems;
        self
    }

    #[must_use]
    pub fn with_vectoring_nozzle_system(
        mut self,
        vectoring_nozzle_system: VectoringNozzleSystemData,
    ) -> Self {
        self.0
            .vectoring_nozzle_systems
            .push(vectoring_nozzle_system);
        self
    }

    #[must_use]
    pub fn with_vectoring_nozzle_systems(
        mut self,
        vectoring_nozzle_systems: Vec<VectoringNozzleSystemData>,
    ) -> Self {
        self.0.vectoring_nozzle_systems = vectoring_nozzle_systems;
        self
    }
}
