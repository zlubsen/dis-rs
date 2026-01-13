use crate::codec::{
    Codec, CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState, EncoderState,
};
use crate::constants::MAX_TRACK_JAM_NUMBER_OF_TARGETS;
use crate::electromagnetic_emission::model::{
    ElectromagneticEmission, EmitterBeam, EmitterSystem, FundamentalParameter, PulseWidthFloat,
    SiteAppPair, TrackJam,
};
use crate::records::codec::{
    decode_entity_coordinate_vector, encode_entity_coordinate_vector_meters,
};
use crate::records::model::{
    BeamData, EntityCoordinateVector, EntityId, FrequencyFloat, UnitsMeters,
};
use crate::types::model::{CdisFloat, UVINT16, UVINT8};
use crate::{BodyProperties, CdisBody};
use dis_rs::electromagnetic_emission::model::{Beam, FundamentalParameterData, JammingTechnique};
use dis_rs::enumerations::{
    BeamStatusBeamState, EmitterName, EmitterSystemFunction, HighDensityTrackJam,
};
use dis_rs::model::{
    BeamData as DisBeamData, EntityId as DisEntityId, EventId, PduBody, SimulationAddress,
    VectorF32,
};
use dis_rs::BodyRaw;
use num_traits::ToPrimitive;
use std::collections::HashMap;
use std::time::Instant;

type Counterpart = dis_rs::electromagnetic_emission::model::ElectromagneticEmission;
type EmitterSystemCounterpart = dis_rs::electromagnetic_emission::model::EmitterSystem;
type EmitterBeamCounterpart = Beam;
type TrackJamCounterpart = dis_rs::electromagnetic_emission::model::TrackJam;

pub(crate) fn encode_electromagnetic_emission_body_and_update_state(
    dis_body: &Counterpart,
    state: &mut EncoderState,
    options: &CodecOptions,
) -> (CdisBody, CodecStateResult) {
    let state_for_id = state.ee.get(&dis_body.emitting_entity_id);

    let (cdis_body, state_result) =
        ElectromagneticEmission::encode(dis_body, state_for_id, options);

    if state_result == CodecStateResult::StateUpdateElectromagneticEmission {
        state
            .ee
            .entry(dis_body.emitting_entity_id)
            .and_modify(|ee| {
                ee.heartbeat = Instant::now();
                set_fundamental_params_encoder_state(dis_body, &mut ee.previous_fundamental_params);
                set_beam_data_encoder_state(dis_body, &mut ee.previous_beam_data);
            })
            .or_insert_with(|| EncoderStateElectromagneticEmission::new(dis_body));
    }

    (cdis_body.into_cdis_body(), state_result)
}

fn set_fundamental_params_encoder_state(
    dis_body: &Counterpart,
    map: &mut HashMap<(u8, u8), FundamentalParameterData>,
) {
    map.clear();
    for emitter_system in &dis_body.emitter_systems {
        for beam in &emitter_system.beams {
            map.insert((emitter_system.number, beam.number), beam.parameter_data);
        }
    }
}

fn set_beam_data_encoder_state(dis_body: &Counterpart, map: &mut HashMap<(u8, u8), DisBeamData>) {
    map.clear();
    for emitter_system in &dis_body.emitter_systems {
        for beam in &emitter_system.beams {
            map.insert((emitter_system.number, beam.number), beam.beam_data);
        }
    }
}

fn set_emitter_system_partial_fields_state(
    dis_body: &Counterpart,
    map: &mut HashMap<u8, EmitterSystemPartialFields>,
) {
    map.clear();
    for emitter_system in &dis_body.emitter_systems {
        map.insert(
            emitter_system.number,
            EmitterSystemPartialFields {
                emitter_name: emitter_system.name,
                emitter_function: emitter_system.function,
                emitter_location: emitter_system.location,
            },
        );
    }
}

pub(crate) fn decode_electromagnetic_emission_body_and_update_state(
    cdis_body: &ElectromagneticEmission,
    state: &mut DecoderState,
    options: &CodecOptions,
) -> (PduBody, CodecStateResult) {
    let emitting_id = dis_rs::model::EntityId::from(&cdis_body.emitting_id);
    let state_for_id = state.ee.get(&emitting_id);
    let (dis_body, state_result) = cdis_body.decode(state_for_id, options);

    if state_result == CodecStateResult::StateUpdateElectromagneticEmission {
        state
            .ee
            .entry(emitting_id)
            .and_modify(|ee| {
                ee.heartbeat = Instant::now();
                set_fundamental_params_encoder_state(&dis_body, &mut ee.fundamental_params);
                set_beam_data_encoder_state(&dis_body, &mut ee.beam_data);
                set_emitter_system_partial_fields_state(&dis_body, &mut ee.emitter_system_fields);
            })
            .or_insert(DecoderStateElectromagneticEmission::new(&dis_body));
    }

    (dis_body.into_pdu_body(), state_result)
}

/// Encoder maintained state for a given `EntityId`
/// - Timestamp `heartbeat` for EE heartbeat for this `EntityId`.
/// - Previously send `FundamentalParameterData` for a specific (emitter system number, beam number) pair.
/// - Previously send `BeamData` for a specific (emitter system number, beam number) pair.
///
/// The `EncoderStateElectromagneticEmission` is initialised when a first EE PDU from an `EntityId` is received.
/// It is updated after receiving a DIS PDU and a full update to C-DIS is needed/send.
/// The state of the `FundamentalParameterData` and (Dis)BeamData is used during construction of the
/// fundamental params and beam data lists. When the data has not changed the param is left of the list,
/// and consequently the index in the specific beam where the data is referenced will be left out of partial updates (as `Option::None`).
#[derive(Debug)]
pub struct EncoderStateElectromagneticEmission {
    pub heartbeat: Instant,
    pub previous_fundamental_params: HashMap<(u8, u8), FundamentalParameterData>,
    pub previous_beam_data: HashMap<(u8, u8), DisBeamData>,
}

impl EncoderStateElectromagneticEmission {
    fn new(dis_body: &Counterpart) -> Self {
        let mut previous_fundamental_params = HashMap::new();
        let mut previous_beam_data = HashMap::new();

        set_fundamental_params_encoder_state(dis_body, &mut previous_fundamental_params);
        set_beam_data_encoder_state(dis_body, &mut previous_beam_data);

        EncoderStateElectromagneticEmission {
            heartbeat: Instant::now(),
            previous_fundamental_params,
            previous_beam_data,
        }
    }
}

impl Default for EncoderStateElectromagneticEmission {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
            previous_fundamental_params: HashMap::new(),
            previous_beam_data: HashMap::new(),
        }
    }
}

/// Decoder maintained state for a given `EntityId`:
/// - Timestamp `heartbeat` of last received EE for this `EntityId`
/// - Last received `FundamentalParameterData` for a specific (emitter system number, beam number) pair.
/// - Last received `BeamData` for a specific (emitter system number, beam number) pair.
/// - Last received fields for Emitter Systems: Name, Function, Location with respect to Entity.
///
/// The `DecoderStateElectromagneticEmission` for an `EntityId` is initialised when a full update C-DIS PDU
/// is received. The state is updated when new full update C-DIS PDUs are received.
/// Fields in partial update PDUs are complemented from the state.
#[derive(Debug)]
pub struct DecoderStateElectromagneticEmission {
    pub heartbeat: Instant,
    pub fundamental_params: HashMap<(u8, u8), FundamentalParameterData>,
    pub beam_data: HashMap<(u8, u8), DisBeamData>,
    pub emitter_system_fields: HashMap<u8, EmitterSystemPartialFields>,
}

