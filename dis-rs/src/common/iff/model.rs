use crate::common::{BodyInfo, Interaction};
use crate::common::model::{EntityId, EventId, PduType, VectorF32};

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
            system_id: 0,
            system_designator: 0,
            system_specific_data: 0,
            fundamental_operational_data: (), // TODO
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
        Some(&self.originating_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        Some(&self.receiving_id)
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

pub enum ParameterCapable {
    Capable,
    NotCapable,
}

impl From<u8> for ParameterCapable {
    fn from(value: u8) -> Self {
        match value {
            0 => ParameterCapable::Capable,
            _ => ParameterCapable::NotCapable,
        }
    }
}

impl From<ParameterCapable> for u8 {
    fn from(value: ParameterCapable) -> Self {
        match value {
            ParameterCapable::Capable => 0,
            ParameterCapable::NotCapable => 1,
        }
    }
}

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

pub struct InformationLayers {

}