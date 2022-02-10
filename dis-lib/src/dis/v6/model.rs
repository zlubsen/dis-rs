use crate::dis::common::model::ProtocolVersion;
use crate::dis::v6::builder::PduHeaderBuilder;
use crate::dis::v6::other::model::Other;
use super::entity_state::model::EntityState;

// TODO re-export the PDU types
// TODO review pub settings in PDU modules

pub const PDU_HEADER_LEN_BYTES : usize = 12;

#[derive(Copy, Clone, Debug)]
pub struct PduHeader {
    pub protocol_version : ProtocolVersion,
    pub exercise_id : u8,
    pub pdu_type : PduType,
    pub protocol_family : ProtocolFamily,
    pub time_stamp : u32,
    pub pdu_length : u16,
    pub padding : u16,
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    AnnounceObjectPdu = 129,
    DeleteObjectPdu = 130,
    DescribeApplicationPdu = 131,
    DescribeEventPdu = 132,
    DescribeObjectPdu = 133,
    RequestEventPdu = 134,
    RequestObjectPdu = 135,
    TimeSpacePositionIndicatorFIPdu = 140,
    AppearanceFIPdu = 141,
    ArticulatedPartsFIPdu = 142,
    FireFIPdu = 143,
    DetonationFIPdu = 144,
    PointObjectStatePdu = 150,
    LinearObjectStatePdu = 151,
    ArealObjectStatePdu = 152,
    EnvironmentPdu = 153,
    TransferControlRequestPdu = 155,
    TransferControlPdu = 156,
    TransferControlAcknowledgePdu = 157,
    IntercomControlPdu = 160,
    IntercomSignalPdu = 161,
    AggregatePdu = 170,
}

impl From<u8> for PduType {
    fn from(value: u8) -> Self {
        match value {
            1 => PduType::EntityStatePdu,
            2 => PduType::FirePdu,
            3 => PduType::DetonationPdu,
            4 => PduType::CollisionPdu,
            5 => PduType::ServiceRequestPdu,
            6 => PduType::ResupplyOfferPdu,
            7 => PduType::ResupplyReceivedPdu,
            8 => PduType::ResupplyCancelPdu,
            9 => PduType::RepairCompletePdu,
            10 => PduType::RepairResponsePdu,
            11 => PduType::CreateEntityPdu,
            12 => PduType::RemoveEntityPdu,
            13 => PduType::StartResumePdu,
            14 => PduType::StopFreezePdu,
            15 => PduType::AcknowledgePdu,
            16 => PduType::ActionRequestPdu,
            17 => PduType::ActionResponsePdu,
            18 => PduType::DataQueryPdu,
            19 => PduType::SetDataPdu,
            20 => PduType::DataPdu,
            21 => PduType::EventReportPdu,
            22 => PduType::CommentPdu,
            23 => PduType::ElectromagneticEmissionPdu,
            24 => PduType::DesignatorPdu,
            25 => PduType::TransmitterPdu,
            26 => PduType::SignalPdu,
            27 => PduType::ReceiverPdu,
            129 => PduType::AnnounceObjectPdu,
            130 => PduType::DeleteObjectPdu,
            131 => PduType::DescribeApplicationPdu,
            132 => PduType::DescribeEventPdu,
            133 => PduType::DescribeObjectPdu,
            134 => PduType::RequestEventPdu,
            135 => PduType::RequestObjectPdu,
            140 => PduType::TimeSpacePositionIndicatorFIPdu,
            141 => PduType::AppearanceFIPdu,
            142 => PduType::ArticulatedPartsFIPdu,
            143 => PduType::FireFIPdu,
            144 => PduType::DetonationFIPdu,
            150 => PduType::PointObjectStatePdu,
            151 => PduType::LinearObjectStatePdu,
            152 => PduType::ArealObjectStatePdu,
            153 => PduType::EnvironmentPdu,
            155 => PduType::TransferControlRequestPdu,
            156 => PduType::TransferControlPdu,
            157 => PduType::TransferControlAcknowledgePdu,
            160 => PduType::IntercomControlPdu,
            161 => PduType::IntercomSignalPdu,
            170 => PduType::AggregatePdu,
            0 | _ => PduType::OtherPdu,
            // unspecified_enum => PduType::Unspecified(unspecified_enum),
        }
    }
}

