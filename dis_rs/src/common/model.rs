use crate::common::entity_state::model::EntityState;
use crate::common::{BodyInfo, Interaction};
use crate::common::defaults::{DEFAULT_APPLICATION_ID, DEFAULT_ENTITY_ID, DEFAULT_EVENT_ID, DEFAULT_SITE_ID};
use crate::common::other::model::Other;
use crate::enumerations::{Country, EntityKind, MunitionDescriptorFuse, MunitionDescriptorWarhead, PduType, PlatformDomain, ProtocolFamily, ProtocolVersion};
use crate::common::fire::model::Fire;
use crate::v7::model::PduStatus;
use crate::PDU_HEADER_LEN_BYTES;

pub struct Pdu {
    pub header : PduHeader,
    pub body : PduBody,
}

impl Pdu {
    pub fn finalize_from_parts(mut header: PduHeader, body: PduBody, time_stamp: u32) -> Self {
        header.fields()
            .time_stamp(time_stamp)
            .body_length(body.body_length() as u16)
            .finish();
        Self {
            header,
            body,
        }
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

#[derive(Copy, Clone, Debug)]
pub struct PduHeader {
    pub protocol_version : ProtocolVersion,
    pub exercise_id : u8,
    pub pdu_type : PduType,
    pub protocol_family : ProtocolFamily,
    pub time_stamp : u32,
    pub pdu_length : u16,
    pub pdu_status : Option<PduStatus>,
    pub padding : u16,
}

#[buildstructor::buildstructor]
impl PduHeader {
    #[builder]
    pub fn new(protocol_version: ProtocolVersion, exercise_id: u8, pdu_type: PduType, protocol_family: ProtocolFamily) -> Self {
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

    #[builder]
    pub fn v6_new(exercise_id: u8, pdu_type: PduType) -> Self {
        PduHeader::new(ProtocolVersion::IEEE1278_1A1998, exercise_id, pdu_type, pdu_type.into())
    }

    #[builder]
    pub fn v7_new(exercise_id: u8, pdu_type: PduType) -> Self {
        PduHeader::new(ProtocolVersion::IEEE1278_12012, exercise_id, pdu_type, pdu_type.into())
    }

    #[builder(entry = "fields", exit = "finish", visibility="pub")]
    fn add_pdu_data(&mut self, time_stamp: u32, body_length: u16, pdu_status: Option<PduStatus>) {
        self.time_stamp = time_stamp;
        self.pdu_length = body_length + PDU_HEADER_LEN_BYTES;
        self.pdu_status = pdu_status;
    }
}

pub enum PduBody {
    Other(Other),
    EntityState(EntityState),
    Fire(Fire),
    Detonation,
    Collision,
    ServiceRequest,
    ResupplyOffer,
    ResupplyReceived,
    ResupplyCancel,
    RepairComplete,
    RepairResponse,
    CreateEntity,
    RemoveEntity,
    StartResume,
    StopFreeze,
    Acknowledge,
    ActionRequest,
    ActionResponse,
    DataQuery,
    SetData,
    Data,
    EventReport,
    Comment,
    ElectromagneticEmission,
    Designator,
    Transmitter,
    Signal,
    Receiver,
    IFF,
    UnderwaterAcoustic,
    SupplementalEmissionEntityState,
    IntercomSignal,
    IntercomControl,
    AggregateState,
    IsGroupOf,
    TransferOwnership,
    IsPartOf,
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
    CreateEntityR,
    RemoveEntityR,
    StartResumeR,
    StopFreezeR,
    AcknowledgeR,
    ActionRequestR,
    ActionResponseR,
    DataQueryR,
    SetDataR,
    DataR,
    EventReportR,
    CommentR,
    RecordR,
    SetRecordR,
    RecordQueryR,
    CollisionElastic,
    EntityStateUpdate,
    DirectedEnergyFire,
    EntityDamageStatus,
    InformationOperationsAction,
    InformationOperationsReport,
    Attribute,
}

impl BodyInfo for PduBody {
    fn body_length(&self) -> u16 {
        match self {
            PduBody::Other(body) => { body.body_length() }
            PduBody::EntityState(body) => { body.body_length() }
            PduBody::Fire(body) => { body.body_length() }
            PduBody::Detonation => { 0 }
            PduBody::Collision => { 0 }
            PduBody::ServiceRequest => { 0 }
            PduBody::ResupplyOffer => { 0 }
            PduBody::ResupplyReceived => { 0 }
            PduBody::ResupplyCancel => { 0 }
            PduBody::RepairComplete => { 0 }
            PduBody::RepairResponse => { 0 }
            PduBody::CreateEntity => { 0 }
            PduBody::RemoveEntity => { 0 }
            PduBody::StartResume => { 0 }
            PduBody::StopFreeze => { 0 }
            PduBody::Acknowledge => { 0 }
            PduBody::ActionRequest => { 0 }
            PduBody::ActionResponse => { 0 }
            PduBody::DataQuery => { 0 }
            PduBody::SetData => { 0 }
            PduBody::Data => { 0 }
            PduBody::EventReport => { 0 }
            PduBody::Comment => { 0 }
            PduBody::ElectromagneticEmission => { 0 }
            PduBody::Designator => { 0 }
            PduBody::Transmitter => { 0 }
            PduBody::Signal => { 0 }
            PduBody::Receiver => { 0 }
            PduBody::IFF => { 0 }
            PduBody::UnderwaterAcoustic => { 0 }
            PduBody::SupplementalEmissionEntityState => { 0 }
            PduBody::IntercomSignal => { 0 }
            PduBody::IntercomControl => { 0 }
            PduBody::AggregateState => { 0 }
            PduBody::IsGroupOf => { 0 }
            PduBody::TransferOwnership => { 0 }
            PduBody::IsPartOf => { 0 }
            PduBody::MinefieldState => { 0 }
            PduBody::MinefieldQuery => { 0 }
            PduBody::MinefieldData => { 0 }
            PduBody::MinefieldResponseNACK => { 0 }
            PduBody::EnvironmentalProcess => { 0 }
            PduBody::GriddedData => { 0 }
            PduBody::PointObjectState => { 0 }
            PduBody::LinearObjectState => { 0 }
            PduBody::ArealObjectState => { 0 }
            PduBody::TSPI => { 0 }
            PduBody::Appearance => { 0 }
            PduBody::ArticulatedParts => { 0 }
            PduBody::LEFire => { 0 }
            PduBody::LEDetonation => { 0 }
            PduBody::CreateEntityR => { 0 }
            PduBody::RemoveEntityR => { 0 }
            PduBody::StartResumeR => { 0 }
            PduBody::StopFreezeR => { 0 }
            PduBody::AcknowledgeR => { 0 }
            PduBody::ActionRequestR => { 0 }
            PduBody::ActionResponseR => { 0 }
            PduBody::DataQueryR => { 0 }
            PduBody::SetDataR => { 0 }
            PduBody::DataR => { 0 }
            PduBody::EventReportR => { 0 }
            PduBody::CommentR => { 0 }
            PduBody::RecordR => { 0 }
            PduBody::SetRecordR => { 0 }
            PduBody::RecordQueryR => { 0 }
            PduBody::CollisionElastic => { 0 }
            PduBody::EntityStateUpdate => { 0 }
            PduBody::DirectedEnergyFire => { 0 }
            PduBody::EntityDamageStatus => { 0 }
            PduBody::InformationOperationsAction => { 0 }
            PduBody::InformationOperationsReport => { 0 }
            PduBody::Attribute => { 0 }
        }
    }

    fn body_type(&self) -> PduType {
        match self {
            PduBody::Other(body) => { body.body_type() }
            PduBody::EntityState(body) => { body.body_type() }
            PduBody::Fire(body) => { body.body_type() }
            PduBody::Detonation => { PduType::Detonation }
            PduBody::Collision => { PduType::Collision }
            PduBody::ServiceRequest => { PduType::ServiceRequest }
            PduBody::ResupplyOffer => { PduType::ResupplyOffer }
            PduBody::ResupplyReceived => { PduType::ResupplyReceived }
            PduBody::ResupplyCancel => { PduType::ResupplyCancel }
            PduBody::RepairComplete => { PduType::RepairComplete }
            PduBody::RepairResponse => { PduType::RepairResponse }
            PduBody::CreateEntity => { PduType::CreateEntity }
            PduBody::RemoveEntity => { PduType::RemoveEntity }
            PduBody::StartResume => { PduType::StartResume }
            PduBody::StopFreeze => { PduType::StopFreeze }
            PduBody::Acknowledge => { PduType::Acknowledge }
            PduBody::ActionRequest => { PduType::ActionRequest }
            PduBody::ActionResponse => { PduType::ActionResponse }
            PduBody::DataQuery => { PduType::DataQuery }
            PduBody::SetData => { PduType::SetData }
            PduBody::Data => { PduType::Data }
            PduBody::EventReport => { PduType::EventReport }
            PduBody::Comment => { PduType::Comment }
            PduBody::ElectromagneticEmission => { PduType::ElectromagneticEmission }
            PduBody::Designator => { PduType::Designator }
            PduBody::Transmitter => { PduType::Transmitter }
            PduBody::Signal => { PduType::Signal }
            PduBody::Receiver => { PduType::Receiver }
            PduBody::IFF => { PduType::IFF }
            PduBody::UnderwaterAcoustic => { PduType::UnderwaterAcoustic }
            PduBody::SupplementalEmissionEntityState => { PduType::SupplementalEmissionEntityState }
            PduBody::IntercomSignal => { PduType::IntercomSignal }
            PduBody::IntercomControl => { PduType::IntercomControl }
            PduBody::AggregateState => { PduType::AggregateState }
            PduBody::IsGroupOf => { PduType::IsGroupOf }
            PduBody::TransferOwnership => { PduType::TransferOwnership }
            PduBody::IsPartOf => { PduType::IsPartOf }
            PduBody::MinefieldState => { PduType::MinefieldState }
            PduBody::MinefieldQuery => { PduType::MinefieldQuery }
            PduBody::MinefieldData => { PduType::MinefieldData }
            PduBody::MinefieldResponseNACK => { PduType::MinefieldResponseNACK }
            PduBody::EnvironmentalProcess => { PduType::EnvironmentalProcess }
            PduBody::GriddedData => { PduType::GriddedData }
            PduBody::PointObjectState => { PduType::PointObjectState }
            PduBody::LinearObjectState => { PduType::LinearObjectState }
            PduBody::ArealObjectState => { PduType::ArealObjectState }
            PduBody::TSPI => { PduType::TSPI }
            PduBody::Appearance => { PduType::Appearance }
            PduBody::ArticulatedParts => { PduType::ArticulatedParts }
            PduBody::LEFire => { PduType::LEFire }
            PduBody::LEDetonation => { PduType::LEDetonation }
            PduBody::CreateEntityR => { PduType::CreateEntityR }
            PduBody::RemoveEntityR => { PduType::RemoveEntityR }
            PduBody::StartResumeR => { PduType::StartResumeR }
            PduBody::StopFreezeR => { PduType::StopFreezeR }
            PduBody::AcknowledgeR => { PduType::AcknowledgeR }
            PduBody::ActionRequestR => { PduType::ActionRequestR }
            PduBody::ActionResponseR => { PduType::ActionResponseR }
            PduBody::DataQueryR => { PduType::DataQueryR }
            PduBody::SetDataR => { PduType::SetDataR }
            PduBody::DataR => { PduType::DataR }
            PduBody::EventReportR => { PduType::EventReportR }
            PduBody::CommentR => { PduType::CommentR }
            PduBody::RecordR => { PduType::RecordR }
            PduBody::SetRecordR => { PduType::SetRecordR }
            PduBody::RecordQueryR => { PduType::RecordQueryR }
            PduBody::CollisionElastic => { PduType::CollisionElastic }
            PduBody::EntityStateUpdate => { PduType::EntityStateUpdate }
            PduBody::DirectedEnergyFire => { PduType::DirectedEnergyFire }
            PduBody::EntityDamageStatus => { PduType::EntityDamageStatus }
            PduBody::InformationOperationsAction => { PduType::InformationOperationsAction }
            PduBody::InformationOperationsReport => { PduType::InformationOperationsReport }
            PduBody::Attribute => { PduType::Attribute }
        }
    }
}

impl Interaction for PduBody {
    fn originator(&self) -> Option<&EntityId> {
        match self {
            PduBody::Other(body) => { body.originator() }
            PduBody::EntityState(body) => { body.originator() }
            PduBody::Fire(body) => { body.originator() }
            PduBody::Detonation => { None }
            PduBody::Collision => { None }
            PduBody::ServiceRequest => { None }
            PduBody::ResupplyOffer => { None }
            PduBody::ResupplyReceived => { None }
            PduBody::ResupplyCancel => { None }
            PduBody::RepairComplete => { None }
            PduBody::RepairResponse => { None }
            PduBody::CreateEntity => { None }
            PduBody::RemoveEntity => { None }
            PduBody::StartResume => { None }
            PduBody::StopFreeze => { None }
            PduBody::Acknowledge => { None }
            PduBody::ActionRequest => { None }
            PduBody::ActionResponse => { None }
            PduBody::DataQuery => { None }
            PduBody::SetData => { None }
            PduBody::Data => { None }
            PduBody::EventReport => { None }
            PduBody::Comment => { None }
            PduBody::ElectromagneticEmission => { None }
            PduBody::Designator => { None }
            PduBody::Transmitter => { None }
            PduBody::Signal => { None }
            PduBody::Receiver => { None }
            PduBody::IFF => { None }
            PduBody::UnderwaterAcoustic => { None }
            PduBody::SupplementalEmissionEntityState => { None }
            PduBody::IntercomSignal => { None }
            PduBody::IntercomControl => { None }
            PduBody::AggregateState => { None }
            PduBody::IsGroupOf => { None }
            PduBody::TransferOwnership => { None }
            PduBody::IsPartOf => { None }
            PduBody::MinefieldState => { None }
            PduBody::MinefieldQuery => { None }
            PduBody::MinefieldData => { None }
            PduBody::MinefieldResponseNACK => { None }
            PduBody::EnvironmentalProcess => { None }
            PduBody::GriddedData => { None }
            PduBody::PointObjectState => { None }
            PduBody::LinearObjectState => { None }
            PduBody::ArealObjectState => { None }
            PduBody::TSPI => { None }
            PduBody::Appearance => { None }
            PduBody::ArticulatedParts => { None }
            PduBody::LEFire => { None }
            PduBody::LEDetonation => { None }
            PduBody::CreateEntityR => { None }
            PduBody::RemoveEntityR => { None }
            PduBody::StartResumeR => { None }
            PduBody::StopFreezeR => { None }
            PduBody::AcknowledgeR => { None }
            PduBody::ActionRequestR => { None }
            PduBody::ActionResponseR => { None }
            PduBody::DataQueryR => { None }
            PduBody::SetDataR => { None }
            PduBody::DataR => { None }
            PduBody::EventReportR => { None }
            PduBody::CommentR => { None }
            PduBody::RecordR => { None }
            PduBody::SetRecordR => { None }
            PduBody::RecordQueryR => { None }
            PduBody::CollisionElastic => { None }
            PduBody::EntityStateUpdate => { None }
            PduBody::DirectedEnergyFire => { None }
            PduBody::EntityDamageStatus => { None }
            PduBody::InformationOperationsAction => { None }
            PduBody::InformationOperationsReport => { None }
            PduBody::Attribute => { None }
        }
    }

    fn receiver(&self) -> Option<&EntityId> {
        match self {
            PduBody::Other(body) => { body.receiver() }
            PduBody::EntityState(body) => { body.receiver() }
            PduBody::Fire(body) => { body.receiver() }
            PduBody::Detonation => { None }
            PduBody::Collision => { None }
            PduBody::ServiceRequest => { None }
            PduBody::ResupplyOffer => { None }
            PduBody::ResupplyReceived => { None }
            PduBody::ResupplyCancel => { None }
            PduBody::RepairComplete => { None }
            PduBody::RepairResponse => { None }
            PduBody::CreateEntity => { None }
            PduBody::RemoveEntity => { None }
            PduBody::StartResume => { None }
            PduBody::StopFreeze => { None }
            PduBody::Acknowledge => { None }
            PduBody::ActionRequest => { None }
            PduBody::ActionResponse => { None }
            PduBody::DataQuery => { None }
            PduBody::SetData => { None }
            PduBody::Data => { None }
            PduBody::EventReport => { None }
            PduBody::Comment => { None }
            PduBody::ElectromagneticEmission => { None }
            PduBody::Designator => { None }
            PduBody::Transmitter => { None }
            PduBody::Signal => { None }
            PduBody::Receiver => { None }
            PduBody::IFF => { None }
            PduBody::UnderwaterAcoustic => { None }
            PduBody::SupplementalEmissionEntityState => { None }
            PduBody::IntercomSignal => { None }
            PduBody::IntercomControl => { None }
            PduBody::AggregateState => { None }
            PduBody::IsGroupOf => { None }
            PduBody::TransferOwnership => { None }
            PduBody::IsPartOf => { None }
            PduBody::MinefieldState => { None }
            PduBody::MinefieldQuery => { None }
            PduBody::MinefieldData => { None }
            PduBody::MinefieldResponseNACK => { None }
            PduBody::EnvironmentalProcess => { None }
            PduBody::GriddedData => { None }
            PduBody::PointObjectState => { None }
            PduBody::LinearObjectState => { None }
            PduBody::ArealObjectState => { None }
            PduBody::TSPI => { None }
            PduBody::Appearance => { None }
            PduBody::ArticulatedParts => { None }
            PduBody::LEFire => { None }
            PduBody::LEDetonation => { None }
            PduBody::CreateEntityR => { None }
            PduBody::RemoveEntityR => { None }
            PduBody::StartResumeR => { None }
            PduBody::StopFreezeR => { None }
            PduBody::AcknowledgeR => { None }
            PduBody::ActionRequestR => { None }
            PduBody::ActionResponseR => { None }
            PduBody::DataQueryR => { None }
            PduBody::SetDataR => { None }
            PduBody::DataR => { None }
            PduBody::EventReportR => { None }
            PduBody::CommentR => { None }
            PduBody::RecordR => { None }
            PduBody::SetRecordR => { None }
            PduBody::RecordQueryR => { None }
            PduBody::CollisionElastic => { None }
            PduBody::EntityStateUpdate => { None }
            PduBody::DirectedEnergyFire => { None }
            PduBody::EntityDamageStatus => { None }
            PduBody::InformationOperationsAction => { None }
            PduBody::InformationOperationsReport => { None }
            PduBody::Attribute => { None }
        }
    }
}

impl From<PduType> for ProtocolFamily {
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
            PduType::SupplementalEmissionEntityState => ProtocolFamily::DistributedEmissionRegeneration,
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
            PduType::CreateEntityR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::RemoveEntityR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::StartResumeR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::StopFreezeR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::AcknowledgeR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::ActionRequestR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::ActionResponseR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::DataQueryR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::SetDataR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::DataR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::EventReportR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::CommentR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::RecordR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::SetRecordR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::RecordQueryR => ProtocolFamily::SimulationManagementwithReliability,
            PduType::CollisionElastic => ProtocolFamily::EntityInformationInteraction,
            PduType::EntityStateUpdate => ProtocolFamily::EntityInformationInteraction,
            PduType::DirectedEnergyFire => ProtocolFamily::Warfare,
            PduType::EntityDamageStatus => ProtocolFamily::Warfare,
            PduType::InformationOperationsAction => ProtocolFamily::InformationOperations,
            PduType::InformationOperationsReport => ProtocolFamily::InformationOperations,
            PduType::Attribute => ProtocolFamily::EntityInformationInteraction,
            PduType::Unspecified(unspecified_value) => ProtocolFamily::Unspecified(unspecified_value)
        }
    }
}

#[derive(Copy, Clone, Debug, buildstructor::Builder)]
pub struct SimulationAddress {
    pub site_id : u16,
    pub application_id : u16,
}

impl Default for SimulationAddress {
    fn default() -> Self {
        Self {
            site_id: DEFAULT_SITE_ID,
            application_id: DEFAULT_APPLICATION_ID
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EntityId {
    pub simulation_address : SimulationAddress,
    pub entity_id : u16
}

impl Default for EntityId {
    fn default() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            entity_id: DEFAULT_ENTITY_ID
        }
    }
}

#[buildstructor::buildstructor]
impl EntityId {
    #[builder]
    pub fn new(site_id : u16, application_id : u16, entity_id : u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_id,
                application_id
            },
            entity_id
        }
    }

