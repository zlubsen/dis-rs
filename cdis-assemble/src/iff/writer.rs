use crate::constants::{
    EIGHT_BITS, FIVE_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, SIX_BITS, TEN_BITS, THIRTY_TWO_BITS,
    THREE_BITS,
};
use crate::iff::model::{
    CdisFundamentalOperationalData, Iff, IffFundamentalParameterData, IffLayer2, IffLayer3,
    IffLayer4, IffLayer5, Mode5BasicData, Mode5InterrogatorBasicData, ModeSBasicData,
};
use crate::records::model::CdisRecord;
use crate::types::model::CdisFloat;
use crate::writing::{serialize_when_present, write_value_unsigned, SerializeCdis};
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
use dis_rs::iff::model::{
    IffDataRecord, Mode5MessageFormats, Mode5TransponderBasicData, ModeSInterrogatorBasicData,
    ModeSTransponderBasicData, SystemId,
};

impl SerializeCdisPdu for Iff {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf,
            cursor,
            self.fields_present_length(),
            self.fields_present_field(),
        );
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            ONE_BIT,
            self.relative_antenna_location_units.into(),
        );
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.full_update_flag.into());

        let cursor = self.emitting_entity_id.serialize(buf, cursor);
        let cursor = serialize_when_present(&self.event_id, buf, cursor);
        let cursor = serialize_when_present(&self.relative_antenna_location, buf, cursor);
        let cursor = serialize_when_present(&self.system_id, buf, cursor);
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_designator);
        let cursor = if let Some(data) = self.system_specific_data {
            write_value_unsigned(buf, cursor, EIGHT_BITS, data)
        } else {
            cursor
        };

        let cursor = self.fundamental_operational_data.serialize(buf, cursor);

        let cursor = if let Some(layer) = &self.layer_2 {
            layer.serialize(buf, cursor)
        } else {
            cursor
        };

        let cursor = if let Some(layer) = &self.layer_3 {
            layer.serialize(buf, cursor)
        } else {
            cursor
        };

        let cursor = if let Some(layer) = &self.layer_4 {
            layer.serialize(buf, cursor)
        } else {
            cursor
        };

        let cursor = if let Some(layer) = &self.layer_5 {
            layer.serialize(buf, cursor)
        } else {
            cursor
        };

        cursor
    }
}

impl SerializeCdis for SystemId {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u16>(buf, cursor, FOUR_BITS, self.system_type.into());
        let cursor = write_value_unsigned::<u16>(buf, cursor, FIVE_BITS, self.system_name.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.system_mode.into());
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, (&self.change_options).into());

        cursor
    }
}

impl SerializeCdis for CdisFundamentalOperationalData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, u8::from(&self.system_status));
        let cursor = if let Some(data) = self.data_field_1 {
            write_value_unsigned(buf, cursor, EIGHT_BITS, data)
        } else {
            cursor
        };

        let cursor =
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, u8::from(&self.information_layers));

        let cursor = if let Some(data) = self.data_field_2 {
            write_value_unsigned(buf, cursor, EIGHT_BITS, data)
        } else {
            cursor
        };

        let cursor = if let Some(data) = self.parameter_1 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else {
            cursor
        };
        let cursor = if let Some(data) = self.parameter_2 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else {
            cursor
        };
        let cursor = if let Some(data) = self.parameter_3 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else {
            cursor
        };
        let cursor = if let Some(data) = self.parameter_4 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else {
            cursor
        };
        let cursor = if let Some(data) = self.parameter_5 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else {
            cursor
        };
        let cursor = if let Some(data) = self.parameter_6 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else {
            cursor
        };

        cursor
    }
}

impl SerializeCdis for IffLayer2 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, ONE_BIT); // always 1 for IffLayer2
        let cursor = self
            .layer_header
            .serialize_with_length(self.record_length(), buf, cursor);
        let cursor = self.beam_data.serialize(buf, cursor);
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.operational_parameter_1);
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.operational_parameter_2);
        let cursor = write_value_unsigned::<usize>(
            buf,
            cursor,
            EIGHT_BITS,
            self.iff_fundamental_parameters.len(),
        );

        let cursor = self
            .iff_fundamental_parameters
            .iter()
            .fold(cursor, |cursor, param| param.serialize(buf, cursor));

        cursor
    }
}

impl SerializeCdis for IffFundamentalParameterData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.erp);
        let cursor = self.frequency.serialize(buf, cursor);
        let cursor = write_value_unsigned(buf, cursor, TEN_BITS, self.pgrf);
        let cursor = write_value_unsigned(buf, cursor, TEN_BITS, self.pulse_width);
        let cursor = write_value_unsigned(buf, cursor, TEN_BITS, self.burst_length);

        let cursor =
            write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.applicable_modes.into());

        let cursor =
            write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_specific_data.part_1);
        let cursor =
            write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_specific_data.part_2);
        let cursor =
            write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_specific_data.part_3);

        cursor
    }
}

impl SerializeCdis for IffLayer3 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let data_records_present_value = if self.iff_data_records.is_empty() {
            0u8
        } else {
            1u8
        };
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, data_records_present_value);
        let cursor = self
            .layer_header
            .serialize_with_length(self.record_length(), buf, cursor);
        let cursor = self.reporting_simulation_site.serialize(buf, cursor);
        let cursor = self.reporting_simulation_application.serialize(buf, cursor);
        let cursor = self.mode_5_basic_data.serialize(buf, cursor);

        let cursor = if !self.iff_data_records.is_empty() {
            let cursor =
                write_value_unsigned::<usize>(buf, cursor, FIVE_BITS, self.iff_data_records.len());
            self.iff_data_records.iter().fold(cursor, |cursor, record| {
                SerializeCdis::serialize(record, buf, cursor)
            })
        } else {
            cursor
        };

        cursor
    }
}

