use crate::acknowledge_r::model::AcknowledgeR;
use crate::action_request_r::model::ActionRequestR;
use crate::action_response_r::model::ActionResponseR;
use crate::aggregate_state::model::AggregateState;
use crate::comment_r::model::CommentR;
use crate::common::acknowledge::model::Acknowledge;
use crate::common::action_request::model::ActionRequest;
use crate::common::action_response::model::ActionResponse;
use crate::common::attribute::model::Attribute;
use crate::common::collision::model::Collision;
use crate::common::collision_elastic::model::CollisionElastic;
use crate::common::comment::model::Comment;
use crate::common::create_entity::model::CreateEntity;
use crate::common::data::model::Data;
use crate::common::data_query::model::DataQuery;
use crate::common::designator::model::Designator;
use crate::common::detonation::model::Detonation;
use crate::common::electromagnetic_emission::model::ElectromagneticEmission;
use crate::common::entity_state::model::EntityState;
use crate::common::entity_state_update::model::EntityStateUpdate;
use crate::common::event_report::model::EventReport;
use crate::common::fire::model::Fire;
use crate::common::iff::model::Iff;
use crate::common::other::model::Other;
use crate::common::receiver::model::Receiver;
use crate::common::remove_entity::model::RemoveEntity;
use crate::common::set_data::model::SetData;
use crate::common::signal::model::Signal;
use crate::common::start_resume::model::StartResume;
use crate::common::stop_freeze::model::StopFreeze;
use crate::common::transmitter::model::Transmitter;
use crate::common::{BodyInfo, Interaction};
use crate::constants::{
    EIGHT_OCTETS, FIFTEEN_OCTETS, LEAST_SIGNIFICANT_BIT, NANOSECONDS_PER_TIME_UNIT, NO_REMAINDER,
    PDU_HEADER_LEN_BYTES, SIX_OCTETS,
};
use crate::create_entity_r::model::CreateEntityR;
use crate::data_query_r::model::DataQueryR;
use crate::data_r::model::DataR;
use crate::enumerations::{
    ArticulatedPartsTypeClass, ArticulatedPartsTypeMetric, AttachedPartDetachedIndicator,
    AttachedParts, ChangeIndicator, EntityAssociationAssociationStatus,
    EntityAssociationGroupMemberType, EntityAssociationPhysicalAssociationType,
    EntityAssociationPhysicalConnectionType, SeparationPreEntityIndicator,
    SeparationReasonForSeparation, StationName,
};
use crate::enumerations::{
    Country, EntityKind, ExplosiveMaterialCategories, MunitionDescriptorFuse,
    MunitionDescriptorWarhead, PduType, PlatformDomain, ProtocolFamily, ProtocolVersion,
    VariableRecordType,
};
use crate::event_report_r::model::EventReportR;
use crate::fixed_parameters::{NO_APPLIC, NO_ENTITY, NO_SITE};
use crate::is_group_of::model::IsGroupOf;
use crate::is_part_of::model::IsPartOf;
use crate::record_query_r::model::RecordQueryR;
use crate::record_r::model::RecordR;
use crate::remove_entity_r::model::RemoveEntityR;
use crate::repair_complete::model::RepairComplete;
use crate::repair_response::model::RepairResponse;
use crate::resupply_cancel::model::ResupplyCancel;
use crate::resupply_offer::model::ResupplyOffer;
use crate::resupply_received::model::ResupplyReceived;
use crate::sees::model::SEES;
use crate::service_request::model::ServiceRequest;
use crate::set_data_r::model::SetDataR;
use crate::set_record_r::model::SetRecordR;
use crate::start_resume_r::model::StartResumeR;
use crate::stop_freeze_r::model::StopFreezeR;
use crate::transfer_ownership::model::TransferOwnership;
use crate::underwater_acoustic::model::UnderwaterAcoustic;
use crate::DisError;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

pub use crate::v7::model::PduStatus;

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Pdu {
    pub header: PduHeader,
    pub body: PduBody,
}

impl Pdu {
    pub fn finalize_from_parts(
        header: PduHeader,
        body: PduBody,
        time_stamp: impl Into<TimeStamp>,
    ) -> Self {
        let time_stamp: TimeStamp = time_stamp.into();
        Self {
            header: header
                .with_pdu_type(body.body_type())
                .with_time_stamp(time_stamp.raw_timestamp)
                .with_length(body.body_length()),
            body,
        }
    }

    #[must_use]
    pub fn pdu_length(&self) -> u16 {
        PDU_HEADER_LEN_BYTES + self.body.body_length()
    }
}

impl Interaction for Pdu {
    fn originator(&self) -> Option<&EntityId> {
        self.body.originator()
    }

    fn receiver(&self) -> Option<&EntityId> {
        self.body.receiver()
    }
}

/// 6.2.66 PDU Header record
#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PduHeader {
    pub protocol_version: ProtocolVersion,
    pub exercise_id: u8,
    pub pdu_type: PduType,
    pub protocol_family: ProtocolFamily,
    pub time_stamp: u32,
    pub pdu_length: u16,
    pub pdu_status: Option<PduStatus>,
    pub padding: u16,
}

impl PduHeader {
    #[must_use]
    pub fn new(protocol_version: ProtocolVersion, exercise_id: u8, pdu_type: PduType) -> Self {
        let protocol_family = pdu_type.into();
        Self {
            protocol_version,
            exercise_id,
            pdu_type,
            protocol_family,
            time_stamp: 0u32,
            pdu_length: 0u16,
            pdu_status: None,
            padding: 0u16,
        }
    }

    #[must_use]
    pub fn new_v6(exercise_id: u8, pdu_type: PduType) -> Self {
        PduHeader::new(ProtocolVersion::IEEE1278_1A1998, exercise_id, pdu_type)
    }

    #[must_use]
    pub fn new_v7(exercise_id: u8, pdu_type: PduType) -> Self {
        PduHeader::new(ProtocolVersion::IEEE1278_12012, exercise_id, pdu_type)
    }

    #[must_use]
    pub fn with_pdu_type(mut self, pdu_type: PduType) -> Self {
        self.protocol_family = pdu_type.into();
        self.pdu_type = pdu_type;
        self
    }

    #[allow(clippy::return_self_not_must_use)]
    pub fn with_time_stamp(mut self, time_stamp: impl Into<u32>) -> Self {
        let time_stamp: u32 = time_stamp.into();
        self.time_stamp = time_stamp;
        self
    }

    #[must_use]
    pub fn with_length(mut self, body_length: u16) -> Self {
        self.pdu_length = PDU_HEADER_LEN_BYTES + body_length;
        self
    }

    #[must_use]
    pub fn with_pdu_status(mut self, pdu_status: PduStatus) -> Self {
        self.pdu_status = Some(pdu_status);
        self
    }
}

#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
#[cfg_attr(feature = "serde", serde(tag = "type"))]
pub enum PduBody {
    Other(Other),
    EntityState(EntityState),
    Fire(Fire),
    Detonation(Detonation),
    Collision(Collision),
    ServiceRequest(ServiceRequest),
    ResupplyOffer(ResupplyOffer),
    ResupplyReceived(ResupplyReceived),
    ResupplyCancel(ResupplyCancel),
    RepairComplete(RepairComplete),
    RepairResponse(RepairResponse),
    CreateEntity(CreateEntity),
    RemoveEntity(RemoveEntity),
    StartResume(StartResume),
    StopFreeze(StopFreeze),
    Acknowledge(Acknowledge),
    ActionRequest(ActionRequest),
    ActionResponse(ActionResponse),
    DataQuery(DataQuery),
    SetData(SetData),
    Data(Data),
    EventReport(EventReport),
    Comment(Comment),
    ElectromagneticEmission(ElectromagneticEmission),
    Designator(Designator),
    Transmitter(Transmitter),
    Signal(Signal),
    Receiver(Receiver),
    IFF(Iff),
    UnderwaterAcoustic(UnderwaterAcoustic),
    SupplementalEmissionEntityState(SEES),
    IntercomSignal,
    IntercomControl,
    AggregateState(AggregateState),
    IsGroupOf(IsGroupOf),
    TransferOwnership(TransferOwnership),
    IsPartOf(IsPartOf),
    MinefieldState,
    MinefieldQuery,
    MinefieldData,
    MinefieldResponseNACK,
    EnvironmentalProcess,
    GriddedData,
    PointObjectState,
    LinearObjectState,
    ArealObjectState,
    TSPI,
    Appearance,
    ArticulatedParts,
    LEFire,
    LEDetonation,
    CreateEntityR(CreateEntityR),
    RemoveEntityR(RemoveEntityR),
    StartResumeR(StartResumeR),
    StopFreezeR(StopFreezeR),
    AcknowledgeR(AcknowledgeR),
    ActionRequestR(ActionRequestR),
    ActionResponseR(ActionResponseR),
    DataQueryR(DataQueryR),
    SetDataR(SetDataR),
    DataR(DataR),
    EventReportR(EventReportR),
    CommentR(CommentR),
    RecordR(RecordR),
    SetRecordR(SetRecordR),
    RecordQueryR(RecordQueryR),
    CollisionElastic(CollisionElastic),
    EntityStateUpdate(EntityStateUpdate),
    DirectedEnergyFire,
    EntityDamageStatus,
    InformationOperationsAction,
    InformationOperationsReport,
    Attribute(Attribute),
}

