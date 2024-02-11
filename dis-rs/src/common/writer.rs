use bytes::{BufMut, BytesMut};
use crate::common::model::{Pdu, PduBody, PduHeader};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::PDU_HEADER_LEN_BYTES;
use crate::common::model::{ArticulatedPart, AttachedPart, BeamData, ClockTime, DescriptorRecord, EntityAssociationParameter, EntityId, EntityTypeParameter, EventId, FixedDatum, length_padded_to_num, Location, MunitionDescriptor, Orientation, SeparationParameter, SimulationAddress, VariableDatum, VariableParameter, VectorF32};
use crate::enumerations::{ProtocolVersion, VariableParameterRecordType};
use crate::model::SupplyQuantity;

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
                    crate::v7::writer::serialize_pdu_status(&status, &self.pdu_type, buf);
                    buf.put_u8(0u8);
                } else { buf.put_u16(0u16) }
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
            PduBody::Detonation(body) => { body.serialize_pdu(version, buf) }
            PduBody::Collision(body) => { body.serialize_pdu(version, buf) }
            PduBody::ServiceRequest(body) => { body.serialize_pdu(version, buf) }
            PduBody::ResupplyOffer(body) => { body.serialize_pdu(version, buf) }
            PduBody::ResupplyReceived(body) => { body.serialize_pdu(version, buf) }
            PduBody::ResupplyCancel(body) => { body.serialize_pdu(version, buf) }
            PduBody::RepairComplete(body) => { body.serialize_pdu(version, buf) }
            PduBody::RepairResponse(body) => { body.serialize_pdu(version, buf) }
            PduBody::CreateEntity(body) => { body.serialize_pdu(version, buf) }
            PduBody::RemoveEntity(body) => { body.serialize_pdu(version, buf) }
            PduBody::StartResume(body) => { body.serialize_pdu(version, buf) }
            PduBody::StopFreeze(body) => { body.serialize_pdu(version, buf) }
            PduBody::Acknowledge(body) => { body.serialize_pdu(version, buf) }
            PduBody::ActionRequest(body) => { body.serialize_pdu(version, buf) }
            PduBody::ActionResponse(body) => { body.serialize_pdu(version, buf) }
            PduBody::DataQuery(body) => { body.serialize_pdu(version, buf) }
            PduBody::SetData(body) => { body.serialize_pdu(version, buf) }
            PduBody::Data(body) => { body.serialize_pdu(version, buf) }
            PduBody::EventReport(body) => { body.serialize_pdu(version, buf) }
            PduBody::Comment(body) => { body.serialize_pdu(version, buf) }
            PduBody::ElectromagneticEmission(body) => { body.serialize_pdu(version, buf) }
            PduBody::Designator(body) => { body.serialize_pdu(version, buf) }
            PduBody::Transmitter(body) => { body.serialize_pdu(version, buf) }
            PduBody::Signal(body) => { body.serialize_pdu(version, buf) }
            PduBody::Receiver(body) => { body.serialize_pdu(version, buf) }
            PduBody::IFF(body) => { body.serialize_pdu(version, buf) }
            // PduBody::UnderwaterAcoustic(body) => { body.serialize_pdu(version, buf) }
            // PduBody::SupplementalEmissionEntityState(body) => { body.serialize_pdu(version, buf) }
            // PduBody::IntercomSignal(body) => { body.serialize_pdu(version, buf) }
            // PduBody::IntercomControl(body) => { body.serialize_pdu(version, buf) }
            // PduBody::AggregateState(body) => { body.serialize_pdu(version, buf) }
            // PduBody::IsGroupOf(body) => { body.serialize_pdu(version, buf) }
            // PduBody::TransferOwnership(body) => { body.serialize_pdu(version, buf) }
            // PduBody::IsPartOf(body) => { body.serialize_pdu(version, buf) }
            // PduBody::MinefieldState(body) => { body.serialize_pdu(version, buf) }
            // PduBody::MinefieldQuery(body) => { body.serialize_pdu(version, buf) }
            // PduBody::MinefieldData(body) => { body.serialize_pdu(version, buf) }
            // PduBody::MinefieldResponseNACK(body) => { body.serialize_pdu(version, buf) }
            // PduBody::EnvironmentalProcess(body) => { body.serialize_pdu(version, buf) }
            // PduBody::GriddedData(body) => { body.serialize_pdu(version, buf) }
            // PduBody::PointObjectState(body) => { body.serialize_pdu(version, buf) }
            // PduBody::LinearObjectState(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ArealObjectState(body) => { body.serialize_pdu(version, buf) }
            // PduBody::TSPI(body) => { body.serialize_pdu(version, buf) }
            // PduBody::Appearance(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ArticulatedParts(body) => { body.serialize_pdu(version, buf) }
            // PduBody::LEFire(body) => { body.serialize_pdu(version, buf) }
            // PduBody::LEDetonation(body) => { body.serialize_pdu(version, buf) }
            PduBody::CreateEntityR(body) => { body.serialize_pdu(version, buf) }
            PduBody::RemoveEntityR(body) => { body.serialize_pdu(version, buf) }
            PduBody::StartResumeR(body) => { body.serialize_pdu(version, buf) }
            PduBody::StopFreezeR(body) => { body.serialize_pdu(version, buf) }
            PduBody::AcknowledgeR(body) => { body.serialize_pdu(version, buf) }
            PduBody::ActionRequestR(body) => { body.serialize_pdu(version, buf) }
            PduBody::ActionResponseR(body) => { body.serialize_pdu(version, buf) }
            PduBody::DataQueryR(body) => { body.serialize_pdu(version, buf) }
            PduBody::SetDataR(body) => { body.serialize_pdu(version, buf) }
            PduBody::DataR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::EventReportR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::CommentR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::RecordR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::SetRecordR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::RecordQueryR(body) => { body.serialize_pdu(version, buf) }
            PduBody::CollisionElastic(body) => { body.serialize_pdu(version, buf) }
            PduBody::EntityStateUpdate(body) => { body.serialize_pdu(version, buf) }
            // PduBody::DirectedEnergyFire(body) => { body.serialize_pdu(version, buf) }
            // PduBody::EntityDamageStatus(body) => { body.serialize_pdu(version, buf) }
            // PduBody::InformationOperationsAction(body) => { body.serialize_pdu(version, buf) }
            // PduBody::InformationOperationsReport(body) => { body.serialize_pdu(version, buf) }
            PduBody::Attribute(body) => { body.serialize_pdu(version, buf) }
            _ => { 0 }
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
            DescriptorRecord::Explosion { entity_type, explosive_material, explosive_force } => {
                let entity_bytes = entity_type.serialize(buf);
                buf.put_u16((*explosive_material).into());
                buf.put_u16(0u16);
                buf.put_f32(*explosive_force);
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

impl Serialize for ClockTime {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_i32(self.hour);
        buf.put_u32(self.time_past_hour);
        8
    }
}

