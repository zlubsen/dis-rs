use bytes::{BufMut, BytesMut};
use crate::dis::common::model::PDU_HEADER_LEN_BYTES;
use crate::dis::common::Serialize;
use crate::dis::v6::model::{Pdu, PduHeader};

impl Serialize for PduHeader {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        buf.put_u8(self.protocol_version.into());
        buf.put_u8(self.exercise_id);
        buf.put_u8(self.pdu_type.into());
        buf.put_u8(self.protocol_family.into());
        buf.put_u32(self.time_stamp);
        buf.put_u16(self.pdu_length);
        buf.put_u16(0u16);

        PDU_HEADER_LEN_BYTES
    }
}

impl Serialize for Pdu {
    fn serialize(&self, buf: &mut BytesMut) -> usize {
        match self {
            Pdu::Other(pdu) => { pdu.serialize(buf) }
            Pdu::EntityState(pdu) => { pdu.serialize(buf) }
            _ => { todo!() }
            // Pdu::Fire => {}
            // Pdu::Detonation => {}
            // Pdu::Collision => {}
            // Pdu::ServiceRequest => {}
            // Pdu::ResupplyOffer => {}
            // Pdu::ResupplyReceived => {}
            // Pdu::ResupplyCancel => {}
            // Pdu::RepairComplete => {}
            // Pdu::RepairResponse => {}
            // Pdu::CreateEntity => {}
            // Pdu::RemoveEntity => {}
            // Pdu::StartResume => {}
            // Pdu::StopFreeze => {}
            // Pdu::Acknowledge => {}
            // Pdu::ActionRequest => {}
            // Pdu::ActionResponse => {}
            // Pdu::DataQuery => {}
            // Pdu::SetData => {}
            // Pdu::Data => {}
            // Pdu::EventReport => {}
            // Pdu::Comment => {}
            // Pdu::ElectromagneticEmission => {}
            // Pdu::Designator => {}
            // Pdu::Transmitter => {}
            // Pdu::Signal => {}
            // Pdu::Receiver => {}
            // Pdu::AnnounceObject => {}
            // Pdu::DeleteObject => {}
            // Pdu::DescribeApplication => {}
            // Pdu::DescribeEvent => {}
            // Pdu::DescribeObject => {}
            // Pdu::RequestEvent => {}
            // Pdu::RequestObject => {}
            // Pdu::TimeSpacePositionIndicatorFI => {}
            // Pdu::AppearanceFI => {}
            // Pdu::ArticulatedPartsFI => {}
            // Pdu::FireFI => {}
            // Pdu::DetonationFI => {}
            // Pdu::PointObjectState => {}
            // Pdu::LinearObjectState => {}
            // Pdu::ArealObjectState => {}
            // Pdu::Environment => {}
            // Pdu::TransferControlRequest => {}
            // Pdu::TransferControl => {}
            // Pdu::TransferControlAcknowledge => {}
            // Pdu::IntercomControl => {}
            // Pdu::IntercomSignal => {}
            // Pdu::Aggregate => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::dis::common::model::{PDU_HEADER_LEN_BYTES, ProtocolFamily, ProtocolVersion};
    use crate::dis::common::Serialize;
    use crate::dis::v6::builder::PduHeaderBuilder;
    use crate::dis::common::model::PduType;

    #[test]
    fn serialize_header() {
        let header = PduHeaderBuilder::new()
            .protocol_version(ProtocolVersion::Ieee1278_1a1998)
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
