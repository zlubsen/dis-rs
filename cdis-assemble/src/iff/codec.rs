use crate::codec::{
    Codec, CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState, EncoderState,
};
use crate::iff::model::{
    CdisFundamentalOperationalData, Iff, IffFundamentalParameterData, IffLayer2, IffLayer3,
    IffLayer4, IffLayer5, Mode5BasicData, Mode5InterrogatorBasicData, ModeSBasicData,
};
use crate::records::codec::{
    decode_entity_coordinate_vector, decode_layer_header_with_length,
    encode_entity_coordinate_vector, encode_layer_header_with_length,
};
use crate::records::model::{
    BeamData, CdisRecord, EntityCoordinateVector, EntityId, FrequencyFloat, LayerHeader,
    UnitsMeters,
};
use crate::types::model::{CdisFloat, UVINT16};
use crate::{BodyProperties, CdisBody};
use dis_rs::iff::model::{
    FundamentalOperationalData, IffDataSpecification, Mode5TransponderBasicData, SystemId,
};
use dis_rs::model::{EventId, PduBody, SimulationAddress, VectorF32};
use dis_rs::BodyRaw;
use num_traits::{ToPrimitive, Zero};
use std::time::Instant;

type Counterpart = dis_rs::iff::model::Iff;

pub(crate) fn encode_iff_body_and_update_state(
    dis_body: &Counterpart,
    state: &mut EncoderState,
    options: &CodecOptions,
) -> (CdisBody, CodecStateResult) {
    let state_for_id = state.iff.get(&dis_body.emitting_entity_id);

    let (cdis_body, state_result) = Iff::encode(dis_body, state_for_id, options);

    if state_result == CodecStateResult::StateUpdateIff {
        state
            .iff
            .entry(dis_body.emitting_entity_id)
            .and_modify(|iff| iff.heartbeat = Instant::now())
            .or_default();
    }

    (cdis_body.into_cdis_body(), state_result)
}

pub(crate) fn decode_iff_body_and_update_state(
    cdis_body: &Iff,
    state: &mut DecoderState,
    options: &CodecOptions,
) -> (PduBody, CodecStateResult) {
    let state_for_id = state.iff.get(&dis_rs::model::EntityId::from(
        &cdis_body.emitting_entity_id,
    ));
    let (dis_body, state_result) = cdis_body.decode(state_for_id, options);

    if state_result == CodecStateResult::StateUpdateIff {
        state
            .iff
            .entry(dis_rs::model::EntityId::from(&cdis_body.emitting_entity_id))
            .and_modify(|iff| {
                iff.heartbeat = Instant::now();
            })
            .or_insert(DecoderStateIff::new(&dis_body));
    }

    (dis_body.into_pdu_body(), state_result)
}

#[derive(Debug)]
pub struct EncoderStateIff {
    pub heartbeat: Instant,
}

impl Default for EncoderStateIff {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateIff {
    pub heartbeat: Instant,
    pub system_id: SystemId,
}

impl DecoderStateIff {
    #[must_use]
    pub fn new(pdu: &Counterpart) -> Self {
        Self {
            heartbeat: Instant::now(),
            system_id: pdu.system_id,
        }
    }
}

impl Default for DecoderStateIff {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
            system_id: SystemId::default(),
        }
    }
}

fn evaluate_timeout_for_iff(last_heartbeat: &Instant, options: &CodecOptions) -> bool {
    let elapsed = last_heartbeat.elapsed().as_secs_f32();
    elapsed > (options.federation_parameters.HBT_PDU_IFF * options.hbt_cdis_full_update_mplier)
}

fn encode_iff_event_id(event_id: EventId) -> Option<EntityId> {
    if event_id.event_id == 0
        && event_id.simulation_address.application_id == 0
        && event_id.simulation_address.site_id == 0
    {
        None
    } else {
        Some(EntityId::from(&event_id))
    }
}

fn decode_iff_event_id(event_id: Option<EntityId>) -> EventId {
    if let Some(event_id) = event_id {
        EventId::from(&event_id)
    } else {
        EventId::default()
    }
}

