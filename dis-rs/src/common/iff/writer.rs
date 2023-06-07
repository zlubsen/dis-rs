use crate::common::iff::model::{ChangeOptionsRecord, LayersPresenceApplicability, ParameterCapable};
use crate::constants::{BIT_0_IN_BYTE, BIT_1_IN_BYTE, BIT_2_IN_BYTE, BIT_3_IN_BYTE, BIT_4_IN_BYTE, BIT_5_IN_BYTE, BIT_6_IN_BYTE, BIT_7_IN_BYTE};

impl From<ParameterCapable> for u8 {
    fn from(value: ParameterCapable) -> Self {
        match value {
            ParameterCapable::Capable => 0,
            ParameterCapable::NotCapable => 1,
        }
    }
}

impl From<ChangeOptionsRecord> for u8 {
    fn from(record: ChangeOptionsRecord) -> Self {
        let mut byte = 0u8;
        if record.change_indicator {
            byte = byte + BIT_0_IN_BYTE;
        }
        if record.system_specific_field_1 {
            byte = byte + BIT_1_IN_BYTE;
        }
        if record.system_specific_field_2 {
            byte = byte + BIT_2_IN_BYTE;
        }
        if record.heartbeat_indicator {
            byte = byte + BIT_3_IN_BYTE;
        }
        if record.transponder_interrogator_indicator {
            byte = byte + BIT_4_IN_BYTE;
        }
        if record.simulation_mode {
            byte = byte + BIT_5_IN_BYTE;
        }
        if record.interactive_capable {
            byte = byte + BIT_6_IN_BYTE;
        }
        if record.test_mode {
            byte = byte + BIT_7_IN_BYTE;
        }
        byte
    }
}

impl From<LayersPresenceApplicability> for u8 {
    fn from(value: LayersPresenceApplicability) -> Self {
        match value {
            LayersPresenceApplicability::NotPresentApplicable => 0,
            LayersPresenceApplicability::PresentApplicable => 1,
        }
    }
}
