use bytes::{BufMut, BytesMut};
use crate::common::iff::model::{ChangeOptionsRecord, FundamentalOperationalData, Iff, IffDataRecord, IffDataSpecification, IffFundamentalParameterData, IffLayer2, IffLayer3, IffLayer4, IffLayer5, InformationLayers, LayerHeader, LayersPresenceApplicability, Mode5BasicData, Mode5InterrogatorBasicData, Mode5InterrogatorStatus, Mode5MessageFormats, Mode5TransponderBasicData, ModeSInterrogatorBasicData, ModeSTransponderBasicData, ParameterCapable, SystemId, SystemSpecificData, SystemStatus};
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::{BIT_0_IN_BYTE, BIT_1_IN_BYTE, BIT_2_IN_BYTE, BIT_3_IN_BYTE, BIT_4_IN_BYTE, BIT_5_IN_BYTE, BIT_6_IN_BYTE, BIT_7_IN_BYTE, FOUR_OCTETS, ONE_OCTET, SIX_OCTETS, THREE_OCTETS};
use crate::length_padded_to_num_bytes;

impl SerializePdu for Iff {
    fn serialize_pdu(&self, _version: SupportedVersion, buf: &mut BytesMut) -> u16 {
        let entity_id_bytes = self.emitting_entity_id.serialize(buf);
        let event_id_bytes = self.event_id.serialize(buf);
        let antenna_location_bytes = self.relative_antenna_location.serialize(buf);
        let system_id_bytes = self.system_id.serialize(buf);
        buf.put_u8(self.system_designator);
        buf.put_u8(self.system_specific_data);
        let fundamental_data_bytes = self.fundamental_operational_data.serialize(buf);

        let layer_2_bytes = if let Some(layer_2) = &self.layer_2 {
            layer_2.serialize(buf)
        } else { 0 };
        let layer_3_bytes = if let Some(layer_3) = &self.layer_3 {
            layer_3.serialize(buf)
        } else { 0 };
        let layer_4_bytes = if let Some(layer_4) = &self.layer_4 {
            layer_4.serialize(buf)
        } else { 0 };
        let layer_5_bytes = if let Some(layer_5) = &self.layer_5 {
            layer_5.serialize(buf)
        } else { 0 };

        entity_id_bytes + event_id_bytes + antenna_location_bytes
            + system_id_bytes + 2 + fundamental_data_bytes
            + layer_2_bytes
            + layer_3_bytes
            + layer_4_bytes
            + layer_5_bytes
    }
}

impl Serialize for SystemId {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.system_type.into());
        buf.put_u16(self.system_name.into());
        buf.put_u8(self.system_mode.into());
        let _ = self.change_options.serialize(buf);

        SIX_OCTETS as u16
    }
}

impl Serialize for FundamentalOperationalData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let system_status_bytes = self.system_status.serialize(buf);
        buf.put_u8(self.data_field_1);
        let information_layers_bytes = self.information_layers.serialize(buf);
        buf.put_u8(self.data_field_2);
        buf.put_u16(self.parameter_1);
        buf.put_u16(self.parameter_2);
        buf.put_u16(self.parameter_3);
        buf.put_u16(self.parameter_4);
        buf.put_u16(self.parameter_5);
        buf.put_u16(self.parameter_6);

        system_status_bytes + information_layers_bytes + 14
    }
}

impl Serialize for SystemStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let system_on_off_status = u8::from(self.system_on_off_status) << 7;
        let parameter_1 = u8::from(&self.parameter_1_capable) << 6;
        let parameter_2 = u8::from(&self.parameter_2_capable) << 5;
        let parameter_3 = u8::from(&self.parameter_3_capable) << 4;
        let parameter_4 = u8::from(&self.parameter_4_capable) << 3;
        let parameter_5 = u8::from(&self.parameter_5_capable) << 2;
        let parameter_6 = u8::from(&self.parameter_6_capable) << 1;
        let operational_status = u8::from(&self.operational_status);
        buf.put_u8(system_on_off_status | parameter_1 | parameter_2 | parameter_3 | parameter_4 | parameter_5 | parameter_6 | operational_status);

        ONE_OCTET as u16
    }
}

impl Serialize for InformationLayers {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let layer_1 = u8::from(&self.layer_1) << 6;
        let layer_2 = u8::from(&self.layer_2) << 5;
        let layer_3 = u8::from(&self.layer_3) << 4;
        let layer_4 = u8::from(&self.layer_4) << 3;
        let layer_5 = u8::from(&self.layer_5) << 2;
        let layer_6 = u8::from(&self.layer_6) << 1;
        let layer_7 = u8::from(&self.layer_7);

        buf.put_u8(layer_1 | layer_2 | layer_3 | layer_4 | layer_5 | layer_6 | layer_7);

        ONE_OCTET as u16
    }
}

