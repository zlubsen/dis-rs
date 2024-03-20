use crate::enumerations::{UAPassiveParameterIndex, UAStateChangeUpdateIndicator};
use crate::model::{EntityId, EventId};
use crate::underwater_acoustic::model::{APA, PropulsionPlantConfiguration, Shaft, UAEmitterSystem, UnderwaterAcoustic};

pub struct UnderwaterAcousticBuilder(UnderwaterAcoustic);

impl UnderwaterAcousticBuilder {
    pub fn new() -> Self {
        UnderwaterAcousticBuilder(UnderwaterAcoustic::default())
    }

    pub fn new_from_body(body: UnderwaterAcoustic) -> Self {
        UnderwaterAcousticBuilder(body)
    }

    pub fn build(self) -> UnderwaterAcoustic {
        self.0
    }

    pub fn with_emitting_entity_id(mut self, emitting_entity_id: EntityId) -> Self {
        self.0.emitting_entity_id = emitting_entity_id;
        self
    }

    pub fn with_event_id(mut self, event_id: EventId) -> Self {
        self.0.event_id = event_id;
        self
    }

    pub fn with_state_change_update_indicator(mut self, state_change_update_indicator: UAStateChangeUpdateIndicator) -> Self {
        self.0.state_change_update_indicator = state_change_update_indicator;
        self
    }

    pub fn with_passive_parameter_index(mut self, passive_parameter_index: UAPassiveParameterIndex) -> Self {
        self.0.passive_parameter_index = passive_parameter_index;
        self
    }

    pub fn with_propulsion_plant_configuration(mut self, propulsion_plan_configuration: PropulsionPlantConfiguration) -> Self {
        self.0.propulsion_plant_configuration = propulsion_plan_configuration;
        self
    }

    pub fn with_shaft(mut self, shaft: Shaft) -> Self {
        self.0.shafts.push(shaft);
        self
    }

    pub fn with_shafts(mut self, shafts: Vec<Shaft>) -> Self {
        self.0.shafts = shafts;
        self
    }

    pub fn with_apa(mut self, apa: APA) -> Self {
        self.0.apas.push(apa);
        self
    }

    pub fn with_apas(mut self, apas: Vec<APA>) -> Self {
        self.0.apas = apas;
        self
    }

    pub fn with_emitter_system(mut self, system: UAEmitterSystem) -> Self {
        self.0.emitter_systems.push(system);
        self
    }

    pub fn with_emitter_systems(mut self, systems: Vec<UAEmitterSystem>) -> Self {
        self.0.emitter_systems = systems;
        self
    }
}