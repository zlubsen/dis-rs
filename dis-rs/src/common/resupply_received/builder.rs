use crate::common::model::EntityId;
use crate::common::model::SupplyQuantity;
use crate::resupply_received::model::ResupplyReceived;

pub struct ResupplyReceivedBuilder(ResupplyReceived);

impl ResupplyReceivedBuilder {
    pub fn new() -> Self {
        ResupplyReceivedBuilder(ResupplyReceived::default())
    }

    pub fn new_from_body(body: ResupplyReceived) -> Self {
        ResupplyReceivedBuilder(body)
    }

    pub fn build(self) -> ResupplyReceived {
        self.0
    }

    pub fn with_requesting_id(mut self, requesting_id: EntityId) -> Self {
        self.0.requesting_id = requesting_id;
        self
    }

    pub fn with_servicing_id(mut self, servicing_id: EntityId) -> Self {
        self.0.servicing_id = servicing_id;
        self
    }

    pub fn with_supply(mut self, supplies: SupplyQuantity) -> Self {
        self.0.supplies.push(supplies);
        self
    }

    pub fn with_supplies(mut self, supplies: Vec<SupplyQuantity>) -> Self {
        self.0.supplies = supplies;
        self
    }
}