impl DecoderStateElectromagneticEmission {
    fn new(dis_body: &Counterpart) -> Self {
        let mut previous_fundamental_params = HashMap::new();
        let mut previous_beam_data = HashMap::new();
        let mut previous_emitter_system_fields = HashMap::new();

        set_fundamental_params_encoder_state(dis_body, &mut previous_fundamental_params);
        set_beam_data_encoder_state(dis_body, &mut previous_beam_data);
        set_emitter_system_partial_fields_state(dis_body, &mut previous_emitter_system_fields);

        Self {
            heartbeat: Instant::now(),
            fundamental_params: previous_fundamental_params,
            beam_data: previous_beam_data,
            emitter_system_fields: previous_emitter_system_fields,
        }
    }
}

impl Default for DecoderStateElectromagneticEmission {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
            fundamental_params: HashMap::new(),
            beam_data: HashMap::new(),
            emitter_system_fields: HashMap::new(),
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct EmitterSystemPartialFields {
    pub emitter_name: EmitterName,
    pub emitter_function: EmitterSystemFunction,
    pub emitter_location: VectorF32,
}

impl ElectromagneticEmission {
    #[allow(
        clippy::missing_panics_doc,
        reason = "Unwrap only occurs within an if statement that checks for Option::is_some()"
    )]
    #[must_use]
    pub fn encode(
        item: &Counterpart,
        state: Option<&EncoderStateElectromagneticEmission>,
        options: &CodecOptions,
    ) -> (Self, CodecStateResult) {
        let site_app_pairs_list = construct_site_app_pairs_list(item);

        let (
            full_update_flag,
            fundamental_param_list,
            beam_data_list,
            site_app_pairs_list,
            emitter_systems,
            state_result,
        ) = if options.update_mode == CodecUpdateMode::PartialUpdate
            && state.is_some_and(|state| !evaluate_timeout_for_ee(&state.heartbeat, options))
        {
            // Do not update stateful fields when a full update is not required
            let fundamental_param_list =
                construct_fundamental_params_list_partial(state.unwrap(), item);
            let beam_data_list = construct_beam_data_list_partial(state.unwrap(), item);

            let emitter_systems = item
                .emitter_systems
                .iter()
                .map(|emitter| {
                    EmitterSystem::encode_partial_update(
                        emitter,
                        &fundamental_param_list,
                        &beam_data_list,
                        &site_app_pairs_list,
                    )
                })
                .collect::<Vec<EmitterSystem>>();
            (
                false,
                fundamental_param_list,
                beam_data_list,
                site_app_pairs_list,
                emitter_systems,
                CodecStateResult::StateUnaffected,
            )
        } else {
            // full update mode, or partial with a (state) timeout on the entity
            let fundamental_param_list = construct_fundamental_params_list_full(item);
            let beam_data_list = construct_beam_data_list_full(item);

            let emitter_systems = item
                .emitter_systems
                .iter()
                .map(|emitter| {
                    EmitterSystem::encode_full_update(
                        emitter,
                        &fundamental_param_list,
                        &beam_data_list,
                        &site_app_pairs_list,
                    )
                })
                .collect::<Vec<EmitterSystem>>();
            (
                true,
                fundamental_param_list,
                beam_data_list,
                site_app_pairs_list,
                emitter_systems,
                if options.update_mode == CodecUpdateMode::PartialUpdate {
                    CodecStateResult::StateUpdateElectromagneticEmission
                } else {
                    CodecStateResult::StateUnaffected
                },
            )
        };

        (
            Self {
                full_update_flag,
                emitting_id: EntityId::encode(&item.emitting_entity_id),
                event_id: EntityId::from(&item.event_id),
                state_update_indicator: item.state_update_indicator,
                fundamental_params: fundamental_param_list,
                beam_data: beam_data_list,
                site_app_pairs: site_app_pairs_list,
                emitter_systems,
            },
            state_result,
        )
    }

    #[must_use]
    pub fn decode(
        &self,
        state: Option<&DecoderStateElectromagneticEmission>,
        options: &CodecOptions,
    ) -> (Counterpart, CodecStateResult) {
        match options.update_mode {
            CodecUpdateMode::FullUpdate => {
                let mut emitter_systems = self
                    .emitter_systems
                    .iter()
                    .map(|system| {
                        system.decode_full_update(
                            self.fundamental_params.as_slice(),
                            self.beam_data.as_slice(),
                            self.site_app_pairs.as_slice(),
                        )
                    })
                    .collect();

                (
                    Counterpart::builder()
                        .with_emitting_entity_id(self.emitting_id.decode())
                        .with_event_id(EventId::from(&self.event_id))
                        .with_state_update_indicator(self.state_update_indicator)
                        .with_emitter_systems(&mut emitter_systems)
                        .build(),
                    CodecStateResult::StateUnaffected,
                )
            }
            CodecUpdateMode::PartialUpdate => {
                let (mut emitters, state_result) = if self.full_update_flag {
                    (
                        self.emitter_systems
                            .iter()
                            .map(|system| {
                                system.decode_full_update(
                                    self.fundamental_params.as_slice(),
                                    self.beam_data.as_slice(),
                                    self.site_app_pairs.as_slice(),
                                )
                            })
                            .collect(),
                        CodecStateResult::StateUpdateElectromagneticEmission,
                    )
                } else {
                    (
                        self.emitter_systems
                            .iter()
                            .map(|system| {
                                system.decode_partial_update(
                                    state,
                                    self.fundamental_params.as_slice(),
                                    self.beam_data.as_slice(),
                                    self.site_app_pairs.as_slice(),
                                )
                            })
                            .collect(),
                        CodecStateResult::StateUnaffected,
                    )
                };
                (
                    Counterpart::builder()
                        .with_emitting_entity_id(self.emitting_id.decode())
                        .with_event_id(EventId::from(&self.event_id))
                        .with_state_update_indicator(self.state_update_indicator)
                        .with_emitter_systems(&mut emitters)
                        .build(),
                    state_result,
                )
            }
        }
    }
}

fn evaluate_timeout_for_ee(last_heartbeat: &Instant, options: &CodecOptions) -> bool {
    let elapsed = last_heartbeat.elapsed().as_secs_f32();
    elapsed > (options.federation_parameters.HBT_PDU_EE * options.hbt_cdis_full_update_mplier)
}

fn construct_fundamental_params_list_full(item: &Counterpart) -> Vec<FundamentalParameter> {
    let mut params = item
        .emitter_systems
        .iter()
        .flat_map(|emitter| {
            emitter
                .beams
                .iter()
                .map(|beam| FundamentalParameter::encode(&beam.parameter_data))
                .collect::<Vec<FundamentalParameter>>()
        })
        .collect::<Vec<FundamentalParameter>>();

    params.sort();
    params.dedup();

    params
}

