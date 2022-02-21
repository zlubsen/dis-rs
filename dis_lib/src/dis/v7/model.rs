use dis_rs_macros::PduConversion;
use crate::dis::common::model::{PduType, ProtocolFamily, ProtocolVersion};
use crate::dis::v7::other::model::Other;

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

#[derive(Copy, Clone, Debug)]
pub struct PduStatus {
    pub transferred_entity_indicator: Option<TransferredEntityIndicator>,
    pub lvc_indicator : Option<LvcIndicator>,
    pub coupled_extension_indicator : Option<CoupledExtensionIndicator>,
    pub fire_type_indicator : Option<FireTypeIndicator>,
    pub detonation_type_indicator : Option<DetonationTypeIndicator>,
    pub radio_attached_indicator : Option<RadioAttachedIndicator>,
    pub intercom_attached_indicator : Option<IntercomAttachedIndicator>,
    pub iff_simulation_mode : Option<IffSimulationMode>,
    pub active_interrogation_indicator : Option<ActiveInterrogationIndicator>,
}

impl Default for PduStatus {
    fn default() -> Self {
        PduStatus {
            transferred_entity_indicator: None,
            lvc_indicator: None,
            coupled_extension_indicator: None,
            fire_type_indicator: None,
            detonation_type_indicator: None,
            radio_attached_indicator: None,
            intercom_attached_indicator: None,
            iff_simulation_mode: None,
            active_interrogation_indicator: None,
        }
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum TransferredEntityIndicator {
    NoDifference = 0,
    Difference = 1,
}

impl Default for TransferredEntityIndicator {
    fn default() -> Self {
        TransferredEntityIndicator::NoDifference
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum LvcIndicator {
    NoStatement = 0,
    Live = 1,
    Virtual = 2,
    Constructive = 3,
}

impl Default for LvcIndicator {
    fn default() -> Self {
        LvcIndicator::NoStatement
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum CoupledExtensionIndicator {
    NotCoupled = 0,
    Coupled = 1,
}

impl Default for CoupledExtensionIndicator {
    fn default() -> Self {
        CoupledExtensionIndicator::Coupled
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum FireTypeIndicator {
    Munition = 0,
    Expendable = 1,
}

impl Default for FireTypeIndicator {
    fn default() -> Self {
        FireTypeIndicator::Munition
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum DetonationTypeIndicator {
    Munition = 0,
    Expandable = 1,
    NonMunitionExplosion = 2,
}

impl Default for DetonationTypeIndicator {
    fn default() -> Self {
        DetonationTypeIndicator::Munition
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum RadioAttachedIndicator {
    NoStatement = 0,
    Unattached = 1,
    Attached = 2,
}

impl Default for RadioAttachedIndicator {
    fn default() -> Self {
        RadioAttachedIndicator::NoStatement
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum IntercomAttachedIndicator {
    NoStatement = 0,
    Unattached = 1,
    Attached = 2,
}

impl Default for IntercomAttachedIndicator {
    fn default() -> Self {
        IntercomAttachedIndicator::NoStatement
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum IffSimulationMode {
    Regeneration = 0,
    Interactive = 1,
}

impl Default for IffSimulationMode {
    fn default() -> Self {
        IffSimulationMode::Regeneration
    }
}

#[derive(Copy, Clone, Debug, PduConversion)]
#[repr(u8)]
pub enum ActiveInterrogationIndicator {
    NotActive = 0,
    Active = 1,
}

impl Default for ActiveInterrogationIndicator {
    fn default() -> Self {
        ActiveInterrogationIndicator::NotActive
    }
}

pub fn build_pdu_status_cei_lvc_tei(cei : CoupledExtensionIndicator, lvc : LvcIndicator, tei: TransferredEntityIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: Some(tei),
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: Some(cei),
        fire_type_indicator: None,
        detonation_type_indicator: None,
        radio_attached_indicator: None,
        intercom_attached_indicator: None,
        iff_simulation_mode: None,
        active_interrogation_indicator: None
    }
}

pub fn build_pdu_status_fti_cei_lvc(fti : FireTypeIndicator, cei : CoupledExtensionIndicator, lvc : LvcIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: None,
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: Some(cei),
        fire_type_indicator: Some(fti),
        detonation_type_indicator: None,
        radio_attached_indicator: None,
        intercom_attached_indicator: None,
        iff_simulation_mode: None,
        active_interrogation_indicator: None
    }
}

pub fn build_pdu_status_dti_cei_lvc(dti : DetonationTypeIndicator, cei : CoupledExtensionIndicator, lvc : LvcIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: None,
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: Some(cei),
        fire_type_indicator: None,
        detonation_type_indicator: Some(dti),
        radio_attached_indicator: None,
        intercom_attached_indicator: None,
        iff_simulation_mode: None,
        active_interrogation_indicator: None
    }
}

pub fn build_pdu_status_cei_lvc(cei : CoupledExtensionIndicator, lvc : LvcIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: None,
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: Some(cei),
        fire_type_indicator: None,
        detonation_type_indicator: None,
        radio_attached_indicator: None,
        intercom_attached_indicator: None,
        iff_simulation_mode: None,
        active_interrogation_indicator: None
    }
}

pub fn build_pdu_status_rai_cei_lvc_tei(rai : RadioAttachedIndicator, cei : CoupledExtensionIndicator, lvc : LvcIndicator, tei: TransferredEntityIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: Some(tei),
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: Some(cei),
        fire_type_indicator: None,
        detonation_type_indicator: None,
        radio_attached_indicator: Some(rai),
        intercom_attached_indicator: None,
        iff_simulation_mode: None,
        active_interrogation_indicator: None
    }
}

pub fn build_pdu_status_aii_ism_cei_lvc_tei(aii : ActiveInterrogationIndicator, ism : IffSimulationMode, cei : CoupledExtensionIndicator, lvc : LvcIndicator, tei: TransferredEntityIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: Some(tei),
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: Some(cei),
        fire_type_indicator: None,
        detonation_type_indicator: None,
        radio_attached_indicator: None,
        intercom_attached_indicator: None,
        iff_simulation_mode: Some(ism),
        active_interrogation_indicator: Some(aii)
    }
}

pub fn build_pdu_status_iai_cei_lvc_tei(iai : IntercomAttachedIndicator, cei : CoupledExtensionIndicator, lvc : LvcIndicator, tei: TransferredEntityIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: Some(tei),
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: Some(cei),
        fire_type_indicator: None,
        detonation_type_indicator: None,
        radio_attached_indicator: None,
        intercom_attached_indicator: Some(iai),
        iff_simulation_mode: None,
        active_interrogation_indicator: None
    }
}

pub fn build_pdu_status_lvc(lvc : LvcIndicator) -> PduStatus {
    PduStatus {
        transferred_entity_indicator: None,
        lvc_indicator: Some(lvc),
        coupled_extension_indicator: None,
        fire_type_indicator: None,
        detonation_type_indicator: None,
        radio_attached_indicator: None,
        intercom_attached_indicator: None,
        iff_simulation_mode: None,
        active_interrogation_indicator: None
    }
}

pub enum Pdu {
    Other(Other), // No implementation for Other PDU Type
//    EntityState(EntityState),
// TODO implement other PDU structs
}