impl BodyInfo for PduBody {
    #[allow(clippy::match_same_arms)]
    fn body_length(&self) -> u16 {
        match self {
            PduBody::Other(body) => body.body_length(),
            PduBody::EntityState(body) => body.body_length(),
            PduBody::Fire(body) => body.body_length(),
            PduBody::Detonation(body) => body.body_length(),
            PduBody::Collision(body) => body.body_length(),
            PduBody::ServiceRequest(body) => body.body_length(),
            PduBody::ResupplyOffer(body) => body.body_length(),
            PduBody::ResupplyReceived(body) => body.body_length(),
            PduBody::ResupplyCancel(body) => body.body_length(),
            PduBody::RepairComplete(body) => body.body_length(),
            PduBody::RepairResponse(body) => body.body_length(),
            PduBody::CreateEntity(body) => body.body_length(),
            PduBody::RemoveEntity(body) => body.body_length(),
            PduBody::StartResume(body) => body.body_length(),
            PduBody::StopFreeze(body) => body.body_length(),
            PduBody::Acknowledge(body) => body.body_length(),
            PduBody::ActionRequest(body) => body.body_length(),
            PduBody::ActionResponse(body) => body.body_length(),
            PduBody::DataQuery(body) => body.body_length(),
            PduBody::SetData(body) => body.body_length(),
            PduBody::Data(body) => body.body_length(),
            PduBody::EventReport(body) => body.body_length(),
            PduBody::Comment(body) => body.body_length(),
            PduBody::ElectromagneticEmission(body) => body.body_length(),
            PduBody::Designator(body) => body.body_length(),
            PduBody::Transmitter(body) => body.body_length(),
            PduBody::Signal(body) => body.body_length(),
            PduBody::Receiver(body) => body.body_length(),
            PduBody::IFF(body) => body.body_length(),
            PduBody::UnderwaterAcoustic(body) => body.body_length(),
            PduBody::SupplementalEmissionEntityState(body) => body.body_length(),
            PduBody::IntercomSignal => 0,
            PduBody::IntercomControl => 0,
            PduBody::AggregateState(body) => body.body_length(),
            PduBody::IsGroupOf(body) => body.body_length(),
            PduBody::TransferOwnership(body) => body.body_length(),
            PduBody::IsPartOf(body) => body.body_length(),
            PduBody::MinefieldState => 0,
            PduBody::MinefieldQuery => 0,
            PduBody::MinefieldData => 0,
            PduBody::MinefieldResponseNACK => 0,
            PduBody::EnvironmentalProcess => 0,
            PduBody::GriddedData => 0,
            PduBody::PointObjectState => 0,
            PduBody::LinearObjectState => 0,
            PduBody::ArealObjectState => 0,
            PduBody::TSPI => 0,
            PduBody::Appearance => 0,
            PduBody::ArticulatedParts => 0,
            PduBody::LEFire => 0,
            PduBody::LEDetonation => 0,
            PduBody::CreateEntityR(body) => body.body_length(),
            PduBody::RemoveEntityR(body) => body.body_length(),
            PduBody::StartResumeR(body) => body.body_length(),
            PduBody::StopFreezeR(body) => body.body_length(),
            PduBody::AcknowledgeR(body) => body.body_length(),
            PduBody::ActionRequestR(body) => body.body_length(),
            PduBody::ActionResponseR(body) => body.body_length(),
            PduBody::DataQueryR(body) => body.body_length(),
            PduBody::SetDataR(body) => body.body_length(),
            PduBody::DataR(body) => body.body_length(),
            PduBody::EventReportR(body) => body.body_length(),
            PduBody::CommentR(body) => body.body_length(),
            PduBody::RecordR(body) => body.body_length(),
            PduBody::SetRecordR(body) => body.body_length(),
            PduBody::RecordQueryR(body) => body.body_length(),
            PduBody::CollisionElastic(body) => body.body_length(),
            PduBody::EntityStateUpdate(body) => body.body_length(),
            PduBody::DirectedEnergyFire => 0,
            PduBody::EntityDamageStatus => 0,
            PduBody::InformationOperationsAction => 0,
            PduBody::InformationOperationsReport => 0,
            PduBody::Attribute(body) => body.body_length(),
        }
    }

    fn body_type(&self) -> PduType {
        match self {
            PduBody::Other(body) => body.body_type(),
            PduBody::EntityState(body) => body.body_type(),
            PduBody::Fire(body) => body.body_type(),
            PduBody::Detonation(body) => body.body_type(),
            PduBody::Collision(body) => body.body_type(),
            PduBody::ServiceRequest(body) => body.body_type(),
            PduBody::ResupplyOffer(body) => body.body_type(),
            PduBody::ResupplyReceived(body) => body.body_type(),
            PduBody::ResupplyCancel(body) => body.body_type(),
            PduBody::RepairComplete(body) => body.body_type(),
            PduBody::RepairResponse(body) => body.body_type(),
            PduBody::CreateEntity(body) => body.body_type(),
            PduBody::RemoveEntity(body) => body.body_type(),
            PduBody::StartResume(body) => body.body_type(),
            PduBody::StopFreeze(body) => body.body_type(),
            PduBody::Acknowledge(body) => body.body_type(),
            PduBody::ActionRequest(body) => body.body_type(),
            PduBody::ActionResponse(body) => body.body_type(),
            PduBody::DataQuery(body) => body.body_type(),
            PduBody::SetData(body) => body.body_type(),
            PduBody::Data(body) => body.body_type(),
            PduBody::EventReport(body) => body.body_type(),
            PduBody::Comment(body) => body.body_type(),
            PduBody::ElectromagneticEmission(body) => body.body_type(),
            PduBody::Designator(body) => body.body_type(),
            PduBody::Transmitter(body) => body.body_type(),
            PduBody::Signal(body) => body.body_type(),
            PduBody::Receiver(body) => body.body_type(),
            PduBody::IFF(body) => body.body_type(),
            PduBody::UnderwaterAcoustic(body) => body.body_type(),
            PduBody::SupplementalEmissionEntityState(body) => body.body_type(),
            PduBody::IntercomSignal => PduType::IntercomSignal,
            PduBody::IntercomControl => PduType::IntercomControl,
            PduBody::AggregateState(body) => body.body_type(),
            PduBody::IsGroupOf(body) => body.body_type(),
            PduBody::TransferOwnership(body) => body.body_type(),
            PduBody::IsPartOf(body) => body.body_type(),
            PduBody::MinefieldState => PduType::MinefieldState,
            PduBody::MinefieldQuery => PduType::MinefieldQuery,
            PduBody::MinefieldData => PduType::MinefieldData,
            PduBody::MinefieldResponseNACK => PduType::MinefieldResponseNACK,
            PduBody::EnvironmentalProcess => PduType::EnvironmentalProcess,
            PduBody::GriddedData => PduType::GriddedData,
            PduBody::PointObjectState => PduType::PointObjectState,
            PduBody::LinearObjectState => PduType::LinearObjectState,
            PduBody::ArealObjectState => PduType::ArealObjectState,
            PduBody::TSPI => PduType::TSPI,
            PduBody::Appearance => PduType::Appearance,
            PduBody::ArticulatedParts => PduType::ArticulatedParts,
            PduBody::LEFire => PduType::LEFire,
            PduBody::LEDetonation => PduType::LEDetonation,
            PduBody::CreateEntityR(body) => body.body_type(),
            PduBody::RemoveEntityR(body) => body.body_type(),
            PduBody::StartResumeR(body) => body.body_type(),
            PduBody::StopFreezeR(body) => body.body_type(),
            PduBody::AcknowledgeR(body) => body.body_type(),
            PduBody::ActionRequestR(body) => body.body_type(),
            PduBody::ActionResponseR(body) => body.body_type(),
            PduBody::DataQueryR(body) => body.body_type(),
            PduBody::SetDataR(body) => body.body_type(),
            PduBody::DataR(body) => body.body_type(),
            PduBody::EventReportR(body) => body.body_type(),
            PduBody::CommentR(body) => body.body_type(),
            PduBody::RecordR(body) => body.body_type(),
            PduBody::SetRecordR(body) => body.body_type(),
            PduBody::RecordQueryR(body) => body.body_type(),
            PduBody::CollisionElastic(body) => body.body_type(),
            PduBody::EntityStateUpdate(body) => body.body_type(),
            PduBody::DirectedEnergyFire => PduType::DirectedEnergyFire,
            PduBody::EntityDamageStatus => PduType::EntityDamageStatus,
            PduBody::InformationOperationsAction => PduType::InformationOperationsAction,
            PduBody::InformationOperationsReport => PduType::InformationOperationsReport,
            PduBody::Attribute(body) => body.body_type(),
        }
    }
}

