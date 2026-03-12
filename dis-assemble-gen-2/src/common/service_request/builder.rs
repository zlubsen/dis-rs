use crate::common::model::EntityId;
use crate::common::model::SupplyQuantity;
use crate::common::service_request::model::ServiceRequest;
use crate::enumerations::ServiceRequestServiceTypeRequested;

pub struct ServiceRequestBuilder(ServiceRequest);

impl Default for ServiceRequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceRequestBuilder {
    #[must_use]
    pub fn new() -> Self {
        ServiceRequestBuilder(ServiceRequest::default())
    }

    #[must_use]
    pub fn new_from_body(body: ServiceRequest) -> Self {
        ServiceRequestBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> ServiceRequest {
        self.0
    }

    #[must_use]
    pub fn with_requesting_id(mut self, requesting_id: EntityId) -> Self {
        self.0.requesting_id = requesting_id;
        self
    }

    #[must_use]
    pub fn with_servicing_id(mut self, servicing_id: EntityId) -> Self {
        self.0.servicing_id = servicing_id;
        self
    }

    #[must_use]
    pub fn with_service_type_requested(
        mut self,
        service_type_requested: ServiceRequestServiceTypeRequested,
    ) -> Self {
        self.0.service_type_requested = service_type_requested;
        self
    }

    #[must_use]
    pub fn with_supply(mut self, supplies: SupplyQuantity) -> Self {
        self.0.supplies.push(supplies);
        self
    }

    #[must_use]
    pub fn with_supplies(mut self, supplies: Vec<SupplyQuantity>) -> Self {
        self.0.supplies = supplies;
        self
    }
}
