use crate::common::designator::builder::DesignatorBuilder;
use crate::common::model::{EntityId, Location, PduBody, VectorF32};
use crate::common::{BodyInfo, Interaction};
use crate::enumerations::PduType;
use crate::enumerations::{DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub const DESIGNATOR_BODY_LENGTH: u16 = 76;

/// 5.7.4 Designator PDU
///
/// 7.6.3 Designator PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Designator {
    pub designating_entity_id: EntityId,
    pub system_name: DesignatorSystemName,
    pub designated_entity_id: EntityId,
    pub code: DesignatorCode,
    pub power: f32,
    pub wavelength: f32,
    pub spot_wrt_designated_entity: VectorF32,
    pub spot_location: Location,
    pub dead_reckoning_algorithm: DeadReckoningAlgorithm,
    pub linear_acceleration: VectorF32,
}

impl Designator {
    #[must_use]
    pub fn builder() -> DesignatorBuilder {
        DesignatorBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> DesignatorBuilder {
        DesignatorBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::Designator(self)
    }
}

impl BodyInfo for Designator {
    fn body_length(&self) -> u16 {
        DESIGNATOR_BODY_LENGTH
    }

    fn body_type(&self) -> PduType {
        PduType::Designator
    }
}

impl Interaction for Designator {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.designating_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.designated_entity_id)
    }
}

impl From<Designator> for PduBody {
    #[inline]
    fn from(value: Designator) -> Self {
        value.into_pdu_body()
    }
}