impl Interaction for PduBody {
    #[allow(clippy::match_same_arms)]
    fn originator(&self) -> Option<&EntityId> {
        match self {
            PduBody::Other(body) => body.originator(),
            PduBody::EntityState(body) => body.originator(),
            PduBody::Fire(body) => body.originator(),
            PduBody::Detonation(body) => body.originator(),
            PduBody::Collision(body) => body.originator(),
            PduBody::ServiceRequest(body) => body.originator(),
            PduBody::ResupplyOffer(body) => body.originator(),
            PduBody::ResupplyReceived(body) => body.originator(),
            PduBody::ResupplyCancel(body) => body.originator(),
            PduBody::RepairComplete(body) => body.originator(),
            PduBody::RepairResponse(body) => body.originator(),
            PduBody::CreateEntity(body) => body.originator(),
            PduBody::RemoveEntity(body) => body.originator(),
            PduBody::StartResume(body) => body.originator(),
            PduBody::StopFreeze(body) => body.originator(),
            PduBody::Acknowledge(body) => body.originator(),
            PduBody::ActionRequest(body) => body.originator(),
            PduBody::ActionResponse(body) => body.originator(),
            PduBody::DataQuery(body) => body.originator(),
            PduBody::SetData(body) => body.originator(),
            PduBody::Data(body) => body.originator(),
            PduBody::EventReport(body) => body.originator(),
            PduBody::Comment(body) => body.originator(),
            PduBody::ElectromagneticEmission(body) => body.originator(),
            PduBody::Designator(body) => body.originator(),
            PduBody::Transmitter(body) => body.originator(),
            PduBody::Signal(body) => body.originator(),
            PduBody::Receiver(body) => body.originator(),
            PduBody::IFF(body) => body.originator(),
            PduBody::UnderwaterAcoustic(body) => body.originator(),
            PduBody::SupplementalEmissionEntityState(body) => body.originator(),
            PduBody::IntercomSignal => None,
            PduBody::IntercomControl => None,
            PduBody::AggregateState(body) => body.originator(),
            PduBody::IsGroupOf(body) => body.originator(),
            PduBody::TransferOwnership(body) => body.originator(),
            PduBody::IsPartOf(body) => body.originator(),
            PduBody::MinefieldState => None,
            PduBody::MinefieldQuery => None,
            PduBody::MinefieldData => None,
            PduBody::MinefieldResponseNACK => None,
            PduBody::EnvironmentalProcess => None,
            PduBody::GriddedData => None,
            PduBody::PointObjectState => None,
            PduBody::LinearObjectState => None,
            PduBody::ArealObjectState => None,
            PduBody::TSPI => None,
            PduBody::Appearance => None,
            PduBody::ArticulatedParts => None,
            PduBody::LEFire => None,
            PduBody::LEDetonation => None,
            PduBody::CreateEntityR(body) => body.originator(),
            PduBody::RemoveEntityR(body) => body.originator(),
            PduBody::StartResumeR(body) => body.originator(),
            PduBody::StopFreezeR(body) => body.originator(),
            PduBody::AcknowledgeR(body) => body.originator(),
            PduBody::ActionRequestR(body) => body.originator(),
            PduBody::ActionResponseR(body) => body.originator(),
            PduBody::DataQueryR(body) => body.originator(),
            PduBody::SetDataR(body) => body.originator(),
            PduBody::DataR(body) => body.originator(),
            PduBody::EventReportR(body) => body.originator(),
            PduBody::CommentR(body) => body.originator(),
            PduBody::RecordR(body) => body.originator(),
            PduBody::SetRecordR(body) => body.originator(),
            PduBody::RecordQueryR(body) => body.originator(),
            PduBody::CollisionElastic(body) => body.originator(),
            PduBody::EntityStateUpdate(body) => body.originator(),
            PduBody::DirectedEnergyFire => None,
            PduBody::EntityDamageStatus => None,
            PduBody::InformationOperationsAction => None,
            PduBody::InformationOperationsReport => None,
            PduBody::Attribute(body) => body.originator(),
        }
    }

    #[allow(clippy::match_same_arms)]
    fn receiver(&self) -> Option<&EntityId> {
        match self {
            PduBody::Other(body) => body.receiver(),
            PduBody::EntityState(body) => body.receiver(),
            PduBody::Fire(body) => body.receiver(),
            PduBody::Detonation(body) => body.receiver(),
            PduBody::Collision(body) => body.receiver(),
            PduBody::ServiceRequest(body) => body.receiver(),
            PduBody::ResupplyOffer(body) => body.receiver(),
            PduBody::ResupplyReceived(body) => body.receiver(),
            PduBody::ResupplyCancel(body) => body.receiver(),
            PduBody::RepairComplete(body) => body.receiver(),
            PduBody::RepairResponse(body) => body.receiver(),
            PduBody::CreateEntity(body) => body.receiver(),
            PduBody::RemoveEntity(body) => body.receiver(),
            PduBody::StartResume(body) => body.receiver(),
            PduBody::StopFreeze(body) => body.receiver(),
            PduBody::Acknowledge(body) => body.receiver(),
            PduBody::ActionRequest(body) => body.receiver(),
            PduBody::ActionResponse(body) => body.receiver(),
            PduBody::DataQuery(body) => body.receiver(),
            PduBody::SetData(body) => body.receiver(),
            PduBody::Data(body) => body.receiver(),
            PduBody::EventReport(body) => body.receiver(),
            PduBody::Comment(body) => body.receiver(),
            PduBody::ElectromagneticEmission(body) => body.receiver(),
            PduBody::Designator(body) => body.receiver(),
            PduBody::Transmitter(body) => body.receiver(),
            PduBody::Signal(body) => body.receiver(),
            PduBody::Receiver(body) => body.receiver(),
            PduBody::IFF(body) => body.receiver(),
            PduBody::UnderwaterAcoustic(body) => body.receiver(),
            PduBody::SupplementalEmissionEntityState(body) => body.receiver(),
            PduBody::IntercomSignal => None,
            PduBody::IntercomControl => None,
            PduBody::AggregateState(body) => body.receiver(),
            PduBody::IsGroupOf(body) => body.receiver(),
            PduBody::TransferOwnership(body) => body.receiver(),
            PduBody::IsPartOf(body) => body.receiver(),
            PduBody::MinefieldState => None,
            PduBody::MinefieldQuery => None,
            PduBody::MinefieldData => None,
            PduBody::MinefieldResponseNACK => None,
            PduBody::EnvironmentalProcess => None,
            PduBody::GriddedData => None,
            PduBody::PointObjectState => None,
            PduBody::LinearObjectState => None,
            PduBody::ArealObjectState => None,
            PduBody::TSPI => None,
            PduBody::Appearance => None,
            PduBody::ArticulatedParts => None,
            PduBody::LEFire => None,
            PduBody::LEDetonation => None,
            PduBody::CreateEntityR(body) => body.receiver(),
            PduBody::RemoveEntityR(body) => body.receiver(),
            PduBody::StartResumeR(body) => body.receiver(),
            PduBody::StopFreezeR(body) => body.receiver(),
            PduBody::AcknowledgeR(body) => body.receiver(),
            PduBody::ActionRequestR(body) => body.receiver(),
            PduBody::ActionResponseR(body) => body.receiver(),
            PduBody::DataQueryR(body) => body.receiver(),
            PduBody::SetDataR(body) => body.receiver(),
            PduBody::DataR(body) => body.receiver(),
            PduBody::EventReportR(body) => body.receiver(),
            PduBody::CommentR(body) => body.receiver(),
            PduBody::RecordR(body) => body.receiver(),
            PduBody::SetRecordR(body) => body.receiver(),
            PduBody::RecordQueryR(body) => body.receiver(),
            PduBody::CollisionElastic(body) => body.receiver(),
            PduBody::EntityStateUpdate(body) => body.receiver(),
            PduBody::DirectedEnergyFire => None,
            PduBody::EntityDamageStatus => None,
            PduBody::InformationOperationsAction => None,
            PduBody::InformationOperationsReport => None,
            PduBody::Attribute(body) => body.receiver(),
        }
    }
}