fn construct_fundamental_params_list_partial(
    state: &EncoderStateElectromagneticEmission,
    item: &Counterpart,
) -> Vec<FundamentalParameter> {
    let mut params = vec![];
    for emitter_system in &item.emitter_systems {
        for beam in &emitter_system.beams {
            if let Some(stored_param) = state
                .previous_fundamental_params
                .get(&(emitter_system.number, beam.number))
            {
                if *stored_param != beam.parameter_data {
                    // param has changed, so it is included in the list
                    params.push(FundamentalParameter::encode(&beam.parameter_data));
                }
            }
        }
    }

    params.sort();
    params.dedup();

    params
}

fn find_fundamental_param_index(
    list: &[FundamentalParameter],
    item: &FundamentalParameter,
) -> Option<usize> {
    list.iter()
        .enumerate()
        .find(|(_, param)| *param == item)
        .map(|(index, _)| index)
}

fn construct_beam_data_list_full(item: &Counterpart) -> Vec<BeamData> {
    let mut beams = item
        .emitter_systems
        .iter()
        .flat_map(|emitter| {
            emitter
                .beams
                .iter()
                .map(|beam| BeamData::encode(&beam.beam_data))
                .collect::<Vec<BeamData>>()
        })
        .collect::<Vec<BeamData>>();

    beams.sort();
    beams.dedup();

    beams
}

fn construct_beam_data_list_partial(
    state: &EncoderStateElectromagneticEmission,
    item: &Counterpart,
) -> Vec<BeamData> {
    let mut beams = vec![];
    for emitter_system in &item.emitter_systems {
        for beam in &emitter_system.beams {
            if let Some(stored_beam) = state
                .previous_beam_data
                .get(&(emitter_system.number, beam.number))
            {
                if *stored_beam != beam.beam_data {
                    // beam data has changed, so it is included in the list
                    beams.push(BeamData::encode(&beam.beam_data));
                }
            }
        }
    }

    beams.sort();
    beams.dedup();

    beams
}

fn find_beam_data_index(list: &[BeamData], item: &BeamData) -> Option<usize> {
    list.iter()
        .enumerate()
        .find(|(_, beam_data)| *beam_data == item)
        .map(|(index, _)| index)
}

fn construct_site_app_pairs_list(item: &Counterpart) -> Vec<SiteAppPair> {
    let mut pairs: Vec<SimulationAddress> = item
        .emitter_systems
        .iter()
        .flat_map(|emitter| {
            emitter
                .beams
                .iter()
                .flat_map(|beam| {
                    beam.track_jam_data
                        .iter()
                        .map(|tj| tj.entity_id.simulation_address)
                        .collect::<Vec<SimulationAddress>>()
                })
                .collect::<Vec<SimulationAddress>>()
        })
        .collect::<Vec<SimulationAddress>>();

    pairs.sort();
    pairs.dedup();

    pairs
        .iter()
        .map(|sa| SiteAppPair {
            site: UVINT16::from(sa.site_id),
            application: UVINT16::from(sa.application_id),
        })
        .collect()
}

fn find_site_app_pair_index(list: &[SiteAppPair], item: &SiteAppPair) -> Option<usize> {
    list.iter()
        .enumerate()
        .find(|(_, pair)| *pair == item)
        .map(|(index, _)| index)
}