fn encode_iff_relative_antenna_location(
    vector: &VectorF32,
) -> (Option<EntityCoordinateVector>, UnitsMeters) {
    if vector.first_vector_component.is_zero()
        && vector.second_vector_component.is_zero()
        && vector.third_vector_component.is_zero()
    {
        (None, UnitsMeters::default())
    } else {
        let (location, units) = encode_entity_coordinate_vector(vector);
        (Some(location), units)
    }
}

fn encode_iff_fundamental_operational_data(
    data: &FundamentalOperationalData,
) -> CdisFundamentalOperationalData {
    CdisFundamentalOperationalData {
        system_status: data.system_status,
        data_field_1: if data.data_field_1 != 0 {
            Some(data.data_field_1)
        } else {
            None
        },
        information_layers: data.information_layers,
        data_field_2: if data.data_field_2 != 0 {
            Some(data.data_field_2)
        } else {
            None
        },
        parameter_1: if data.parameter_1 != 0 {
            Some(data.parameter_1)
        } else {
            None
        },
        parameter_2: if data.parameter_2 != 0 {
            Some(data.parameter_2)
        } else {
            None
        },
        parameter_3: if data.parameter_3 != 0 {
            Some(data.parameter_3)
        } else {
            None
        },
        parameter_4: if data.parameter_4 != 0 {
            Some(data.parameter_4)
        } else {
            None
        },
        parameter_5: if data.parameter_5 != 0 {
            Some(data.parameter_5)
        } else {
            None
        },
        parameter_6: if data.parameter_6 != 0 {
            Some(data.parameter_6)
        } else {
            None
        },
    }
}

fn decode_iff_fundamental_operational_data(
    data: &CdisFundamentalOperationalData,
) -> FundamentalOperationalData {
    FundamentalOperationalData::builder()
        .with_system_status(data.system_status)
        .with_data_field_1(data.data_field_1.unwrap_or_default())
        .with_information_layers(data.information_layers)
        .with_data_field_2(data.data_field_2.unwrap_or_default())
        .with_parameter_1(data.parameter_1.unwrap_or_default())
        .with_parameter_1(data.parameter_2.unwrap_or_default())
        .with_parameter_1(data.parameter_3.unwrap_or_default())
        .with_parameter_1(data.parameter_4.unwrap_or_default())
        .with_parameter_1(data.parameter_5.unwrap_or_default())
        .with_parameter_1(data.parameter_6.unwrap_or_default())
        .build()
}

impl Iff {
    #[must_use]
    pub fn encode(
        item: &Counterpart,
        state: Option<&EncoderStateIff>,
        options: &CodecOptions,
    ) -> (Self, CodecStateResult) {
        let (system_id, state_result) = match options.update_mode {
            CodecUpdateMode::FullUpdate => (Some(item.system_id), CodecStateResult::StateUpdateIff),
            CodecUpdateMode::PartialUpdate => {
                if state.is_some_and(|state| !evaluate_timeout_for_iff(&state.heartbeat, options)) {
                    (None, CodecStateResult::StateUnaffected)
                } else {
                    (Some(item.system_id), CodecStateResult::StateUpdateIff)
                }
            }
        };

        // set these fields when their value is non-zero.
        let (relative_antenna_location, relative_antenna_location_units) =
            encode_iff_relative_antenna_location(&item.relative_antenna_location);
        let event_id = encode_iff_event_id(item.event_id);
        let system_specific_data = if item.system_specific_data == 0 {
            None
        } else {
            Some(item.system_specific_data)
        };
        let fundamental_operational_data =
            encode_iff_fundamental_operational_data(&item.fundamental_operational_data);

        (
            Self {
                relative_antenna_location_units,
                full_update_flag: false,
                emitting_entity_id: EntityId::encode(&item.emitting_entity_id),
                event_id,
                relative_antenna_location,
                system_id,
                system_designator: item.system_designator,
                system_specific_data,
                fundamental_operational_data,
                layer_2: item.layer_2.clone().map(|layer| IffLayer2::encode(&layer)),
                layer_3: item.layer_3.clone().map(|layer| IffLayer3::encode(&layer)),
                layer_4: item.layer_4.clone().map(|layer| IffLayer4::encode(&layer)),
                layer_5: item.layer_5.clone().map(|layer| IffLayer5::encode(&layer)),
            },
            state_result,
        )
    }