impl From<PduType> for ProtocolFamily {
    #[allow(clippy::match_same_arms)]
    fn from(pdu_type: PduType) -> Self {
        match pdu_type {
            PduType::Other => ProtocolFamily::Other,
            PduType::EntityState => ProtocolFamily::EntityInformationInteraction,
            PduType::Fire => ProtocolFamily::Warfare,
            PduType::Detonation => ProtocolFamily::Warfare,
            PduType::Collision => ProtocolFamily::EntityInformationInteraction,
            PduType::ServiceRequest => ProtocolFamily::Logistics,
            PduType::ResupplyOffer => ProtocolFamily::Logistics,
            PduType::ResupplyReceived => ProtocolFamily::Logistics,
            PduType::ResupplyCancel => ProtocolFamily::Logistics,
            PduType::RepairComplete => ProtocolFamily::Logistics,
            PduType::RepairResponse => ProtocolFamily::Logistics,
            PduType::CreateEntity => ProtocolFamily::SimulationManagement,
            PduType::RemoveEntity => ProtocolFamily::SimulationManagement,
            PduType::StartResume => ProtocolFamily::SimulationManagement,
            PduType::StopFreeze => ProtocolFamily::SimulationManagement,
            PduType::Acknowledge => ProtocolFamily::SimulationManagement,
            PduType::ActionRequest => ProtocolFamily::SimulationManagement,
            PduType::ActionResponse => ProtocolFamily::SimulationManagement,
            PduType::DataQuery => ProtocolFamily::SimulationManagement,
            PduType::SetData => ProtocolFamily::SimulationManagement,
            PduType::Data => ProtocolFamily::SimulationManagement,
            PduType::EventReport => ProtocolFamily::SimulationManagement,
            PduType::Comment => ProtocolFamily::SimulationManagement,
            PduType::ElectromagneticEmission => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::Designator => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::Transmitter => ProtocolFamily::RadioCommunications,
            PduType::Signal => ProtocolFamily::RadioCommunications,
            PduType::Receiver => ProtocolFamily::RadioCommunications,
            PduType::IFF => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::UnderwaterAcoustic => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::SupplementalEmissionEntityState => {
                ProtocolFamily::DistributedEmissionRegeneration
            }
            PduType::IntercomSignal => ProtocolFamily::RadioCommunications,
            PduType::IntercomControl => ProtocolFamily::RadioCommunications,
            PduType::AggregateState => ProtocolFamily::EntityManagement,
            PduType::IsGroupOf => ProtocolFamily::EntityManagement,
            PduType::TransferOwnership => ProtocolFamily::EntityManagement,
            PduType::IsPartOf => ProtocolFamily::EntityManagement,
            PduType::MinefieldState => ProtocolFamily::Minefield,
            PduType::MinefieldQuery => ProtocolFamily::Minefield,
            PduType::MinefieldData => ProtocolFamily::Minefield,
            PduType::MinefieldResponseNACK => ProtocolFamily::Minefield,
            PduType::EnvironmentalProcess => ProtocolFamily::SyntheticEnvironment,
            PduType::GriddedData => ProtocolFamily::SyntheticEnvironment,
            PduType::PointObjectState => ProtocolFamily::SyntheticEnvironment,
            PduType::LinearObjectState => ProtocolFamily::SyntheticEnvironment,
            PduType::ArealObjectState => ProtocolFamily::SyntheticEnvironment,
            PduType::TSPI => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::Appearance => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::ArticulatedParts => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::LEFire => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::LEDetonation => ProtocolFamily::LiveEntity_LE_InformationInteraction,
            PduType::CreateEntityR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::RemoveEntityR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::StartResumeR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::StopFreezeR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::AcknowledgeR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::ActionRequestR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::ActionResponseR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::DataQueryR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::SetDataR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::DataR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::EventReportR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::CommentR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::RecordR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::SetRecordR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::RecordQueryR => ProtocolFamily::SimulationManagementWithReliability,
            PduType::CollisionElastic => ProtocolFamily::EntityInformationInteraction,
            PduType::EntityStateUpdate => ProtocolFamily::EntityInformationInteraction,
            PduType::DirectedEnergyFire => ProtocolFamily::Warfare,
            PduType::EntityDamageStatus => ProtocolFamily::Warfare,
            PduType::InformationOperationsAction => ProtocolFamily::InformationOperations,
            PduType::InformationOperationsReport => ProtocolFamily::InformationOperations,
            PduType::Attribute => ProtocolFamily::EntityInformationInteraction,
            PduType::Unspecified(unspecified_value) => {
                ProtocolFamily::Unspecified(unspecified_value)
            }
        }
    }
}

/// 6.2.80 Simulation Address record
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SimulationAddress {
    pub site_id: u16,
    pub application_id: u16,
}

impl SimulationAddress {
    #[must_use]
    pub fn new(site_id: u16, application_id: u16) -> Self {
        SimulationAddress {
            site_id,
            application_id,
        }
    }
}

impl Default for SimulationAddress {
    fn default() -> Self {
        Self {
            site_id: NO_SITE,
            application_id: NO_APPLIC,
        }
    }
}

impl Display for SimulationAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.site_id, self.application_id)
    }
}

#[allow(clippy::get_first)]
impl TryFrom<&[&str]> for SimulationAddress {
    type Error = DisError;

    fn try_from(value: &[&str]) -> Result<Self, Self::Error> {
        const NUM_DIGITS: usize = 2;
        if value.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "SimulationAddress string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        Ok(Self {
            site_id: value
                .get(0)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u16>()
                .map_err(|_| DisError::ParseError("Invalid site id digit".to_string()))?,
            application_id: value
                .get(1)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u16>()
                .map_err(|_| DisError::ParseError("Invalid application id digit".to_string()))?,
        })
    }
}

impl FromStr for SimulationAddress {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ss = s.split(':').collect::<Vec<&str>>();
        Self::try_from(ss.as_slice())
    }
}

impl TryFrom<&str> for SimulationAddress {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SimulationAddress::from_str(value)
    }
}

impl TryFrom<String> for SimulationAddress {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

/// 6.2.28 Entity Identifier record
/// 6.2.81 Simulation Identifier record
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityId {
    pub simulation_address: SimulationAddress,
    pub entity_id: u16,
}

impl EntityId {
    #[must_use]
    pub fn new(site_id: u16, application_id: u16, entity_id: u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_id,
                application_id,
            },
            entity_id,
        }
    }

    #[must_use]
    pub fn new_sim_address(simulation_address: SimulationAddress, entity_id: u16) -> Self {
        Self {
            simulation_address,
            entity_id,
        }
    }

    #[must_use]
    pub fn new_simulation_identifier(simulation_address: SimulationAddress) -> Self {
        Self {
            simulation_address,
            entity_id: NO_ENTITY,
        }
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn record_length(&self) -> u16 {
        SIX_OCTETS as u16
    }
}

impl Default for EntityId {
    fn default() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            entity_id: NO_ENTITY,
        }
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.simulation_address, self.entity_id)
    }
}

#[allow(clippy::get_first)]
impl FromStr for EntityId {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 3;
        let mut ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "EntityId string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        let entity_id = ss
            .pop()
            .expect("Impossible - checked for correct number of digits")
            .parse::<u16>()
            .map_err(|_| DisError::ParseError("Invalid entity id digit".to_string()))?;
        Ok(Self {
            simulation_address: ss.as_slice().try_into()?,
            entity_id,
        })
    }
}

impl TryFrom<&str> for EntityId {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EntityId::from_str(value)
    }
}

impl TryFrom<String> for EntityId {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EventId {
    pub simulation_address: SimulationAddress,
    pub event_id: u16,
}

impl EventId {
    #[must_use]
    pub fn new(site_id: u16, application_id: u16, event_id: u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_id,
                application_id,
            },
            event_id,
        }
    }

    #[must_use]
    pub fn new_sim_address(simulation_address: SimulationAddress, event_id: u16) -> Self {
        Self {
            simulation_address,
            event_id,
        }
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            event_id: NO_ENTITY,
        }
    }
}

impl Display for EventId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", self.simulation_address, self.event_id)
    }
}

#[allow(clippy::get_first)]
impl FromStr for EventId {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 3;
        let mut ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "EventId string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        let event_id = ss
            .pop()
            .expect("Impossible - checked for correct number of digits")
            .parse::<u16>()
            .map_err(|_| DisError::ParseError("Invalid event id digit".to_string()))?;
        Ok(Self {
            simulation_address: ss.as_slice().try_into()?,
            event_id,
        })
    }
}

impl TryFrom<&str> for EventId {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EventId::from_str(value)
    }
}

impl TryFrom<String> for EventId {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

/// 6.2.96 Vector record
/// 6.2.7 Angular Velocity Vector record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VectorF32 {
    pub first_vector_component: f32,
    pub second_vector_component: f32,
    pub third_vector_component: f32,
}

impl VectorF32 {
    #[must_use]
    pub fn new(first: f32, second: f32, third: f32) -> Self {
        VectorF32 {
            first_vector_component: first,
            second_vector_component: second,
            third_vector_component: third,
        }
    }

    #[must_use]
    pub fn with_first(mut self, first: f32) -> Self {
        self.first_vector_component = first;
        self
    }

    #[must_use]
    pub fn with_second(mut self, second: f32) -> Self {
        self.second_vector_component = second;
        self
    }

    #[must_use]
    pub fn with_third(mut self, third: f32) -> Self {
        self.third_vector_component = third;
        self
    }
}

