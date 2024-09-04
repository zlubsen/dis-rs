use dis_rs::enumerations::IffApplicableModes;
use dis_rs::iff::model::{InformationLayers, SystemId, SystemSpecificData, SystemStatus};
use crate::{BodyProperties, CdisBody, CdisInteraction};
use crate::constants::{EIGHTY_SIX_BITS, EIGHT_BITS, ONE_BIT, SIXTEEN_BITS, TWENTY_BITS, TWENTY_FOUR_BITS};
use crate::electromagnetic_emission::model::FrequencyFloat;
use crate::records::model::{BeamData, CdisRecord, LayerHeader};
use crate::records::model::{EntityCoordinateVector, EntityId, UnitsMeters};

#[derive(Clone, Default, Debug, PartialEq)]
pub struct Iff {
    pub relative_antenna_location_units: UnitsMeters,
    pub full_update_flag: bool,
    pub emitting_entity_id: EntityId,
    pub event_id: Option<EntityId>,
    pub relative_antenna_location: Option<EntityCoordinateVector>,
    pub system_id: Option<SystemId>,
    pub system_designator: u8,
    pub system_specific_data: Option<u8>,
    pub fundamental_operational_data: CdisFundamentalOperationalData,
    pub layer_2: Option<IffLayer2>, // 13.23.2 Layer 2 emissions data
    pub layer_3: Option<IffLayer3>, // Mode 5 Functional Data
    pub layer_4: Option<IffLayer4>, // Mode S Functional Data
    pub layer_5: Option<IffLayer5>, // Data Communications
}

// TODO parser/serializer/codec for layer 1 and layer 2
// TODO layer 3
// TODO layer 4
// TODO layer 5

impl BodyProperties for Iff {
    type FieldsPresent = IffLayer1FieldsPresent;
    type FieldsPresentOutput = u16;
    const FIELDS_PRESENT_LENGTH: usize = 12;

    fn fields_present_field(&self) -> Self::FieldsPresentOutput {
        (if self.event_id.is_some() { Self::FieldsPresent::EVENT_ID_BIT } else { 0 })
        | (if self.relative_antenna_location.is_some() { Self::FieldsPresent::RELATIVE_ANTENNA_LOCATION_BIT } else { 0 })
        | (if self.system_id.is_some() { Self::FieldsPresent::SYSTEM_ID_DETAILS_BIT } else { 0 })
        | (if self.system_specific_data.is_some() { Self::FieldsPresent::SYSTEM_SPECIFIC_DATA_BIT } else { 0 })
        | (if self.fundamental_operational_data.data_field_1.is_some() { Self::FieldsPresent::DATA_FIELD_1_BIT } else { 0 })
        | (if self.fundamental_operational_data.data_field_2.is_some() { Self::FieldsPresent::DATA_FIELD_2_BIT } else { 0 })
        | (if self.fundamental_operational_data.parameter_1.is_some() { Self::FieldsPresent::PARAMETER_1_BIT } else { 0 })
        | (if self.fundamental_operational_data.parameter_2.is_some() { Self::FieldsPresent::PARAMETER_2_BIT } else { 0 })
        | (if self.fundamental_operational_data.parameter_3.is_some() { Self::FieldsPresent::PARAMETER_3_BIT } else { 0 })
        | (if self.fundamental_operational_data.parameter_4.is_some() { Self::FieldsPresent::PARAMETER_4_BIT } else { 0 })
        | (if self.fundamental_operational_data.parameter_5.is_some() { Self::FieldsPresent::PARAMETER_5_BIT } else { 0 })
        | (if self.fundamental_operational_data.parameter_6.is_some() { Self::FieldsPresent::PARAMETER_6_BIT } else { 0 })
    }

    fn body_length_bits(&self) -> usize {
        const FIXED_LENGTH_BITS: usize = 14 + 8;
        FIXED_LENGTH_BITS +
            self.emitting_entity_id.record_length() +
            (if let Some(record) = self.event_id { record.record_length() } else { 0 }) +
            (if let Some(record) = self.relative_antenna_location { record.record_length() } else { 0 }) +
            (if self.system_id.is_some() { TWENTY_BITS } else { 0 }) +
            (if self.system_specific_data.is_some() { EIGHT_BITS } else { 0 }) +
            (self.fundamental_operational_data.record_length()) +
            (if let Some(record) = &self.layer_2 { record.record_length() } else { 0 }) +
            (if let Some(record) = &self.layer_3 { record.record_length() } else { 0 }) +
            (if let Some(record) = &self.layer_4 { record.record_length() } else { 0 }) +
            (if let Some(record) = &self.layer_5 { record.record_length() } else { 0 })
    }

