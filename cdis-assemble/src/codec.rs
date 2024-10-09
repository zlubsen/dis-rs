use std::collections::HashMap;
use dis_rs::model::{EntityId, Pdu, PduBody, TimeStamp};
use dis_rs::VariableParameters;
use crate::{BodyProperties, CdisBody, CdisPdu};
use crate::acknowledge::model::Acknowledge;
use crate::action_request::model::ActionRequest;
use crate::action_response::model::ActionResponse;
use crate::collision::model::Collision;
use crate::comment::model::Comment;
use crate::create_entity::model::CreateEntity;
use crate::data::model::Data;
use crate::data_query::model::DataQuery;
use crate::designator::codec::{decode_designator_body_and_update_state, DecoderStateDesignator, encode_designator_body_and_update_state, EncoderStateDesignator};
use crate::detonation::model::Detonation;
use crate::electromagnetic_emission::codec::{decode_electromagnetic_emission_body_and_update_state, DecoderStateElectromagneticEmission, encode_electromagnetic_emission_body_and_update_state, EncoderStateElectromagneticEmission};
use crate::entity_state::codec::{decode_entity_state_body_and_update_state, DecoderStateEntityState, encode_entity_state_body_and_update_state, EncoderStateEntityState};
use crate::event_report::model::EventReport;
use crate::fire::model::Fire;
use crate::iff::codec::{decode_iff_body_and_update_state, encode_iff_body_and_update_state, DecoderStateIff, EncoderStateIff};
use crate::receiver::model::Receiver;
use crate::records::model::CdisHeader;
use crate::remove_entity::model::RemoveEntity;
use crate::set_data::model::SetData;
use crate::signal::model::Signal;
use crate::start_resume::model::StartResume;
use crate::stop_freeze::model::StopFreeze;
use crate::transmitter::codec::{decode_transmitter_body_and_update_state, encode_transmitter_body_and_update_state, DecoderStateTransmitter, EncoderStateTransmitter};
use crate::unsupported::Unsupported;

pub const DEFAULT_HBT_CDIS_FULL_UPDATE_MPLIER: f32 = 2.4;

/// The Codec trait is to be implemented for any type that needs conversion
/// to (encoding) and from (decoding) C-DIS type equivalents.
pub trait Codec {
    /// The Record, Type, ... that is to be converted from and to.
    type Counterpart;
    const SCALING: f32 = 0.0;
    const SCALING_2: f32 = 0.0;
    const CONVERSION: f32 = 0.0;
    const NORMALISATION: f32 = 0.0;

    /// Function that encodes `Self::Counterpart` into `Self`.
    fn encode(item: &Self::Counterpart) -> Self;
    /// Method that decodes `Self` into `Self::Counterpart`.
    fn decode(&self) -> Self::Counterpart;
}

#[derive(Debug, Default)]
pub struct EncoderState {
    pub entity_state: HashMap<EntityId, EncoderStateEntityState>,
    pub ee: HashMap<EntityId, EncoderStateElectromagneticEmission>,
    pub designator: HashMap<EntityId, EncoderStateDesignator>,
    pub transmitter: HashMap<(EntityId, u16), EncoderStateTransmitter>,
    pub iff: HashMap<EntityId, EncoderStateIff>,
}

impl EncoderState {
    pub fn new() -> Self {
        Self {
            entity_state: Default::default(),
            ee: Default::default(),
            designator: Default::default(),
            transmitter: Default::default(),
            iff: Default::default(),
        }
    }
}

#[derive(Debug, Default)]
pub struct DecoderState {
    pub entity_state: HashMap<EntityId, DecoderStateEntityState>,
    pub ee: HashMap<EntityId, DecoderStateElectromagneticEmission>,
    pub designator: HashMap<EntityId, DecoderStateDesignator>,
    pub transmitter: HashMap<(EntityId, u16), DecoderStateTransmitter>,
    pub iff: HashMap<EntityId, DecoderStateIff>,
}