// TODO rename Location to World Coordinate
/// 6.2.98 World Coordinates record
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Location {
    pub x_coordinate: f64,
    pub y_coordinate: f64,
    pub z_coordinate: f64,
}

impl Location {
    #[must_use]
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Location {
            x_coordinate: x,
            y_coordinate: y,
            z_coordinate: z,
        }
    }

    #[must_use]
    pub fn with_x(mut self, x: f64) -> Self {
        self.x_coordinate = x;
        self
    }

    #[must_use]
    pub fn with_y(mut self, y: f64) -> Self {
        self.y_coordinate = y;
        self
    }

    #[must_use]
    pub fn with_z(mut self, z: f64) -> Self {
        self.z_coordinate = z;
        self
    }
}

// TODO rename Orientation to EulerAngle
/// 6.2.32 Euler Angles record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Orientation {
    pub psi: f32,
    pub theta: f32,
    pub phi: f32,
}

impl Orientation {
    #[must_use]
    #[allow(clippy::similar_names)]
    pub fn new(psi: f32, theta: f32, phi: f32) -> Self {
        Orientation { psi, theta, phi }
    }

    #[must_use]
    pub fn with_psi(mut self, psi: f32) -> Self {
        self.psi = psi;
        self
    }

    #[must_use]
    pub fn with_theta(mut self, theta: f32) -> Self {
        self.theta = theta;
        self
    }

    #[must_use]
    pub fn with_phi(mut self, phi: f32) -> Self {
        self.phi = phi;
        self
    }
}

/// 6.2.30 Entity Type record
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityType {
    pub kind: EntityKind,
    pub domain: PlatformDomain,
    pub country: Country,
    pub category: u8,
    pub subcategory: u8,
    pub specific: u8,
    pub extra: u8,
}

impl EntityType {
    #[must_use]
    pub fn with_kind(mut self, kind: EntityKind) -> Self {
        self.kind = kind;
        self
    }

    #[must_use]
    pub fn with_domain(mut self, domain: PlatformDomain) -> Self {
        self.domain = domain;
        self
    }

    #[must_use]
    pub fn with_country(mut self, country: Country) -> Self {
        self.country = country;
        self
    }

    #[must_use]
    pub fn with_category(mut self, category: u8) -> Self {
        self.category = category;
        self
    }

    #[must_use]
    pub fn with_subcategory(mut self, subcategory: u8) -> Self {
        self.subcategory = subcategory;
        self
    }

    #[must_use]
    pub fn with_specific(mut self, specific: u8) -> Self {
        self.specific = specific;
        self
    }

    #[must_use]
    pub fn with_extra(mut self, extra: u8) -> Self {
        self.extra = extra;
        self
    }

    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn record_length(&self) -> u16 {
        EIGHT_OCTETS as u16
    }
}

impl Display for EntityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}:{}:{}:{}:{}:{}:{}",
            u8::from(self.kind),
            u8::from(self.domain),
            u16::from(self.country),
            self.category,
            self.subcategory,
            self.specific,
            self.extra
        )
    }
}

#[allow(clippy::get_first)]
impl FromStr for EntityType {
    type Err = DisError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const NUM_DIGITS: usize = 7;
        let ss = s.split(':').collect::<Vec<&str>>();
        if ss.len() != NUM_DIGITS {
            return Err(DisError::ParseError(format!(
                "EntityType string pattern does not contain precisely {NUM_DIGITS} digits"
            )));
        }
        Ok(Self {
            kind: ss
                .get(0)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid kind digit".to_string()))?
                .into(),
            domain: ss
                .get(1)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid domain digit".to_string()))?
                .into(),
            country: ss
                .get(2)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u16>()
                .map_err(|_| DisError::ParseError("Invalid country digit".to_string()))?
                .into(),
            category: ss
                .get(3)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid category digit".to_string()))?,
            subcategory: ss
                .get(4)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid subcategory digit".to_string()))?,
            specific: ss
                .get(5)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid specific digit".to_string()))?,
            extra: ss
                .get(6)
                .expect("Impossible - checked for correct number of digits")
                .parse::<u8>()
                .map_err(|_| DisError::ParseError("Invalid extra digit".to_string()))?,
        })
    }
}

impl TryFrom<&str> for EntityType {
    type Error = DisError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        EntityType::from_str(value)
    }
}

impl TryFrom<String> for EntityType {
    type Error = DisError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        TryFrom::<&str>::try_from(&value)
    }
}

/// 6.2.19 Descriptor records
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum DescriptorRecord {
    /// 6.2.19.2 Munition Descriptor record
    #[cfg_attr(feature = "serde", serde(rename = "munition"))]
    Munition {
        entity_type: EntityType,
        munition: MunitionDescriptor,
    },
    /// 6.2.19.4 Expendable Descriptor record
    #[cfg_attr(feature = "serde", serde(rename = "expendable"))]
    Expendable { entity_type: EntityType },
    /// 6.2.19.3 Explosion Descriptor record
    #[cfg_attr(feature = "serde", serde(rename = "explosion"))]
    Explosion {
        entity_type: EntityType,
        explosive_material: ExplosiveMaterialCategories,
        explosive_force: f32,
    },
}

impl DescriptorRecord {
    #[must_use]
    pub fn new_munition(entity_type: EntityType, munition: MunitionDescriptor) -> Self {
        DescriptorRecord::Munition {
            entity_type,
            munition,
        }
    }

    #[must_use]
    pub fn new_expendable(entity_type: EntityType) -> Self {
        DescriptorRecord::Expendable { entity_type }
    }

    #[must_use]
    pub fn new_explosion(
        entity_type: EntityType,
        explosive_material: ExplosiveMaterialCategories,
        explosive_force: f32,
    ) -> Self {
        DescriptorRecord::Explosion {
            entity_type,
            explosive_material,
            explosive_force,
        }
    }
}

impl Default for DescriptorRecord {
    fn default() -> Self {
        DescriptorRecord::new_munition(EntityType::default(), MunitionDescriptor::default())
    }
}

/// 6.2.19.2 Munition Descriptor record
#[derive(Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MunitionDescriptor {
    pub warhead: MunitionDescriptorWarhead,
    pub fuse: MunitionDescriptorFuse,
    pub quantity: u16,
    pub rate: u16,
}

impl MunitionDescriptor {
    #[must_use]
    pub fn with_warhead(mut self, warhead: MunitionDescriptorWarhead) -> Self {
        self.warhead = warhead;
        self
    }

    #[must_use]
    pub fn with_fuse(mut self, fuse: MunitionDescriptorFuse) -> Self {
        self.fuse = fuse;
        self
    }

    #[must_use]
    pub fn with_quantity(mut self, quantity: u16) -> Self {
        self.quantity = quantity;
        self
    }

    #[must_use]
    pub fn with_rate(mut self, rate: u16) -> Self {
        self.rate = rate;
        self
    }
}

/// Custom type to model timestamps, just wrapping a `u32` value. By default,
/// the `PduHeader` uses this type. Users can decide to convert the raw value
/// to a `DisTimeStamp`, which models the Absolute and Relative interpretations of the value as defined by the standard.
///
/// The standard defines the value to be a number of DIS time units since the top of the hour.
/// There are 2^31 - 1 time units in an hour.
/// This results in each time unit representing exactly 3600/(2^31) seconds (approximately 1.67638063 μs).
///
/// This raw timestamp could also be interpreted as a Unix timestamp, or something else
/// like a monotonically increasing timestamp. This is left up to the client applications of the protocol _by this library_.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TimeStamp {
    pub raw_timestamp: u32,
}

impl TimeStamp {
    #[must_use]
    pub fn new(raw_timestamp: u32) -> Self {
        Self { raw_timestamp }
    }
}

impl From<u32> for TimeStamp {
    fn from(value: u32) -> Self {
        Self {
            raw_timestamp: value,
        }
    }
}

impl From<TimeStamp> for u32 {
    fn from(value: TimeStamp) -> Self {
        value.raw_timestamp
    }
}

impl From<DisTimeStamp> for TimeStamp {
    fn from(value: DisTimeStamp) -> Self {
        let raw_timestamp = match value {
            DisTimeStamp::Absolute {
                units_past_the_hour,
                nanoseconds_past_the_hour: _,
            } => (units_past_the_hour << 1) | LEAST_SIGNIFICANT_BIT,
            DisTimeStamp::Relative {
                units_past_the_hour,
                nanoseconds_past_the_hour: _,
            } => units_past_the_hour << 1,
        };

        Self { raw_timestamp }
    }
}

/// A timestamp type that models the timestamp mechanism as described in the
/// DIS standard (section 6.2.88 Timestamp). This timestamp interprets an u32 value
/// as an Absolute or a Relative timestamp based on the Least Significant Bit.
/// The remaining (upper) bits represent the units of time passed since the
/// beginning of the current hour in the selected time reference.
/// The `DisTimeStamp` stores both the units past the hour, and a conversion to
/// nanoseconds past the hour.
#[derive(Debug)]
pub enum DisTimeStamp {
    Absolute {
        units_past_the_hour: u32,
        nanoseconds_past_the_hour: u32,
    },
    Relative {
        units_past_the_hour: u32,
        nanoseconds_past_the_hour: u32,
    },
}

