use crate::common::model::{EntityId, FixedDatum, PduBody, VariableDatum};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{ActionId, PduType};

pub const BASE_ACTION_REQUEST_BODY_LENGTH: u16 = 40;

pub struct ActionRequest {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub action_id: ActionId,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl Default for ActionRequest {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRequest {
    pub fn new() -> Self {
        Self {
            originating_id: Default::default(),
            receiving_id: Default::default(),
            request_id: 0,
            action_id: Default::default(),
            fixed_datum_records: vec![],
            variable_datum_records: vec![],
        }
    }

    pub fn with_origination_id(mut self, originating_id: EntityId) -> Self {
        self.originating_id = originating_id;
        self
    }

    pub fn with_receiving_id(mut self, receiving_id: EntityId) -> Self {
        self.receiving_id = receiving_id;
        self
    }

    pub fn with_request_id(mut self, request_id: u32) -> Self {
        self.request_id = request_id;
        self
    }

    pub fn with_action_id(mut self, action_id: ActionId) -> Self {
        self.action_id = action_id;
        self
    }

    // pub fn with_action_id(mut self, action_id: ActionId) -> Self {
    //     self.action_id = action_id;
    //     self
    // }
    //
    // pub fn with_action_id(mut self, action_id: ActionId) -> Self {
    //     self.action_id = action_id;
    //     self
    // }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ActionRequest(self)
    }
}

impl BodyInfo for ActionRequest {
    fn body_length(&self) -> u16 {
        BASE_ACTION_REQUEST_BODY_LENGTH +
            todo!()
    }

    fn body_type(&self) -> PduType {
        PduType::ActionRequest
    }
}

impl Interaction for ActionRequest {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}