    fn into_cdis_body(self) -> CdisBody {
        CdisBody::Iff(self)
    }
}

impl CdisInteraction for Iff {
    fn originator(&self) -> Option<&EntityId> {
        Some(&self.emitting_entity_id)
    }

    fn receiver(&self) -> Option<&EntityId> {
        None
    }
}

pub struct IffLayer1FieldsPresent;

impl IffLayer1FieldsPresent {
    pub const EVENT_ID_BIT: u16 = 0x0800;
    pub const RELATIVE_ANTENNA_LOCATION_BIT: u16 = 0x0400;
    pub const SYSTEM_ID_DETAILS_BIT: u16 = 0x0200;
    pub const SYSTEM_SPECIFIC_DATA_BIT: u16 = 0x0100;
    pub const DATA_FIELD_1_BIT: u16 = 0x0080;
    pub const DATA_FIELD_2_BIT: u16 = 0x0040;
    pub const PARAMETER_1_BIT: u16 = 0x0020;
    pub const PARAMETER_2_BIT: u16 = 0x0010;
    pub const PARAMETER_3_BIT: u16 = 0x0008;
    pub const PARAMETER_4_BIT: u16 = 0x0004;
    pub const PARAMETER_5_BIT: u16 = 0x0002;
    pub const PARAMETER_6_BIT: u16 = 0x0001;
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct CdisFundamentalOperationalData {
    pub system_status: SystemStatus,
    pub data_field_1: Option<u8>,
    pub information_layers: InformationLayers,
    pub data_field_2: Option<u8>,
    pub parameter_1: Option<u16>,
    pub parameter_2: Option<u16>,
    pub parameter_3: Option<u16>,
    pub parameter_4: Option<u16>,
    pub parameter_5: Option<u16>,
    pub parameter_6: Option<u16>,
}

impl CdisRecord for CdisFundamentalOperationalData {
    fn record_length(&self) -> usize {
        SIXTEEN_BITS +
            (if self.data_field_1.is_some() { EIGHT_BITS } else { 0 }) +
            (if self.data_field_2.is_some() { EIGHT_BITS } else { 0 }) +
            (if self.parameter_1.is_some() { SIXTEEN_BITS } else { 0 }) +
            (if self.parameter_2.is_some() { SIXTEEN_BITS } else { 0 }) +
            (if self.parameter_3.is_some() { SIXTEEN_BITS } else { 0 }) +
            (if self.parameter_4.is_some() { SIXTEEN_BITS } else { 0 }) +
            (if self.parameter_5.is_some() { SIXTEEN_BITS } else { 0 }) +
            (if self.parameter_6.is_some() { SIXTEEN_BITS } else { 0 })
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct IffLayer2 {
    pub layer_header: LayerHeader,
    pub beam_data: BeamData,
    pub operational_parameter_1: u8,
    pub operational_parameter_2: u8,
    pub iff_fundamental_parameters: Vec<IffFundamentalParameterData>,
}

impl CdisRecord for IffLayer2 {
    fn record_length(&self) -> usize {
        ONE_BIT + self.layer_header.record_length() +
            self.beam_data.record_length() +
            TWENTY_FOUR_BITS +
            self.iff_fundamental_parameters.iter()
                .map(|record| record.record_length()).sum::<usize>()
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct IffFundamentalParameterData {
    pub erp: u8,
    pub frequency: FrequencyFloat, // FIXME move FrequencyFloat to a common file instead of in EE.
    pub pgrf: u16,
    pub pulse_width: u16,
    pub burst_length: u16,
    pub applicable_modes: IffApplicableModes,
    pub system_specific_data: SystemSpecificData,
}

impl CdisRecord for IffFundamentalParameterData {
    fn record_length(&self) -> usize {
        EIGHTY_SIX_BITS
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct IffLayer3 {
    pub layer_header: LayerHeader,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct IffLayer4 {
    pub layer_header: LayerHeader,
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct IffLayer5 {
    pub layer_header: LayerHeader,
}