impl DisTimeStamp {
    #[must_use]
    pub fn new_absolute_from_secs(seconds_past_the_hour: u32) -> Self {
        let nanoseconds_past_the_hour = DisTimeStamp::seconds_to_nanoseconds(seconds_past_the_hour);
        let units_past_the_hour =
            DisTimeStamp::nanoseconds_to_dis_time_units(nanoseconds_past_the_hour);
        Self::Absolute {
            units_past_the_hour,
            nanoseconds_past_the_hour,
        }
    }

    #[must_use]
    pub fn new_relative_from_secs(seconds_past_the_hour: u32) -> Self {
        let nanoseconds_past_the_hour = DisTimeStamp::seconds_to_nanoseconds(seconds_past_the_hour);
        let units_past_the_hour =
            DisTimeStamp::nanoseconds_to_dis_time_units(nanoseconds_past_the_hour);
        Self::Relative {
            units_past_the_hour,
            nanoseconds_past_the_hour,
        }
    }

    #[must_use]
    pub fn new_absolute_from_units(units_past_the_hour: u32) -> Self {
        Self::Absolute {
            units_past_the_hour,
            nanoseconds_past_the_hour: Self::dis_time_units_to_nanoseconds(units_past_the_hour),
        }
    }

    #[must_use]
    pub fn new_relative_from_units(units_past_the_hour: u32) -> Self {
        Self::Relative {
            units_past_the_hour,
            nanoseconds_past_the_hour: Self::dis_time_units_to_nanoseconds(units_past_the_hour),
        }
    }

    /// Helper function to convert seconds to nanoseconds
    fn seconds_to_nanoseconds(seconds: u32) -> u32 {
        seconds * 1_000_000
    }

    /// Helper function to convert nanoseconds pas the hour to DIS Time Units past the hour.
    #[allow(clippy::cast_possible_truncation)]
    fn nanoseconds_to_dis_time_units(nanoseconds_past_the_hour: u32) -> u32 {
        (nanoseconds_past_the_hour as f32 / NANOSECONDS_PER_TIME_UNIT) as u32
    }

    #[allow(clippy::cast_possible_truncation)]
    fn dis_time_units_to_nanoseconds(dis_time_units: u32) -> u32 {
        (dis_time_units as f32 * NANOSECONDS_PER_TIME_UNIT) as u32
    }
}

impl From<u32> for DisTimeStamp {
    fn from(value: u32) -> Self {
        let absolute_bit = (value & LEAST_SIGNIFICANT_BIT) == LEAST_SIGNIFICANT_BIT;
        let units_past_the_hour = value >> 1;
        let nanoseconds_past_the_hour =
            (units_past_the_hour as f32 * NANOSECONDS_PER_TIME_UNIT) as u32;

        if absolute_bit {
            Self::Absolute {
                units_past_the_hour,
                nanoseconds_past_the_hour,
            }
        } else {
            Self::Relative {
                units_past_the_hour,
                nanoseconds_past_the_hour,
            }
        }
    }
}

impl From<TimeStamp> for DisTimeStamp {
    fn from(value: TimeStamp) -> Self {
        DisTimeStamp::from(value.raw_timestamp)
    }
}

impl From<DisTimeStamp> for u32 {
    fn from(value: DisTimeStamp) -> Self {
        TimeStamp::from(value).raw_timestamp
    }
}

/// 6.2.14 Clock Time record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ClockTime {
    pub hour: i32,
    pub time_past_hour: u32,
}

impl ClockTime {
    #[must_use]
    pub fn new(hour: i32, time_past_hour: u32) -> Self {
        Self {
            hour,
            time_past_hour,
        }
    }
}

/// 6.2.18 Datum Specification record
#[derive(Clone, Default, Debug, PartialEq)]
pub struct DatumSpecification {
    pub fixed_datum_records: Vec<FixedDatum>,
    pub variable_datum_records: Vec<VariableDatum>,
}

impl DatumSpecification {
    #[must_use]
    pub fn new(
        fixed_datum_records: Vec<FixedDatum>,
        variable_datum_records: Vec<VariableDatum>,
    ) -> Self {
        Self {
            fixed_datum_records,
            variable_datum_records,
        }
    }
}

pub const FIXED_DATUM_LENGTH: u16 = 8;
pub const BASE_VARIABLE_DATUM_LENGTH: u16 = 8;

/// 6.2.37 Fixed Datum record
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct FixedDatum {
    pub datum_id: VariableRecordType,
    pub datum_value: u32,
}

impl FixedDatum {
    #[must_use]
    pub fn new(datum_id: VariableRecordType, datum_value: u32) -> Self {
        Self {
            datum_id,
            datum_value,
        }
    }
}

/// 6.2.93 Variable Datum record
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct VariableDatum {
    pub datum_id: VariableRecordType,
    pub datum_value: Vec<u8>,
}

impl VariableDatum {
    #[must_use]
    pub fn new(datum_id: VariableRecordType, datum_value: Vec<u8>) -> Self {
        Self {
            datum_id,
            datum_value,
        }
    }
}

/// Struct to hold the length (in bits or bytes) of parts of a padded record.
/// Such that `data_length` + `padding_length` = `record_length`.
#[derive(Debug)]
pub struct PaddedRecordLengths {
    pub data_length: usize,
    pub padding_length: usize,
    pub record_length: usize,
}

impl PaddedRecordLengths {
    #[must_use]
    pub fn new(
        data_length_bytes: usize,
        padding_length_bytes: usize,
        record_length_bytes: usize,
    ) -> Self {
        Self {
            data_length: data_length_bytes,
            padding_length: padding_length_bytes,
            record_length: record_length_bytes,
        }
    }
}

/// Calculates the length of a data record when padded to `pad_to_num` octets or bits,
/// given that the length of the data in the record is `data_length`.
/// The function returns a tuple consisting of the length of the data, the length of the padding, and the total (padded) length of the record.
///
/// For example, a piece of data of 12 bytes that needs to be aligned to 16 bytes will have a
/// data length of 12 bytes, a padding of 4 bytes and a final length of 12 + 4 bytes. The function will return 16 in this case.
pub(crate) fn length_padded_to_num(data_length: usize, pad_to_num: usize) -> PaddedRecordLengths {
    let data_remaining = data_length % pad_to_num;
    let padding_num = if data_remaining == 0 {
        0usize
    } else {
        pad_to_num - data_remaining
    };
    let record_length = data_length + padding_num;
    assert_eq!(
        record_length % pad_to_num,
        NO_REMAINDER,
        "The length for the data record is not aligned to {pad_to_num} octets. Data length is {data_length} octets."
    );

    PaddedRecordLengths::new(data_length, padding_num, record_length)
}

/// 6.2.94 Variable Parameter record
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum VariableParameter {
    Articulated(ArticulatedPart),
    Attached(AttachedPart),
    Separation(SeparationParameter),
    EntityType(EntityTypeParameter),
    EntityAssociation(EntityAssociationParameter),
    Unspecified(u8, [u8; FIFTEEN_OCTETS]),
}

/// 6.2.94.2 Articulated Part VP record
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ArticulatedPart {
    pub change_indicator: ChangeIndicator,
    pub attachment_id: u16,
    pub type_metric: ArticulatedPartsTypeMetric,
    pub type_class: ArticulatedPartsTypeClass,
    pub parameter_value: f32,
}

impl ArticulatedPart {
    #[must_use]
    pub fn with_change_indicator(mut self, change_indicator: ChangeIndicator) -> Self {
        self.change_indicator = change_indicator;
        self
    }

    #[must_use]
    pub fn with_attachment_id(mut self, attachment_id: u16) -> Self {
        self.attachment_id = attachment_id;
        self
    }

    #[must_use]
    pub fn with_type_metric(mut self, type_metric: ArticulatedPartsTypeMetric) -> Self {
        self.type_metric = type_metric;
        self
    }

    #[must_use]
    pub fn with_type_class(mut self, type_class: ArticulatedPartsTypeClass) -> Self {
        self.type_class = type_class;
        self
    }

    #[must_use]
    pub fn with_parameter_value(mut self, parameter_value: f32) -> Self {
        self.parameter_value = parameter_value;
        self
    }

    #[must_use]
    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::Articulated(self)
    }
}

/// 6.2.94.3 Attached Part VP record
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct AttachedPart {
    pub detached_indicator: AttachedPartDetachedIndicator,
    pub attachment_id: u16,
    pub parameter_type: AttachedParts,
    pub attached_part_type: EntityType,
}

impl AttachedPart {
    #[must_use]
    pub fn with_detached_indicator(
        mut self,
        detached_indicator: AttachedPartDetachedIndicator,
    ) -> Self {
        self.detached_indicator = detached_indicator;
        self
    }

