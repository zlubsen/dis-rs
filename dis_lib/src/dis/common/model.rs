use dis_rs_macros::PduConversion;

pub const PDU_HEADER_LEN_BYTES: usize = 12;

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
    Ieee1278_1a1998 = 6,
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

impl Default for ProtocolFamily {
    fn default() -> Self {
        ProtocolFamily::Other
    }
}

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

#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
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

impl Default for PduType {
    fn default() -> Self {
        PduType::OtherPdu
    }
}