impl Serialize for FixedDatum {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u32(self.datum_id.into());
        buf.put_u32(self.datum_value);

        8
    }
}

impl Serialize for VariableDatum {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        const SIXTY_FOUR_BITS: usize = 64;
        let data_length_bits: usize = self.datum_value.len() * 8;
        let padded_record_bits = length_padded_to_num(
            data_length_bits,
            SIXTY_FOUR_BITS);
        let record_length_bits = padded_record_bits.record_length as u16;
        let record_length_bytes = record_length_bits / 8;
        let padding_length_bytes = padded_record_bits.padding_length * 8;

        buf.put_u32(self.datum_id.into());
        buf.put_u32(data_length_bits as u32);
        buf.put_slice(self.datum_value.as_slice());
        buf.put_bytes(0, padding_length_bytes);

        8 + (record_length_bytes)
    }
}

impl Serialize for VariableParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        match self {
            VariableParameter::Articulated(inner) => {
                buf.put_u8(VariableParameterRecordType::ArticulatedPart.into());
                1 + inner.serialize(buf)
            }
            VariableParameter::Attached(inner) => {
                buf.put_u8(VariableParameterRecordType::AttachedPart.into());
                1 + inner.serialize(buf)
            }
            VariableParameter::Separation(inner) => {
                buf.put_u8(VariableParameterRecordType::Separation.into());
                1 + inner.serialize(buf)
            }
            VariableParameter::EntityType(inner) => {
                buf.put_u8(VariableParameterRecordType::EntityType.into());
                1 + inner.serialize(buf)
            }
            VariableParameter::EntityAssociation(inner) => {
                buf.put_u8(VariableParameterRecordType::EntityAssociation.into());
                1 + inner.serialize(buf)
            }
            VariableParameter::Unspecified(parameter_type, value) => {
                buf.put_u8(*parameter_type);
                buf.put(&value[..]);
                16
            }
        }
    }
}

