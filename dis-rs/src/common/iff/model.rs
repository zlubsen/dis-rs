use crate::common::{BodyInfo, Interaction};
use crate::common::model::BeamData;
use crate::common::model::{EntityId, EventId, VectorF32};
use crate::enumerations::{PduType, IffSystemType, IffSystemMode, IffSystemName};

pub const FUNDAMENTAL_OPERATIONAL_DATA_LENGTH: u16 = 16;

pub struct Iff {
    pub emitting_entity_id: EntityId,
    pub event_id: EventId,
    pub relative_antenna_location: VectorF32,
    pub system_id: SystemId,
    pub system_designator: u8,
    pub system_specific_data: u8, // TODO 8-bit record defined by system type
    pub fundamental_operational_data: FundamentalOperationalData,
    pub layer_2: Option<IffLayer2>, // TODO
    pub layer_3: Option<IffLayer3>, // TODO
    pub layer_4: Option<IffLayer4>, // TODO
    pub layer_5: Option<IffLayer5>, // TODO
}

impl Default for Iff {
    fn default() -> Self {
        Self::new()
    }
}

impl Iff {
    pub fn new() -> Self {
        Self {
            emitting_entity_id: Default::default(),
            event_id: Default::default(),
            relative_antenna_location: Default::default(),
            system_id: SystemId::default(),
            system_designator: 0,
            system_specific_data: 0,
            fundamental_operational_data: FundamentalOperationalData::default(),
            layer_2: None,
            layer_3: None,
            layer_4: None,
            layer_5: None,
        }
    }
}

impl BodyInfo for Iff {
    fn body_length(&self) -> u16 {
        todo!()
    }

    fn body_type(&self) -> PduType {
        PduType::IFF
    }
}

impl Interaction for Iff {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.emitting_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        // TODO can we derive the receiving entity from a layer?
        None
    }
}

pub struct SystemId {
    pub system_type: IffSystemType,
    pub system_name: IffSystemName,
    pub system_mode: IffSystemMode,
    pub change_options: ChangeOptionsRecord,
}

impl Default for SystemId {
    fn default() -> Self {
        SystemId {
            system_type: Default::default(),
            system_name: Default::default(),
            system_mode: Default::default(),
            change_options: ChangeOptionsRecord::default(),
        }
    }
}

pub struct ChangeOptionsRecord {
    pub change_indicator: bool,
    pub system_specific_field_1: bool,
    pub system_specific_field_2: bool,
    pub heartbeat_indicator: bool,
    pub transponder_interrogator_indicator: bool,
    pub simulation_mode: bool,
    pub interactive_capable: bool,
    pub test_mode: bool,
}

impl Default for ChangeOptionsRecord {
    fn default() -> Self {
        ChangeOptionsRecord {
            change_indicator: false,
            system_specific_field_1: false,
            system_specific_field_2: false,
            heartbeat_indicator: false,
            transponder_interrogator_indicator: false,
            simulation_mode: false,
            interactive_capable: false,
            test_mode: false,
        }
    }
}

pub struct FundamentalOperationalData {
    pub system_status: SystemStatus,
    pub data_field_1: u8,
    pub information_layers: InformationLayers,
    pub data_field_2: u8,
    pub parameter_1: u16,
    pub parameter_2: u16,
    pub parameter_3: u16,
    pub parameter_4: u16,
    pub parameter_5: u16,
    pub parameter_6: u16,
    pub parameter_7: u16,
}

impl Default for FundamentalOperationalData {
    fn default() -> Self {
        FundamentalOperationalData {
            system_status: SystemStatus::default(),
            data_field_1: 0,
            information_layers: InformationLayers::default(),
            data_field_2: 0,
            parameter_1: 0,
            parameter_2: 0,
            parameter_3: 0,
            parameter_4: 0,
            parameter_5: 0,
            parameter_6: 0,
            parameter_7: 0,
        }
    }
}

pub enum ParameterCapable {
    Capable,
    NotCapable,
}

// TODO use ParameterCapable enum instead of bool
pub struct SystemStatus {
    pub system_on_off_status: bool,
    pub parameter_1_capable: bool,
    pub parameter_2_capable: bool,
    pub parameter_3_capable: bool,
    pub parameter_4_capable: bool,
    pub parameter_5_capable: bool,
    pub parameter_6_capable: bool,
    pub operational_status: bool,
}

impl Default for SystemStatus {
    fn default() -> Self {
        SystemStatus {
            system_on_off_status: false,
            parameter_1_capable: false,
            parameter_2_capable: false,
            parameter_3_capable: false,
            parameter_4_capable: false,
            parameter_5_capable: false,
            parameter_6_capable: false,
            operational_status: false,
        }
    }
}

pub enum LayersPresenceApplicability {
    NotPresentApplicable,
    PresentApplicable,
}

pub struct InformationLayers {
    pub layer_1: LayersPresenceApplicability,
    pub layer_2: LayersPresenceApplicability,
    pub layer_3: LayersPresenceApplicability,
    pub layer_4: LayersPresenceApplicability,
    pub layer_5: LayersPresenceApplicability,
    pub layer_6: LayersPresenceApplicability,
    pub layer_7: LayersPresenceApplicability,
}

impl Default for InformationLayers {
    fn default() -> Self {
        InformationLayers {
            layer_1: LayersPresenceApplicability::NotPresentApplicable,
            layer_2: LayersPresenceApplicability::NotPresentApplicable,
            layer_3: LayersPresenceApplicability::NotPresentApplicable,
            layer_4: LayersPresenceApplicability::NotPresentApplicable,
            layer_5: LayersPresenceApplicability::NotPresentApplicable,
            layer_6: LayersPresenceApplicability::NotPresentApplicable,
            layer_7: LayersPresenceApplicability::NotPresentApplicable,
        }
    }
}

pub struct IffLayer2 {
    pub layer_header: LayerHeader,
    pub beam_data: BeamData,
    pub operational_parameter_1: u8,
    pub operational_parameter_2: u8,
    pub iff_fundamental_parameters: Vec<IffFundamentalParameterData>,
}

pub struct IffFundamentalParameterData {
    // TODO
}

pub struct IffLayer3 {
    // TODO
}

pub struct IffLayer4 {
    // TODO
}

pub struct IffLayer5 {
    // TODO
}

pub struct LayerHeader {
    pub layer_number: u8,
    pub layer_specific_information: u8,
    pub length: u16,
}