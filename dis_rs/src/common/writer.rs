use bytes::{BufMut, BytesMut};
use crate::common::model::{Pdu, PduBody, PduHeader};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::PDU_HEADER_LEN_BYTES;
use crate::{DescriptorRecord, EntityId, EventId, Location, MunitionDescriptor, Orientation, SimulationAddress, VectorF32};
use crate::enumerations::{ProtocolVersion};

impl Serialize for PduHeader {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.protocol_version.into());
        buf.put_u8(self.exercise_id);
        buf.put_u8(self.pdu_type.into());
        buf.put_u8(self.protocol_family.into());
        buf.put_u32(self.time_stamp);
        buf.put_u16(self.pdu_length);
        match self.protocol_version {
            ProtocolVersion::IEEE1278_12012 => {
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
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let header_size = self.header.serialize(buf);
        let version: SupportedVersion = self.header.protocol_version.into();
        let body_size = match &self.body {
            PduBody::Other(body) => { body.serialize_pdu(version, buf) } // TODO check if buffer capacity is enough for the body of an 'Other' PDU; perhaps make Serialize trait fallible
            PduBody::EntityState(body) => { body.serialize_pdu(version, buf) }
            PduBody::Fire(body) => { body.serialize_pdu(version, buf) }
            // PduBody::Detonation(body) => { body.serialize(buf) }
            // PduBody::Collision(body) => { body.serialize(buf) }
            // PduBody::ServiceRequest(body) => { body.serialize(buf) }
            // PduBody::ResupplyOffer(body) => { body.serialize(buf) }
            // PduBody::ResupplyReceived(body) => { body.serialize(buf) }
            // PduBody::ResupplyCancel(body) => { body.serialize(buf) }
            // PduBody::RepairComplete(body) => { body.serialize(buf) }
            // PduBody::RepairResponse(body) => { body.serialize(buf) }
            // PduBody::CreateEntity(body) => { body.serialize(buf) }
            // PduBody::RemoveEntity(body) => { body.serialize(buf) }
            // PduBody::StartResume(body) => { body.serialize(buf) }
            // PduBody::StopFreeze(body) => { body.serialize(buf) }
            // PduBody::Acknowledge(body) => { body.serialize(buf) }
            // PduBody::ActionRequest(body) => { body.serialize(buf) }
            // PduBody::ActionResponse(body) => { body.serialize(buf) }
            // PduBody::DataQuery(body) => { body.serialize(buf) }
            // PduBody::SetData(body) => { body.serialize(buf) }
            // PduBody::Data(body) => { body.serialize(buf) }
            // PduBody::EventReport(body) => { body.serialize(buf) }
            // PduBody::Comment(body) => { body.serialize(buf) }
            // PduBody::ElectromagneticEmission(body) => { body.serialize(buf) }
            // PduBody::Designator(body) => { body.serialize(buf) }
            // PduBody::Transmitter(body) => { body.serialize(buf) }
            // PduBody::Signal(body) => { body.serialize(buf) }
            // PduBody::Receiver(body) => { body.serialize(buf) }
            // PduBody::IFF(body) => { body.serialize(buf) }
            // PduBody::UnderwaterAcoustic(body) => { body.serialize(buf) }
            // PduBody::SupplementalEmissionEntityState(body) => { body.serialize(buf) }
            // PduBody::IntercomSignal(body) => { body.serialize(buf) }
            // PduBody::IntercomControl(body) => { body.serialize(buf) }
            // PduBody::AggregateState(body) => { body.serialize(buf) }
            // PduBody::IsGroupOf(body) => { body.serialize(buf) }
            // PduBody::TransferOwnership(body) => { body.serialize(buf) }
            // PduBody::IsPartOf(body) => { body.serialize(buf) }
            // PduBody::MinefieldState(body) => { body.serialize(buf) }
            // PduBody::MinefieldQuery(body) => { body.serialize(buf) }
            // PduBody::MinefieldData(body) => { body.serialize(buf) }
            // PduBody::MinefieldResponseNACK(body) => { body.serialize(buf) }
            // PduBody::EnvironmentalProcess(body) => { body.serialize(buf) }
            // PduBody::GriddedData(body) => { body.serialize(buf) }
            // PduBody::PointObjectState(body) => { body.serialize(buf) }
            // PduBody::LinearObjectState(body) => { body.serialize(buf) }
            // PduBody::ArealObjectState(body) => { body.serialize(buf) }
            // PduBody::TSPI(body) => { body.serialize(buf) }
            // PduBody::Appearance(body) => { body.serialize(buf) }
            // PduBody::ArticulatedParts(body) => { body.serialize(buf) }
            // PduBody::LEFire(body) => { body.serialize(buf) }
            // PduBody::LEDetonation(body) => { body.serialize(buf) }
            // PduBody::CreateEntityR(body) => { body.serialize(buf) }
            // PduBody::RemoveEntityR(body) => { body.serialize(buf) }
            // PduBody::StartResumeR(body) => { body.serialize(buf) }
            // PduBody::StopFreezeR(body) => { body.serialize(buf) }
            // PduBody::AcknowledgeR(body) => { body.serialize(buf) }
            // PduBody::ActionRequestR(body) => { body.serialize(buf) }
            // PduBody::ActionResponseR(body) => { body.serialize(buf) }
            // PduBody::DataQueryR(body) => { body.serialize(buf) }
            // PduBody::SetDataR(body) => { body.serialize(buf) }
            // PduBody::DataR(body) => { body.serialize(buf) }
            // PduBody::EventReportR(body) => { body.serialize(buf) }
            // PduBody::CommentR(body) => { body.serialize(buf) }
            // PduBody::RecordR(body) => { body.serialize(buf) }
            // PduBody::SetRecordR(body) => { body.serialize(buf) }
            // PduBody::RecordQueryR(body) => { body.serialize(buf) }
            // PduBody::CollisionElastic(body) => { body.serialize(buf) }
            // PduBody::EntityStateUpdate(body) => { body.serialize(buf) }
            // PduBody::DirectedEnergyFire(body) => { body.serialize(buf) }
            // PduBody::EntityDamageStatus(body) => { body.serialize(buf) }
            // PduBody::InformationOperationsAction(body) => { body.serialize(buf) }
            // PduBody::InformationOperationsReport(body) => { body.serialize(buf) }
            // PduBody::Attribute(body) => { body.serialize(buf) }
            _ => { todo!() }
        };
        header_size + body_size
    }
}

