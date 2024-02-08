use crate::common::model::EntityId;
use crate::enumerations::RepairCompleteRepair;
use crate::repair_complete::model::RepairComplete;

pub struct RepairCompleteBuilder(RepairComplete);

impl RepairCompleteBuilder {
    pub fn new() -> Self {
        RepairCompleteBuilder(RepairComplete::default())
    }

    pub fn new_from_body(body: RepairComplete) -> Self {
        RepairCompleteBuilder(body)
    }

    pub fn build(self) -> RepairComplete {
        self.0
    }

    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    pub fn with_repairing_id(mut self, repairing_id: EntityId) -> Self {
        self.0.repairing_id = repairing_id;
        self
    }

    pub fn with_repair(mut self, repair: RepairCompleteRepair) -> Self {
        self.0.repair = repair;
        self
    }
}