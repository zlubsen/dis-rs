use dis_rs::model::{Pdu, PduBody, TimeStamp};
use crate::{CdisBody, CdisPdu};
use crate::entity_state::model::EntityState;
use crate::records::model::{CdisHeader};
use crate::unsupported::Unsupported;

pub trait Codec {
    /// The DIS PDU, Body, Record, ... that is to be converted.
    type Counterpart;
    const SCALING: f32 = 0.0;
    const SCALING_2: f32 = 0.0;
    const CONVERSION: f32 = 0.0;
    const NORMALISATION: f32 = 0.0;

    fn encode(item: &Self::Counterpart) -> Self;
    fn decode(&self) -> Self::Counterpart;
}

impl Codec for CdisPdu {
    type Counterpart = Pdu;

    fn encode(item: &Self::Counterpart) -> Self {
        CdisPdu::finalize_from_parts(
            CdisHeader::encode(&item.header),
            CdisBody::encode(&item.body),
            None::<TimeStamp>)
    }

    fn decode(&self) -> Self::Counterpart {
        let header = self.header.decode();
        let ts = header.time_stamp;
        Self::Counterpart::finalize_from_parts(
            header,
            self.body.decode(),
            ts
        )
    }
}

impl Codec for CdisBody {
    type Counterpart = PduBody;

