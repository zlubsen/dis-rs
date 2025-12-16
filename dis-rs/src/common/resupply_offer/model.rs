use crate::common::model::{EntityId, PduBody};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::model::{SupplyQuantity, SUPPLY_QUANTITY_RECORD_LENGTH};
use crate::resupply_offer::builder::ResupplyOfferBuilder;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const RESUPPLY_OFFER_BASE_BODY_LENGTH: u16 = 16;

/// 5.5.6 Resupply Offer PDU
///
/// 7.4.3 Resupply Offer PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ResupplyOffer {
    pub requesting_id: EntityId,
    pub servicing_id: EntityId,
    pub supplies: Vec<SupplyQuantity>,
}

impl ResupplyOffer {
    #[must_use]
    pub fn builder() -> ResupplyOfferBuilder {
        ResupplyOfferBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> ResupplyOfferBuilder {
        ResupplyOfferBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::ResupplyOffer(self)
    }
}

impl BodyInfo for ResupplyOffer {
    fn body_length(&self) -> u16 {
        RESUPPLY_OFFER_BASE_BODY_LENGTH
            + (self.supplies.len() as u16 * SUPPLY_QUANTITY_RECORD_LENGTH)
    }

    fn body_type(&self) -> PduType {
        PduType::ResupplyOffer
    }
}

impl Interaction for ResupplyOffer {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.requesting_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.servicing_id)
    }
}

impl From<ResupplyOffer> for PduBody {
    #[inline]
    fn from(value: ResupplyOffer) -> Self {
        value.into_pdu_body()
    }
}
