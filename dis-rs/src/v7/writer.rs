use bytes::{BufMut, BytesMut};
use crate::enumerations::PduType;
use crate::v7::model::PduStatus;

/// Serialization function for PduStatus. The function formats the 8-bit field for in the `PduHeader`.
/// PduStatus needs the PduType to determine which combination of indicators in the status field to write to the buffer.
/// Therefore it uses a separate function instead of the `Serialize` trait.
pub fn serialize_pdu_status(pdu_status: &PduStatus, pdu_type : &PduType) -> u8 {
    let tei : u8 = if let Some(tei) = pdu_status.transferred_entity_indicator {
        u8::from(tei)
    } else {0u8};
    let lvc : u8 = if let Some(lvc) = pdu_status.lvc_indicator {
        u8::from(lvc) << 1
    } else {0u8};
    let cei : u8 = if let Some(cei) = pdu_status.coupled_extension_indicator {
        u8::from(cei) << 3
    } else {0u8};
    let fti : u8 = if let Some(fti) = pdu_status.fire_type_indicator {
        u8::from(fti) << 4
    } else {0u8};
    let dti : u8 = if let Some(dti) = pdu_status.detonation_type_indicator {
        u8::from(dti) << 4
    } else {0u8};
    let rai : u8 = if let Some(rai) = pdu_status.radio_attached_indicator {
        u8::from(rai) << 4
    } else {0u8};
    let iai : u8 = if let Some(iai) = pdu_status.intercom_attached_indicator {
        u8::from(iai) << 4
    } else {0u8};
    let ism : u8 = if let Some(ism) = pdu_status.iff_simulation_mode {
        u8::from(ism) << 4
    } else {0u8};
    let aii : u8 = if let Some(aii) = pdu_status.active_interrogation_indicator {
        u8::from(aii) << 5
    } else {0u8};
    
    let status_bits = match u8::from(*pdu_type) {
        1 => { combine_cei_lvc_tei(cei, lvc, tei) }
        2 => { combine_fti_cei_lvc(fti, cei, lvc) }
        3 => { combine_dti_cei_lvc(dti, cei, lvc) }
        4..=22 => { combine_cei_lvc(cei, lvc) }
        23 => { combine_cei_lvc_tei(cei, lvc, tei) }
        24 => { combine_cei_lvc_tei(cei, lvc, tei) }
        25 => { combine_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        26 => { combine_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        27 => { combine_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        28 => { combine_aii_ism_cei_lvc_tei(aii, ism, cei, lvc, tei) }
        29..=30 => { combine_cei_lvc(cei, lvc) }
        31 => { combine_iai_cei_lvc_tei(iai, cei, lvc, tei) }
        32 => { combine_iai_cei_lvc_tei(iai, cei, lvc, tei) }
        33..=40 => { combine_cei_lvc(cei, lvc) }
        41 => { combine_cei_lvc_tei(cei, lvc, tei) }
        42..=66 => { combine_cei_lvc(cei, lvc) }
        67 => { combine_cei_lvc_tei(cei, lvc, tei) }
        68 => { combine_cei_lvc(cei, lvc) }
        69 => { combine_cei_lvc(cei, lvc) }
        70 => { combine_cei_lvc(cei, lvc) }
        71 => { combine_cei_lvc(cei, lvc) }
        72 => { combine_lvc(lvc) }
        _ => { // also covers 73..=255
            // default to zeroes for Unspecified PduTypes 
            0u8
        }
    };

    status_bits
}

fn combine_cei_lvc_tei(cei: u8, lvc: u8, tei: u8) -> u8 {
    cei | lvc | tei
}

fn combine_fti_cei_lvc(fti: u8, cei: u8, lvc: u8) -> u8 {
    fti | cei | lvc
}

fn combine_dti_cei_lvc(dti: u8, cei: u8, lvc: u8) -> u8 {
    dti | cei | lvc
}

fn combine_rai_cei_lvc_tei(rai: u8, cei: u8, lvc: u8, tei: u8) -> u8 {
    rai | cei | lvc | tei
}

fn combine_aii_ism_cei_lvc_tei(aii: u8, ism: u8, cei: u8, lvc: u8, tei: u8) -> u8 {
    aii | ism | cei | lvc | tei
}

fn combine_iai_cei_lvc_tei(iai: u8, cei: u8, lvc: u8, tei: u8) -> u8 {
    iai | cei | lvc | tei
}

fn combine_cei_lvc(cei: u8, lvc: u8) -> u8 {
    cei | lvc
}

fn combine_lvc(lvc: u8) -> u8 {
    lvc
}