impl SerializeCdis for IffLayer4 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let data_records_present_value = if self.iff_data_records.is_empty() {
            0u8
        } else {
            1u8
        };
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, data_records_present_value);
        let cursor = self
            .layer_header
            .serialize_with_length(self.record_length(), buf, cursor);
        let cursor = self.reporting_simulation_site.serialize(buf, cursor);
        let cursor = self.reporting_simulation_application.serialize(buf, cursor);

        let cursor = self.mode_s_basic_data.serialize(buf, cursor);

        let cursor = if !self.iff_data_records.is_empty() {
            let cursor =
                write_value_unsigned::<usize>(buf, cursor, FIVE_BITS, self.iff_data_records.len());
            self.iff_data_records.iter().fold(cursor, |cursor, record| {
                SerializeCdis::serialize(record, buf, cursor)
            })
        } else {
            cursor
        };

        cursor
    }
}

impl SerializeCdis for IffLayer5 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let data_records_present_value = if self.iff_data_records.is_empty() {
            0u8
        } else {
            1u8
        };
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, data_records_present_value);
        let cursor = self
            .layer_header
            .serialize_with_length(self.record_length(), buf, cursor);
        let cursor = self.reporting_simulation_site.serialize(buf, cursor);
        let cursor = self.reporting_simulation_application.serialize(buf, cursor);

        let cursor = write_value_unsigned(buf, cursor, SIX_BITS, u8::from(&self.applicable_layers));
        let cursor = write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.data_category.into());

        let cursor = if !self.iff_data_records.is_empty() {
            let cursor =
                write_value_unsigned::<usize>(buf, cursor, FIVE_BITS, self.iff_data_records.len());
            self.iff_data_records.iter().fold(cursor, |cursor, record| {
                SerializeCdis::serialize(record, buf, cursor)
            })
        } else {
            cursor
        };

        cursor
    }
}

impl SerializeCdis for Mode5BasicData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        match self {
            Mode5BasicData::Interrogator(record) => record.serialize(buf, cursor),
            Mode5BasicData::Transponder(record) => SerializeCdis::serialize(record, buf, cursor),
        }
    }
}

impl SerializeCdis for Mode5InterrogatorBasicData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned(buf, cursor, EIGHT_BITS, u8::from(&self.interrogator_status));
        let cursor = write_value_unsigned(
            buf,
            cursor,
            THIRTY_TWO_BITS,
            u32::from(&self.message_formats_present),
        );
        let cursor = self.interrogated_entity_id.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for Mode5TransponderBasicData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, SIXTEEN_BITS, u16::from(&self.status));
        let cursor = write_value_unsigned(buf, cursor, SIXTEEN_BITS, self.pin);
        let cursor = SerializeCdis::serialize(&self.mode_5_message_formats_present, buf, cursor);

        let cursor =
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, u16::from(&self.enhanced_mode_1));
        let cursor = write_value_unsigned(buf, cursor, SIXTEEN_BITS, self.national_origin);
        let cursor =
            write_value_unsigned(buf, cursor, EIGHT_BITS, u8::from(&self.supplemental_data));

        let cursor =
            write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.navigation_source.into());
        let cursor = write_value_unsigned(buf, cursor, FIVE_BITS, self.figure_of_merit);

        cursor
    }
}

impl SerializeCdis for Mode5MessageFormats {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let value = u32::from(self);
        write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, value)
    }
}

impl SerializeCdis for IffDataRecord {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned::<u32>(buf, cursor, SIXTEEN_BITS, self.record_type.into());
        let cursor = write_value_unsigned(
            buf,
            cursor,
            EIGHT_BITS,
            self.record_length().saturating_div(EIGHT_BITS),
        );
        let cursor = self
            .record_specific_fields
            .iter()
            .fold(cursor, |cursor, byte| {
                write_value_unsigned(buf, cursor, EIGHT_BITS, *byte)
            });

        cursor
    }
}

impl SerializeCdis for ModeSBasicData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        match self {
            ModeSBasicData::Interrogator(record) => record.serialize(buf, cursor),
            ModeSBasicData::Transponder(record) => SerializeCdis::serialize(record, buf, cursor),
        }
    }
}

impl SerializeCdis for ModeSInterrogatorBasicData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(
            buf,
            cursor,
            EIGHT_BITS,
            u8::from(&self.mode_s_interrogator_status),
        );
        let cursor = write_value_unsigned(
            buf,
            cursor,
            EIGHT_BITS,
            u8::from(&self.mode_s_levels_present),
        );

        cursor
    }
}

impl SerializeCdis for ModeSTransponderBasicData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, SIXTEEN_BITS, u16::from(&self.status));
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, u8::from(&self.levels_present));
        let cursor = write_value_unsigned(
            buf,
            cursor,
            THREE_BITS,
            u8::from(self.aircraft_present_domain),
        );
        let cursor =
            write_value_unsigned(buf, cursor, FOUR_BITS, self.aircraft_identification.len());

        let cursor = self
            .aircraft_identification
            .as_bytes()
            .iter()
            .fold(cursor, |cursor, char| {
                write_value_unsigned(buf, cursor, EIGHT_BITS, *char)
            });

        let cursor = write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, self.aircraft_address);
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            THREE_BITS,
            self.aircraft_identification_type.into(),
        );
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, u8::from(&self.dap_source));
        let cursor = write_value_unsigned(buf, cursor, SIXTEEN_BITS, u16::from(&self.altitude));
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.capability_report.into());

        cursor
    }
}
