use crate::common::model::{EntityId, PduBody, SupplyQuantity, SUPPLY_QUANTITY_RECORD_LENGTH};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{PduType, ServiceRequestServiceTypeRequested};
use crate::service_request::builder::ServiceRequestBuilder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const SERVICE_REQUEST_BASE_BODY_LENGTH: u16 = 28;

/// 5.5.5 Service Request PDU
///
/// 7.4.2 Service Request PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ServiceRequest {
    pub requesting_id: EntityId,
    pub servicing_id: EntityId,
    pub service_type_requested: ServiceRequestServiceTypeRequested,
    pub supplies: Vec<SupplyQuantity>,
}

impl ServiceRequest {
    #[must_use]
    pub fn builder() -> ServiceRequestBuilder {
        ServiceRequestBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> ServiceRequestBuilder {
        ServiceRequestBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ServiceRequest(self)
    }
}

impl BodyInfo for ServiceRequest {
    fn body_length(&self) -> u16 {
        SERVICE_REQUEST_BASE_BODY_LENGTH
            + (self.supplies.len() as u16 * SUPPLY_QUANTITY_RECORD_LENGTH)
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
