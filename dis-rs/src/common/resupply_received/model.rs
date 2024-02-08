use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::{PduType};
use crate::model::{SUPPLY_QUANTITY_RECORD_LENGTH, SupplyQuantity};
use crate::resupply_received::builder::ResupplyReceivedBuilder;

const RESUPPLY_RECEIVED_BASE_BODY_LENGTH : u16 = 28;

/// 5.5.7 Resupply Received PDU
///
/// 7.4.4 Resupply Received PDU
#[derive(Debug, Default, PartialEq)]
pub struct ResupplyReceived {
    pub requesting_id: EntityId,
    pub servicing_id: EntityId,
    pub supplies: Vec<SupplyQuantity>,
}

impl ResupplyReceived {
    pub fn builder() -> ResupplyReceivedBuilder {
        ResupplyReceivedBuilder::new()
    }

    pub fn into_builder(self) -> ResupplyReceivedBuilder {
        ResupplyReceivedBuilder::new_from_body(self)
    }

    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ResupplyReceived(self)
    }
}

impl BodyInfo for ResupplyReceived {
    fn body_length(&self) -> u16 {
        RESUPPLY_RECEIVED_BASE_BODY_LENGTH + (self.supplies.len() as u16 * SUPPLY_QUANTITY_RECORD_LENGTH)
    }

    fn body_type(&self) -> PduType {
        PduType::ResupplyReceived
    }
}

impl Interaction for ResupplyReceived {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.requesting_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.servicing_id)
    }
}