    #[must_use]
    pub fn with_attachment_id(mut self, attachment_id: u16) -> Self {
        self.attachment_id = attachment_id;
        self
    }

    #[must_use]
    pub fn with_parameter_type(mut self, parameter_type: AttachedParts) -> Self {
        self.parameter_type = parameter_type;
        self
    }

    #[must_use]
    pub fn with_attached_part_type(mut self, attached_part_type: EntityType) -> Self {
        self.attached_part_type = attached_part_type;
        self
    }

    #[must_use]
    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::Attached(self)
    }
}

/// 6.2.94.6 Separation VP record
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SeparationParameter {
    pub reason: SeparationReasonForSeparation,
    pub pre_entity_indicator: SeparationPreEntityIndicator,
    pub parent_entity_id: EntityId,
    pub station_name: StationName,
    pub station_number: u16,
}

impl SeparationParameter {
    #[must_use]
    pub fn with_reason(mut self, reason: SeparationReasonForSeparation) -> Self {
        self.reason = reason;
        self
    }

    #[must_use]
    pub fn with_pre_entity_indicator(
        mut self,
        pre_entity_indicator: SeparationPreEntityIndicator,
    ) -> Self {
        self.pre_entity_indicator = pre_entity_indicator;
        self
    }

    #[must_use]
    pub fn with_parent_entity_id(mut self, parent_entity_id: EntityId) -> Self {
        self.parent_entity_id = parent_entity_id;
        self
    }

    #[must_use]
    pub fn with_station_name(mut self, station_name: StationName) -> Self {
        self.station_name = station_name;
        self
    }

    #[must_use]
    pub fn with_station_number(mut self, station_number: u16) -> Self {
        self.station_number = station_number;
        self
    }

    #[must_use]
    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::Separation(self)
    }
}

/// 6.2.94.5 Entity Type VP record
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityTypeParameter {
    pub change_indicator: ChangeIndicator,
    pub entity_type: EntityType,
}

impl EntityTypeParameter {
    #[must_use]
    pub fn with_change_indicator(mut self, change_indicator: ChangeIndicator) -> Self {
        self.change_indicator = change_indicator;
        self
    }

    #[must_use]
    pub fn with_entity_type(mut self, entity_type: EntityType) -> Self {
        self.entity_type = entity_type;
        self
    }

    #[must_use]
    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::EntityType(self)
    }
}

/// 6.2.94.4 Entity Association VP Record
#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct EntityAssociationParameter {
    pub change_indicator: ChangeIndicator,
    pub association_status: EntityAssociationAssociationStatus,
    pub association_type: EntityAssociationPhysicalAssociationType,
    pub entity_id: EntityId,
    pub own_station_location: StationName,
    pub physical_connection_type: EntityAssociationPhysicalConnectionType,
    pub group_member_type: EntityAssociationGroupMemberType,
    pub group_number: u16,
}

impl EntityAssociationParameter {
    #[must_use]
    pub fn with_change_indicator(mut self, change_indicator: ChangeIndicator) -> Self {
        self.change_indicator = change_indicator;
        self
    }

    #[must_use]
    pub fn with_association_status(
        mut self,
        association_status: EntityAssociationAssociationStatus,
    ) -> Self {
        self.association_status = association_status;
        self
    }

    #[must_use]
    pub fn with_association_type(
        mut self,
        association_type: EntityAssociationPhysicalAssociationType,
    ) -> Self {
        self.association_type = association_type;
        self
    }

    #[must_use]
    pub fn with_entity_id(mut self, entity_id: EntityId) -> Self {
        self.entity_id = entity_id;
        self
    }

    #[must_use]
    pub fn with_own_station_location(mut self, own_station_location: StationName) -> Self {
        self.own_station_location = own_station_location;
        self
    }

    #[must_use]
    pub fn with_physical_connection_type(
        mut self,
        physical_connection_type: EntityAssociationPhysicalConnectionType,
    ) -> Self {
        self.physical_connection_type = physical_connection_type;
        self
    }

    #[must_use]
    pub fn with_group_member_type(
        mut self,
        group_member_type: EntityAssociationGroupMemberType,
    ) -> Self {
        self.group_member_type = group_member_type;
        self
    }

    #[must_use]
    pub fn with_group_number(mut self, group_number: u16) -> Self {
        self.group_number = group_number;
        self
    }

    #[must_use]
    pub fn to_variable_parameter(self) -> VariableParameter {
        VariableParameter::EntityAssociation(self)
    }
}

/// 6.2.11 Beam Data record
#[derive(Copy, Clone, Default, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct BeamData {
    pub azimuth_center: f32,
    pub azimuth_sweep: f32,
    pub elevation_center: f32,
    pub elevation_sweep: f32,
    pub sweep_sync: f32,
}

impl BeamData {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    #[must_use]
    pub fn with_azimuth_center(mut self, azimuth_center: f32) -> Self {
        self.azimuth_center = azimuth_center;
        self
    }

    #[must_use]
    pub fn with_azimuth_sweep(mut self, azimuth_sweep: f32) -> Self {
        self.azimuth_sweep = azimuth_sweep;
        self
    }

    #[must_use]
    pub fn with_elevation_center(mut self, elevation_center: f32) -> Self {
        self.elevation_center = elevation_center;
        self
    }

    #[must_use]
    pub fn with_elevation_sweep(mut self, elevation_sweep: f32) -> Self {
        self.elevation_sweep = elevation_sweep;
        self
    }

    #[must_use]
    pub fn with_sweep_sync(mut self, sweep_sync: f32) -> Self {
        self.sweep_sync = sweep_sync;
        self
    }
}

pub const SUPPLY_QUANTITY_RECORD_LENGTH: u16 = 12;

/// 6.2.86 Supply Quantity record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SupplyQuantity {
    pub supply_type: EntityType,
    pub quantity: f32,
}

impl SupplyQuantity {
    #[must_use]
    pub fn with_supply_type(mut self, supply_type: EntityType) -> Self {
        self.supply_type = supply_type;
        self
    }

    #[must_use]
    pub fn with_quantity(mut self, quantity: f32) -> Self {
        self.quantity = quantity;
        self
    }
}

pub const BASE_RECORD_SPEC_RECORD_LENGTH: u16 = 16;

/// 6.2.73 Record Specification record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecordSpecification {
    pub record_sets: Vec<RecordSet>,
}

impl RecordSpecification {
    #[must_use]
    pub fn with_record_set(mut self, record: RecordSet) -> Self {
        self.record_sets.push(record);
        self
    }

    #[must_use]
    pub fn with_record_sets(mut self, records: Vec<RecordSet>) -> Self {
        self.record_sets = records;
        self
    }
}

/// Part of 6.2.73 Record Specification record
#[derive(Clone, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RecordSet {
    pub record_id: VariableRecordType,
    pub record_serial_number: u32,
    pub record_length_bytes: u16,
    pub records: Vec<Vec<u8>>,
}

impl RecordSet {
    #[must_use]
    pub fn with_record_id(mut self, record_id: VariableRecordType) -> Self {
        self.record_id = record_id;
        self
    }

    #[must_use]
    pub fn with_record_serial_number(mut self, record_serial_number: u32) -> Self {
        self.record_serial_number = record_serial_number;
        self
    }

    /// Adds `record` to be the Record Values in this `RecordSet`.
    /// It is specified in the DIS standard that all Record Values in a `RecordSet` are of the same length.
    /// It is up to the caller of the function to ensure only Record Values of same length are added,
    /// the length of the last added value is assumed for all previously added.
    #[must_use]
    #[allow(clippy::cast_possible_truncation)]
    pub fn with_record(mut self, record: Vec<u8>) -> Self {
        self.record_length_bytes = record.len() as u16;
        self.records.push(record);
        self
    }

