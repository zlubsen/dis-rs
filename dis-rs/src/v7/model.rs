use crate::enumerations::{
    ActiveInterrogationIndicator, CoupledExtensionIndicator, DetonationTypeIndicator,
    FireTypeIndicator, IffSimulationMode, IntercomAttachedIndicator, LvcIndicator,
    RadioAttachedIndicator, TransferredEntityIndicator,
};

/// 5.2.7 PDU status record
///
/// 6.2.67 PDU Status record
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PduStatus {
    pub transferred_entity_indicator: Option<TransferredEntityIndicator>,
    pub lvc_indicator: Option<LvcIndicator>,
    pub coupled_extension_indicator: Option<CoupledExtensionIndicator>,
    pub fire_type_indicator: Option<FireTypeIndicator>,
    pub detonation_type_indicator: Option<DetonationTypeIndicator>,
    pub radio_attached_indicator: Option<RadioAttachedIndicator>,
    pub intercom_attached_indicator: Option<IntercomAttachedIndicator>,
    pub iff_simulation_mode: Option<IffSimulationMode>,
    pub active_interrogation_indicator: Option<ActiveInterrogationIndicator>,
}

#[allow(clippy::derivable_impls)]
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

impl PduStatus {
    #[must_use]
    pub fn with_transferred_entity_indicator(mut self, tei: TransferredEntityIndicator) -> Self {
        self.transferred_entity_indicator = Some(tei);
        self
    }

    #[must_use]
    pub fn with_lvc_indicator(mut self, lvc: LvcIndicator) -> Self {
        self.lvc_indicator = Some(lvc);
        self
    }

    #[must_use]
    pub fn with_coupled_extension_indicator(mut self, cei: CoupledExtensionIndicator) -> Self {
        self.coupled_extension_indicator = Some(cei);
        self
    }

    #[must_use]
    pub fn with_fire_type_indicator(mut self, fti: FireTypeIndicator) -> Self {
        self.fire_type_indicator = Some(fti);
        self
    }

    #[must_use]
    pub fn with_detonation_type_indicator(mut self, dti: DetonationTypeIndicator) -> Self {
        self.detonation_type_indicator = Some(dti);
        self
    }

    #[must_use]
    pub fn with_radio_attached_indicator(mut self, rai: RadioAttachedIndicator) -> Self {
        self.radio_attached_indicator = Some(rai);
        self
    }

    #[must_use]
    pub fn with_intercom_attached_indicator(mut self, iai: IntercomAttachedIndicator) -> Self {
        self.intercom_attached_indicator = Some(iai);
        self
    }

    #[must_use]
    pub fn with_iff_simulation_mode(mut self, ism: IffSimulationMode) -> Self {
        self.iff_simulation_mode = Some(ism);
        self
    }

    #[must_use]
    pub fn with_active_interrogation_indicator(
        mut self,
        aii: ActiveInterrogationIndicator,
    ) -> Self {
        self.active_interrogation_indicator = Some(aii);
        self
    }
}