    #[must_use]
    pub fn decode(
        &self,
        state: Option<&DecoderStateIff>,
        options: &CodecOptions,
    ) -> (Counterpart, CodecStateResult) {
        let (system_id, state_result) = match options.update_mode {
            CodecUpdateMode::FullUpdate => (
                self.system_id.unwrap_or_default(),
                CodecStateResult::StateUnaffected,
            ),
            CodecUpdateMode::PartialUpdate => {
                if self.full_update_flag {
                    (
                        self.system_id.unwrap_or_default(),
                        CodecStateResult::StateUpdateIff,
                    )
                } else {
                    (
                        self.system_id.unwrap_or_else(|| {
                            if let Some(state) = state {
                                state.system_id
                            } else {
                                SystemId::default()
                            }
                        }),
                        CodecStateResult::StateUnaffected,
                    )
                }
            }
        };

        let relative_antenna_location =
            if let Some(relative_antenna_location) = self.relative_antenna_location {
                decode_entity_coordinate_vector(
                    &relative_antenna_location,
                    self.relative_antenna_location_units,
                )
            } else {
                VectorF32::default()
            };

        let builder = Counterpart::builder()
            .with_emitting_entity_id(self.emitting_entity_id.decode())
            .with_event_id(decode_iff_event_id(self.event_id))
            .with_relative_antenna_location(relative_antenna_location)
            .with_system_id(system_id)
            .with_system_designator(self.system_designator)
            .with_system_specific_data(self.system_specific_data.unwrap_or_default())
            .with_fundamental_operational_data(decode_iff_fundamental_operational_data(
                &self.fundamental_operational_data,
            ));
        let builder = if let Some(layer) = &self.layer_2 {
            builder.with_layer_2(layer.decode())
        } else {
            builder
        };
        let builder = if let Some(layer) = &self.layer_3 {
            builder.with_layer_3(layer.decode())
        } else {
            builder
        };
        let builder = if let Some(layer) = &self.layer_4 {
            builder.with_layer_4(layer.decode())
        } else {
            builder
        };
        let builder = if let Some(layer) = &self.layer_5 {
            builder.with_layer_5(layer.decode())
        } else {
            builder
        };
        let body = builder.build();

        (body, state_result)
    }
}

impl Codec for IffLayer2 {
    type Counterpart = dis_rs::iff::model::IffLayer2;

    fn encode(item: &Self::Counterpart) -> Self {
        let mut layer = Self {
            layer_header: LayerHeader::default(),
            beam_data: BeamData::encode(&item.beam_data),
            operational_parameter_1: item.operational_parameter_1,
            operational_parameter_2: item.operational_parameter_2,
            iff_fundamental_parameters: item
                .iff_fundamental_parameters
                .iter()
                .map(IffFundamentalParameterData::encode)
                .collect(),
        };

        let header =
            encode_layer_header_with_length(&item.layer_header, layer.record_length() as u16);
        layer.layer_header = header;
        layer
    }

    fn decode(&self) -> Self::Counterpart {
        let mut layer = Self::Counterpart::builder()
            .with_beam_data(self.beam_data.decode())
            .with_operational_parameter_1(self.operational_parameter_1)
            .with_operational_parameter_2(self.operational_parameter_2)
            .with_iff_fundamental_parameters(
                self.iff_fundamental_parameters
                    .iter()
                    .map(crate::codec::Codec::decode)
                    .collect(),
            )
            .build();

        let header = decode_layer_header_with_length(&self.layer_header, layer.data_length());
        layer.layer_header = header;
        layer
    }
}

impl Codec for IffLayer3 {
    type Counterpart = dis_rs::iff::model::IffLayer3;

