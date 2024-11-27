use crate::common::iff::model::{
    ChangeOptionsRecord, DamageStatus, DapSource, DapValue, EnabledStatus, EnhancedMode1Code,
    FundamentalOperationalData, Iff, IffDataRecord, IffDataSpecification,
    IffFundamentalParameterData, IffLayer2, IffLayer3, IffLayer4, IffLayer5, IffPresence,
    InformationLayers, LatLonAltSource, LayerHeader, LayersPresenceApplicability,
    MalfunctionStatus, Mode5BasicData, Mode5InterrogatorBasicData, Mode5InterrogatorStatus,
    Mode5MessageFormats, Mode5TransponderBasicData, Mode5TransponderStatus,
    Mode5TransponderSupplementalData, ModeSAltitude, ModeSBasicData, ModeSInterrogatorBasicData,
    ModeSInterrogatorStatus, ModeSLevelsPresent, ModeSTransponderBasicData, ModeSTransponderStatus,
    OnOffStatus, OperationalStatus, ParameterCapable, SquitterStatus, SystemId, SystemSpecificData,
    SystemStatus,
};
use crate::common::model::length_padded_to_num;
use crate::common::{Serialize, SerializePdu, SupportedVersion};
use crate::constants::{
    EIGHT_OCTETS, FOUR_OCTETS, ONE_OCTET, SIX_OCTETS, THREE_OCTETS, TWO_OCTETS,
};
use crate::DisError;
use bytes::{BufMut, BytesMut};

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
        } else {
            0
        };
        let layer_3_bytes = if let Some(layer_3) = &self.layer_3 {
            layer_3.serialize(buf)
        } else {
            0
        };
        let layer_4_bytes = if let Some(layer_4) = &self.layer_4 {
            layer_4.serialize(buf)
        } else {
            0
        };
        let layer_5_bytes = if let Some(layer_5) = &self.layer_5 {
            layer_5.serialize(buf)
        } else {
            0
        };

        entity_id_bytes
            + event_id_bytes
            + antenna_location_bytes
            + system_id_bytes
            + 2
            + fundamental_data_bytes
            + layer_2_bytes
            + layer_3_bytes
            + layer_4_bytes
            + layer_5_bytes
    }
}

impl Serialize for IffLayer2 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let layer_header_bytes = self.layer_header.serialize(buf);
        let beam_data_bytes = self.beam_data.serialize(buf);
        buf.put_u8(self.operational_parameter_1);
        buf.put_u8(self.operational_parameter_2);
        buf.put_u16(self.iff_fundamental_parameters.len() as u16);
        let params_bytes: u16 = self
            .iff_fundamental_parameters
            .iter()
            .map(|param| param.serialize(buf))
            .sum();

        layer_header_bytes + beam_data_bytes + 4 + params_bytes
    }
}

impl Serialize for IffLayer3 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let layer_header_bytes = self.layer_header.serialize(buf);
        let reporting_simulation_bytes = self.reporting_simulation.serialize(buf);
        let basic_data_bytes = match &self.mode_5_basic_data {
            Mode5BasicData::Interrogator(data) => data.serialize(buf),
            Mode5BasicData::Transponder(data) => data.serialize(buf),
        };
        buf.put_u16(0u16);
        let iff_data_specification_bytes = self.data_records.serialize(buf);

        layer_header_bytes
            + reporting_simulation_bytes
            + basic_data_bytes
            + 2
            + iff_data_specification_bytes
    }
}

impl Serialize for IffLayer4 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let layer_header_bytes = self.layer_header.serialize(buf);
        let reporting_simulation_bytes = self.reporting_simulation.serialize(buf);
        let basic_data_bytes = match &self.mode_s_basic_data {
            ModeSBasicData::Interrogator(data) => data.serialize(buf),
            ModeSBasicData::Transponder(data) => data.serialize(buf),
        };
        buf.put_u16(0u16);
        let iff_data_records_bytes = self.data_records.serialize(buf);

        layer_header_bytes
            + reporting_simulation_bytes
            + basic_data_bytes
            + 2
            + iff_data_records_bytes
    }
}

impl Serialize for IffLayer5 {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let layer_header_bytes = self.layer_header.serialize(buf);
        let reporting_simulation_bytes = self.reporting_simulation.serialize(buf);
        buf.put_u16(0u16);
        let applicable_layers_bytes = self.applicable_layers.serialize(buf);
        buf.put_u8(self.data_category.into());
        buf.put_u16(0u16);
        let data_records_bytes = self.data_records.serialize(buf);

        layer_header_bytes
            + reporting_simulation_bytes
            + 2
            + applicable_layers_bytes
            + 3
            + data_records_bytes
    }
}

impl Serialize for ChangeOptionsRecord {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let byte: u8 = self.into();
        buf.put_u8(byte);