impl DecoderState {
    pub fn new() -> Self {
        Self {
            entity_state: Default::default(),
            ee: Default::default(),
            designator: Default::default(),
            transmitter: Default::default(),
            iff: Default::default(),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum CodecUpdateMode {
    #[default]
    FullUpdate,
    PartialUpdate,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum CodecOptimizeMode {
    Bandwidth,
    #[default]
    Completeness,
}

#[derive(Copy, Clone, Debug, Default)]
pub struct CodecOptions {
    pub update_mode: CodecUpdateMode,
    pub optimize_mode: CodecOptimizeMode,
    pub federation_parameters: VariableParameters,
    pub hbt_cdis_full_update_mplier: f32,
}

impl CodecOptions {
    pub fn new_full_update() -> Self {
        Self {
            update_mode: Default::default(),
            optimize_mode: Default::default(),
            federation_parameters: Default::default(),
            hbt_cdis_full_update_mplier: DEFAULT_HBT_CDIS_FULL_UPDATE_MPLIER
        }
    }

    pub fn new_partial_update() -> Self {
        Self {
            update_mode: CodecUpdateMode::PartialUpdate,
            optimize_mode: Default::default(),
            federation_parameters: Default::default(),
            hbt_cdis_full_update_mplier: DEFAULT_HBT_CDIS_FULL_UPDATE_MPLIER,
        }
    }

    pub fn optimize_bandwidth(mut self) -> Self {
        self.optimize_mode = CodecOptimizeMode::Bandwidth;
        self
    }

    pub fn optimize_completeness(mut self) -> Self {
        self.optimize_mode = CodecOptimizeMode::Completeness;
        self
    }

    pub fn with_federation_parameters(mut self, parameters: VariableParameters) -> Self {
        self.federation_parameters = parameters;
        self
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub enum CodecStateResult {
    #[default]
    StateUnaffected,
    StateUpdateEntityState,
    StateUpdateElectromagneticEmission,
    StateUpdateDesignator,
    StateUpdateTransmitter,
    StateUpdateIff,
}

impl CdisPdu {
    pub fn encode(item: &Pdu, state: &mut EncoderState, options: &CodecOptions) -> (Self, CodecStateResult) {
        let header = CdisHeader::encode(&item.header);
        let (body, state_results) = CdisBody::encode(&item.body, state, options);
        let pdu = CdisPdu::finalize_from_parts(
            header,
            body,
            None::<TimeStamp>);
        (pdu, state_results)
    }

    pub fn decode(&self, state: &mut DecoderState, options: &CodecOptions) -> (Pdu, CodecStateResult) {
        let header = self.header.decode();
        let ts = header.time_stamp;
        let (body, state_result) = self.body.decode(state, options);
        let pdu = Pdu::finalize_from_parts(
            header,
            body,
            ts);
        (pdu, state_result)
    }
}

impl CdisBody {
    pub fn encode(item: &PduBody, state: &mut EncoderState, options: &CodecOptions) -> (Self, CodecStateResult) {
        match item {
            PduBody::Other(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::EntityState(dis_body) => {
                encode_entity_state_body_and_update_state(dis_body, state, options)
            }
            PduBody::Fire(dis_body) => { (Fire::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::Detonation(dis_body) => { (Detonation::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::Collision(dis_body) => { (Collision::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::ServiceRequest(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::ResupplyOffer(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::ResupplyReceived(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::ResupplyCancel(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::RepairComplete(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::RepairResponse(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::CreateEntity(dis_body) => { (CreateEntity::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::RemoveEntity(dis_body) => { (RemoveEntity::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::StartResume(dis_body) => { (StartResume::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::StopFreeze(dis_body) => { (StopFreeze::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::Acknowledge(dis_body) => { (Acknowledge::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::ActionRequest(dis_body) => { (ActionRequest::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::ActionResponse(dis_body) => { (ActionResponse::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::DataQuery(dis_body) => { (DataQuery::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::SetData(dis_body) => { (SetData::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::Data(dis_body) => { (Data::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::EventReport(dis_body) => { (EventReport::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::Comment(dis_body) => { (Comment::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::ElectromagneticEmission(dis_body) => {
                encode_electromagnetic_emission_body_and_update_state(dis_body, state, options)
            }
            PduBody::Designator(dis_body) => {
                encode_designator_body_and_update_state(dis_body, state, options)
            }
            PduBody::Transmitter(dis_body) => {
                encode_transmitter_body_and_update_state(dis_body, state, options)
            }
            PduBody::Signal(dis_body) => { (Signal::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::Receiver(dis_body) => { (Receiver::encode(dis_body).into_cdis_body(), CodecStateResult::StateUnaffected) }
            PduBody::IFF(dis_body) => {
                encode_iff_body_and_update_state(dis_body, state, options)
            }
            PduBody::UnderwaterAcoustic(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::SupplementalEmissionEntityState(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::IntercomSignal => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::IntercomControl => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::AggregateState(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::IsGroupOf(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::TransferOwnership(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::IsPartOf(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::MinefieldState => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::MinefieldQuery => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::MinefieldData => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::MinefieldResponseNACK => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::EnvironmentalProcess => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::GriddedData => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::PointObjectState => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::LinearObjectState => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::ArealObjectState => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::TSPI => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::Appearance => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::ArticulatedParts => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::LEFire => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::LEDetonation => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::CreateEntityR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::RemoveEntityR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::StartResumeR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::StopFreezeR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::AcknowledgeR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::ActionRequestR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::ActionResponseR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::DataQueryR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::SetDataR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::DataR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::EventReportR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::CommentR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::RecordR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::SetRecordR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::RecordQueryR(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::CollisionElastic(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::EntityStateUpdate(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::DirectedEnergyFire => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::EntityDamageStatus => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::InformationOperationsAction => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::InformationOperationsReport => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
            PduBody::Attribute(_) => { (Self::Unsupported(Unsupported), CodecStateResult::StateUnaffected) }
        }
    }

    #[allow(clippy::wildcard_in_or_patterns)]
    pub fn decode(&self, state: &mut DecoderState, options: &CodecOptions) -> (PduBody, CodecStateResult) {
        match self {
            CdisBody::EntityState(cdis_body) => {
                decode_entity_state_body_and_update_state(cdis_body, state, options)
            }
            CdisBody::Fire(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::Detonation(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::Collision(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::CreateEntity(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::RemoveEntity(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::StartResume(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::StopFreeze(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::Acknowledge(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::ActionRequest(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::ActionResponse(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::DataQuery(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::SetData(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::Data(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::EventReport(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::Comment(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::ElectromagneticEmission(cdis_body) => {
                decode_electromagnetic_emission_body_and_update_state(cdis_body, state, options)
            }
            CdisBody::Designator(cdis_body) => {
                decode_designator_body_and_update_state(cdis_body, state, options)
            }
            CdisBody::Transmitter(cdis_body) => {
                decode_transmitter_body_and_update_state(cdis_body, state, options)
            }
            CdisBody::Signal(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::Receiver(cdis_body) => {
                (cdis_body.decode().into_pdu_body(), CodecStateResult::StateUnaffected)
            }
            CdisBody::Iff(cdis_body) => {
                decode_iff_body_and_update_state(cdis_body, state, options)
            }
            CdisBody::Unsupported(_) => {
                (PduBody::Other(dis_rs::other::model::Other::builder().build()), CodecStateResult::StateUnaffected)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::entity_state::model::{EntityMarking, EntityState};
    use dis_rs::enumerations::{Country, DeadReckoningAlgorithm, EntityKind, EntityMarkingCharacterSet, ForceId, PduType, PlatformDomain, ProtocolVersion};
    use dis_rs::model::{EntityId, EntityType, Pdu, PduBody, PduHeader, TimeStamp};
    use crate::{BodyProperties, CdisBody, CdisPdu};
    use crate::codec::{CodecOptions, CodecStateResult, DecoderState, EncoderState};
    use crate::entity_state::model::CdisEntityCapabilities;
    use crate::records::model::{CdisEntityMarking, CdisHeader, CdisProtocolVersion, LinearVelocity, Orientation, UnitsDekameters, WorldCoordinates};
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};

    #[test]
    fn cdis_pdu_entity_state_body_encode() {
        let mut encoder_state = EncoderState::new();
        let codec_option = CodecOptions::new_full_update();

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

        let (cdis_pdu, state_result) = CdisPdu::encode(&dis_pdu, &mut encoder_state, &codec_option);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

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
        let mut decoder_state = DecoderState::new();
        let codec_options = CodecOptions::new_full_update();

        let cdis_body = crate::EntityState {
            units: UnitsDekameters::Dekameter,
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

        let (dis, state_result) = cdis.decode(&mut decoder_state, &codec_options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

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