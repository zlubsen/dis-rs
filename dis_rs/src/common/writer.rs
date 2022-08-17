use bytes::{BufMut, BytesMut};
use crate::common::model::{Pdu, PduBody, PduHeader, ProtocolVersion};
use crate::common::Serialize;
use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;
use crate::{EntityId, EventId, Location, Orientation, SimulationAddress, VectorF32};

impl Serialize for PduHeader {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.protocol_version.into());
        buf.put_u8(self.exercise_id);
        buf.put_u8(self.pdu_type.into());
        buf.put_u8(self.protocol_family.into());
        buf.put_u32(self.time_stamp);
        buf.put_u16(self.pdu_length);
        match self.protocol_version {
            ProtocolVersion::Ieee1278_1_2012 => {
                if let Some(status) = self.pdu_status {
                    status.serialize(buf);
                    buf.put_u8(0u8);
                }
            }
            _ => { buf.put_u16(0u16) }
        }

        PDU_HEADER_LEN_BYTES
    }
}

impl Serialize for Pdu {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let header_size = self.header.serialize(buf);
        let body_size = match &self.body {
            PduBody::Other(body) => { body.serialize(buf) } // TODO check if buffer capacity is enough for the body of an 'Other' PDU; perhaps make Serialize trait fallible
            PduBody::EntityState(body) => { body.serialize(buf) }
            // PduBody::Fire => {}
            // PduBody::Detonation => {}
            // PduBody::Collision => {}
            // PduBody::ServiceRequest => {}
            // PduBody::ResupplyOffer => {}
            // PduBody::ResupplyReceived => {}
            // PduBody::ResupplyCancel => {}
            // PduBody::RepairComplete => {}
            // PduBody::RepairResponse => {}
            // PduBody::CreateEntity => {}
            // PduBody::RemoveEntity => {}
            // PduBody::StartResume => {}
            // PduBody::StopFreeze => {}
            // PduBody::Acknowledge => {}
            // PduBody::ActionRequest => {}
            // PduBody::ActionResponse => {}
            // PduBody::DataQuery => {}
            // PduBody::SetData => {}
            // PduBody::Data => {}
            // PduBody::EventReport => {}
            // PduBody::Comment => {}
            // PduBody::ElectromagneticEmission => {}
            // PduBody::Designator => {}
            // PduBody::Transmitter => {}
            // PduBody::Signal => {}
            // PduBody::Receiver => {}
            // PduBody::IFF => {}
            // PduBody::UnderwaterAcoustic => {}
            // PduBody::SupplementalEmissionEntityState => {}
            // PduBody::IntercomSignal => {}
            // PduBody::IntercomControl => {}
            // PduBody::AggregateState => {}
            // PduBody::IsGroupOf => {}
            // PduBody::TransferOwnership => {}
            // PduBody::IsPartOf => {}
            // PduBody::MinefieldState => {}
            // PduBody::MinefieldQuery => {}
            // PduBody::MinefieldData => {}
            // PduBody::MinefieldResponseNACK => {}
            // PduBody::EnvironmentalProcess => {}
            // PduBody::GriddedData => {}
            // PduBody::PointObjectState => {}
            // PduBody::LinearObjectState => {}
            // PduBody::ArealObjectState => {}
            // PduBody::TSPI => {}
            // PduBody::Appearance => {}
            // PduBody::ArticulatedParts => {}
            // PduBody::LEFire => {}
            // PduBody::LEDetonation => {}
            // PduBody::CreateEntityR => {}
            // PduBody::RemoveEntityR => {}
            // PduBody::StartResumeR => {}
            // PduBody::StopFreezeR => {}
            // PduBody::AcknowledgeR => {}
            // PduBody::ActionRequestR => {}
            // PduBody::ActionResponseR => {}
            // PduBody::DataQueryR => {}
            // PduBody::SetDataR => {}
            // PduBody::DataR => {}
            // PduBody::EventReportR => {}
            // PduBody::CommentR => {}
            // PduBody::RecordR => {}
            // PduBody::SetRecordR => {}
            // PduBody::RecordQueryR => {}
            // PduBody::CollisionElastic => {}
            // PduBody::EntityStateUpdate => {}
            // PduBody::DirectedEnergyFire => {}
            // PduBody::EntityDamageStatus => {}
            // PduBody::InformationOperationsAction => {}
            // PduBody::InformationOperationsReport => {}
            // PduBody::Attribute => {}
            _ => { todo!() }
        };
        header_size + body_size
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::builder::PduHeaderBuilder;
    use crate::common::model::{PduType, ProtocolFamily, ProtocolVersion};
    use crate::common::Serialize;
    use crate::common::symbolic_names::PDU_HEADER_LEN_BYTES;

    #[test]
    fn serialize_header() {
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::Ieee1278_1a_1998)
            .exercise_id(1)
            .pdu_type(PduType::EntityStatePdu)
            .protocol_family(ProtocolFamily::EntityInformationInteraction)
            .time_stamp(10)
            .pdu_length(PDU_HEADER_LEN_BYTES as u16)
            .build().expect("Should be Ok");
        let mut buf = BytesMut::with_capacity(PDU_HEADER_LEN_BYTES);

        header.serialize(&mut buf);

        let expected : [u8;12] = [0x06, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0c, 0x00, 0x00];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}

impl Serialize for EntityId {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let num_bytes = self.simulation_address.serialize(buf);
        buf.put_u16(self.entity_id);
        num_bytes + 2
    }
}

impl Serialize for SimulationAddress {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u16(self.site_id);
        buf.put_u16(self.application_id);
        4
    }
}

impl Serialize for EventId {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        let num_bytes = self.simulation_address.serialize(buf);
        buf.put_u16(self.event_id);
        num_bytes + 2
    }
}

impl Serialize for VectorF32 {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32(self.first_vector_component);
        buf.put_f32(self.second_vector_component);
        buf.put_f32(self.third_vector_component);
        12
    }
}

impl Serialize for Location {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f64(self.x_coordinate);
        buf.put_f64(self.y_coordinate);
        buf.put_f64(self.z_coordinate);
        24
    }
}

impl Serialize for Orientation {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_f32(self.psi);
        buf.put_f32(self.theta);
        buf.put_f32(self.phi);
        12
    }
}
