use crate::dis::common::model::{PduType, ProtocolFamily, ProtocolVersion};

#[derive(Copy, Clone, Debug)]
pub struct PduHeader {
    pub protocol_version : ProtocolVersion,
    pub exercise_id : u8,
    pub pdu_type : PduType,
    pub protocol_family : ProtocolFamily,
    pub time_stamp : u32,
    pub pdu_length : u16,
    pub pdu_status : PduStatus,
    pub padding : u8,
}

pub struct PduStatus {
    pub transferred_entity_indicator: TransferredEntityIndicator,
    pub lvc_indicator : LvcIndicator,
    pub coupled_extension_indicator : CoupledExtensionIndicator,
    pub fire_type_indicator : FireTypeIndicator,
    pub detonation_type_indicator : DetonationTypeIndicator,
    pub radio_attached_indicator : RadioAttachedIndicator,
    pub intercom_attached_indicator : IntercomAttachedIndicator,
    pub iff_simulation_mode : IffSimulationMode,
    pub active_interrogation_indicator : ActiveInterrogationIndicator,
}

pub enum TransferredEntityIndicator {}
pub enum LvcIndicator {}
pub enum CoupledExtensionIndicator {}
pub enum FireTypeIndicator {}
pub enum DetonationTypeIndicator {}
pub enum RadioAttachedIndicator {}
pub enum IntercomAttachedIndicator {}
pub enum IffSimulationMode {}
pub enum ActiveInterrogationIndicator {}

pub enum Pdu {
    Other, // No implementation for Other PDU Type
//    EntityState(EntityState),
// TODO implement other PDU structs
}