use crate::common::model::EntityId;
use crate::enumerations::RepairCompleteRepair;
use crate::repair_complete::model::RepairComplete;

pub struct RepairCompleteBuilder(RepairComplete);

impl RepairCompleteBuilder {
    #[must_use]
    pub fn new() -> Self {
        RepairCompleteBuilder(RepairComplete::default())
    }

    #[must_use]
    pub fn new_from_body(body: RepairComplete) -> Self {
        RepairCompleteBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> RepairComplete {
        self.0
    }

    #[must_use]
    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    #[must_use]
    pub fn with_repairing_id(mut self, repairing_id: EntityId) -> Self {
        self.0.repairing_id = repairing_id;
        self
    }

    #[must_use]
    pub fn with_repair(mut self, repair: RepairCompleteRepair) -> Self {
        self.0.repair = repair;
        self
    }
}
