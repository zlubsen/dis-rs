use crate::common::model::{
    length_padded_to_num, EntityId, FixedDatum, PduBody, VariableDatum, BASE_VARIABLE_DATUM_LENGTH,
    FIXED_DATUM_LENGTH,
};
use crate::common::{BodyInfo, Interaction};
use crate::constants::EIGHT_OCTETS;
use crate::enumerations::EventType;
use crate::enumerations::PduType;
use crate::event_report_r::builder::EventReportRBuilder;

pub const BASE_EVENT_REPORT_R_BODY_LENGTH: u16 = 28;

/// 5.12.4.12 Event Report-R PDU
///
/// 7.11.12 Event Report-R PDU
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EventReportR {
    pub originating_id: EntityId,
    pub receiving_id: EntityId,
    pub event_type: EventType,
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl EventReportR {
    #[must_use]
    pub fn builder() -> EventReportRBuilder {
        EventReportRBuilder::new()
    }

    #[must_use]
    pub fn into_builder(self) -> EventReportRBuilder {
        EventReportRBuilder::new_from_body(self)
    }

    #[must_use]
    pub fn into_pdu_body(self) -> PduBody {
        PduBody::EventReportR(self)
    }
}

impl BodyInfo for EventReportR {
    fn body_length(&self) -> u16 {
        BASE_EVENT_REPORT_R_BODY_LENGTH
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
        PduType::EventReportR
    }
}

impl Interaction for EventReportR {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
    }
}