impl Serialize for ArticulatedPart {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.change_indicator.into());
        buf.put_u16(self.attachment_id.into());
        let type_class: u32 = self.type_class.into();
        let type_metric: u32 = self.type_metric.into();
        buf.put_u32(type_class + type_metric);
        buf.put_f32(self.parameter_value);
        buf.put_u32(0u32);

        15
    }
}

impl Serialize for AttachedPart {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.detached_indicator.into());
        buf.put_u16(self.attachment_id);
        buf.put_u32(self.parameter_type.into());
        self.attached_part_type.serialize(buf);

        15
    }
}

impl Serialize for SeparationParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.reason.into());
        buf.put_u8(self.pre_entity_indicator.into());
        buf.put_u8(0u8);
        self.parent_entity_id.serialize(buf);
        buf.put_u16(0u16);
        buf.put_u16(self.station_name.into());
        buf.put_u16(self.station_number);

        15
    }
}

impl Serialize for EntityTypeParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.change_indicator.into());
        self.entity_type.serialize(buf);
        buf.put_u16(0u16);
        buf.put_u32(0u32);

        15
    }
}

impl Serialize for EntityAssociationParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.change_indicator.into());
        buf.put_u8(self.association_status.into());
        buf.put_u8(self.association_type.into());
        self.entity_id.serialize(buf);
        buf.put_u16(self.own_station_location.into());
        buf.put_u8(self.physical_connection_type.into());
        buf.put_u8(self.group_member_type.into());
        buf.put_u16(self.group_number);

        15
    }
}

impl Serialize for BeamData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.azimuth_center);
        buf.put_f32(self.azimuth_sweep);
        buf.put_f32(self.elevation_center);
        buf.put_f32(self.elevation_sweep);
        buf.put_f32(self.sweep_sync);

        20
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::Serialize;
    use crate::constants::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::{LvcIndicator, PduType};
    use crate::common::model::PduHeader;
    use crate::v7::model::PduStatus;

    #[test]
    fn serialize_header() {
        let header = PduHeader::new_v6(1, PduType::EntityState)
            .with_time_stamp(10u32)
            .with_length(0);
        let mut buf = BytesMut::with_capacity(PDU_HEADER_LEN_BYTES as usize);

        header.serialize(&mut buf);

        let expected : [u8;12] = [0x06, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0c, 0x00, 0x00];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }

    #[test]
    fn serialize_header_v7_no_status() {
        let header = PduHeader::new_v7(1, PduType::EntityState)
            .with_time_stamp(10u32)
            .with_length(0);
        let mut buf = BytesMut::with_capacity(PDU_HEADER_LEN_BYTES as usize);

        header.serialize(&mut buf);

        let expected : [u8;12] = [0x07, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0c, 0x00, 0x00];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }

    #[test]
    fn serialize_header_v7_with_status() {
        let header = PduHeader::new_v7(1, PduType::EntityState)
            .with_time_stamp(10u32)
            .with_length(0)
            .with_pdu_status(PduStatus::default().with_lvc_indicator(LvcIndicator::Live));
        let mut buf = BytesMut::with_capacity(PDU_HEADER_LEN_BYTES as usize);

        header.serialize(&mut buf);

        let expected : [u8;12] = [0x07, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x0a, 0x00, 0x0c, 0x02, 0x00];
        assert_eq!(buf.as_ref(), expected.as_ref());
    }
}

impl Serialize for SupplyQuantity {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let type_bytes = self.supply_type.serialize(buf);
        buf.put_f32(self.quantity);

        type_bytes + 4
    }
}