    /// Sets `records` to be the records in this `RecordSet`.
    /// It is specified in the DIS standard that all Record Values in a `RecordSet` are of the same length (i.e., the inner `Vec`).
    #[must_use]
    pub fn with_records(mut self, records: Vec<Vec<u8>>) -> Self {
        self.record_length_bytes = if let Some(record) = records.first() {
            record.len()
        } else {
            0
        } as u16;
        self.records = records;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ENTITY_TYPE_STR: &str = "0:1:2:3:4:5:6";
    const ENTITY_TYPE_STR_INVALID: &str = "0,1,2,3,4,5,6";
    const ENTITY_TYPE_STR_INVALID_EXTRA: &str = "0:1:2:3:4:5:six";
    const ENTITY_TYPE_STR_NOT_SEVEN_DIGITS: &str = "0:1:2:3:4:5";
    const ENTITY_TYPE: EntityType = EntityType {
        kind: EntityKind::Other,
        domain: PlatformDomain::Land,
        country: Country::Albania_ALB_,
        category: 3,
        subcategory: 4,
        specific: 5,
        extra: 6,
    };
    const SIMULATION_ADDRESS_STR: &str = "0:1";
    const SIMULATION_ADDRESS_STR_INVALID: &str = "0,1";
    const SIMULATION_ADDRESS_STR_INVALID_APPLICATION_ID: &str = "0:one";
    const SIMULATION_ADDRESS_STR_NOT_TWO_DIGITS: &str = "0";
    const SIMULATION_ADDRESS: SimulationAddress = SimulationAddress {
        site_id: 0,
        application_id: 1,
    };
    const ENTITY_ID_STR: &str = "0:1:2";
    const ENTITY_ID_STR_INVALID: &str = "0,1,2";
    const ENTITY_ID_STR_INVALID_ENTITY_ID: &str = "0:1:two";
    const ENTITY_ID_STR_NOT_THREE_DIGITS: &str = "0:1";
    const ENTITY_ID: EntityId = EntityId {
        simulation_address: SIMULATION_ADDRESS,
        entity_id: 2,
    };
    const EVENT_ID_STR: &str = "0:1:2";
    const EVENT_ID_STR_INVALID: &str = "0,1,2";
    const EVENT_ID_STR_INVALID_EVENT_ID: &str = "0:1:two";
    const EVENT_ID_STR_NOT_THREE_DIGITS: &str = "0:1";
    const EVENT_ID: EventId = EventId {
        simulation_address: SIMULATION_ADDRESS,
        event_id: 2,
    };

    #[test]
    fn entity_type_display() {
        assert_eq!(ENTITY_TYPE_STR, ENTITY_TYPE.to_string());
    }

    #[test]
    fn entity_type_from_str() {
        assert_eq!(EntityType::from_str(ENTITY_TYPE_STR).unwrap(), ENTITY_TYPE);
        let err = EntityType::from_str(ENTITY_TYPE_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityType string pattern does not contain precisely 7 digits"
        );
        let err = EntityType::from_str(ENTITY_TYPE_STR_INVALID_EXTRA);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid extra digit");
    }

    #[test]
    fn entity_type_from_str_not_seven_digits() {
        let err = EntityType::from_str(ENTITY_TYPE_STR_NOT_SEVEN_DIGITS);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityType string pattern does not contain precisely 7 digits"
        );
    }

    #[test]
    fn entity_type_try_from_str() {
        assert_eq!(
            TryInto::<EntityType>::try_into(ENTITY_TYPE_STR).unwrap(),
            ENTITY_TYPE
        );
        let err = TryInto::<EntityType>::try_into(ENTITY_TYPE_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityType string pattern does not contain precisely 7 digits"
        );
        let err = TryInto::<EntityType>::try_into(ENTITY_TYPE_STR_INVALID_EXTRA);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid extra digit");
    }

    #[test]
    fn entity_type_try_from_string() {
        assert_eq!(
            TryInto::<EntityType>::try_into(ENTITY_TYPE_STR.to_string()).unwrap(),
            ENTITY_TYPE
        );
        let err = TryInto::<EntityType>::try_into(ENTITY_TYPE_STR_INVALID.to_string());
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityType string pattern does not contain precisely 7 digits"
        );
        let err = TryInto::<EntityType>::try_into(ENTITY_TYPE_STR_INVALID_EXTRA.to_string());
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid extra digit");
    }

    #[test]
    fn simulation_address_display() {
        assert_eq!(SIMULATION_ADDRESS_STR, SIMULATION_ADDRESS.to_string());
    }

    #[test]
    fn simulation_address_from_str() {
        assert_eq!(
            SimulationAddress::from_str(SIMULATION_ADDRESS_STR).unwrap(),
            SIMULATION_ADDRESS
        );
        let err = SimulationAddress::from_str(SIMULATION_ADDRESS_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "SimulationAddress string pattern does not contain precisely 2 digits"
        );
        let err = SimulationAddress::from_str(SIMULATION_ADDRESS_STR_INVALID_APPLICATION_ID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid application id digit");
    }

    #[test]
    fn simulation_address_from_str_not_two_digits() {
        let err = SimulationAddress::from_str(SIMULATION_ADDRESS_STR_NOT_TWO_DIGITS);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "SimulationAddress string pattern does not contain precisely 2 digits"
        );
    }

    #[test]
    fn simulation_address_try_from_str() {
        assert_eq!(
            TryInto::<SimulationAddress>::try_into(SIMULATION_ADDRESS_STR).unwrap(),
            SIMULATION_ADDRESS
        );
        let err = TryInto::<SimulationAddress>::try_into(SIMULATION_ADDRESS_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "SimulationAddress string pattern does not contain precisely 2 digits"
        );
        let err =
            TryInto::<SimulationAddress>::try_into(SIMULATION_ADDRESS_STR_INVALID_APPLICATION_ID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid application id digit");
    }

    #[test]
    fn simulation_address_try_from_string() {
        assert_eq!(
            TryInto::<SimulationAddress>::try_into(SIMULATION_ADDRESS_STR.to_string()).unwrap(),
            SIMULATION_ADDRESS
        );
        let err =
            TryInto::<SimulationAddress>::try_into(SIMULATION_ADDRESS_STR_INVALID.to_string());
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "SimulationAddress string pattern does not contain precisely 2 digits"
        );
        let err = TryInto::<SimulationAddress>::try_into(
            SIMULATION_ADDRESS_STR_INVALID_APPLICATION_ID.to_string(),
        );
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid application id digit");
    }

    #[test]
    fn entity_id_display() {
        assert_eq!(ENTITY_ID_STR, ENTITY_ID.to_string());
    }

    #[test]
    fn entity_id_from_str() {
        assert_eq!(EntityId::from_str(ENTITY_ID_STR).unwrap(), ENTITY_ID);
        let err = EntityId::from_str(ENTITY_ID_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityId string pattern does not contain precisely 3 digits"
        );
        let err = EntityId::from_str(ENTITY_ID_STR_INVALID_ENTITY_ID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid entity id digit");
    }

    #[test]
    fn entity_id_from_str_not_three_digits() {
        let err = EntityId::from_str(ENTITY_ID_STR_NOT_THREE_DIGITS);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityId string pattern does not contain precisely 3 digits"
        );
    }

    #[test]
    fn entity_id_try_from_str() {
        assert_eq!(
            TryInto::<EntityId>::try_into(ENTITY_ID_STR).unwrap(),
            ENTITY_ID
        );
        let err = TryInto::<EntityId>::try_into(ENTITY_ID_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityId string pattern does not contain precisely 3 digits"
        );
        let err = TryInto::<EntityId>::try_into(ENTITY_ID_STR_INVALID_ENTITY_ID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid entity id digit");
    }

    #[test]
    fn entity_id_try_from_string() {
        assert_eq!(
            TryInto::<EntityId>::try_into(ENTITY_ID_STR.to_string()).unwrap(),
            ENTITY_ID
        );
        let err = TryInto::<EntityId>::try_into(ENTITY_ID_STR_INVALID.to_string());
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EntityId string pattern does not contain precisely 3 digits"
        );
        let err = TryInto::<EntityId>::try_into(ENTITY_ID_STR_INVALID_ENTITY_ID.to_string());
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid entity id digit");
    }

    #[test]
    fn event_id_display() {
        assert_eq!(EVENT_ID_STR, EVENT_ID.to_string());
    }

    #[test]
    fn event_id_from_str() {
        assert_eq!(EventId::from_str(EVENT_ID_STR).unwrap(), EVENT_ID);
        let err = EventId::from_str(EVENT_ID_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EventId string pattern does not contain precisely 3 digits"
        );
        let err = EventId::from_str(EVENT_ID_STR_INVALID_EVENT_ID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid event id digit");
    }

    #[test]
    fn event_id_from_str_not_three_digits() {
        let err = EventId::from_str(EVENT_ID_STR_NOT_THREE_DIGITS);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EventId string pattern does not contain precisely 3 digits"
        );
    }

    #[test]
    fn event_id_try_from_str() {
        assert_eq!(
            TryInto::<EventId>::try_into(EVENT_ID_STR).unwrap(),
            EVENT_ID
        );
        let err = TryInto::<EventId>::try_into(EVENT_ID_STR_INVALID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EventId string pattern does not contain precisely 3 digits"
        );
        let err = TryInto::<EventId>::try_into(EVENT_ID_STR_INVALID_EVENT_ID);
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid event id digit");
    }

    #[test]
    fn event_id_try_from_string() {
        assert_eq!(
            TryInto::<EventId>::try_into(EVENT_ID_STR.to_string()).unwrap(),
            EVENT_ID
        );
        let err = TryInto::<EventId>::try_into(EVENT_ID_STR_INVALID.to_string());
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(
            err.unwrap_err().to_string(),
            "EventId string pattern does not contain precisely 3 digits"
        );
        let err = TryInto::<EventId>::try_into(EVENT_ID_STR_INVALID_EVENT_ID.to_string());
        assert!(err.is_err());
        assert!(matches!(err, Err(DisError::ParseError(_))));
        assert_eq!(err.unwrap_err().to_string(), "Invalid event id digit");
    }
}