    #[builder]
    pub fn with_sim_address_new(simulation_address: SimulationAddress, entity_id : u16) -> Self {
        Self {
            simulation_address,
            entity_id
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct EventId {
    pub simulation_address : SimulationAddress,
    pub event_id : u16
}

#[buildstructor::buildstructor]
impl EventId {
    #[builder]
    pub fn new(simulation_address: SimulationAddress, event_id: u16) -> Self {
        Self {
            simulation_address,
            event_id
        }
    }

    #[builder]
    pub fn with_sim_address_new(simulation_address: SimulationAddress, event_id : u16) -> Self {
        Self {
            simulation_address,
            event_id
        }
    }
}

impl Default for EventId {
    fn default() -> Self {
        Self {
            simulation_address: SimulationAddress::default(),
            event_id: DEFAULT_EVENT_ID
        }
    }
}

#[derive(buildstructor::Builder, Default)]
pub struct VectorF32 {
    pub first_vector_component : f32,
    pub second_vector_component : f32,
    pub third_vector_component : f32,
}

#[derive(buildstructor::Builder, Default)]
pub struct Location {
    pub x_coordinate : f64,
    pub y_coordinate : f64,
    pub z_coordinate : f64,
}

#[derive(buildstructor::Builder, Default)]
pub struct Orientation {
    pub psi : f32,
    pub theta : f32,
    pub phi : f32,
}

#[derive(buildstructor::Builder, Default)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityType {
    pub kind : EntityKind,
    pub domain : PlatformDomain,
    pub country : Country,
    pub category : u8,
    pub subcategory : u8,
    pub specific : u8,
    pub extra : u8,
}

#[derive(buildstructor::Builder, Default)]
pub struct BurstDescriptor {
    pub munition : EntityType,
    pub warhead : MunitionDescriptorWarhead,
    pub fuse : MunitionDescriptorFuse,
    pub quantity : u16,
    pub rate : u16,
}