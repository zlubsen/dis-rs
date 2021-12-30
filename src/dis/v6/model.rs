use crate::dis::errors::DisError;
use crate::dis::v6::entity_state::model::EntityState;

// TODO re-export the PDU types
// TODO review pub settings in PDU modules

pub const PDU_HEADER_MIN_LEN_BYTES : usize = 1280;

pub struct PduHeader {
    pub protocol_version : ProtocolVersion,
    pub exercise_id : u8,
    pub pdu_type : PduType,
    pub protocol_family : ProtocolFamily,
    pub time_stamp : u32,
    pub pdu_length : u16,
    pub padding : u16,
}

pub enum ProtocolVersion {
    Other = 0,
    VERSION_1_0_MAY_92 = 1,             // DIS PDU version 1.0 (May 92)
    IEEE_1278_1993 = 2,                 // IEEE 1278-1993
    VERSION_2_0_THIRD_DRAFT = 3,        // DIS PDU version 2.0 - third draft (May 93)
    VERSION_2_0_FOURTH_DRAFT = 4,       // DIS PDU version 2.0 - fourth draft (revised) March 16, 1994
    IEEE_1278_1_1995 = 5,               // IEEE 1278.1-1995
}

impl TryFrom<u8> for ProtocolVersion {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProtocolVersion::Other),
            1 => Ok(ProtocolVersion::VERSION_1_0_MAY_92),
            2 => Ok(ProtocolVersion::IEEE_1278_1993),
            3 => Ok(ProtocolVersion::VERSION_2_0_THIRD_DRAFT),
            4 => Ok(ProtocolVersion::VERSION_2_0_FOURTH_DRAFT),
            5 => Ok(ProtocolVersion::IEEE_1278_1_1995),
            undefined_enum => Err(DisError::InvalidProtocolVersionValue(undefined_enum)),
        }
    }
}

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

impl TryFrom<u8> for PduType {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(PduType::OtherPdu),
            1 => Ok(PduType::EntityStatePdu),
            2 => Ok(PduType::FirePdu),
            3 => Ok(PduType::DetonationPdu),
            4 => Ok(PduType::CollisionPdu),
            5 => Ok(PduType::ServiceRequestPdu),
            6 => Ok(PduType::ResupplyOfferPdu),
            7 => Ok(PduType::ResupplyReceivedPdu),
            8 => Ok(PduType::ResupplyCancelPdu),
            9 => Ok(PduType::RepairCompletePdu),
            10 => Ok(PduType::RepairResponsePdu),
            11 => Ok(PduType::CreateEntityPdu),
            12 => Ok(PduType::RemoveEntityPdu),
            13 => Ok(PduType::StartResumePdu),
            14 => Ok(PduType::StopFreezePdu),
            15 => Ok(PduType::AcknowledgePdu),
            16 => Ok(PduType::ActionRequestPdu),
            17 => Ok(PduType::ActionResponsePdu),
            18 => Ok(PduType::DataQueryPdu),
            19 => Ok(PduType::SetDataPdu),
            20 => Ok(PduType::DataPdu),
            21 => Ok(PduType::EventReportPdu),
            22 => Ok(PduType::CommentPdu),
            23 => Ok(PduType::ElectromagneticEmissionPdu),
            24 => Ok(PduType::DesignatorPdu),
            25 => Ok(PduType::TransmitterPdu),
            26 => Ok(PduType::SignalPdu),
            27 => Ok(PduType::ReceiverPdu),
            129 => Ok(PduType::AnnounceObjectPdu),
            130 => Ok(PduType::DeleteObjectPdu),
            131 => Ok(PduType::DescribeApplicationPdu),
            132 => Ok(PduType::DescribeEventPdu),
            133 => Ok(PduType::DescribeObjectPdu),
            134 => Ok(PduType::RequestEventPdu),
            135 => Ok(PduType::RequestObjectPdu),
            140 => Ok(PduType::TimeSpacePositionIndicatorFIPdu),
            141 => Ok(PduType::AppearanceFIPdu),
            142 => Ok(PduType::ArticulatedPartsFIPdu),
            143 => Ok(PduType::FireFIPdu),
            144 => Ok(PduType::DetonationFIPdu),
            150 => Ok(PduType::PointObjectStatePdu),
            151 => Ok(PduType::LinearObjectStatePdu),
            152 => Ok(PduType::ArealObjectStatePdu),
            153 => Ok(PduType::EnvironmentPdu),
            155 => Ok(PduType::TransferControlRequestPdu),
            156 => Ok(PduType::TransferControlPdu),
            157 => Ok(PduType::TransferControlAcknowledgePdu),
            160 => Ok(PduType::IntercomControlPdu),
            161 => Ok(PduType::IntercomSignalPdu),
            170 => Ok(PduType::AggregatePdu),
            undefined_enum => Err(DisError::InvalidPduTypeValue(undefined_enum)),
        }
    }
}

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
}

impl TryFrom<u8> for ProtocolFamily {
    type Error = DisError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProtocolFamily::Other),
            1 => Ok(ProtocolFamily::EntityInformationInteraction),
            2 => Ok(ProtocolFamily::Warfare),
            3 => Ok(ProtocolFamily::Logistics),
            4 => Ok(ProtocolFamily::RadioCommunication),
            5 => Ok(ProtocolFamily::SimulationManagement),
            6 => Ok(ProtocolFamily::DistributedEmissionRegeneration),
            129 => Ok(ProtocolFamily::ExperimentalCGF),
            130 => Ok(ProtocolFamily::ExperimentalEntityInteractionInformationFieldInstrumentation),
            131 => Ok(ProtocolFamily::ExperimentalWarfareFieldInstrumentation),
            132 => Ok(ProtocolFamily::ExperimentalEnvironmentObjectInformationInteraction),
            133 => Ok(ProtocolFamily::ExperimentalEntityManagement),
            undefined_enum => Err(DisError::InvalidProtocolFamilyValue(undefined_enum)),
        }
    }
}

pub enum Pdu {
    Other, // No implementation for Other PDU Type
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