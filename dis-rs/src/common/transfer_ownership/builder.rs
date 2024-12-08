use crate::enumerations::{RequiredReliabilityService, TransferControlTransferType};
use crate::model::{EntityId, RecordSpecification};
use crate::transfer_ownership::model::TransferOwnership;

pub struct TransferOwnershipBuilder(TransferOwnership);

impl Default for TransferOwnershipBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl TransferOwnershipBuilder {
    #[must_use]
    pub fn new() -> Self {
        TransferOwnershipBuilder(TransferOwnership::default())
    }

    #[must_use]
    pub fn new_from_body(body: TransferOwnership) -> Self {
        TransferOwnershipBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> TransferOwnership {
        self.0
    }

    #[must_use]
    pub fn with_originating_id(mut self, originating_id: EntityId) -> Self {
        self.0.originating_id = originating_id;
        self
    }

    #[must_use]
    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.0.receiving_id = receiving_id;
        self
    }

    #[must_use]
    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.0.request_id = request_id;
        self
    }

    #[must_use]
    pub fn with_required_reliability_service(
        mut self,
        required_reliability_service: RequiredReliabilityService,
    ) -> Self {
        self.0.required_reliability_service = required_reliability_service;
        self
    }

    #[must_use]
    pub fn with_transfer_type(mut self, transfer_type: TransferControlTransferType) -> Self {
        self.0.transfer_type = transfer_type;
        self
    }

    #[must_use]
    pub fn with_transfer_entity_id(mut self, transfer_entity_id: EntityId) -> Self {
        self.0.transfer_entity_id = transfer_entity_id;
        self
    }

    #[must_use]
    pub fn with_record_specification(mut self, record_specification: RecordSpecification) -> Self {
        self.0.record_specification = record_specification;
        self
    }
}
