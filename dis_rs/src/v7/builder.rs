
// TODO create (more) ergonomic builder for PduStatus struct

use crate::v7::model::PduStatus;
use crate::enumerations::{ActiveInterrogationIndicator, CoupledExtensionIndicator, DetonationTypeIndicator, FireTypeIndicator, IffSimulationMode, IntercomAttachedIndicator, LvcIndicator, RadioAttachedIndicator, TransferredEntityIndicator};

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