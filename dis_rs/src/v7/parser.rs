use nom::IResult;
use nom::number::complete::be_u8;
use crate::enumerations::PduType;
use crate::v7::builder::{build_pdu_status_aii_ism_cei_lvc_tei, build_pdu_status_cei_lvc, build_pdu_status_cei_lvc_tei, build_pdu_status_dti_cei_lvc, build_pdu_status_fti_cei_lvc, build_pdu_status_iai_cei_lvc_tei, build_pdu_status_lvc, build_pdu_status_rai_cei_lvc_tei};
use crate::v7::model::{ActiveInterrogationIndicator, CoupledExtensionIndicator, DetonationTypeIndicator, FireTypeIndicator, IffSimulationMode, IntercomAttachedIndicator, LvcIndicator, PduStatus, RadioAttachedIndicator, TransferredEntityIndicator};

pub fn parse_pdu_status(pdu_type: PduType) -> impl Fn(&[u8]) -> IResult<&[u8], (PduStatus, u16)> {
    move | input: &[u8] | {
        let type_u8 : u8 = pdu_type.into();
        let (input, status) = be_u8(input)?;
        let (input, padding) = be_u8(input)?;
        Ok(
            (input,
             (parse_pdu_status_fields(type_u8, status), padding as u16)
            )
        )
    }
}

/// Parses the pdu status sub-fields into a PduStatus struct, depending on the PduType
///
/// Note: parser should not be fed input that is consumed, but a copy of the pdu status byte (as it is already parsed earlier)
pub fn parse_pdu_status_fields(pdu_type: u8, input : u8) -> PduStatus {
    let tei = status_tei(input);
    let lvc = status_lvc(input);
    let cei = status_cei(input);
    let fti = status_fti(input);
    let dti = status_dti(input);
    let rai = status_rai(input);
    let iai = status_iai(input);
    let ism = status_ism(input);
    let aii = status_aii(input);

    match pdu_type {
        1 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        2 => { build_pdu_status_fti_cei_lvc(fti, cei, lvc) }
        3 => { build_pdu_status_dti_cei_lvc(dti, cei, lvc) }
        4..=22 => { build_pdu_status_cei_lvc(cei, lvc) }
        23 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        24 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        25 => { build_pdu_status_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        26 => { build_pdu_status_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        27 => { build_pdu_status_rai_cei_lvc_tei(rai, cei, lvc, tei) }
        28 => { build_pdu_status_aii_ism_cei_lvc_tei(aii, ism, cei, lvc, tei) }
        29..=30 => { build_pdu_status_cei_lvc(cei, lvc) }
        31 => { build_pdu_status_iai_cei_lvc_tei(iai, cei, lvc, tei) }
        32 => { build_pdu_status_iai_cei_lvc_tei(iai, cei, lvc, tei) }
        33..=40 => { build_pdu_status_cei_lvc(cei, lvc) }
        41 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        42..=66 => { build_pdu_status_cei_lvc(cei, lvc) }
        67 => { build_pdu_status_cei_lvc_tei(cei, lvc, tei) }
        68 => { build_pdu_status_cei_lvc(cei, lvc) }
        69 => { build_pdu_status_cei_lvc(cei, lvc) }
        70 => { build_pdu_status_cei_lvc(cei, lvc) }
        71 => { build_pdu_status_cei_lvc(cei, lvc) }
        72 => { build_pdu_status_lvc(lvc) }
        73..=255 | _ => {
            PduStatus {
                transferred_entity_indicator: None,
                lvc_indicator: None,
                coupled_extension_indicator: None,
                fire_type_indicator: None,
                detonation_type_indicator: None,
                radio_attached_indicator: None,
                intercom_attached_indicator: None,
                iff_simulation_mode: None,
                active_interrogation_indicator: None
            }
        }
    }
}

fn status_tei(pdu_status_field : u8) -> TransferredEntityIndicator {
    let tei = pdu_status_field & 0x01;
    TransferredEntityIndicator::from(tei)
}

fn status_lvc(pdu_status_field : u8) -> LvcIndicator {
    let lvc = pdu_status_field & 0x06;
    LvcIndicator::from(lvc)
}

fn status_cei(pdu_status_field : u8) -> CoupledExtensionIndicator {
    let cei = pdu_status_field & 0x08;
    CoupledExtensionIndicator::from(cei)
}

fn status_fti(pdu_status_field : u8) -> FireTypeIndicator {
    let fti = pdu_status_field & 0x10;
    FireTypeIndicator::from(fti)
}

fn status_dti(pdu_status_field : u8) -> DetonationTypeIndicator {
    let dti = pdu_status_field & 0x30;
    DetonationTypeIndicator::from(dti)
}

fn status_rai(pdu_status_field : u8) -> RadioAttachedIndicator {
    let rai = pdu_status_field & 0x30;
    RadioAttachedIndicator::from(rai)
}

fn status_iai(pdu_status_field : u8) -> IntercomAttachedIndicator {
    let iai = pdu_status_field & 0x30;
    IntercomAttachedIndicator::from(iai)
}

fn status_ism(pdu_status_field : u8) -> IffSimulationMode {
    let ism = pdu_status_field & 0x10;
    IffSimulationMode::from(ism)
}

fn status_aii(pdu_status_field : u8) -> ActiveInterrogationIndicator {
    let aii = pdu_status_field & 0x20;
    ActiveInterrogationIndicator::from(aii)
}

// TODO parse test for PduStatus field