        ONE_OCTET as u16
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

impl From<&ParameterCapable> for u8 {
    fn from(value: &ParameterCapable) -> Self {
        match value {
            ParameterCapable::Capable => 0,
            ParameterCapable::NotCapable => 1,
        }
    }
}

impl From<&OperationalStatus> for u8 {
    fn from(value: &OperationalStatus) -> Self {
        match value {
            OperationalStatus::Operational => 0,
            OperationalStatus::SystemFailed => 1,
        }
    }
}

impl From<&LayersPresenceApplicability> for u8 {
    fn from(value: &LayersPresenceApplicability) -> Self {
        match value {
            LayersPresenceApplicability::NotPresentApplicable => 0,
            LayersPresenceApplicability::PresentApplicable => 1,
        }
    }
}

impl Serialize for IffDataRecord {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let padded_record_lengths =
            length_padded_to_num(SIX_OCTETS + self.record_specific_fields.len(), FOUR_OCTETS);
        let record_length_bytes = padded_record_lengths.record_length as u16;

        buf.put_u32(self.record_type.into());
        buf.put_u16(record_length_bytes);
        buf.put(&*self.record_specific_fields);
        buf.put_bytes(0u8, padded_record_lengths.padding_length);

        record_length_bytes
    }
}

impl Serialize for IffDataSpecification {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.iff_data_records.len() as u16);
        let records_bytes: u16 = self
            .iff_data_records
            .iter()
            .map(|record| record.serialize(buf))
            .sum();

        2 + records_bytes
    }
}

impl Serialize for InformationLayers {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let byte = u8::from(self);
        buf.put_u8(byte);

        ONE_OCTET as u16
    }
}

impl Serialize for IffFundamentalParameterData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_f32(self.erp);
        buf.put_f32(self.frequency);
        buf.put_f32(self.pgrf);
        buf.put_f32(self.pulse_width);
        buf.put_f32(self.burst_length);
        buf.put_u8(self.applicable_modes.into());
        let system_specific_data_bytes = self.system_specific_data.serialize(buf);

        21 + system_specific_data_bytes
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

impl Serialize for SystemSpecificData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.part_1);
        buf.put_u8(self.part_2);
        buf.put_u8(self.part_3);

        THREE_OCTETS as u16
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

impl Serialize for DapSource {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let indicated_air_speed = u8::from(&self.indicated_air_speed) << 7;
        let mach_number = u8::from(&self.mach_number) << 6;
        let ground_speed = u8::from(&self.ground_speed) << 5;
        let magnetic_heading = u8::from(&self.magnetic_heading) << 4;
        let track_angle_rate = u8::from(&self.track_angle_rate) << 3;
        let true_track_angle = u8::from(&self.true_track_angle) << 2;
        let true_airspeed = u8::from(&self.true_airspeed) << 1;
        let vertical_rate = u8::from(&self.vertical_rate);

        buf.put_u8(
            indicated_air_speed
                | mach_number
                | ground_speed
                | magnetic_heading
                | track_angle_rate
                | true_track_angle
                | true_airspeed
                | vertical_rate,
        );

        ONE_OCTET as u16
    }
}

impl From<&DapValue> for u8 {
    fn from(value: &DapValue) -> Self {
        match value {
            DapValue::ComputeLocally => 0,
            DapValue::DataRecordAvailable => 1,
        }
    }
}

impl Serialize for EnhancedMode1Code {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let bytes = u16::from(self);
        buf.put_u16(bytes);

        TWO_OCTETS as u16
    }
}

impl Serialize for Mode5InterrogatorBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let _status_bytes = self.status.serialize(buf);
        buf.put_u8(0u8);
        buf.put_u16(0u16);
        let _formats_present_bytes = self.mode_5_message_formats_present.serialize(buf);
        let _entity_id_bytes = self.interrogated_entity_id.serialize(buf);
        buf.put_u16(0u16);

        16
    }
}

impl Serialize for Mode5InterrogatorStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let byte = u8::from(self);
        buf.put_u8(byte);

        ONE_OCTET as u16
    }
}

impl Serialize for Mode5MessageFormats {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let value = u32::from(self);
        buf.put_u32(value);

        4
    }
}

impl Serialize for Mode5TransponderBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let status_bytes = self.status.serialize(buf);
        buf.put_u16(self.pin);
        let formats_present_bytes = self.mode_5_message_formats_present.serialize(buf);
        let enhanced_mode_1_bytes = self.enhanced_mode_1.serialize(buf);
        buf.put_u16(self.national_origin);
        let sd_bytes = self.supplemental_data.serialize(buf);
        buf.put_u8(self.navigation_source.into());
        buf.put_u8(self.figure_of_merit);
        buf.put_u8(0u8);

        status_bytes + formats_present_bytes + enhanced_mode_1_bytes + sd_bytes + 7
    }
}

impl From<&OnOffStatus> for u8 {
    fn from(value: &OnOffStatus) -> Self {
        match value {
            OnOffStatus::Off => 0,
            OnOffStatus::On => 1,
        }
    }
}

impl From<&DamageStatus> for u8 {
    fn from(value: &DamageStatus) -> Self {
        match value {
            DamageStatus::NoDamage => 0,
            DamageStatus::Damaged => 1,
        }
    }
}

