mod entity_state;
// TODO define different PDU types in separate modules

// TODO re-export the PDU types
// TODO review pub settings in PDU modules
pub use entity_state::EntityState;

struct PduHeader {
    protocol_version : ProtocolVersion, // enum
    exercise_id : u8,
    pdu_type : PduType, // enum
    protocol_family : ProtocolFamily, // enum
    time_stamp : u32,
    pdu_length : u16,
    padding_field : u16,
}

enum ProtocolVersion {
    Other = 0,
    VERSION_1_0_MAY_92 = 1,             // DIS PDU version 1.0 (May 92)
    IEEE_1278_1993 = 2,                 // IEEE 1278-1993
    VERSION_2_0_THIRD_DRAFT = 3,        // DIS PDU version 2.0 - third draft (May 93)
    VERSION_2_0_FOURTH_DRAFT = 4,       // DIS PDU version 2.0 - fourth draft (revised) March 16, 1994
    IEEE_1278_1_1995 = 5,               // IEEE 1278.1-1995
}

enum PduType {
    Other = 0,
    EntityState = 1,
    Fire = 2,
    Detonation = 3,
    Collision = 4,
    ServiceRequest = 5,
    ResupplyOffer = 6,
    ResupplyReceived = 7,
    ResupplyCancel = 8,
    RepairComplete = 9,
    RepairResponse = 10,
    CreateEntity = 11,
    RemoveEntity = 12,
    StartResume = 13,
    StopFreeze = 14,
    Acknowledge = 15,
    ActionRequest = 16,
    ActionResponse = 17,
    DataQuery = 18,
    SetData = 19,
    Data = 20,
    EventReport = 21,
    Comment = 22,
    ElectromagneticEmission = 23,
    Designator = 24,
    Transmitter = 25,
    Signal = 26,
    Receiver = 27,
    AnnounceObject = 129,
    DeleteObject = 130,
    DescribeApplication = 131,
    DescribeEvent = 132,
    DescribeObject = 133,
    RequestEvent = 134,
    RequestObject = 135,
    TimeSpacePositionIndicatorFI = 140,
    AppearanceFI = 141,
    ArticulatedPartsFI = 142,
    FireFI = 143,
    DetonationFI = 144,
    PointObjectState = 150,
    LinearObjectState = 151,
    ArealObjectState = 152,
    Environment = 153,
    TransferControlRequest = 155,
    TransferControl = 156,
    TransferControlAcknowledge = 157,
    IntercomControl = 160,
    IntercomSignal = 161,
    Aggregate = 170,
}

enum ProtocolFamily {
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