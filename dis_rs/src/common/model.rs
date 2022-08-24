use dis_rs_macros::PduConversion;
use crate::common::builder::PduHeaderBuilder;
use crate::common::entity_state::model::EntityState;
use crate::common::Interaction;
use crate::common::other::model::Other;
use crate::{Country, EntityKind};
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
    pub warhead : Warhead,
    pub fuse : Fuse,
    pub quantity : u16,
    pub rate : u16,
}

// TODO enumeration refactoring
#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Warhead {
    Other = 0000,
    Cargo = 0010, //Cargo (Variable Submunitions)
    Fuel_Air_Explosive = 0020,
    Glass_Blads = 0030,
    _1_um = 0031,
    _5_um = 0032,
    _10_um = 0033,
    HE = 1000, // High Explosive
    HE_Plastic = 1100,
    HE_Incendiary = 1200,
    HE_Fragmentation = 1300,
    HE_Antitank = 1400,
    HE_Bomblets = 1500,
    HE_Shaped_Charge = 1600,
    HE_Continuous_Rod = 1610,
    HE_Tungsten_Ball = 1615,
    HE_Blast_Fragmentation = 1620,
    HE_Steerable_Darts_with_HE = 1625,
    HE_Darts = 1630,
    HE_Flechettes = 1635,
    HE_Directed_Fragmentation = 1640,
    HE_Semi_Armor_Piercing = 1645,
    HE_Shaped_Charge_Fragmentation = 1650,
    HE_Semi_Armor_Piercing_Fragmentation = 1655,
    HE_Hallow_Charge = 1660,
    HE_Double_Hallow_Charge = 1665,
    HE_General_Purpose = 1670,
    HE_Blast_Penetrator = 1675,
    HE_Rod_Penetrator = 1680,
    HE_Antipersonnel = 1685,
    Smoke = 2000,
    Illumination = 3000,
    Practice = 4000,
    Kinetic = 5000,
    Mines = 6000,
    Nuclear = 7000,
    Nuclear_IMT = 7010,
    Chemical_General = 8000,
    Chemical_Blister_Agent = 8100,
    HD = 8110,
    Thickened_HD = 8115,
    Dusty_HD = 8120,
    Chemical_Blood_Agent = 8200,
    AC_HCN = 8210,
    CK_CNCI = 8215,
    CG_Phosgene = 8220,
    Chemical_Nerve_Agent = 8300,
    VX = 8310,
    Thickened_VX = 8315,
    Dusty_VX = 8320,
    GA_Tabun = 8325,
    Thickened_GA = 8330,
    Dusty_GA = 8335,
    GB = 8340,
    Thickened_GB = 8345,
    Dusty_GB = 8350,
    GD = 8355,
    Thickened_GD = 8360,
    Dusty_GD = 8365,
    GF = 8370,
    Thickened_GF = 8375,
    Dusty_GF = 8380,
    Biological = 9000,
    Biological_Virus = 9100,
    Biological_Bacteria = 9200,
    Biological_Rickettsia = 9300,
    Biological_Genetically_Modified_Micro_organisms = 9400,
    Biological_Toxin = 9500,
}

impl Default for Warhead {
    fn default() -> Self {
        Warhead::Other
    }
}

// TODO enumeration refactoring
#[derive(Copy, Clone, Debug, PartialEq, PduConversion)]
#[repr(u16)]
pub enum Fuse {
    Other = 0000,
    Intelligent_Influence = 0010,
    Sensor = 0020,
    Self_destruct = 0030,
    Ultra_Quick = 0040,
    Body = 0050,
    Deep_Intrusion = 0060,
    Multifunction = 0100,
    Point_Detonation = 0200,
    Base_Detonation = 0300,
    Contact = 1000,
    Contact_Instant = 1100,
    Contact_Delayed = 1200,
    Contact_Electronic = 1300,
    Contact_Graze = 1400,
    Contact_Crush = 1500,
    Contact_Hydrostatic = 1600,
    Contact_Mechanical = 1700,
    Contact_Chemical = 1800,
    Contact_Piezoelectric = 1900,
    Contact_Point_Initiating = 1910,
    Contact_Point_Initiating_Base_Detonating = 1920,
    Contact_Base_Detonating = 1930,
    Contact_Ballistic_Cap_and_Base = 1940,
    Contact_Base = 1950,
    Contact_Nose = 1960,
    Contact_Fitted_in_Standoff_Probe = 1970,
    Contact_Non_aligned = 1980,
    Timed = 2000,
    Timed_Programmable = 2100,
    Timed_Burnout = 2200,
    Timed_Pyrotechnic = 2300,
    Timed_Electronic = 2400,
    Timed_Base_Delay = 2500,
    Timed_Reinforced_Nose_Impact_Delay = 2600,
    Timed_Short_Delay_Impact = 2700,
    Timed_Nose_Mounted_Variable_Delay = 2800,
    Timed_Long_Delay_Side = 2900,
    Timed_Selectable_Delay = 2910,
    Timed_Impact = 2920,
    Timed_Sequence = 2930,
    Proximity = 3000,
    Proximity_Active_Laser = 3100,
    Proximity_Magnetic = 3200,
    Proximity_Active_Radar = 3300,
    Proximity_Radio_Frequency = 3400,
    Proximity_Programmable = 3500,
    Proximity_Programmable_Prefragmented = 3600,
    Proximity_Infrared = 3700,
    Command = 4000,
    Command_Electronic_Remotely_Set = 4100,
    Altitude = 5000,
    Altitude_Radio_Altimeter = 5100,
    Altitude_Air_Burst = 5200,
    Depth = 6000,
    Acoustic = 7000,
    Pressure = 8000,
    Pressure_Delay = 8010,
    Inert = 8100,
    Dummy = 8110,
    Practice = 8120,
    Plug_Representing = 8130,
    Training = 8150,
    Pyrotechnic = 9000,
    Pyrotechnic_Delay = 9010,
    Electro_optical = 9100,
    Electromechanical = 9110,
    Electromechanical_Nose = 9120,
    Strikerless = 9200,
    Strikerless_Nose_Impact = 9210,
    Strikerless_Compression_Ignition = 9220,
    Compression_Ignition = 9300,
    Compression_Ignition_Strikerless_Nose_Impact = 9310,
    Percussion = 9400,
    Percussion_Instantaneous = 9410,
    Electronic = 9500,
    Electronic_Internally_Mounted = 9510,
    Electronic_Range_Setting = 9520,
    Electronic_Programmed = 9530,
    Mechanical = 9600,
    Mechanical_Nose = 9610,
    Mechanical_Tail = 9620,
}

impl Default for Fuse {
    fn default() -> Self {
        Fuse::Other
    }
}