use bytes::{BufMut, BytesMut};
use crate::common::model::{Pdu, PduBody, PduHeader};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::{EIGHT_OCTETS, PDU_HEADER_LEN_BYTES};
use crate::common::model::{ClockTime, DescriptorRecord, EntityId, EventId, FixedDatum, Location, MunitionDescriptor, Orientation, SimulationAddress, VariableDatum, VectorF32};
use crate::enumerations::ProtocolVersion;
use crate::{ArticulatedPart, AttachedPart, BeamData, EntityAssociationParameter, EntityTypeParameter, length_padded_to_num_bytes, SeparationParameter, VariableParameter, VariableParameterRecordType};

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
            PduBody::Detonation(body) => { body.serialize_pdu(version, buf) }
            PduBody::Collision(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ServiceRequest(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ResupplyOffer(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ResupplyReceived(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ResupplyCancel(body) => { body.serialize_pdu(version, buf) }
            // PduBody::RepairComplete(body) => { body.serialize_pdu(version, buf) }
            // PduBody::RepairResponse(body) => { body.serialize_pdu(version, buf) }
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
            // PduBody::CreateEntityR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::RemoveEntityR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::StartResumeR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::StopFreezeR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::AcknowledgeR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ActionRequestR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::ActionResponseR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::DataQueryR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::SetDataR(body) => { body.serialize_pdu(version, buf) }
            // PduBody::DataR(body) => { body.serialize_pdu(version, buf) }
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
        buf.put_u32(self.datum_id.into());
        let padded_record = length_padded_to_num_bytes(
            EIGHT_OCTETS + self.datum_value.len(),
            EIGHT_OCTETS);
        let data_length_with_padding = padded_record.record_length_bytes as u16;
        buf.put_u32(data_length_with_padding.into());
        buf.put_slice(self.datum_value.as_slice());
        buf.put_bytes(0, padded_record.padding_length_bytes);

        8 + data_length_with_padding
    }
}

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::common::Serialize;
    use crate::constants::PDU_HEADER_LEN_BYTES;
    use crate::enumerations::PduType;
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

impl Serialize for VariableParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        match self {
            VariableParameter::Articulated(parameter) => {
                buf.put_u8(VariableParameterRecordType::ArticulatedPart.into());
                let record_bytes = parameter.serialize(buf);
                1 + record_bytes
            }
            VariableParameter::Attached(parameter) => {
                buf.put_u8(VariableParameterRecordType::AttachedPart.into());
                let record_bytes = parameter.serialize(buf);
                1 + record_bytes
            }
            VariableParameter::Separation(parameter) => {
                buf.put_u8(VariableParameterRecordType::Separation.into());
                let record_bytes = parameter.serialize(buf);
                1 + record_bytes
            }
            VariableParameter::EntityType(parameter) => {
                buf.put_u8(VariableParameterRecordType::EntityType.into());
                let record_bytes = parameter.serialize(buf);
                1 + record_bytes
            }
            VariableParameter::EntityAssociation(parameter) => {
                buf.put_u8(VariableParameterRecordType::EntityAssociation.into());
                let record_bytes = parameter.serialize(buf);
                1 + record_bytes
            }
            VariableParameter::Unspecified(type_designator, parameter) => {
                buf.put_u8(*type_designator);
                for byte in parameter {
                    buf.put_u8(*byte);
                }
                16
            }
        }
    }
}

impl Serialize for ArticulatedPart {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.change_indicator.into());
        buf.put_u16(self.attachment_id);
        let type_class : u32 = self.type_class.into();
        let type_metric : u32 = self.type_metric.into();
        let on_wire_value = type_class + type_metric;
        buf.put_u32(on_wire_value);
        buf.put_f32(self.parameter_value);
        buf.put_u32(0u32); // 32-bit padding
        15
    }
}

impl Serialize for AttachedPart {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.detached_indicator.into());
        buf.put_u16(self.attachment_id);
        buf.put_u32(self.parameter_type.into());
        let entity_type_bytes = self.attached_part_type.serialize(buf);
        7 + entity_type_bytes
    }
}

impl Serialize for SeparationParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.reason.into());
        buf.put_u8(self.pre_entity_indicator.into());
        buf.put_u8(0u8);
        let parent_entity_id_bytes = self.parent_entity_id.serialize(buf);
        buf.put_u16(0u16);
        buf.put_u16(self.station_name.into());
        buf.put_u16(self.station_number);
        9 + parent_entity_id_bytes
    }
}

impl Serialize for EntityTypeParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.change_indicator.into());
        let entity_type_bytes = self.entity_type.serialize(buf);
        buf.put_u16(0u16);
        buf.put_u32(0u32);
        7 + entity_type_bytes
    }
}

impl Serialize for EntityAssociationParameter {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.change_indicator.into());
        buf.put_u8(self.association_status.into());
        buf.put_u8(self.association_type.into());
        let entity_id_bytes = self.entity_id.serialize(buf);
        buf.put_u16(self.own_station_location.into());
        buf.put_u8(self.physical_connection_type.into());
        buf.put_u8(self.group_member_type.into());
        buf.put_u16(self.group_number);
        9 + entity_id_bytes
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
