use crate::common::model::{EntityId, PduBody, PduHeader};
use crate::common::other::model::Other;
use crate::common::parser::entity_id;
use crate::constants::PDU_HEADER_LEN_BYTES;
use crate::enumerations::PduType;
use nom::bytes::complete::take;
use nom::combinator::peek;
use nom::sequence::tuple;
use nom::IResult;

pub(crate) fn other_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move |input: &[u8]| {
        // Based on the PDU type, peek at the originating and receiving EntityIds.
        let (input, originating, receiving) = match header.pdu_type {
            // PDUs with only an origin
            PduType::EntityState
            | PduType::EntityStateUpdate
            | PduType::ElectromagneticEmission
            | PduType::Designator
            | PduType::Transmitter
            | PduType::Signal
            | PduType::Receiver
            | PduType::SupplementalEmissionEntityState
            | PduType::UnderwaterAcoustic
            | PduType::IsGroupOf
            | PduType::CreateEntityR
            | PduType::RemoveEntityR
            | PduType::AggregateState
            | PduType::IFF => {
                let (input, originating) = peek_originating_field(input)?;
                (input, Some(originating), None)
            }
            // PDUs with both an origin and a receiver
            PduType::Fire
            | PduType::Detonation
            | PduType::Collision
            | PduType::ServiceRequest
            | PduType::ResupplyOffer
            | PduType::ResupplyReceived
            | PduType::ResupplyCancel
            | PduType::RepairComplete
            | PduType::RepairResponse
            | PduType::CreateEntity
            | PduType::RemoveEntity
            | PduType::StartResume
            | PduType::StopFreeze
            | PduType::Acknowledge
            | PduType::ActionRequest
            | PduType::ActionResponse
            | PduType::DataQuery
            | PduType::SetData
            | PduType::Data
            | PduType::EventReport
            | PduType::IsPartOf
            | PduType::StartResumeR
            | PduType::StopFreezeR
            | PduType::AcknowledgeR
            | PduType::ActionRequestR
            | PduType::ActionResponseR
            | PduType::DataQueryR
            | PduType::SetDataR
            | PduType::DataR
            | PduType::EventReportR
            | PduType::CommentR
            | PduType::RecordR
            | PduType::SetRecordR
            | PduType::RecordQueryR
            | PduType::CollisionElastic
            | PduType::Comment => {
                let (input, (origin, receiving)) = peek_originating_receiving_fields(input)?;
                (input, Some(origin), Some(receiving))
            }
            // All others, and/or not evaluated TODO determine if these PDUs have an originating and receiving ID when implementing
            PduType::IntercomSignal
            | PduType::IntercomControl
            | PduType::TransferOwnership
            | PduType::MinefieldState
            | PduType::MinefieldQuery
            | PduType::MinefieldData
            | PduType::MinefieldResponseNACK
            | PduType::EnvironmentalProcess
            | PduType::GriddedData
            | PduType::PointObjectState
            | PduType::LinearObjectState
            | PduType::ArealObjectState
            | PduType::TSPI
            | PduType::Appearance
            | PduType::ArticulatedParts
            | PduType::LEFire
            | PduType::LEDetonation
            | PduType::DirectedEnergyFire
            | PduType::EntityDamageStatus
            | PduType::InformationOperationsAction
            | PduType::InformationOperationsReport
            | PduType::Attribute
            | PduType::Other
            | PduType::Unspecified(_) => (input, None, None),
        };

        let body_length_bytes = header.pdu_length.saturating_sub(PDU_HEADER_LEN_BYTES);
        let (input, body) = take(body_length_bytes)(input)?;
        let inner_body = body.to_vec();
        let body = Other::builder()
            .with_body(inner_body)
            .with_origin(originating)
            .with_receiver(receiving)
            .build();

        Ok((input, body.into_pdu_body()))
    }
}

fn peek_originating_field(input: &[u8]) -> IResult<&[u8], EntityId> {
    let (input, originating_id) = peek(entity_id)(input)?;
    Ok((input, originating_id))
}

fn peek_originating_receiving_fields(input: &[u8]) -> IResult<&[u8], (EntityId, EntityId)> {
    let (input, fields) = peek(tuple((entity_id, entity_id)))(input)?;
    Ok((input, fields))
}

#[cfg(test)]
mod tests {
    use crate::common::model::{PduBody, PduHeader};
    use crate::common::other::parser::other_body;
    use crate::enumerations::PduType;

    #[test]
    fn parse_other_body() {
        let header = PduHeader::new_v6(1, PduType::Other)
            .with_time_stamp(0u32)
            .with_length(10u16);
        let input: [u8; 10] = [0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00];
        let (input, body) = other_body(&header)(&input).expect("Should be Ok");
        if let PduBody::Other(pdu) = body {
            assert_eq!(pdu.body.len(), 10);
            assert_eq!(*pdu.body.first().unwrap(), 1u8);
        }
        assert!(input.is_empty());
    }

    #[test]
    fn parse_other_body_with_originating_id() {
        // EntityStatePdu has only an originating EntityId
        let header = PduHeader::new_v6(1, PduType::EntityState)
            .with_length(6u16)
            .with_time_stamp(0u32);
        let input: [u8; 6] = [0x00, 0x10, 0x00, 0x0A, 0x00, 0x01];
        let (input, body) = other_body(&header)(&input).expect("Should be Ok");
        if let PduBody::Other(pdu) = body {
            if let Some(originating) = pdu.originating_entity_id {
                assert_eq!(originating.simulation_address.site_id, 16);
                assert_eq!(originating.simulation_address.application_id, 10);
                assert_eq!(originating.entity_id, 1);
            } else {
                assert!(pdu.originating_entity_id.is_some());
            } // should fail
        }
        assert!(input.is_empty());
    }

    #[test]
    fn parse_other_body_with_receiving_id() {
        // FirePdu has both originating (Firing) and receiving (Target) EntityIds
        let header = PduHeader::new_v6(1, PduType::Fire)
            .with_length(12u16)
            .with_time_stamp(0u32);
        let input: [u8; 12] = [
            0x00, 0x10, 0x00, 0x0A, 0x00, 0x01, 0x00, 0x20, 0x00, 0x0B, 0x00, 0x08,
        ];
        let (input, body) = other_body(&header)(&input).expect("Should be Ok");
        if let PduBody::Other(pdu) = body {
            if let Some(originating) = pdu.originating_entity_id {
                assert_eq!(originating.simulation_address.site_id, 16);
                assert_eq!(originating.simulation_address.application_id, 10);
                assert_eq!(originating.entity_id, 1);
            } else {
                assert!(pdu.originating_entity_id.is_some());
            } // should fail
            if let Some(receiving) = pdu.receiving_entity_id {
                assert_eq!(receiving.simulation_address.site_id, 32);
                assert_eq!(receiving.simulation_address.application_id, 11);
                assert_eq!(receiving.entity_id, 8);
            } else {
                assert!(pdu.receiving_entity_id.is_some());
            } // should fail
        }
        assert!(input.is_empty());
    }
}