    fn encode(item: &Self::Counterpart) -> Self {
        match item {
            PduBody::Other(_) => { Self::Unsupported(Unsupported) }
            PduBody::EntityState(body) => { Self::EntityState(EntityState::encode(body)) }
            PduBody::Fire(_) => { Self::Unsupported(Unsupported) }
            PduBody::Detonation(_) => { Self::Unsupported(Unsupported) }
            PduBody::Collision(_) => { Self::Unsupported(Unsupported) }
            PduBody::ServiceRequest(_) => { Self::Unsupported(Unsupported) }
            PduBody::ResupplyOffer(_) => { Self::Unsupported(Unsupported) }
            PduBody::ResupplyReceived(_) => { Self::Unsupported(Unsupported) }
            PduBody::ResupplyCancel(_) => { Self::Unsupported(Unsupported) }
            PduBody::RepairComplete(_) => { Self::Unsupported(Unsupported) }
            PduBody::RepairResponse(_) => { Self::Unsupported(Unsupported) }
            PduBody::CreateEntity(_) => { Self::Unsupported(Unsupported) }
            PduBody::RemoveEntity(_) => { Self::Unsupported(Unsupported) }
            PduBody::StartResume(_) => { Self::Unsupported(Unsupported) }
            PduBody::StopFreeze(_) => { Self::Unsupported(Unsupported) }
            PduBody::Acknowledge(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionRequest(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionResponse(_) => { Self::Unsupported(Unsupported) }
            PduBody::DataQuery(_) => { Self::Unsupported(Unsupported) }
            PduBody::SetData(_) => { Self::Unsupported(Unsupported) }
            PduBody::Data(_) => { Self::Unsupported(Unsupported) }
            PduBody::EventReport(_) => { Self::Unsupported(Unsupported) }
            PduBody::Comment(_) => { Self::Unsupported(Unsupported) }
            PduBody::ElectromagneticEmission(_) => { Self::Unsupported(Unsupported) }
            PduBody::Designator(_) => { Self::Unsupported(Unsupported) }
            PduBody::Transmitter(_) => { Self::Unsupported(Unsupported) }
            PduBody::Signal(_) => { Self::Unsupported(Unsupported) }
            PduBody::Receiver(_) => { Self::Unsupported(Unsupported) }
            PduBody::IFF(_) => { Self::Unsupported(Unsupported) }
            PduBody::UnderwaterAcoustic(_) => { Self::Unsupported(Unsupported) }
            PduBody::SupplementalEmissionEntityState(_) => { Self::Unsupported(Unsupported) }
            PduBody::IntercomSignal => { Self::Unsupported(Unsupported) }
            PduBody::IntercomControl => { Self::Unsupported(Unsupported) }
            PduBody::AggregateState(_) => { Self::Unsupported(Unsupported) }
            PduBody::IsGroupOf(_) => { Self::Unsupported(Unsupported) }
            PduBody::TransferOwnership(_) => { Self::Unsupported(Unsupported) }
            PduBody::IsPartOf(_) => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldState => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldQuery => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldData => { Self::Unsupported(Unsupported) }
            PduBody::MinefieldResponseNACK => { Self::Unsupported(Unsupported) }
            PduBody::EnvironmentalProcess => { Self::Unsupported(Unsupported) }
            PduBody::GriddedData => { Self::Unsupported(Unsupported) }
            PduBody::PointObjectState => { Self::Unsupported(Unsupported) }
            PduBody::LinearObjectState => { Self::Unsupported(Unsupported) }
            PduBody::ArealObjectState => { Self::Unsupported(Unsupported) }
            PduBody::TSPI => { Self::Unsupported(Unsupported) }
            PduBody::Appearance => { Self::Unsupported(Unsupported) }
            PduBody::ArticulatedParts => { Self::Unsupported(Unsupported) }
            PduBody::LEFire => { Self::Unsupported(Unsupported) }
            PduBody::LEDetonation => { Self::Unsupported(Unsupported) }
            PduBody::CreateEntityR(_) => { Self::Unsupported(Unsupported) }
            PduBody::RemoveEntityR(_) => { Self::Unsupported(Unsupported) }
            PduBody::StartResumeR(_) => { Self::Unsupported(Unsupported) }
            PduBody::StopFreezeR(_) => { Self::Unsupported(Unsupported) }
            PduBody::AcknowledgeR(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionRequestR(_) => { Self::Unsupported(Unsupported) }
            PduBody::ActionResponseR(_) => { Self::Unsupported(Unsupported) }
            PduBody::DataQueryR(_) => { Self::Unsupported(Unsupported) }
            PduBody::SetDataR(_) => { Self::Unsupported(Unsupported) }
            PduBody::DataR(_) => { Self::Unsupported(Unsupported) }
            PduBody::EventReportR(_) => { Self::Unsupported(Unsupported) }
            PduBody::CommentR(_) => { Self::Unsupported(Unsupported) }
            PduBody::RecordR(_) => { Self::Unsupported(Unsupported) }
            PduBody::SetRecordR(_) => { Self::Unsupported(Unsupported) }
            PduBody::RecordQueryR(_) => { Self::Unsupported(Unsupported) }
            PduBody::CollisionElastic(_) => { Self::Unsupported(Unsupported) }
            PduBody::EntityStateUpdate(_) => { Self::Unsupported(Unsupported) }
            PduBody::DirectedEnergyFire => { Self::Unsupported(Unsupported) }
            PduBody::EntityDamageStatus => { Self::Unsupported(Unsupported) }
            PduBody::InformationOperationsAction => { Self::Unsupported(Unsupported) }
            PduBody::InformationOperationsReport => { Self::Unsupported(Unsupported) }
            PduBody::Attribute(_) => { Self::Unsupported(Unsupported) }
        }
    }

    fn decode(&self) -> Self::Counterpart {
        match self {
            // TODO add 'Unimplemented' Body to dis-rs, as impl for remaining PduBody types
            CdisBody::EntityState(body) => { PduBody::EntityState(body.decode()) }
            // CdisBody::Fire => {}
            // CdisBody::Detonation => {}
            // CdisBody::Collision => {}
            // CdisBody::CreateEntity => {}
            // CdisBody::RemoveEntity => {}
            // CdisBody::StartResume => {}
            // CdisBody::StopFreeze => {}
            // CdisBody::Acknowledge => {}
            // CdisBody::ActionRequest => {}
            // CdisBody::ActionResponse => {}
            // CdisBody::DataQuery => {}
            // CdisBody::SetData => {}
            // CdisBody::Data => {}
            // CdisBody::EventReport => {}
            // CdisBody::Comment => {}
            // CdisBody::ElectromagneticEmission => {}
            // CdisBody::Designator => {}
            // CdisBody::Transmitter => {}
            // CdisBody::Signal => {}
            // CdisBody::Receiver => {}
            // CdisBody::Iff => {}
            CdisBody::Unsupported(_) | _ => { PduBody::Other(dis_rs::other::model::Other::builder().build())}
        }
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::entity_state::model::{EntityMarking, EntityState};
    use dis_rs::enumerations::{Country, DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId, PduType, PlatformDomain, ProtocolVersion};
    use dis_rs::model::{EntityId, EntityType, Pdu, PduBody, PduHeader, TimeStamp};
    use crate::{BodyProperties, CdisBody, CdisPdu};
    use crate::Codec;
    use crate::entity_state::model::CdisEntityCapabilities;
    use crate::records::model::{CdisEntityMarking, CdisHeader, CdisProtocolVersion, LinearVelocity, Orientation, Units, WorldCoordinates};
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};

    #[test]
    fn cdis_pdu_entity_state_body_encode() {
        let dis_header = PduHeader::new_v7(7, PduType::EntityState);
        let dis_body = EntityState::builder()
            .with_entity_id(EntityId::new(7, 127, 255))
            .with_entity_type(EntityType::default()
                .with_domain(PlatformDomain::Air)
                .with_country(Country::Netherlands_NLD_)
                .with_kind(EntityKind::Platform))
            .with_force_id(ForceId::Friendly8)
            .with_marking(EntityMarking::new("TEST", EntityMarkingCharacterSet::ASCII))
            .build()
            .into_pdu_body();
        let dis_pdu = Pdu::finalize_from_parts(dis_header, dis_body, 1000);

        let cdis_pdu = CdisPdu::encode(&dis_pdu);

        let dis_body = if let PduBody::EntityState(es) = dis_pdu.body {
            es
        } else { assert!(false); dis_rs::entity_state::model::EntityState::default() };
        let cdis_body = if let CdisBody::EntityState(es) = cdis_pdu.body {
            es
        } else { assert!(false); crate::EntityState::default() };

        assert_eq!(dis_pdu.header.exercise_id, cdis_pdu.header.exercise_id.value);
        assert_eq!(dis_pdu.header.pdu_type, cdis_pdu.header.pdu_type);
        assert_eq!(cdis_pdu.header.protocol_version, CdisProtocolVersion::SISO_023_2023);
        assert_eq!(dis_body.force_id, ForceId::from(cdis_body.force_id.unwrap().value));
        assert_eq!(dis_body.entity_id.simulation_address.site_id, cdis_body.entity_id.site.value);
        assert_eq!(dis_body.entity_id.simulation_address.application_id, cdis_body.entity_id.application.value);
        assert_eq!(dis_body.entity_id.entity_id, cdis_body.entity_id.entity.value);
        assert_eq!(dis_body.entity_type.domain, PlatformDomain::from(cdis_body.entity_type.unwrap().domain));
        assert_eq!(dis_body.entity_type.kind, EntityKind::from(cdis_body.entity_type.unwrap().kind));
        assert_eq!(dis_body.entity_type.country, Country::from(cdis_body.entity_type.unwrap().country));
        assert_eq!(dis_body.entity_marking.marking_string, cdis_body.entity_marking.unwrap().marking);
    }

    #[test]
    fn cdis_pdu_entity_state_body_decode() {
        let cdis_body = crate::EntityState {
            units: Units::Dekameter,
            full_update_flag: true,
            entity_id: crate::records::model::EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            force_id: Some(UVINT8::from(u8::from(ForceId::Friendly))),
            entity_type: Some(crate::records::model::EntityType::new(u8::from(EntityKind::Platform), u8::from(PlatformDomain::Air), u16::from(Country::Netherlands_NLD_), UVINT8::from(0), UVINT8::from(0), UVINT8::from(0), UVINT8::from(0))),
            alternate_entity_type: None,
            entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(5), SVINT16::from(5),SVINT16::from(-5))),
            entity_location: Some(WorldCoordinates::new(52.0, 5.0, SVINT24::from(1000))),
            entity_orientation: Some(Orientation::new(4, 3, 2)),
            entity_appearance: None,
            dr_algorithm: DeadReckoningAlgorithm::DRM_FPW_ConstantVelocityLowAccelerationLinearMotionEntity,
            dr_params_other: None,
            dr_params_entity_linear_acceleration: None,
            dr_params_entity_angular_velocity: None,
            entity_marking: Some(CdisEntityMarking::new("TEST".to_string())),
            capabilities: Some(CdisEntityCapabilities(UVINT32::from(0xABC00000))),
            variable_parameters: vec![],
        }.into_cdis_body();
        let cdis_header = CdisHeader {
            protocol_version: CdisProtocolVersion::SISO_023_2023,
            exercise_id: UVINT8::from(8),
            pdu_type: PduType::EntityState,
            timestamp: Default::default(),
            length: 0,
            pdu_status: Default::default(),
        };
        let cdis = CdisPdu::finalize_from_parts(cdis_header, cdis_body, Some(TimeStamp::from(20000)));

        let dis = cdis.decode();

        let dis_body = if let PduBody::EntityState(es) = dis.body {
            es
        } else {
            assert!(false);
            Default::default()
        };
        let cdis_body = if let CdisBody::EntityState(es) = cdis.body {
            es
        } else {
            assert!(false);
            Default::default()
        };

        assert_eq!(dis.header.exercise_id, cdis.header.exercise_id.value);
        assert_eq!(dis.header.pdu_type, cdis.header.pdu_type);
        assert_eq!(dis.header.protocol_version, ProtocolVersion::IEEE1278_12012);
        assert_eq!(dis_body.force_id, ForceId::from(cdis_body.force_id.unwrap().value));
        assert_eq!(dis_body.entity_id.simulation_address.site_id, cdis_body.entity_id.site.value);
        assert_eq!(dis_body.entity_id.simulation_address.application_id, cdis_body.entity_id.application.value);
        assert_eq!(dis_body.entity_id.entity_id, cdis_body.entity_id.entity.value);
        assert_eq!(dis_body.entity_type.domain, PlatformDomain::from(cdis_body.entity_type.unwrap().domain));
        assert_eq!(dis_body.entity_type.kind, EntityKind::from(cdis_body.entity_type.unwrap().kind));
        assert_eq!(dis_body.entity_type.country, Country::from(cdis_body.entity_type.unwrap().country));
        assert_eq!(dis_body.entity_marking.marking_string, cdis_body.entity_marking.unwrap().marking);
        if let dis_rs::enumerations::EntityCapabilities::AirPlatformEntityCapabilities(air_caps) = dis_body.entity_capabilities {
            assert!(air_caps.ammunition_supply);
            assert!(!air_caps.fuel_supply);
            assert!(air_caps.recovery);
            assert!(!air_caps.repair);
        } else { assert!(false) };
    }
}