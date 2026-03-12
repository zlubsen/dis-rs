use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::{PduType, RequiredReliabilityService, TransferControlTransferType};
use crate::model::{
    length_padded_to_num, EntityId, PduBody, RecordSpecification, BASE_RECORD_SPEC_RECORD_LENGTH,
};
use crate::transfer_ownership::builder::TransferOwnershipBuilder;
use crate::BodyRaw;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

const BASE_TRANSFER_OWNERSHIP_BODY_LENGTH: u16 = 28;

/// 5.9.4 Transfer Ownership (TO) PDU
///
/// 7.8.4 Transfer Ownership (TO) PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TransferOwnership {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub request_id: u32,
    pub required_reliability_service: RequiredReliabilityService,
    pub transfer_type: TransferControlTransferType,
    pub transfer_entity_id: EntityId,
    pub record_specification: RecordSpecification,
}

impl BodyRaw for TransferOwnership {
    type Builder = TransferOwnershipBuilder;

    fn builder() -> Self::Builder {
        Self::Builder::new()
    }

    fn into_builder(self) -> Self::Builder {
        Self::Builder::new_from_body(self)
    }

    fn into_pdu_body(self) -> PduBody {
        PduBody::TransferOwnership(self)
    }
}

impl BodyInfo for TransferOwnership {
    fn body_length(&self) -> u16 {
        BASE_TRANSFER_OWNERSHIP_BODY_LENGTH
            + (self
                .record_specification
                .record_sets
                .iter()
                .map(|record| {
                    let data_length_bytes = record
                        .records
                        .iter()
                        .map(|rec| rec.len() as u16)
                        .sum::<u16>();
                    let padded_record =
                        length_padded_to_num(data_length_bytes.into(), EIGHT_OCTETS);
                    BASE_RECORD_SPEC_RECORD_LENGTH + padded_record.record_length as u16
                })
                .sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::TransferOwnership
    }
}

impl Interaction for TransferOwnership {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