impl EmitterSystem {
    #[must_use]
    pub fn encode_full_update(
        item: &EmitterSystemCounterpart,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> Self {
        let emitter_beams = item
            .beams
            .iter()
            .map(|beam| {
                EmitterBeam::encode_full_update(
                    beam,
                    fundamental_params,
                    beam_data_list,
                    site_app_pair_list,
                )
            })
            .collect::<Vec<EmitterBeam>>();

        Self {
            name: Some(item.name),
            function: Some(item.function),
            number: UVINT8::from(item.number),
            location_with_respect_to_entity: Some(encode_entity_coordinate_vector_meters(
                &item.location,
            )),
            emitter_beams,
        }
    }

    #[must_use]
    pub fn encode_partial_update(
        item: &EmitterSystemCounterpart,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> Self {
        let emitter_beams = item
            .beams
            .iter()
            .map(|beam| {
                EmitterBeam::encode_partial_update(
                    beam,
                    fundamental_params,
                    beam_data_list,
                    site_app_pair_list,
                )
            })
            .collect::<Vec<EmitterBeam>>();

        Self {
            name: None,
            function: None,
            number: UVINT8::from(item.number),
            location_with_respect_to_entity: None,
            emitter_beams,
        }
    }

    #[must_use]
    pub fn decode_full_update(
        &self,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> EmitterSystemCounterpart {
        let mut beams = self
            .emitter_beams
            .iter()
            .map(|beam| {
                beam.decode_full_update(fundamental_params, beam_data_list, site_app_pair_list)
            })
            .collect();
        EmitterSystemCounterpart::default()
            .with_name(self.name.unwrap_or(EmitterName::from(0)))
            .with_function(self.function.unwrap_or(EmitterSystemFunction::from(0)))
            .with_number(self.number.value)
            .with_location(decode_entity_coordinate_vector(
                &self.location_with_respect_to_entity.unwrap_or_default(),
                UnitsMeters::Meter,
            ))
            .with_beams(&mut beams)
    }

    #[must_use]
    pub fn decode_partial_update(
        &self,
        state: Option<&DecoderStateElectromagneticEmission>,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> EmitterSystemCounterpart {
        let mut beams = self
            .emitter_beams
            .iter()
            .map(|beam| {
                beam.decode_partial_update(
                    self.number.value,
                    state,
                    fundamental_params,
                    beam_data_list,
                    site_app_pair_list,
                )
            })
            .collect();

        EmitterSystemCounterpart::default()
            .with_name(
                self.name.unwrap_or(match state {
                    None => EmitterName::from(0),
                    Some(state) => {
                        state
                            .emitter_system_fields
                            .get(&self.number.value)
                            .unwrap_or(&EmitterSystemPartialFields::default())
                            .emitter_name
                    }
                }),
            )
            .with_function(
                self.function.unwrap_or(match state {
                    None => EmitterSystemFunction::from(0),
                    Some(state) => {
                        state
                            .emitter_system_fields
                            .get(&self.number.value)
                            .unwrap_or(&EmitterSystemPartialFields::default())
                            .emitter_function
                    }
                }),
            )
            .with_number(self.number.value)
            .with_location(
                if let Some(location) = &self.location_with_respect_to_entity {
                    decode_entity_coordinate_vector(location, UnitsMeters::Meter)
                } else {
                    match state {
                        None => decode_entity_coordinate_vector(
                            &EntityCoordinateVector::default(),
                            UnitsMeters::Meter,
                        ),
                        Some(state) => {
                            state
                                .emitter_system_fields
                                .get(&self.number.value)
                                .unwrap_or(&EmitterSystemPartialFields::default())
                                .emitter_location
                        }
                    }
                },
            )
            .with_beams(&mut beams)
    }
}

impl EmitterBeam {
    #[must_use]
    pub fn encode_full_update(
        item: &EmitterBeamCounterpart,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> Self {
        let fundamental_params_index = find_fundamental_param_index(
            fundamental_params,
            &FundamentalParameter::encode(&item.parameter_data),
        );
        let beam_data_index =
            find_beam_data_index(beam_data_list, &BeamData::encode(&item.beam_data));

        // Table 62: If more than 15 remove list and set High Density Track Jam Flag
        let (track_jam, high_density_track_jam) =
            if item.track_jam_data.len() > MAX_TRACK_JAM_NUMBER_OF_TARGETS {
                (vec![], HighDensityTrackJam::Selected)
            } else {
                // otherwise build the list of tracks/jammers and take the actual high_density_track_jam value
                (
                    item.track_jam_data
                        .iter()
                        .map(|tj| TrackJam::encode_full_update(tj, site_app_pair_list))
                        .collect::<Vec<TrackJam>>(),
                    item.high_density_track_jam,
                )
            };

        Self {
            beam_id: UVINT8::from(item.number),
            beam_parameter_index: item.parameter_index,
            fundamental_params_index: Some(fundamental_params_index.unwrap_or(0) as u8),
            beam_data_index: Some(beam_data_index.unwrap_or(0) as u8),
            beam_function: item.beam_function,
            high_density_track_jam,
            beam_status: item.beam_status == BeamStatusBeamState::Active,
            jamming_technique_kind: Some(UVINT8::from(item.jamming_technique.kind)),
            jamming_technique_category: Some(UVINT8::from(item.jamming_technique.category)),
            jamming_technique_subcategory: Some(UVINT8::from(item.jamming_technique.subcategory)),
            jamming_technique_specific: Some(UVINT8::from(item.jamming_technique.specific)),
            track_jam,
        }
    }

    #[must_use]
    pub fn encode_partial_update(
        item: &EmitterBeamCounterpart,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> Self {
        let fundamental_params_index = find_fundamental_param_index(
            fundamental_params,
            &FundamentalParameter::encode(&item.parameter_data),
        );
        let beam_data_index =
            find_beam_data_index(beam_data_list, &BeamData::encode(&item.beam_data));

        // Table 62: If more than 15 remove list and set High Density Track Jam Flag
        let (track_jam, high_density_track_jam) =
            if item.track_jam_data.len() > MAX_TRACK_JAM_NUMBER_OF_TARGETS {
                (vec![], HighDensityTrackJam::Selected)
            } else {
                // otherwise build the list of tracks/jammers and take the actual high_density_track_jam value
                (
                    item.track_jam_data
                        .iter()
                        .map(|tj| TrackJam::encode_partial_update(tj, site_app_pair_list))
                        .collect::<Vec<TrackJam>>(),
                    item.high_density_track_jam,
                )
            };

        Self {
            beam_id: UVINT8::from(item.number),
            beam_parameter_index: item.parameter_index,
            fundamental_params_index: fundamental_params_index.map(|index| index as u8),
            beam_data_index: beam_data_index.map(|index| index as u8),
            beam_function: item.beam_function,
            high_density_track_jam,
            beam_status: item.beam_status == BeamStatusBeamState::Active,
            jamming_technique_kind: None,
            jamming_technique_category: None,
            jamming_technique_subcategory: None,
            jamming_technique_specific: None,
            track_jam,
        }
    }

    #[must_use]
    pub fn decode_full_update(
        &self,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> EmitterBeamCounterpart {
        let mut tjs = self
            .track_jam
            .iter()
            .map(|tj| tj.decode(site_app_pair_list))
            .collect();
        EmitterBeamCounterpart::default()
            .with_number(self.beam_id.value)
            .with_parameter_index(self.beam_parameter_index)
            .with_parameter_data(match self.fundamental_params_index {
                None => FundamentalParameterData::default(), // should be available in a full update
                Some(index) => fundamental_params
                    .get(index as usize)
                    .unwrap_or(&FundamentalParameter::default())
                    .decode(),
            })
            .with_beam_data(match self.beam_data_index {
                None => DisBeamData::default(), // should be available in a full update
                Some(index) => beam_data_list
                    .get(index as usize)
                    .unwrap_or(&BeamData::default())
                    .decode(),
            })
            .with_beam_function(self.beam_function)
            .with_high_density_track_jam(self.high_density_track_jam)
            .with_jamming_technique(
                JammingTechnique::default()
                    .with_kind(self.jamming_technique_kind.map_or(0, |kind| kind.value))
                    .with_category(
                        self.jamming_technique_category
                            .map_or(0, |category| category.value),
                    )
                    .with_subcategory(
                        self.jamming_technique_subcategory
                            .map_or(0, |subcategory| subcategory.value),
                    )
                    .with_specific(
                        self.jamming_technique_specific
                            .map_or(0, |specific| specific.value),
                    ),
            )
            .with_beam_status(if self.beam_status {
                BeamStatusBeamState::Active
            } else {
                BeamStatusBeamState::Deactivated
            })
            .with_track_jams(&mut tjs)
    }

    #[must_use]
    pub fn decode_partial_update(
        &self,
        emitter_number: u8,
        state: Option<&DecoderStateElectromagneticEmission>,
        fundamental_params: &[FundamentalParameter],
        beam_data_list: &[BeamData],
        site_app_pair_list: &[SiteAppPair],
    ) -> EmitterBeamCounterpart {
        let mut tjs = self
            .track_jam
            .iter()
            .map(|tj| tj.decode(site_app_pair_list))
            .collect();
        EmitterBeamCounterpart::default()
            .with_number(self.beam_id.value)
            .with_parameter_index(self.beam_parameter_index)
            .with_parameter_data(match self.fundamental_params_index {
                None => {
                    // retrieve from state
                    if let Some(state) = state {
                        if let Some(data) = state
                            .fundamental_params
                            .get(&(emitter_number, self.beam_id.value))
                        {
                            *data
                        } else {
                            FundamentalParameterData::default()
                        } // partial update before a full update is received
                    } else {
                        // no state found
                        FundamentalParameterData::default()
                    }
                }
                Some(index) => fundamental_params
                    .get(index as usize)
                    .unwrap_or(&FundamentalParameter::default())
                    .decode(),
            })
            .with_beam_data(match self.beam_data_index {
                None => {
                    if let Some(state) = state {
                        if let Some(data) =
                            state.beam_data.get(&(emitter_number, self.beam_id.value))
                        {
                            *data
                        } else {
                            DisBeamData::default()
                        }
                    } else {
                        DisBeamData::default()
                    }
                }
                Some(index) => beam_data_list
                    .get(index as usize)
                    .unwrap_or(&BeamData::default())
                    .decode(),
            })
            .with_beam_function(self.beam_function)
            .with_high_density_track_jam(self.high_density_track_jam)
            .with_jamming_technique(
                JammingTechnique::default()
                    .with_kind(self.jamming_technique_kind.map_or(0, |kind| kind.value))
                    .with_category(
                        self.jamming_technique_category
                            .map_or(0, |category| category.value),
                    )
                    .with_subcategory(
                        self.jamming_technique_subcategory
                            .map_or(0, |subcategory| subcategory.value),
                    )
                    .with_specific(
                        self.jamming_technique_specific
                            .map_or(0, |specific| specific.value),
                    ),
            )
            .with_beam_status(if self.beam_status {
                BeamStatusBeamState::Active
            } else {
                BeamStatusBeamState::Deactivated
            })
            .with_track_jams(&mut tjs)
    }
}

impl TrackJam {
    fn encode_full_update(item: &TrackJamCounterpart, list: &[SiteAppPair]) -> Self {
        let pair = SiteAppPair {
            site: UVINT16::from(item.entity_id.simulation_address.site_id),
            application: UVINT16::from(item.entity_id.simulation_address.application_id),
        };
        let site_app_pair_index = find_site_app_pair_index(list, &pair);
        Self {
            site_app_pair_index: site_app_pair_index.unwrap_or(0) as u8,
            entity_id: UVINT16::from(item.entity_id.entity_id),
            emitter_number: Some(UVINT8::from(item.emitter)),
            beam_number: Some(UVINT8::from(item.beam)),
        }
    }

    fn encode_partial_update(item: &TrackJamCounterpart, list: &[SiteAppPair]) -> Self {
        let pair = SiteAppPair {
            site: UVINT16::from(item.entity_id.simulation_address.site_id),
            application: UVINT16::from(item.entity_id.simulation_address.application_id),
        };
        let site_app_pair_index = find_site_app_pair_index(list, &pair);
        Self {
            site_app_pair_index: site_app_pair_index.unwrap_or(0) as u8,
            entity_id: UVINT16::from(item.entity_id.entity_id),
            emitter_number: None,
            beam_number: None,
        }
    }

    fn decode(&self, list: &[SiteAppPair]) -> TrackJamCounterpart {
        let simulation_address = match list.get(self.site_app_pair_index as usize) {
            None => SimulationAddress::default(), // initialise simulation address as zeroes when not in the SiteAppPair list
            Some(address) => SimulationAddress::new(address.site.value, address.application.value),
        };

        TrackJamCounterpart::default()
            .with_entity_id(DisEntityId::new_sim_address(
                simulation_address,
                self.entity_id.value,
            ))
            .with_emitter(match self.emitter_number {
                None => 0,
                Some(number) => number.value,
            })
            .with_beam(match self.beam_number {
                None => 0,
                Some(number) => number.value,
            })
    }
}

impl Codec for FundamentalParameter {
    type Counterpart = FundamentalParameterData;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            frequency: FrequencyFloat::from_float(item.frequency),
            frequency_range: FrequencyFloat::from_float(item.frequency_range),
            erp: item.effective_power.to_u8().unwrap_or(0), // TODO check if conversion is correct
            prf: UVINT16::from(item.pulse_repetition_frequency.to_u16().unwrap_or(0)), // TODO check if conversion is correct
            pulse_width: PulseWidthFloat::from_float(item.pulse_width),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_frequency(self.frequency.to_float())
            .with_frequency_range(self.frequency_range.to_float())
            .with_effective_power(self.erp.to_f32().unwrap_or(0.0)) // TODO check if conversion is correct
            .with_pulse_repetition_frequency(self.prf.value.to_f32().unwrap_or(0.0)) // TODO check if conversion is correct
            .with_pulse_width(self.pulse_width.to_float())
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::{Codec, CodecOptions, CodecStateResult};
    use crate::electromagnetic_emission::codec::{
        construct_beam_data_list_full, construct_fundamental_params_list_full,
        construct_site_app_pairs_list, DecoderStateElectromagneticEmission,
        EncoderStateElectromagneticEmission,
    };
    use crate::electromagnetic_emission::model::{
        ElectromagneticEmission, EmitterBeam, FundamentalParameter, SiteAppPair,
    };
    use crate::records::model::{BeamData, EntityCoordinateVector, FrequencyFloat};
    use crate::types::model::{CdisFloat, SVINT16, UVINT16, UVINT8};
    use dis_rs::electromagnetic_emission::model::{
        Beam, ElectromagneticEmission as DisEE, EmitterSystem, FundamentalParameterData, TrackJam,
    };
    use dis_rs::enumerations::{
        ElectromagneticEmissionBeamFunction, ElectromagneticEmissionStateUpdateIndicator,
        EmitterName, EmitterSystemFunction, HighDensityTrackJam,
    };
    use dis_rs::model::{BeamData as DisBeamData, EntityId, EventId, VectorF32};
    use dis_rs::BodyRaw;

    #[test]
    fn fundamental_params_list() {
        let param_1 = FundamentalParameterData::default()
            .with_frequency(12.3)
            .with_frequency_range(1.23)
            .with_effective_power(1.0)
            .with_pulse_repetition_frequency(1.0)
            .with_pulse_width(123.1);
        let param_2 = FundamentalParameterData::default()
            .with_frequency(12.3)
            .with_frequency_range(1.23)
            .with_effective_power(1.0)
            .with_pulse_repetition_frequency(1.0)
            .with_pulse_width(123.1);
        let param_3 = FundamentalParameterData::default()
            .with_frequency(23.4)
            .with_frequency_range(2.34)
            .with_effective_power(2.0)
            .with_pulse_repetition_frequency(2.0)
            .with_pulse_width(234.5);

        let body = DisEE::builder()
            .with_emitter_system(
                EmitterSystem::default()
                    .with_beam(Beam::default().with_parameter_data(param_1))
                    .with_beam(Beam::default().with_parameter_data(param_2)),
            )
            .with_emitter_system(
                EmitterSystem::default().with_beam(Beam::default().with_parameter_data(param_3)),
            )
            .build();

        let param_list = construct_fundamental_params_list_full(&body);

        assert_eq!(param_list.len(), 2);
        assert_eq!(
            param_list.first().unwrap().frequency,
            FrequencyFloat::from_float(12.3)
        );
        assert_eq!(
            param_list.get(1).unwrap().frequency,
            FrequencyFloat::from_float(23.4)
        );
    }

    #[test]
    fn beam_data_list() {
        let beam_data_1 = DisBeamData::default()
            .with_azimuth_center(0.0)
            .with_azimuth_sweep(20.0)
            .with_elevation_center(0.0)
            .with_elevation_sweep(20.0)
            .with_sweep_sync(50.0);
        let beam_data_2 = DisBeamData::default()
            .with_azimuth_center(20.0)
            .with_azimuth_sweep(30.0)
            .with_elevation_center(10.0)
            .with_elevation_sweep(30.0)
            .with_sweep_sync(10.0);
        let beam_data_3 = DisBeamData::default()
            .with_azimuth_center(0.0)
            .with_azimuth_sweep(20.0)
            .with_elevation_center(0.0)
            .with_elevation_sweep(20.0)
            .with_sweep_sync(50.0);

        let body = DisEE::builder()
            .with_emitter_system(
                EmitterSystem::default()
                    .with_beam(Beam::default().with_beam_data(beam_data_1))
                    .with_beam(Beam::default().with_beam_data(beam_data_2)),
            )
            .with_emitter_system(
                EmitterSystem::default().with_beam(Beam::default().with_beam_data(beam_data_3)),
            )
            .build();

        let beam_data_list = construct_beam_data_list_full(&body);

        assert_eq!(beam_data_list.len(), 2);
        assert_eq!(beam_data_list.first().unwrap().az_center.value, 0);
        assert_eq!(beam_data_list.first().unwrap().sweep_sync, 511);
        assert_eq!(beam_data_list.get(1).unwrap().az_center.value, 83);
        assert_eq!(beam_data_list.get(1).unwrap().sweep_sync, 102);
    }

    #[test]
    fn site_app_pairs_list() {
        let track_1 = TrackJam::default().with_entity_id(EntityId::new(1, 1, 1));
        let track_2 = TrackJam::default().with_entity_id(EntityId::new(1, 1, 2));
        let track_3 = TrackJam::default().with_entity_id(EntityId::new(2, 2, 2));
        let track_4 = TrackJam::default().with_entity_id(EntityId::new(3, 3, 3));
        let mut list_1 = vec![track_1, track_2];

        let body = DisEE::builder()
            .with_emitter_system(
                EmitterSystem::default()
                    .with_beam(Beam::default().with_track_jams(&mut list_1))
                    .with_beam(Beam::default().with_track_jam(track_3)),
            )
            .with_emitter_system(
                EmitterSystem::default().with_beam(Beam::default().with_track_jam(track_4)),
            )
            .build();

        let pairs = construct_site_app_pairs_list(&body);

        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs.first().unwrap().site.value, 1);
        assert_eq!(pairs.first().unwrap().application.value, 1);
        assert_eq!(pairs.get(1).unwrap().site.value, 2);
        assert_eq!(pairs.get(1).unwrap().application.value, 2);
        assert_eq!(pairs.get(2).unwrap().site.value, 3);
        assert_eq!(pairs.get(2).unwrap().application.value, 3);
    }

    #[test]
    fn encode_decode_beam_data() {
        let input = DisBeamData::new()
            .with_azimuth_center(0.0)
            .with_azimuth_sweep(20.0)
            .with_elevation_center(0.0)
            .with_elevation_sweep(15.0)
            .with_sweep_sync(100.0);

        let encoded = BeamData::encode(&input);

        let output = encoded.decode();
        assert_eq!(input.azimuth_center, output.azimuth_center);
        assert!((19.8f32..20.2f32).contains(&input.azimuth_sweep));
        assert_eq!(input.elevation_center, output.elevation_center);
        assert!((14.9f32..15.1f32).contains(&input.elevation_sweep));
        assert_eq!(encoded.sweep_sync, 1023);
        assert!((99.99f32..100.01f32).contains(&output.sweep_sync));
    }

    #[test]
    fn encode_body_full_update() {
        let dis_body = DisEE::builder()
            .with_emitting_entity_id(EntityId::new(1, 1, 1))
            .with_event_id(EventId::new(1, 1, 100))
            .with_state_update_indicator(
                ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            )
            .with_emitter_system(
                EmitterSystem::default()
                    .with_number(20)
                    .with_name(EmitterName::ANFPS16_5505)
                    .with_function(EmitterSystemFunction::SearchAcquisition_102)
                    .with_location(VectorF32::new(1.0, 2.0, 3.0))
                    .with_beam(
                        Beam::default()
                            .with_number(1)
                            .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                            .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                            .with_parameter_index(1)
                            .with_parameter_data(FundamentalParameterData::default())
                            .with_track_jam(
                                TrackJam::default().with_entity_id(EntityId::new(1, 2, 3)),
                            ),
                    ),
            )
            .build();

        let state = EncoderStateElectromagneticEmission::new(&dis_body);
        let options = CodecOptions::new_full_update();

        let (cdis_body, state_result) =
            ElectromagneticEmission::encode(&dis_body, Some(&state), &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

        assert!(cdis_body.full_update_flag);
        assert_eq!(
            cdis_body.emitting_id,
            crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(1)
            )
        );
        assert_eq!(cdis_body.emitter_systems.len(), 1);
        assert_eq!(
            cdis_body.emitter_systems.first().unwrap(),
            &crate::electromagnetic_emission::model::EmitterSystem {
                name: Some(EmitterName::ANFPS16_5505),
                function: Some(EmitterSystemFunction::SearchAcquisition_102),
                number: UVINT8::from(20),
                location_with_respect_to_entity: Some(EntityCoordinateVector::new(
                    SVINT16::from(1),
                    SVINT16::from(2),
                    SVINT16::from(3)
                )),
                emitter_beams: vec![EmitterBeam {
                    beam_id: UVINT8::from(1),
                    beam_parameter_index: 1,
                    fundamental_params_index: Some(0),
                    beam_data_index: Some(0),
                    beam_function: ElectromagneticEmissionBeamFunction::Acquisition,
                    high_density_track_jam: HighDensityTrackJam::NotSelected,
                    beam_status: true,
                    jamming_technique_kind: Some(UVINT8::from(0)),
                    jamming_technique_category: Some(UVINT8::from(0)),
                    jamming_technique_subcategory: Some(UVINT8::from(0)),
                    jamming_technique_specific: Some(UVINT8::from(0)),
                    track_jam: vec![crate::electromagnetic_emission::model::TrackJam {
                        site_app_pair_index: 0,
                        entity_id: UVINT16::from(3),
                        emitter_number: Some(UVINT8::from(0)),
                        beam_number: Some(UVINT8::from(0))
                    }],
                }],
            }
        );
    }

    #[test]
    fn encode_body_partial_update_partial_update_flag_without_state() {
        let dis_body = DisEE::builder()
            .with_emitting_entity_id(EntityId::new(1, 1, 1))
            .with_event_id(EventId::new(1, 1, 100))
            .with_state_update_indicator(
                ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            )
            .with_emitter_system(
                EmitterSystem::default()
                    .with_number(20)
                    .with_name(EmitterName::ANFPS16_5505)
                    .with_function(EmitterSystemFunction::SearchAcquisition_102)
                    .with_location(VectorF32::new(1.0, 2.0, 3.0))
                    .with_beam(
                        Beam::default()
                            .with_number(1)
                            .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                            .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                            .with_parameter_index(1)
                            .with_parameter_data(FundamentalParameterData::default())
                            .with_track_jam(
                                TrackJam::default().with_entity_id(EntityId::new(1, 2, 3)),
                            ),
                    ),
            )
            .build();

        let options = CodecOptions::new_partial_update();

        let (cdis_body, state_result) = ElectromagneticEmission::encode(&dis_body, None, &options);

        assert_eq!(
            state_result,
            CodecStateResult::StateUpdateElectromagneticEmission
        );

        assert!(cdis_body.full_update_flag);
        assert_eq!(
            cdis_body.emitting_id,
            crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(1)
            )
        );
        assert_eq!(cdis_body.emitter_systems.len(), 1);
        assert_eq!(
            cdis_body.emitter_systems.first().unwrap(),
            &crate::electromagnetic_emission::model::EmitterSystem {
                name: Some(EmitterName::ANFPS16_5505),
                function: Some(EmitterSystemFunction::SearchAcquisition_102),
                number: UVINT8::from(20),
                location_with_respect_to_entity: Some(EntityCoordinateVector::new(
                    SVINT16::from(1),
                    SVINT16::from(2),
                    SVINT16::from(3)
                )),
                emitter_beams: vec![EmitterBeam {
                    beam_id: UVINT8::from(1),
                    beam_parameter_index: 1,
                    fundamental_params_index: Some(0),
                    beam_data_index: Some(0),
                    beam_function: ElectromagneticEmissionBeamFunction::Acquisition,
                    high_density_track_jam: HighDensityTrackJam::NotSelected,
                    beam_status: true,
                    jamming_technique_kind: Some(UVINT8::from(0)),
                    jamming_technique_category: Some(UVINT8::from(0)),
                    jamming_technique_subcategory: Some(UVINT8::from(0)),
                    jamming_technique_specific: Some(UVINT8::from(0)),
                    track_jam: vec![crate::electromagnetic_emission::model::TrackJam {
                        site_app_pair_index: 0,
                        entity_id: UVINT16::from(3),
                        emitter_number: Some(UVINT8::from(0)),
                        beam_number: Some(UVINT8::from(0))
                    }],
                }],
            }
        );
    }

    #[test]
    fn encode_body_partial_update_partial_update_flag_with_state() {
        let dis_body = DisEE::builder()
            .with_emitting_entity_id(EntityId::new(1, 1, 1))
            .with_event_id(EventId::new(1, 1, 100))
            .with_state_update_indicator(
                ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            )
            .with_emitter_system(
                EmitterSystem::default()
                    .with_number(20)
                    .with_name(EmitterName::ANFPS16_5505)
                    .with_function(EmitterSystemFunction::SearchAcquisition_102)
                    .with_location(VectorF32::new(1.0, 2.0, 3.0))
                    .with_beam(
                        Beam::default()
                            .with_number(1)
                            .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                            .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                            .with_parameter_index(1)
                            .with_parameter_data(FundamentalParameterData::default())
                            .with_track_jam(
                                TrackJam::default().with_entity_id(EntityId::new(1, 2, 3)),
                            ),
                    ),
            )
            .build();

        let state = EncoderStateElectromagneticEmission::new(&dis_body);
        let options = CodecOptions::new_partial_update();

        let (cdis_body, state_result) =
            ElectromagneticEmission::encode(&dis_body, Some(&state), &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);

        assert!(!cdis_body.full_update_flag);
        assert_eq!(
            cdis_body.emitting_id,
            crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(1)
            )
        );
        assert_eq!(cdis_body.emitter_systems.len(), 1);
        assert_eq!(
            cdis_body.emitter_systems.first().unwrap(),
            &crate::electromagnetic_emission::model::EmitterSystem {
                name: None,
                function: None,
                number: UVINT8::from(20),
                location_with_respect_to_entity: None,
                emitter_beams: vec![EmitterBeam {
                    beam_id: UVINT8::from(1),
                    beam_parameter_index: 1,
                    fundamental_params_index: None,
                    beam_data_index: None,
                    beam_function: ElectromagneticEmissionBeamFunction::Acquisition,
                    high_density_track_jam: HighDensityTrackJam::NotSelected,
                    beam_status: true,
                    jamming_technique_kind: None,
                    jamming_technique_category: None,
                    jamming_technique_subcategory: None,
                    jamming_technique_specific: None,
                    track_jam: vec![crate::electromagnetic_emission::model::TrackJam {
                        site_app_pair_index: 0,
                        entity_id: UVINT16::from(3),
                        emitter_number: None,
                        beam_number: None,
                    }],
                }]
            }
        );
    }

    #[test]
    fn decode_body_full_update() {
        let cdis_body = ElectromagneticEmission {
            full_update_flag: true,
            emitting_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(1),
            ),
            event_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(100),
            ),
            state_update_indicator: ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            fundamental_params: vec![FundamentalParameter::default()],
            beam_data: vec![BeamData::default()],
            site_app_pairs: vec![SiteAppPair {
                site: UVINT16::from(1),
                application: UVINT16::from(2),
            }],
            emitter_systems: vec![crate::electromagnetic_emission::model::EmitterSystem {
                name: Some(EmitterName::ANFPS16_5505),
                function: Some(EmitterSystemFunction::SearchAcquisition_102),
                number: UVINT8::from(20),
                location_with_respect_to_entity: Some(EntityCoordinateVector::new(
                    SVINT16::from(1),
                    SVINT16::from(2),
                    SVINT16::from(3),
                )),
                emitter_beams: vec![EmitterBeam {
                    beam_id: UVINT8::from(1),
                    beam_parameter_index: 1,
                    fundamental_params_index: Some(0),
                    beam_data_index: Some(0),
                    beam_function: ElectromagneticEmissionBeamFunction::Acquisition,
                    high_density_track_jam: HighDensityTrackJam::NotSelected,
                    beam_status: true,
                    jamming_technique_kind: Some(UVINT8::from(0)),
                    jamming_technique_category: Some(UVINT8::from(0)),
                    jamming_technique_subcategory: Some(UVINT8::from(0)),
                    jamming_technique_specific: Some(UVINT8::from(0)),
                    track_jam: vec![crate::electromagnetic_emission::model::TrackJam {
                        site_app_pair_index: 0,
                        entity_id: UVINT16::from(3),
                        emitter_number: Some(UVINT8::from(0)),
                        beam_number: Some(UVINT8::from(0)),
                    }],
                }],
            }],
        };

        let options = CodecOptions::new_full_update();

        let (dis_body, state_result) = cdis_body.decode(None, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        assert_eq!(dis_body.emitting_entity_id, EntityId::new(1, 1, 1));
        assert_eq!(dis_body.event_id, EventId::new(1, 1, 100));
        let emitter_out = dis_body.emitter_systems.first().unwrap();
        let emitter_in = EmitterSystem::default()
            .with_name(EmitterName::ANFPS16_5505)
            .with_number(20)
            .with_function(EmitterSystemFunction::SearchAcquisition_102)
            .with_location(VectorF32::new(1.0, 2.0, 3.0))
            .with_beam(
                Beam::default()
                    .with_number(1)
                    .with_parameter_index(1)
                    .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                    .with_track_jam(TrackJam::default().with_entity_id(EntityId::new(1, 2, 3))),
            );
        assert_eq!(emitter_in, *emitter_out);
    }

    #[test]
    fn decode_body_partial_update_full_update_flag_without_state() {
        let cdis_body = ElectromagneticEmission {
            full_update_flag: true,
            emitting_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(1),
            ),
            event_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(100),
            ),
            state_update_indicator: ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            fundamental_params: vec![FundamentalParameter::default()],
            beam_data: vec![BeamData::default()],
            site_app_pairs: vec![SiteAppPair {
                site: UVINT16::from(1),
                application: UVINT16::from(2),
            }],
            emitter_systems: vec![crate::electromagnetic_emission::model::EmitterSystem {
                name: Some(EmitterName::ANFPS16_5505),
                function: Some(EmitterSystemFunction::SearchAcquisition_102),
                number: UVINT8::from(20),
                location_with_respect_to_entity: Some(EntityCoordinateVector::new(
                    SVINT16::from(1),
                    SVINT16::from(2),
                    SVINT16::from(3),
                )),
                emitter_beams: vec![EmitterBeam {
                    beam_id: UVINT8::from(1),
                    beam_parameter_index: 1,
                    fundamental_params_index: Some(0),
                    beam_data_index: Some(0),
                    beam_function: ElectromagneticEmissionBeamFunction::Acquisition,
                    high_density_track_jam: HighDensityTrackJam::NotSelected,
                    beam_status: true,
                    jamming_technique_kind: Some(UVINT8::from(0)),
                    jamming_technique_category: Some(UVINT8::from(0)),
                    jamming_technique_subcategory: Some(UVINT8::from(0)),
                    jamming_technique_specific: Some(UVINT8::from(0)),
                    track_jam: vec![crate::electromagnetic_emission::model::TrackJam {
                        site_app_pair_index: 0,
                        entity_id: UVINT16::from(3),
                        emitter_number: Some(UVINT8::from(0)),
                        beam_number: Some(UVINT8::from(0)),
                    }],
                }],
            }],
        };

        let options = CodecOptions::new_partial_update();

        let (dis_body, state_result) = cdis_body.decode(None, &options);

        assert_eq!(
            state_result,
            CodecStateResult::StateUpdateElectromagneticEmission
        );
        assert_eq!(dis_body.emitting_entity_id, EntityId::new(1, 1, 1));
        assert_eq!(dis_body.event_id, EventId::new(1, 1, 100));
        let emitter_out = dis_body.emitter_systems.first().unwrap();
        let emitter_in = EmitterSystem::default()
            .with_name(EmitterName::ANFPS16_5505)
            .with_number(20)
            .with_function(EmitterSystemFunction::SearchAcquisition_102)
            .with_location(VectorF32::new(1.0, 2.0, 3.0))
            .with_beam(
                Beam::default()
                    .with_number(1)
                    .with_parameter_index(1)
                    .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                    .with_track_jam(TrackJam::default().with_entity_id(EntityId::new(1, 2, 3))),
            );
        assert_eq!(emitter_in, *emitter_out);
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn decode_body_partial_update_partial_update_flag_with_state() {
        let dis_body = DisEE::builder()
            .with_emitting_entity_id(EntityId::new(1, 1, 1))
            .with_event_id(EventId::new(1, 1, 100))
            .with_state_update_indicator(
                ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            )
            .with_emitter_system(
                EmitterSystem::default()
                    .with_number(20)
                    .with_name(EmitterName::ANFPS16_5505)
                    .with_function(EmitterSystemFunction::SearchAcquisition_102)
                    .with_location(VectorF32::new(1.0, 2.0, 3.0))
                    .with_beam(
                        Beam::default()
                            .with_number(1)
                            .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                            .with_high_density_track_jam(HighDensityTrackJam::NotSelected)
                            .with_parameter_index(1)
                            .with_parameter_data(FundamentalParameterData::default())
                            .with_track_jam(
                                TrackJam::default().with_entity_id(EntityId::new(1, 2, 3)),
                            ),
                    ),
            )
            .build();

        let cdis_body = ElectromagneticEmission {
            full_update_flag: false,
            emitting_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(1),
            ),
            event_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(100),
            ),
            state_update_indicator: ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            fundamental_params: vec![FundamentalParameter::default()],
            beam_data: vec![BeamData::default()],
            site_app_pairs: vec![SiteAppPair {
                site: UVINT16::from(1),
                application: UVINT16::from(2),
            }],
            emitter_systems: vec![crate::electromagnetic_emission::model::EmitterSystem {
                name: None,
                function: None,
                number: UVINT8::from(20),
                location_with_respect_to_entity: Some(EntityCoordinateVector::new(
                    SVINT16::from(1),
                    SVINT16::from(2),
                    SVINT16::from(3),
                )),
                emitter_beams: vec![EmitterBeam {
                    beam_id: UVINT8::from(1),
                    beam_parameter_index: 1,
                    fundamental_params_index: Some(0),
                    beam_data_index: Some(0),
                    beam_function: ElectromagneticEmissionBeamFunction::Acquisition,
                    high_density_track_jam: HighDensityTrackJam::NotSelected,
                    beam_status: true,
                    jamming_technique_kind: Some(UVINT8::from(0)),
                    jamming_technique_category: Some(UVINT8::from(0)),
                    jamming_technique_subcategory: Some(UVINT8::from(0)),
                    jamming_technique_specific: Some(UVINT8::from(0)),
                    track_jam: vec![crate::electromagnetic_emission::model::TrackJam {
                        site_app_pair_index: 0,
                        entity_id: UVINT16::from(3),
                        emitter_number: Some(UVINT8::from(0)),
                        beam_number: Some(UVINT8::from(0)),
                    }],
                }],
            }],
        };

        let state = DecoderStateElectromagneticEmission::new(&dis_body);
        let options = CodecOptions::new_partial_update();

        let (dis_body, state_result) = cdis_body.decode(Some(&state), &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        assert_eq!(dis_body.emitting_entity_id, EntityId::new(1, 1, 1));
        assert_eq!(dis_body.event_id, EventId::new(1, 1, 100));
        let emitter_out = dis_body.emitter_systems.first().unwrap();
        let emitter_in = EmitterSystem::default()
            .with_name(EmitterName::ANFPS16_5505)
            .with_number(20)
            .with_function(EmitterSystemFunction::SearchAcquisition_102)
            .with_location(VectorF32::new(1.0, 2.0, 3.0))
            .with_beam(
                Beam::default()
                    .with_number(1)
                    .with_parameter_index(1)
                    .with_beam_function(ElectromagneticEmissionBeamFunction::Acquisition)
                    .with_track_jam(TrackJam::default().with_entity_id(EntityId::new(1, 2, 3))),
            );
        assert_eq!(emitter_in, *emitter_out);

        assert_eq!(
            state.fundamental_params.get(&(20, 1)).unwrap(),
            &emitter_out.beams.first().unwrap().parameter_data
        );
    }

    /// Tests only whether fields are zeroed when not present in a partial update
    /// before state has been established by receiving a full update
    #[test]
    fn decode_body_partial_update_partial_update_flag_without_state() {
        let cdis_body = ElectromagneticEmission {
            full_update_flag: false,
            emitting_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(1),
            ),
            event_id: crate::records::model::EntityId::new(
                UVINT16::from(1),
                UVINT16::from(1),
                UVINT16::from(100),
            ),
            state_update_indicator: ElectromagneticEmissionStateUpdateIndicator::HeartbeatUpdate,
            fundamental_params: vec![FundamentalParameter::default()],
            beam_data: vec![BeamData::default()],
            site_app_pairs: vec![SiteAppPair {
                site: UVINT16::from(1),
                application: UVINT16::from(2),
            }],
            emitter_systems: vec![crate::electromagnetic_emission::model::EmitterSystem {
                name: None,
                function: None,
                number: UVINT8::from(20),
                location_with_respect_to_entity: Some(EntityCoordinateVector::new(
                    SVINT16::from(1),
                    SVINT16::from(2),
                    SVINT16::from(3),
                )),
                emitter_beams: vec![EmitterBeam {
                    beam_id: UVINT8::from(1),
                    beam_parameter_index: 1,
                    fundamental_params_index: Some(0),
                    beam_data_index: Some(0),
                    beam_function: ElectromagneticEmissionBeamFunction::Acquisition,
                    high_density_track_jam: HighDensityTrackJam::NotSelected,
                    beam_status: true,
                    jamming_technique_kind: Some(UVINT8::from(0)),
                    jamming_technique_category: Some(UVINT8::from(0)),
                    jamming_technique_subcategory: Some(UVINT8::from(0)),
                    jamming_technique_specific: Some(UVINT8::from(0)),
                    track_jam: vec![crate::electromagnetic_emission::model::TrackJam {
                        site_app_pair_index: 0,
                        entity_id: UVINT16::from(3),
                        emitter_number: Some(UVINT8::from(0)),
                        beam_number: Some(UVINT8::from(0)),
                    }],
                }],
            }],
        };

        let options = CodecOptions::new_partial_update();

        let (dis_body, state_result) = cdis_body.decode(None, &options);

        assert_eq!(state_result, CodecStateResult::StateUnaffected);
        assert_eq!(dis_body.emitting_entity_id, EntityId::new(1, 1, 1));
        assert_eq!(dis_body.event_id, EventId::new(1, 1, 100));
        let emitter_out = dis_body.emitter_systems.first().unwrap();

        assert_eq!(emitter_out.name, EmitterName::default());
        assert_eq!(emitter_out.function, EmitterSystemFunction::default());
        assert_eq!(emitter_out.number, 20);
    }
}
