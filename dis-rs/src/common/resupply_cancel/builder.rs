use crate::common::model::EntityId;
use crate::resupply_cancel::model::ResupplyCancel;

pub struct ResupplyCancelBuilder(ResupplyCancel);

impl ResupplyCancelBuilder {
    #[must_use]
    pub fn new() -> Self {
        ResupplyCancelBuilder(ResupplyCancel::default())
    }

    #[must_use]
    pub fn new_from_body(body: ResupplyCancel) -> Self {
        ResupplyCancelBuilder(body)
    }

    #[must_use]
    pub fn build(self) -> ResupplyCancel {
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
}
