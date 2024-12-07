use crate::electromagnetic_emission::model::{ElectromagneticEmission, EmitterSystem};
use crate::enumerations::ElectromagneticEmissionStateUpdateIndicator;
use crate::model::{EntityId, EventId};

pub struct ElectromagneticEmissionBuilder(ElectromagneticEmission);

impl ElectromagneticEmissionBuilder {
    #[must_use]
    pub fn new() -> Self {
        ElectromagneticEmissionBuilder(ElectromagneticEmission::default())
    }

    #[must_use]
    pub fn new_from_body(body: ElectromagneticEmission) -> Self {
        ElectromagneticEmissionBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> ElectromagneticEmission {
        self.0
    }

    #[must_use]
    pub fn with_emitting_entity_id(mut self, entity_id: EntityId) -> Self {
        self.0.emitting_entity_id = entity_id;
        self
    }

    #[must_use]
    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.0.event_id = event_id;
        self
    }

    #[must_use]
    pub fn with_state_update_indicator(
        mut self,
        state_update_indicator: ElectromagneticEmissionStateUpdateIndicator,
    ) -> Self {
        self.0.state_update_indicator = state_update_indicator;
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_emitter_systems(mut self, systems: &mut Vec<EmitterSystem>) -> Self {
        self.0.emitter_systems.append(systems);
        self
    }

    #[must_use]
    pub fn with_emitter_system(mut self, system: EmitterSystem) -> Self {
        self.0.emitter_systems.push(system);
        self
    }
}
