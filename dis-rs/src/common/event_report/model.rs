use crate::common::model::{
    length_padded_to_num, EntityId, FixedDatum, PduBody, VariableDatum, BASE_VARIABLE_DATUM_LENGTH,
    FIXED_DATUM_LENGTH,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::EventType;
use crate::enumerations::PduType;
use crate::event_report::builder::EventReportBuilder;
use serde::{Deserialize, Serialize};

pub const BASE_EVENT_REPORT_BODY_LENGTH: u16 = 28;

/// 5.6.5.12 Event Report PDU
///
/// 5.9.2.5 Event Report PDU
///
/// 7.5.12 Event Report PDU
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventReport {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub event_type: EventType,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl EventReport {
    #[must_use]
    pub fn builder() -> EventReportBuilder {
        EventReportBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> EventReportBuilder {
        EventReportBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::EventReport(self)
    }
}

impl BodyInfo for EventReport {
    fn body_length(&self) -> u16 {
        BASE_EVENT_REPORT_BODY_LENGTH
            + (FIXED_DATUM_LENGTH * self.fixed_datum_records.len() as u16)
            + (self
                .variable_datum_records
                .iter()
                .map(|datum| {
                    let padded_record = length_padded_to_num(
                        BASE_VARIABLE_DATUM_LENGTH as usize + datum.datum_value.len(),
                        EIGHT_OCTETS,
                    );
                    padded_record.record_length as u16
                })
                .sum::<u16>())
    }

    fn body_type(&self) -> PduType {
        PduType::EventReport
    }
}

impl Interaction for EventReport {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
