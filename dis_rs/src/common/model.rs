use dis_rs_macros::PduConversion;
use crate::common::builder::PduHeaderBuilder;
use crate::common::entity_state::model::EntityState;
use crate::common::Interaction;
use crate::common::other::model::Other;
use crate::{Country, EntityKind, MunitionDescriptorFuse, MunitionDescriptorWarhead};
use crate::common::fire::model::Fire;
use crate::v7::model::PduStatus;

pub struct Pdu {
    pub header : PduHeader,
    pub body : PduBody,
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

impl PduHeader {
    pub fn builder() -> PduHeaderBuilder {
        PduHeaderBuilder::new()
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u8)]
pub enum ProtocolVersion {
    Other = 0,
    // DIS PDU version 1.0 (May 92)
    Version1_0May92 = 1,
    // IEEE 1278-1993
    Ieee1278_1993 = 2,
    // DIS PDU version 2.0 - third draft (May 93)
    Version2_0ThirdDraft = 3,
    // DIS PDU version 2.0 - fourth draft (revised) March 16, 1994
    Version2_0FourthDraft = 4,
    // IEEE 1278.1-1995 / DIS 5
    Ieee1278_1_1995 = 5,
    // IEEE 1278.1a-1998 / DIS 6
    #[allow(non_camel_case_types)]
    Ieee1278_1a_1998 = 6,
    // IEEE 1278.1-2012 / DIS 7
    Ieee1278_1_2012 = 7,
}

impl Default for ProtocolVersion {
    fn default() -> Self {
        ProtocolVersion::Other
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u8)]
pub enum ProtocolFamily {
    Other = 0,
    EntityInformationInteraction = 1,
    Warfare = 2,
    Logistics = 3,
    RadioCommunication = 4,
    SimulationManagement = 5,
    DistributedEmissionRegeneration = 6,
    EntityManagement = 7,
    Minefield = 8,
    SyntheticEnvironment = 9,
    SimulationManagementReliability = 10,
    LiveEntityInformationInteraction = 11,
    NonRealTime = 12,
    InformationOperations = 13,
}

impl Default for ProtocolFamily {
    fn default() -> Self {
        ProtocolFamily::Other
    }
}

// FIXME match PduType from updated list (72 pieces)
impl From<PduType> for ProtocolFamily {
    fn from(pdu_type: PduType) -> Self {
        match pdu_type {
            PduType::EntityStatePdu | PduType::CollisionPdu => ProtocolFamily::EntityInformationInteraction,
            PduType::FirePdu | PduType::DetonationPdu => ProtocolFamily::Warfare,
            PduType::ServiceRequestPdu | PduType::ResupplyOfferPdu | PduType::ResupplyReceivedPdu | PduType::ResupplyCancelPdu | PduType::RepairCompletePdu | PduType::RepairResponsePdu => ProtocolFamily::Logistics,
            PduType::CreateEntityPdu | PduType::RemoveEntityPdu | PduType::StartResumePdu | PduType::StopFreezePdu | PduType::AcknowledgePdu | PduType::ActionRequestPdu | PduType::ActionResponsePdu | PduType::DataQueryPdu | PduType::SetDataPdu | PduType::DataPdu | PduType::EventReportPdu | PduType::CommentPdu => ProtocolFamily::SimulationManagement,
            PduType::ElectromagneticEmissionPdu | PduType::DesignatorPdu => ProtocolFamily::DistributedEmissionRegeneration,
            PduType::TransmitterPdu | PduType::SignalPdu | PduType::ReceiverPdu => ProtocolFamily::RadioCommunication,
            _ => ProtocolFamily::Other,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PduConversion, Eq, Hash)]
#[repr(u8)]
pub enum PduType {
    OtherPdu = 0,
    EntityStatePdu = 1,
    FirePdu = 2,
    DetonationPdu = 3,
    CollisionPdu = 4,
    ServiceRequestPdu = 5,
    ResupplyOfferPdu = 6,
    ResupplyReceivedPdu = 7,
    ResupplyCancelPdu = 8,
    RepairCompletePdu = 9,
    RepairResponsePdu = 10,
    CreateEntityPdu = 11,
    RemoveEntityPdu = 12,
    StartResumePdu = 13,
    StopFreezePdu = 14,
    AcknowledgePdu = 15,
    ActionRequestPdu = 16,
    ActionResponsePdu = 17,
    DataQueryPdu = 18,
    SetDataPdu = 19,
    DataPdu = 20,
    EventReportPdu = 21,
    CommentPdu = 22,
    ElectromagneticEmissionPdu = 23,
    DesignatorPdu = 24,
    TransmitterPdu = 25,
    SignalPdu = 26,
    ReceiverPdu = 27,
    IFF = 28,
    UnderwaterAcoustic = 29,
    SupplementalEmissionEntityState = 30,
    IntercomSignal = 31,
    IntercomControl = 32,
    AggregateState = 33,
    IsGroupOf = 34,
    TransferOwnership = 35,
    IsPartOf = 36,
    MinefieldState = 37,
    MinefieldQuery = 38,
    MinefieldData = 39,
    MinefieldResponseNACK = 40,
    EnvironmentalProcess = 41,
    GriddedData = 42,
    PointObjectState = 43,
    LinearObjectState = 44,
    ArealObjectState = 45,
    TSPI = 46,
    Appearance = 47,
    ArticulatedParts = 48,
    LEFire = 49,
    LEDetonation = 50,
    CreateEntityR = 51,
    RemoveEntityR = 52,
    StartResumeR = 53,
    StopFreezeR = 54,
    AcknowledgeR = 55,
    ActionRequestR = 56,
    ActionResponseR = 57,
    DataQueryR = 58,
    SetDataR = 59,
    DataR = 60,
    EventReportR = 61,
    CommentR = 62,
    RecordR = 63,
    SetRecordR = 64,
    RecordQueryR = 65,
    CollisionElastic = 66,
    EntityStateUpdate = 67,
    DirectedEnergyFire = 68,
    EntityDamageStatus = 69,
    InformationOperationsAction = 70,
    InformationOperationsReport = 71,
    Attribute = 72,
}

impl Default for PduType {
    fn default() -> Self {
        PduType::OtherPdu
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

#[derive(Copy, Clone, Debug)]
pub struct EntityId {
    pub simulation_address : SimulationAddress,
    pub entity_id : u16
}

impl EntityId {
    pub fn new(site_id : u16, application_id : u16, entity_id : u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_id,
                application_id
            },
            entity_id
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SimulationAddress {
    pub site_id : u16,
    pub application_id : u16,
}

#[derive(Copy, Clone, Debug)]
pub struct EventId {
    pub simulation_address : SimulationAddress,
    pub event_id : u16
}

impl EventId {
    pub fn new(site_id : u16, application_id : u16, event_id : u16) -> Self {
        Self {
            simulation_address: SimulationAddress {
                site_id,
                application_id
            },
            event_id
        }
    }
}

pub struct VectorF32 {
    pub first_vector_component : f32,
    pub second_vector_component : f32,
    pub third_vector_component : f32,
}

pub struct Location {
    pub x_coordinate : f64,
    pub y_coordinate : f64,
    pub z_coordinate : f64,
}

// TODO alias to vectorf32?
pub struct Orientation {
    pub psi : f32,
    pub theta : f32,
    pub phi : f32,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct EntityType {
    pub kind : EntityKind,
    pub domain : u8,
    pub country : Country, // TODO u16 instead of big enum? Put codes and names in config file?
    pub category : u8,
    pub subcategory : u8,
    pub specific : u8,
    pub extra : u8,
}

// #[derive(buildstructor::Builder)]
pub struct BurstDescriptor {
    pub munition : EntityType,
    pub warhead : MunitionDescriptorWarhead,
    pub fuse : MunitionDescriptorFuse,
    pub quantity : u16,
    pub rate : u16,
}