    fn encode(item: &Self::Counterpart) -> Self {
        let mut layer = Self {
            layer_header: LayerHeader::default(),
            reporting_simulation_site: UVINT16::from(item.reporting_simulation.site_id),
            reporting_simulation_application: UVINT16::from(
                item.reporting_simulation.application_id,
            ),
            mode_5_basic_data: Mode5BasicData::encode(&item.mode_5_basic_data),
            iff_data_records: item.data_records.iff_data_records.clone(),
        };

        let header =
            encode_layer_header_with_length(&item.layer_header, layer.record_length() as u16);
        layer.layer_header = header;
        layer
    }

    fn decode(&self) -> Self::Counterpart {
        let mut layer = Self::Counterpart::builder()
            .with_reporting_simulation(SimulationAddress::new(
                self.reporting_simulation_site.value,
                self.reporting_simulation_application.value,
            ))
            .with_mode_5_basic_data(self.mode_5_basic_data.decode())
            .with_iff_data_specification(
                IffDataSpecification::builder()
                    .with_iff_data_records(self.iff_data_records.clone())
                    .build(),
            )
            .build();

        let header = decode_layer_header_with_length(&self.layer_header, layer.data_length());
        layer.layer_header = header;
        layer
    }
}

impl Codec for IffLayer4 {
    type Counterpart = dis_rs::iff::model::IffLayer4;

    fn encode(item: &Self::Counterpart) -> Self {
        let mut layer = Self {
            layer_header: LayerHeader::default(),
            reporting_simulation_site: UVINT16::from(item.reporting_simulation.site_id),
            reporting_simulation_application: UVINT16::from(
                item.reporting_simulation.application_id,
            ),
            mode_s_basic_data: ModeSBasicData::encode(&item.mode_s_basic_data),
            iff_data_records: item.data_records.iff_data_records.clone(),
        };

        let header =
            encode_layer_header_with_length(&item.layer_header, layer.record_length() as u16);
        layer.layer_header = header;
        layer
    }

    fn decode(&self) -> Self::Counterpart {
        let mut layer = Self::Counterpart::builder()
            .with_reporting_simulation(SimulationAddress::new(
                self.reporting_simulation_site.value,
                self.reporting_simulation_application.value,
            ))
            .with_mode_s_basic_data(self.mode_s_basic_data.decode())
            .with_iff_data_specification(
                IffDataSpecification::builder()
                    .with_iff_data_records(self.iff_data_records.clone())
                    .build(),
            )
            .build();

        let header = decode_layer_header_with_length(&self.layer_header, layer.data_length());
        layer.layer_header = header;
        layer
    }
}

impl Codec for IffLayer5 {
    type Counterpart = dis_rs::iff::model::IffLayer5;

    fn encode(item: &Self::Counterpart) -> Self {
        let mut layer = Self {
            layer_header: LayerHeader::default(),
            reporting_simulation_site: UVINT16::from(item.reporting_simulation.site_id),
            reporting_simulation_application: UVINT16::from(
                item.reporting_simulation.application_id,
            ),
            applicable_layers: item.applicable_layers,
            data_category: item.data_category,
            iff_data_records: item.data_records.iff_data_records.clone(),
        };

        let header =
            encode_layer_header_with_length(&item.layer_header, layer.record_length() as u16);
        layer.layer_header = header;
        layer
    }

    fn decode(&self) -> Self::Counterpart {
        let mut layer = Self::Counterpart::builder()
            .with_reporting_simulation(SimulationAddress::new(
                self.reporting_simulation_site.value,
                self.reporting_simulation_application.value,
            ))
            .with_applicable_layers(self.applicable_layers)
            .with_data_category(self.data_category)
            .with_iff_data_specification(
                IffDataSpecification::builder()
                    .with_iff_data_records(self.iff_data_records.clone())
                    .build(),
            )
            .build();

        let header = decode_layer_header_with_length(&self.layer_header, layer.data_length());
        layer.layer_header = header;
        layer
    }
}

