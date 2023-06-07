use crate::common::iff::model::{ChangeOptionsRecord, LayersPresenceApplicability, ParameterCapable};
use crate::constants::{BIT_0_IN_BYTE, BIT_1_IN_BYTE, BIT_2_IN_BYTE, BIT_3_IN_BYTE, BIT_4_IN_BYTE, BIT_5_IN_BYTE, BIT_6_IN_BYTE, BIT_7_IN_BYTE};

impl From<u8> for ParameterCapable {
    fn from(value: u8) -> Self {
        match value {
            0 => ParameterCapable::Capable,
            _ => ParameterCapable::NotCapable,
        }
    }
}

impl From<u8> for ChangeOptionsRecord {
    fn from(value: u8) -> Self {
        ChangeOptionsRecord {
            change_indicator: ((value & BIT_0_IN_BYTE) >> 7) == 1u8,
            system_specific_field_1: ((value & BIT_1_IN_BYTE) >> 6) == 1u8,
            system_specific_field_2: ((value & BIT_2_IN_BYTE) >> 5) == 1u8,
            heartbeat_indicator: ((value & BIT_3_IN_BYTE) >> 4) == 1u8,
            transponder_interrogator_indicator: ((value & BIT_4_IN_BYTE) >> 3) == 1u8,
            simulation_mode: ((value & BIT_5_IN_BYTE) >> 2) == 1u8,
            interactive_capable: ((value & BIT_6_IN_BYTE) >> 1) == 1u8,
            test_mode: (value & BIT_7_IN_BYTE) == 1u8,
        }
    }
}

impl From<u8> for LayersPresenceApplicability {
    fn from(value: u8) -> Self {
        match value {
            0 => LayersPresenceApplicability::NotPresentApplicable,
            _ => LayersPresenceApplicability::PresentApplicable,
        }
    }
}