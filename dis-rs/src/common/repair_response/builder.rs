use crate::common::model::EntityId;
use crate::enumerations::RepairResponseRepairResult;
use crate::repair_response::model::RepairResponse;

pub struct RepairResponseBuilder(RepairResponse);

impl RepairResponseBuilder {
    #[must_use]
    pub fn new() -> Self {
        RepairResponseBuilder(RepairResponse::default())
    }

    #[must_use]
    pub fn new_from_body(body: RepairResponse) -> Self {
        RepairResponseBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> RepairResponse {
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
    pub fn with_repair_result(mut self, repair_result: RepairResponseRepairResult) -> Self {
        self.0.repair_result = repair_result;
        self
    }
}