impl Codec for IffFundamentalParameterData {
    type Counterpart = dis_rs::iff::model::IffFundamentalParameterData;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            erp: item.erp.to_u8().unwrap_or(0), // TODO check if conversion is correct
            frequency: FrequencyFloat::from_float(item.frequency),
            pgrf: item.pgrf.to_u16().unwrap_or(0), // TODO check if conversion is correct
            pulse_width: item.pulse_width.to_u16().unwrap_or(0), // TODO check if conversion is correct
            burst_length: item.burst_length.to_u16().unwrap_or(0), // TODO check if conversion is correct
            applicable_modes: item.applicable_modes,
            system_specific_data: item.system_specific_data,
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::builder()
            .with_erp(self.erp.to_f32().unwrap_or(0.0)) // TODO check if conversion is correct
            .with_frequency(self.frequency.to_float())
            .with_pgrf(self.pgrf.to_f32().unwrap_or(0.0)) // TODO check if conversion is correct
            .with_pulse_width(self.pulse_width.to_f32().unwrap_or(0.0)) // TODO check if conversion is correct
            .with_burst_length(self.burst_length.to_f32().unwrap_or(0.0)) // TODO check if conversion is correct
            .with_applicable_modes(self.applicable_modes)
            .with_system_specific_data(self.system_specific_data)
            .build()
    }
}

impl Codec for Mode5BasicData {
    type Counterpart = dis_rs::iff::model::Mode5BasicData;

    fn encode(item: &Self::Counterpart) -> Self {
        match item {
            Self::Counterpart::Interrogator(data) => {
                Self::Interrogator(Mode5InterrogatorBasicData {
                    interrogator_status: data.status.clone(),
                    message_formats_present: data.mode_5_message_formats_present.clone(),
                    interrogated_entity_id: EntityId::encode(&data.interrogated_entity_id),
                })
            }
            Self::Counterpart::Transponder(data) => Self::Transponder(Mode5TransponderBasicData {
                status: data.status.clone(),
                pin: data.pin,
                mode_5_message_formats_present: data.mode_5_message_formats_present.clone(),
                enhanced_mode_1: data.enhanced_mode_1.clone(),
                national_origin: data.national_origin,
                supplemental_data: data.supplemental_data.clone(),
                navigation_source: data.navigation_source,
                figure_of_merit: data.figure_of_merit,
            }),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        match self {
            Mode5BasicData::Interrogator(data) => Self::Counterpart::Interrogator(
                dis_rs::iff::model::Mode5InterrogatorBasicData::builder()
                    .with_status(data.interrogator_status.clone())
                    .with_mode_5_message_formats_present(data.message_formats_present.clone())
                    .with_interrogated_entity_id(data.interrogated_entity_id.decode())
                    .build(),
            ),
            Mode5BasicData::Transponder(data) => Self::Counterpart::Transponder(
                dis_rs::iff::model::Mode5TransponderBasicData::builder()
                    .with_status(data.status.clone())
                    .with_pin(data.pin)
                    .with_mode_5_message_formats_present(
                        data.mode_5_message_formats_present.clone(),
                    )
                    .with_enhanced_mode_1(data.enhanced_mode_1.clone())
                    .with_national_origin(data.national_origin)
                    .with_supplemental_data(data.supplemental_data.clone())
                    .with_national_origin(data.national_origin)
                    .with_figure_of_merit(data.figure_of_merit)
                    .build(),
            ),
        }
    }
}

impl Codec for ModeSBasicData {
    type Counterpart = dis_rs::iff::model::ModeSBasicData;

    fn encode(item: &Self::Counterpart) -> Self {
        match item {
            Self::Counterpart::Interrogator(data) => Self::Interrogator(data.clone()),
            Self::Counterpart::Transponder(data) => Self::Transponder(data.clone()),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        match self {
            ModeSBasicData::Interrogator(data) => Self::Counterpart::Interrogator(data.clone()),
            ModeSBasicData::Transponder(data) => Self::Counterpart::Transponder(data.clone()),
        }
    }
}
