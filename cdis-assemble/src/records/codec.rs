use dis_rs::model::{PduHeader, TimeStamp};
use crate::codec::Codec;
use crate::records::model::{CdisHeader, CdisProtocolVersion, CdisTimeStamp};
use crate::types::model::UVINT8;

impl Codec for CdisHeader {
    type Counterpart = PduHeader;

    fn encode(item: Self::Counterpart) -> Self {
        Self {
            protocol_version: CdisProtocolVersion::SISO_023_2023,
            exercise_id: UVINT8::from(item.exercise_id),
            pdu_type: item.pdu_type,
            timestamp: TimeStamp::from(CdisTimeStamp::from(item.time_stamp)),
            length: 0,
            pdu_status: if let Some(status) = item.pdu_status { status } else { Default::default() },
        }
    }

    fn decode(&self) -> Self::Counterpart {
        let header = PduHeader::new_v7(self.exercise_id.value, self.pdu_type)
            .with_time_stamp(self.timestamp.raw_timestamp);  // FIXME scale CDIS to DIS time units
        todo!()
    }
}