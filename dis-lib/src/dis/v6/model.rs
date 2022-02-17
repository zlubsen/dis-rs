use crate::dis::common::model::{PduType, ProtocolFamily, ProtocolVersion};
use crate::dis::v6::builder::PduHeaderBuilder;
use crate::dis::v6::other::model::Other;
use super::entity_state::model::EntityState;

// TODO re-export the PDU types
// TODO review pub settings in PDU modules

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