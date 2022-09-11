use nom::bytes::complete::take;
use nom::combinator::peek;
use nom::IResult;
use nom::sequence::tuple;
use crate::common::parser::entity_id;
use crate::common::model::{EntityId, PduBody, PduHeader};
use crate::common::other::model::Other;
use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;
use crate::PduType;

pub fn other_body(header: &PduHeader) -> impl Fn(&[u8]) -> IResult<&[u8], PduBody> + '_ {
    move | input: &[u8] | {
        // Based on the PDU type, peek at the originating and receiving EntityIds.
        let (input, originating, receiving) = match header.pdu_type {
            // PDUs with only an origin
            PduType::EntityState |
            PduType::ElectromagneticEmission |
            PduType::Designator |
            PduType::Transmitter |
            PduType::Signal |
            PduType::Receiver |
            PduType::IFF => {
                let (input, originating) = peek_originating_field(input)?;
                (input, Some(originating), None)
            }
            // PDUs with both an origin and a receiver
            PduType::Fire |
            PduType::Detonation |
            PduType::Collision |
            PduType::ServiceRequest |
            PduType::ResupplyOffer |
            PduType::ResupplyReceived |
            PduType::ResupplyCancel |
            PduType::RepairComplete |
            PduType::RepairResponse |
            PduType::CreateEntity |
            PduType::RemoveEntity |
            PduType::StartResume |
            PduType::StopFreeze |
            PduType::Acknowledge |
            PduType::ActionRequest |
            PduType::ActionResponse |
            PduType::DataQuery |
            PduType::SetData |
            PduType::Data |
            PduType::EventReport |
            PduType::Comment => {
                let (input, (origin, receiving)) = peek_originating_receiving_fields(input)?;
                (input, Some(origin), Some(receiving))
            }
            // All others, and/or not evaluated TODO determine if these PDUs have an originating and receiving ID
            PduType::UnderwaterAcoustic |
            PduType::SupplementalEmissionEntityState |
            PduType::IntercomSignal |
            PduType::IntercomControl |
            PduType::AggregateState |
            PduType::IsGroupOf |
            PduType::TransferOwnership |
            PduType::IsPartOf |
            PduType::MinefieldState |
            PduType::MinefieldQuery |
            PduType::MinefieldData |
            PduType::MinefieldResponseNACK |
            PduType::EnvironmentalProcess |
            PduType::GriddedData |
            PduType::PointObjectState |
            PduType::LinearObjectState |
            PduType::ArealObjectState |
            PduType::TSPI |
            PduType::Appearance |
            PduType::ArticulatedParts |
            PduType::LEFire |
            PduType::LEDetonation |
            PduType::CreateEntityR |
            PduType::RemoveEntityR |
            PduType::StartResumeR |
            PduType::StopFreezeR |
            PduType::AcknowledgeR |
            PduType::ActionRequestR |
            PduType::ActionResponseR |
            PduType::DataQueryR |
            PduType::SetDataR |
            PduType::DataR |
            PduType::EventReportR |
            PduType::CommentR |
            PduType::RecordR |
            PduType::SetRecordR |
            PduType::RecordQueryR |
            PduType::CollisionElastic |
            PduType::EntityStateUpdate |
            PduType::DirectedEnergyFire |
            PduType::EntityDamageStatus |
            PduType::InformationOperationsAction |
            PduType::InformationOperationsReport |
            PduType::Attribute |
            PduType::Other |
            PduType::Unspecified(_) => { (input, None, None) }
        };

        let body_length_bytes = header.pdu_length as usize - PDU_HEADER_LEN_BYTES;
        let (input, body) = take(body_length_bytes)(input)?;
        let body = body.to_vec();

        Ok((input, PduBody::Other(Other::new_with_receiver(body, originating, receiving))))
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
    use crate::common::builder::PduHeaderBuilder;
    use crate::common::model::{PduBody};
    use crate::common::other::parser::other_body;
    use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::{PduType, ProtocolVersion, ProtocolFamily};

    #[test]
    fn parse_other_body() {
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::IEEE1278_1A1998)
            .exercise_id(1)
            .pdu_type(PduType::Other)
            .protocol_family(ProtocolFamily::Other)
            .pdu_length((PDU_HEADER_LEN_BYTES + 10) as u16)
            .time_stamp(0)
            .build().expect("Should be good");
        let input : [u8;10] = [0x01,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00,0x00];
        let (input, body) = other_body(&header)(&input).expect("Should be Ok");
        if let PduBody::Other(pdu) = body {
            assert_eq!(pdu.body.len(), 10);
            assert_eq!(*pdu.body.get(0).unwrap(), 1u8);
        }
        assert!(input.is_empty());
    }

    #[test]
    fn parse_other_body_with_originating_id() {
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::IEEE1278_1A1998)
            .exercise_id(1)
            .pdu_type(PduType::EntityState) // EntityStatePdu has only an originating EntityId
            .protocol_family(ProtocolFamily::EntityInformationInteraction)
            .pdu_length((PDU_HEADER_LEN_BYTES + 6) as u16)
            .time_stamp(0)
            .build().expect("Should be good");
        let input : [u8;6] = [0x00,0x10,0x00,0x0A,0x00,0x01];
        let (input, body) = other_body(&header)(&input).expect("Should be Ok");
        if let PduBody::Other(pdu) = body {
            if let Some(originating) = pdu.originating_entity_id {
                assert_eq!(originating.simulation_address.site_id, 16);
                assert_eq!(originating.simulation_address.application_id, 10);
                assert_eq!(originating.entity_id, 1);
            } else { assert!(pdu.originating_entity_id.is_some()) } // should fail
        }
        assert!(input.is_empty());
    }

    #[test]
    fn parse_other_body_with_receiving_id() {
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::IEEE1278_1A1998)
            .exercise_id(1)
            .pdu_type(PduType::Fire)// FirePdu has both originating (Firing) and receiving (Target) EntityIds
            .protocol_family(ProtocolFamily::EntityInformationInteraction)
            .pdu_length((PDU_HEADER_LEN_BYTES + 12) as u16)
            .time_stamp(0)
            .build().expect("Should be good");
        let input : [u8;12] = [0x00,0x10,0x00,0x0A,0x00,0x01,0x00,0x20,0x00,0x0B,0x00,0x08];
        let (input, body) = other_body(&header)(&input).expect("Should be Ok");
        if let PduBody::Other(pdu) = body {
            if let Some(originating) = pdu.originating_entity_id {
                assert_eq!(originating.simulation_address.site_id, 16);
                assert_eq!(originating.simulation_address.application_id, 10);
                assert_eq!(originating.entity_id, 1);
            } else { assert!(pdu.originating_entity_id.is_some()) } // should fail
            if let Some(receiving) = pdu.receiving_entity_id {
                assert_eq!(receiving.simulation_address.site_id, 32);
                assert_eq!(receiving.simulation_address.application_id, 11);
                assert_eq!(receiving.entity_id, 8);
            } else { assert!(pdu.receiving_entity_id.is_some()) } // should fail
        }
        assert!(input.is_empty());
    }
}