impl From<&ParameterCapable> for u8 {
    fn from(value: &ParameterCapable) -> Self {
        match value {
            ParameterCapable::Capable => 0,
            ParameterCapable::NotCapable => 1,
        }
    }
}

impl From<&LayersPresenceApplicability> for u8 {
    fn from(value: &LayersPresenceApplicability) -> Self {
        match value {
            LayersPresenceApplicability::NotPresentApplicable => { 0 }
            LayersPresenceApplicability::PresentApplicable => { 1 }
        }
    }
}

impl Serialize for ChangeOptionsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let mut byte = 0u8;
        if self.change_indicator {
            byte = byte + BIT_0_IN_BYTE;
        }
        if self.system_specific_field_1 {
            byte = byte + BIT_1_IN_BYTE;
        }
        if self.system_specific_field_2 {
            byte = byte + BIT_2_IN_BYTE;
        }
        if self.heartbeat_indicator {
            byte = byte + BIT_3_IN_BYTE;
        }
        if self.transponder_interrogator_indicator {
            byte = byte + BIT_4_IN_BYTE;
        }
        if self.simulation_mode {
            byte = byte + BIT_5_IN_BYTE;
        }
        if self.interactive_capable {
            byte = byte + BIT_6_IN_BYTE;
        }
        if self.test_mode {
            byte = byte + BIT_7_IN_BYTE;
        }
        buf.put_u8(byte);

        ONE_OCTET as u16
    }
}

impl Serialize for IffLayer2 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let layer_header_bytes = self.layer_header.serialize(buf);
        let beam_data_bytes = self.beam_data.serialize(buf);
        buf.put_u8(self.operational_parameter_1);
        buf.put_u8(self.operational_parameter_2);
        buf.put_u16(self.iff_fundamental_parameters.len() as u16);
        let params_bytes: u16 = self.iff_fundamental_parameters.iter().map(|param| param.serialize(buf)).sum();

        layer_header_bytes + beam_data_bytes + 4 + params_bytes
    }
}

impl Serialize for IffLayer3 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let layer_header_bytes = self.layer_header.serialize(buf);
        let reporting_simulation_bytes = self.reporting_simulation.serialize(buf);
        let basic_data_bytes = match &self.mode_5_basic_data {
            Mode5BasicData::Interrogator(data) => { data.serialize(buf) }
            Mode5BasicData::Transponder(data) => { data.serialize(buf) }
        };
        buf.put_u16(0u16);
        let iff_data_specification_bytes = self.iff_data_specification.serialize(buf);

        layer_header_bytes + reporting_simulation_bytes + basic_data_bytes + 2 + iff_data_specification_bytes
    }
}

impl Serialize for IffLayer4 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for IffLayer5 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for LayerHeader {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.layer_number);
        buf.put_u8(self.layer_specific_information);
        buf.put_u16(self.length);

        FOUR_OCTETS as u16
    }
}

impl Serialize for IffFundamentalParameterData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.erp);
        buf.put_f32(self.frequency);
        buf.put_f32(self.frequency);
        buf.put_f32(self.frequency);
        buf.put_f32(self.frequency);
        buf.put_u8(self.applicable_modes.into());
        let system_specific_data_bytes = self.system_specific_data.serialize(buf);

        21 + system_specific_data_bytes
    }
}

impl Serialize for SystemSpecificData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.part_1);
        buf.put_u8(self.part_2);
        buf.put_u8(self.part_3);

        THREE_OCTETS as u16
    }
}

impl Serialize for IffDataSpecification {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.iff_data_records.len() as u16);
        let records_bytes: u16 = self.iff_data_records.iter().map(|record| record.serialize(buf) ).sum();

        2 + records_bytes
    }
}

impl Serialize for IffDataRecord {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let padded_record_lengths = length_padded_to_num_bytes(
            SIX_OCTETS + self.record_specific_fields.len(),
            FOUR_OCTETS);
        let record_length_bytes = padded_record_lengths.record_length_bytes as u16;

        buf.put_u32(self.record_type.into());
        buf.put_u16(record_length_bytes);
        buf.put(&*self.record_specific_fields);
        buf.put_bytes(0u8, padded_record_lengths.padding_length_bytes);

        record_length_bytes
    }
}

impl Serialize for Mode5TransponderBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!();

        16
    }
}

impl Serialize for Mode5InterrogatorBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        self.status.serialize(buf);
        buf.put_u8(0u8);
        buf.put_u16(0u16);
        self.mode_5_message_formats_present.serialize(buf);
        self.interrogated_entity_id.serialize(buf);
        buf.put_u16(0u16);

        16
    }
}

impl Serialize for ModeSTransponderBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()

        24
    }
}

impl Serialize for ModeSInterrogatorBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()

        24
    }
}

impl Serialize for Mode5InterrogatorStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for ModeSTransponderBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for ModeSInterrogatorBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}

impl Serialize for Mode5MessageFormats {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        todo!()
    }
}