impl From<&MalfunctionStatus> for u8 {
    fn from(value: &MalfunctionStatus) -> Self {
        match value {
            MalfunctionStatus::NoMalfunction => 0,
            MalfunctionStatus::Malfunction => 1,
        }
    }
}

impl From<&EnabledStatus> for u8 {
    fn from(value: &EnabledStatus) -> Self {
        match value {
            EnabledStatus::NotEnabled => 0,
            EnabledStatus::Enabled => 1,
        }
    }
}

impl From<&LatLonAltSource> for u8 {
    fn from(value: &LatLonAltSource) -> Self {
        match value {
            LatLonAltSource::ComputeLocally => 0,
            LatLonAltSource::TransponderLocationDataRecordPresent => 1,
        }
    }
}

impl Serialize for Mode5TransponderSupplementalData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let byte = u8::from(self);
        buf.put_u8(byte);

        ONE_OCTET as u16
    }
}

impl Serialize for Mode5TransponderStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.into());

        TWO_OCTETS as u16
    }
}

impl Serialize for ModeSAltitude {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u16(self.into());

        TWO_OCTETS as u16
    }
}

impl Serialize for ModeSInterrogatorBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        const PAD_168_BITS_IN_OCTETS: usize = 21;
        let _status_bytes = self.mode_s_interrogator_status.serialize(buf);
        buf.put_u8(0u8);
        let _levels_present_bytes = self.mode_s_levels_present.serialize(buf);
        buf.put_bytes(0u8, PAD_168_BITS_IN_OCTETS);

        24
    }
}

impl Serialize for ModeSInterrogatorStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.into());

        ONE_OCTET as u16
    }
}

impl Serialize for ModeSLevelsPresent {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        buf.put_u8(self.into());

        ONE_OCTET as u16
    }
}

impl From<&IffPresence> for u32 {
    fn from(value: &IffPresence) -> Self {
        match value {
            IffPresence::NotPresent => 0,
            IffPresence::Present => 1,
        }
    }
}

impl From<&IffPresence> for u8 {
    fn from(value: &IffPresence) -> Self {
        match value {
            IffPresence::NotPresent => 0,
            IffPresence::Present => 1,
        }
    }
}

impl Serialize for ModeSTransponderBasicData {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let _status_bytes = self.status.serialize(buf);
        let _levels_present_bytes = self.levels_present.serialize(buf);
        buf.put_u8(self.aircraft_present_domain.into());
        let _aircraft_id = match put_ascii_string_with_length(buf, &self.aircraft_identification, 8)
        {
            Ok(bytes) => bytes,
            Err(_) => {
                buf.put_bytes(0u8, EIGHT_OCTETS);
                EIGHT_OCTETS as u16
            }
        };
        buf.put_u32(self.aircraft_address);
        buf.put_u8(self.aircraft_identification_type.into());
        self.dap_source.serialize(buf);
        self.altitude.serialize(buf);
        buf.put_u8(self.capability_report.into());

        24
    }
}

fn put_ascii_string_with_length(
    buf: &mut BytesMut,
    value: &str,
    length: usize,
) -> Result<u16, DisError> {
    if value.len() > length {
        Err(DisError::StringTooLongError)
    } else if !value.is_ascii() {
        Err(DisError::StringNotAsciiError)
    } else {
        buf.put_slice(value.as_bytes());
        buf.put_bytes(0u8, length - value.len());
        Ok(length as u16)
    }
}

impl Serialize for ModeSTransponderStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let squitter_status: u8 = u8::from(&self.squitter_status) << 7;
        let squitter_type: u8 = u8::from(self.squitter_type) << 4;
        let squitter_record_source: u8 = u8::from(self.squitter_record_source) << 3;
        let airborne_pos_ri: u8 = u8::from(&self.airborne_position_report_indicator) << 2;
        let airborne_vel_ri: u8 = u8::from(&self.airborne_velocity_report_indicator) << 1;
        let surface_pos_ri: u8 = u8::from(&self.surface_position_report_indicator);
        buf.put_u8(
            squitter_status
                | squitter_type
                | squitter_record_source
                | airborne_pos_ri
                | airborne_vel_ri
                | surface_pos_ri,
        );

        let ident_ri: u8 = u8::from(&self.identification_report_indicator) << 7;
        let event_driven_ri: u8 = u8::from(&self.event_driven_report_indicator) << 6;
        let on_off_status: u8 = u8::from(&self.on_off_status) << 2;
        let damage_status: u8 = u8::from(&self.damage_status) << 1;
        let malfunction_status: u8 = u8::from(&self.malfunction_status);
        buf.put_u8(ident_ri | event_driven_ri | on_off_status | damage_status | malfunction_status);

        TWO_OCTETS as u16
    }
}

impl From<&SquitterStatus> for u8 {
    fn from(value: &SquitterStatus) -> Self {
        match value {
            SquitterStatus::Off => 0,
            SquitterStatus::On => 1,
        }
    }
}

impl Serialize for SystemStatus {
    fn serialize(&self, buf: &mut BytesMut) -> u16 {
        let byte = u8::from(self);
        buf.put_u8(byte);

        ONE_OCTET as u16
    }
}