impl Serialize for EntityId {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let num_bytes = self.simulation_address.serialize(buf);
        buf.put_u16(self.entity_id);
        num_bytes + 2
    }
}

impl Serialize for SimulationAddress {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.site_id);
        buf.put_u16(self.application_id);
        4
    }
}

impl Serialize for EventId {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let num_bytes = self.simulation_address.serialize(buf);
        buf.put_u16(self.event_id);
        num_bytes + 2
    }
}

impl Serialize for VectorF32 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.first_vector_component);
        buf.put_f32(self.second_vector_component);
        buf.put_f32(self.third_vector_component);
        12
    }
}

impl Serialize for Location {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f64(self.x_coordinate);
        buf.put_f64(self.y_coordinate);
        buf.put_f64(self.z_coordinate);
        24
    }
}

impl Serialize for Orientation {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.psi);
        buf.put_f32(self.theta);
        buf.put_f32(self.phi);
        12
    }
}

impl Serialize for DescriptorRecord {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        match self {
            DescriptorRecord::Munition { entity_type, munition } => {
                let entity_bytes = entity_type.serialize(buf);
                let munition_bytes = munition.serialize(buf);
                entity_bytes + munition_bytes
            }
            DescriptorRecord::Expendable { entity_type } => {
                let entity_bytes = entity_type.serialize(buf);
                buf.put_u64(0u64);
                entity_bytes + 8
            }
        }
    }
}

impl Serialize for MunitionDescriptor {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.warhead.into());
        buf.put_u16(self.fuse.into());
        buf.put_u16(self.quantity);
        buf.put_u16(self.rate);
        8
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::Serialize;
    use crate::constants::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::{PduType};
    use crate::PduHeader;

    #[test]
    fn serialize_header() {
        let header = PduHeader::new_v6(1, PduType::EntityState)
            .with_time_stamp(10)
            .with_length(0);
        let mut buf = BytesMut::with_capacity(PDU_HEADER_LEN_BYTES as usize);

        header.serialize(&mut buf);

        let expected : [u8;12] = [0x06, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0c, 0x00, 0x00];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}
