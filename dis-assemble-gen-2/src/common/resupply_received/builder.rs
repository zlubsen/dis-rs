use crate::common::model::EntityId;
use crate::common::model::SupplyQuantity;
use crate::resupply_received::model::ResupplyReceived;

pub struct ResupplyReceivedBuilder(ResupplyReceived);

impl Default for ResupplyReceivedBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl ResupplyReceivedBuilder {
    #[must_use]
    pub fn new() -> Self {
        ResupplyReceivedBuilder(ResupplyReceived::default())
    }

    #[must_use]
    pub fn new_from_body(body: ResupplyReceived) -> Self {
        ResupplyReceivedBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> ResupplyReceived {
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
