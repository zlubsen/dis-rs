use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{PduType, ServiceRequestServiceTypeRequested};
use crate::model::EntityType;
use crate::service_request::builder::ServiceRequestBuilder;

const SERVICE_REQUEST_BASE_BODY_LENGTH : u16 = 28;
const SUPPLY_QUANTITY_RECORD_LENGTH: u16 = 12;

/// 5.5.5 Service Request PDU
///
/// 7.4.2 Service Request PDU
#[derive(Debug, Default, PartialEq)]
pub struct ServiceRequest {
    pub requesting_id: EntityId,
    pub servicing_id: EntityId,
    pub service_type_requested: ServiceRequestServiceTypeRequested,
    pub supplies: Vec<SupplyQuantity>,
}

impl ServiceRequest {
    pub fn builder() -> ServiceRequestBuilder {
        ServiceRequestBuilder::new()
    }

    pub fn into_builder(self) -> ServiceRequestBuilder {
        ServiceRequestBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ServiceRequest(self)
    }
}

impl BodyInfo for ServiceRequest {
    fn body_length(&self) -> u16 {
        SERVICE_REQUEST_BASE_BODY_LENGTH + (self.supplies.len() as u16 * SUPPLY_QUANTITY_RECORD_LENGTH)
    }

    fn body_type(&self) -> PduType {
        PduType::ServiceRequest
    }
}

impl Interaction for ServiceRequest {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.requesting_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.servicing_id)
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct SupplyQuantity {
    pub supply_type: EntityType,
    pub quantity: f32,
}

impl SupplyQuantity {
    pub fn with_supply_type(mut self, supply_type: EntityType) -> Self {
        self.supply_type = supply_type;
        self
    }

    pub fn with_quantity(mut self, quantity: f32) -> Self {
        self.quantity = quantity;
        self
    }
}