impl From<PduType> for u8 {
    fn from(value: PduType) -> Self {
        match value {
            PduType::OtherPdu => { 0u8 }
            PduType::EntityStatePdu => { 1u8 }
            PduType::FirePdu => { 2u8 }
            PduType::DetonationPdu => { 3u8 }
            PduType::CollisionPdu => { 4u8 }
            PduType::ServiceRequestPdu => { 5u8 }
            PduType::ResupplyOfferPdu => { 6u8 }
            PduType::ResupplyReceivedPdu => { 7u8 }
            PduType::ResupplyCancelPdu => { 8u8 }
            PduType::RepairCompletePdu => { 9u8 }
            PduType::RepairResponsePdu => { 10u8 }
            PduType::CreateEntityPdu => { 11u8 }
            PduType::RemoveEntityPdu => { 12u8 }
            PduType::StartResumePdu => { 13u8 }
            PduType::StopFreezePdu => { 14u8 }
            PduType::AcknowledgePdu => { 15u8 }
            PduType::ActionRequestPdu => { 16u8 }
            PduType::ActionResponsePdu => { 17u8 }
            PduType::DataQueryPdu => { 18u8 }
            PduType::SetDataPdu => { 19u8 }
            PduType::DataPdu => { 20u8 }
            PduType::EventReportPdu => { 21u8 }
            PduType::CommentPdu => { 22u8 }
            PduType::ElectromagneticEmissionPdu => { 23u8 }
            PduType::DesignatorPdu => { 24u8 }
            PduType::TransmitterPdu => { 25u8 }
            PduType::SignalPdu => { 26u8 }
            PduType::ReceiverPdu => { 27u8 }
            PduType::AnnounceObjectPdu => { 129u8 }
            PduType::DeleteObjectPdu => { 130u8 }
            PduType::DescribeApplicationPdu => { 131u8 }
            PduType::DescribeEventPdu => { 132u8 }
            PduType::DescribeObjectPdu => { 133u8 }
            PduType::RequestEventPdu => { 134u8 }
            PduType::RequestObjectPdu => { 135u8 }
            PduType::TimeSpacePositionIndicatorFIPdu => { 140u8 }
            PduType::AppearanceFIPdu => { 141u8 }
            PduType::ArticulatedPartsFIPdu => { 142u8 }
            PduType::FireFIPdu => { 143u8 }
            PduType::DetonationFIPdu => { 144u8 }
            PduType::PointObjectStatePdu => { 150u8 }
            PduType::LinearObjectStatePdu => { 151u8 }
            PduType::ArealObjectStatePdu => { 152u8 }
            PduType::EnvironmentPdu => { 153u8 }
            PduType::TransferControlRequestPdu => { 155u8 }
            PduType::TransferControlPdu => { 156u8 }
            PduType::TransferControlAcknowledgePdu => { 157u8 }
            PduType::IntercomControlPdu => { 160u8 }
            PduType::IntercomSignalPdu => { 161u8 }
            PduType::AggregatePdu => { 170u8 }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ProtocolFamily {
    Other = 0,
    EntityInformationInteraction = 1,
    ExperimentalCGF = 129,
    ExperimentalEntityInteractionInformationFieldInstrumentation = 130,
    ExperimentalWarfareFieldInstrumentation = 131,
    ExperimentalEnvironmentObjectInformationInteraction = 132,
    ExperimentalEntityManagement = 133,
    Warfare = 2,
    Logistics = 3,
    RadioCommunication = 4,
    SimulationManagement = 5,
    DistributedEmissionRegeneration = 6,
    // Unspecified(u8),
}

impl From<u8> for ProtocolFamily {
    fn from(value: u8) -> Self {
        match value {
            1 => ProtocolFamily::EntityInformationInteraction,
            2 => ProtocolFamily::Warfare,
            3 => ProtocolFamily::Logistics,
            4 => ProtocolFamily::RadioCommunication,
            5 => ProtocolFamily::SimulationManagement,
            6 => ProtocolFamily::DistributedEmissionRegeneration,
            129 => ProtocolFamily::ExperimentalCGF,
            130 => ProtocolFamily::ExperimentalEntityInteractionInformationFieldInstrumentation,
            131 => ProtocolFamily::ExperimentalWarfareFieldInstrumentation,
            132 => ProtocolFamily::ExperimentalEnvironmentObjectInformationInteraction,
            133 => ProtocolFamily::ExperimentalEntityManagement,
            0 | _ => ProtocolFamily::Other,
            // unspecified_enum => ProtocolFamily::Unspecified(unspecified_enum),
        }
    }
}

impl From<ProtocolFamily> for u8 {
    fn from(value: ProtocolFamily) -> Self {
        match value {
            ProtocolFamily::Other => { 0u8 }
            ProtocolFamily::EntityInformationInteraction => { 1u8 }
            ProtocolFamily::ExperimentalCGF => { 129u8 }
            ProtocolFamily::ExperimentalEntityInteractionInformationFieldInstrumentation => { 130u8 }
            ProtocolFamily::ExperimentalWarfareFieldInstrumentation => { 131u8 }
            ProtocolFamily::ExperimentalEnvironmentObjectInformationInteraction => { 132u8 }
            ProtocolFamily::ExperimentalEntityManagement => { 133u8 }
            ProtocolFamily::Warfare => { 2u8 }
            ProtocolFamily::Logistics => { 3u8 }
            ProtocolFamily::RadioCommunication => { 4u8 }
            ProtocolFamily::SimulationManagement => { 5u8 }
            ProtocolFamily::DistributedEmissionRegeneration => { 6u8 }
        }
    }
}

impl PduHeader {
    pub fn builder() -> PduHeaderBuilder {
        PduHeaderBuilder::new()
    }
}

pub enum Pdu {
    Other(Other),
    EntityState(EntityState),
    // TODO implement other PDU structs
    Fire,
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
    AnnounceObject,
    DeleteObject,
    DescribeApplication,
    DescribeEvent,
    DescribeObject,
    RequestEvent,
    RequestObject,
    TimeSpacePositionIndicatorFI,
    AppearanceFI,
    ArticulatedPartsFI,
    FireFI,
    DetonationFI,
    PointObjectState,
    LinearObjectState,
    ArealObjectState,
    Environment,
    TransferControlRequest,
    TransferControl,
    TransferControlAcknowledge,
    IntercomControl,
    IntercomSignal,
    Aggregate,
}