use dis_rs::iff::model::SystemId;
use crate::iff::model::{CdisFundamentalOperationalData, Iff, IffFundamentalParameterData, IffLayer2, IffLayer3, IffLayer4, IffLayer5};
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
use crate::constants::{EIGHT_BITS, FIVE_BITS, FOUR_BITS, ONE_BIT, SIXTEEN_BITS, TEN_BITS, THREE_BITS, TWELVE_BITS};
use crate::records::model::{CdisRecord, LayerHeader};
use crate::types::model::CdisFloat;
use crate::writing::{serialize_when_present, write_value_signed, write_value_unsigned, SerializeCdis};

impl SerializeCdisPdu for Iff {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, self.fields_present_length(), self.fields_present_field());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.relative_antenna_location_units.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.full_update_flag.into());

        let cursor = self.emitting_entity_id.serialize(buf, cursor);
        let cursor = serialize_when_present(&self.event_id, buf, cursor);
        let cursor = serialize_when_present(&self.relative_antenna_location, buf, cursor);
        let cursor = serialize_when_present(&self.system_id, buf, cursor);
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_designator);
        let cursor = if let Some(data) = self.system_specific_data {
            write_value_unsigned(buf, cursor, EIGHT_BITS, data)
        } else { cursor };

        let cursor = self.fundamental_operational_data.serialize(buf, cursor);

        let cursor = if let Some(layer) = &self.layer_2 {
            layer.serialize(buf, cursor)
        } else { cursor };

        let cursor = if let Some(layer) = &self.layer_3 {
            layer.serialize(buf, cursor)
        } else { cursor };

        let cursor = if let Some(layer) = &self.layer_4 {
            layer.serialize(buf, cursor)
        } else { cursor };

        let cursor = if let Some(layer) = &self.layer_5 {
            layer.serialize(buf, cursor)
        } else { cursor };

        cursor
    }
}

impl SerializeCdis for SystemId {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u16>(buf, cursor, FOUR_BITS, self.system_type.into());
        let cursor = write_value_unsigned::<u16>(buf, cursor, FIVE_BITS, self.system_name.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.system_mode.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.system_type.into());

        cursor
    }
}

impl SerializeCdis for CdisFundamentalOperationalData {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.system_status.into());
        let cursor = if let Some(data) = self.data_field_1 {
            write_value_unsigned(buf, cursor, EIGHT_BITS, data)
        } else { cursor };

        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.information_layers.into());

        let cursor = if let Some(data) = self.data_field_2 {
            write_value_unsigned(buf, cursor, EIGHT_BITS, data)
        } else { cursor };

        let cursor = if let Some(data) = self.parameter_1 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else { cursor };
        let cursor = if let Some(data) = self.parameter_2 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else { cursor };
        let cursor = if let Some(data) = self.parameter_3 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else { cursor };
        let cursor = if let Some(data) = self.parameter_4 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else { cursor };
        let cursor = if let Some(data) = self.parameter_5 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else { cursor };
        let cursor = if let Some(data) = self.parameter_6 {
            write_value_unsigned(buf, cursor, SIXTEEN_BITS, data)
        } else { cursor };

        cursor
    }
}

impl SerializeCdis for IffLayer2 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, ONE_BIT); // always 1 for IffLayer2
        let cursor = self.layer_header.serialize_with_length(self.record_length(), buf, cursor);
        let cursor = self.beam_data.serialize(buf, cursor);
        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.operational_parameter_1);
        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.operational_parameter_2);
        let cursor = write_value_unsigned::<usize>(buf, cursor, EIGHT_BITS, self.iff_fundamental_parameters.len());

        let cursor = self.iff_fundamental_parameters.iter().fold(cursor, | cursor, param| param.serialize(buf, cursor));

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

        let cursor = write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.applicable_modes.into());

        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_specific_data.part_1);
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_specific_data.part_2);
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.system_specific_data.part_3);

        cursor
    }
}

impl SerializeCdis for IffLayer3 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        todo!()
    }
}

impl SerializeCdis for IffLayer4 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        todo!()
    }
}

impl SerializeCdis for IffLayer5 {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        todo!()
    }
}