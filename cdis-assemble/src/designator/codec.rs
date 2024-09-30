use std::time::Instant;
use dis_rs::enumerations::{DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName};
use dis_rs::model::{EntityId as DisEntityId, Location, PduBody, VectorF32};
use crate::{BodyProperties, CdisBody};
use crate::codec::{CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState, EncoderState};
use crate::designator::model::{Designator, DesignatorUnits};
use crate::records::model::{EntityId, LinearAcceleration};
use crate::codec::Codec;
use crate::records::codec::{decode_entity_coordinate_vector, decode_world_coordinates, encode_entity_coordinate_vector, encode_world_coordinates};
use crate::types::model::{UVINT16, UVINT32};

use num_traits::FromPrimitive;

type Counterpart = dis_rs::designator::model::Designator;

pub(crate) fn encode_designator_body_and_update_state(dis_body: &Counterpart,
                                                        state: &mut EncoderState,
                                                        options: &CodecOptions) -> (CdisBody, CodecStateResult) {
    let state_for_id = state.designator.get(&dis_body.designating_entity_id);

    let (cdis_body, state_result) = Designator::encode(dis_body, state_for_id, options);

    if state_result == CodecStateResult::StateUpdateDesignator {
        state.designator.entry(dis_body.designating_entity_id)
            .and_modify(|designator| { designator.heartbeat = Instant::now() })
            .or_default();
    }

    (cdis_body.into_cdis_body(), state_result)
}

pub(crate) fn decode_designator_body_and_update_state(cdis_body: &Designator,
                                                        state: &mut DecoderState,
                                                        options: &CodecOptions) -> (PduBody, CodecStateResult) {
    let state_for_id = state.designator.get(&DisEntityId::from(&cdis_body.designating_entity_id));
    let (dis_body, state_result) = cdis_body.decode(state_for_id, options);

    if state_result == CodecStateResult::StateUpdateDesignator {
        state.designator.entry(DisEntityId::from(&cdis_body.designating_entity_id))
            .and_modify(|designator| {
                designator.heartbeat = Instant::now();
                designator.code_name = dis_body.system_name;
                designator.designated_entity_id = dis_body.designated_entity_id;
                designator.designator_code = dis_body.code;
                designator.designator_power = dis_body.power;
                designator.designator_wavelength = dis_body.wavelength;
                designator.spot_wrt_designated_entity = dis_body.spot_wrt_designated_entity;
                designator.designator_spot_location = dis_body.spot_location;
                designator.dr_algorithm = dis_body.dead_reckoning_algorithm;
                designator.dr_entity_linear_acceleration = dis_body.linear_acceleration;
            })
            .or_insert(DecoderStateDesignator::new(&dis_body));
    }

    (dis_body.into_pdu_body(), state_result)
}

#[derive(Debug)]
pub struct EncoderStateDesignator {
    pub heartbeat: Instant,
}

impl Default for EncoderStateDesignator {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now()
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateDesignator {
    pub heartbeat: Instant,
    pub designated_entity_id: DisEntityId,      // FIXME not part of the state according to section 13.19 (GREEN)
    pub code_name: DesignatorSystemName,
    pub designator_code: DesignatorCode,
    pub designator_power: f32,
    pub designator_wavelength: f32,
    pub spot_wrt_designated_entity: VectorF32,  // FIXME not part of the state according to section 13.19 (GREEN)
    pub designator_spot_location: Location,
    pub dr_algorithm: DeadReckoningAlgorithm,       // FIXME not part of the state according to section 13.19 (GREEN) - check when to include/leave out
    pub dr_entity_linear_acceleration: VectorF32,   // FIXME not part of the state according to section 13.19 (GREEN) - check when to include/leave out
}

impl DecoderStateDesignator {
    pub fn new(body: &Counterpart) -> Self {
        Self {
            heartbeat: Instant::now(),
            designated_entity_id: body.designated_entity_id,
            code_name: body.system_name,
            designator_code: body.code,
            designator_power: body.power,
            designator_wavelength: body.wavelength,
            spot_wrt_designated_entity: body.spot_wrt_designated_entity,
            designator_spot_location: body.spot_location,
            dr_algorithm: body.dead_reckoning_algorithm,
            dr_entity_linear_acceleration: body.linear_acceleration,
        }
    }
}

impl Default for DecoderStateDesignator {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
            designated_entity_id: Default::default(),
            code_name: Default::default(),
            designator_code: Default::default(),
            designator_power: 0.0,
            designator_wavelength: 0.0,
            spot_wrt_designated_entity: Default::default(),
            designator_spot_location: Default::default(),
            dr_algorithm: Default::default(),
            dr_entity_linear_acceleration: Default::default(),
        }
    }
}

impl Designator {
    fn encode(item: &Counterpart, state: Option<&EncoderStateDesignator>, options: &CodecOptions) -> (Self, CodecStateResult) {
        let (
            full_update_flag,
            units,
            designated_entity_id,
            code_name,
            designator_code,
            designator_power,
            designator_wavelength,
            spot_wrt_designated_entity,
            designator_spot_location,
            dr_algorithm,
            dr_entity_linear_acceleration,
            state_result,
        ) = if options.update_mode == CodecUpdateMode::PartialUpdate
            && state.is_some_and(|state|
            !evaluate_timeout_for_designator(&state.heartbeat, options) ) {
            // Do not update stateful fields when a full update is not required
            (false, DesignatorUnits::default(), None, None, None, None, None, None, None, None, None, CodecStateResult::StateUnaffected)
        } else {
            let (spot_wrt_designated_entity, location_wrt_entity_units) = encode_entity_coordinate_vector(&item.spot_wrt_designated_entity);
            let (spot_location, world_location_altitude) = encode_world_coordinates(&item.spot_location);
            let code: u16 = item.code.into();
            let power= u32::from_f32(item.power).unwrap_or_default();
            let wavelength = u32::from_f32(item.wavelength).unwrap_or_default();

            (
                true,
                DesignatorUnits {
                    location_wrt_entity_units,
                    world_location_altitude,
                },
                Some(EntityId::encode(&item.designated_entity_id)),
                Some(item.system_name),
                Some(UVINT16::from(code)),
                Some(UVINT32::from(power)),
                Some(UVINT32::from(wavelength)),
                Some(spot_wrt_designated_entity),
                Some(spot_location),
                Some(item.dead_reckoning_algorithm),
                Some(LinearAcceleration::encode(&item.linear_acceleration)),
                CodecStateResult::StateUpdateDesignator,
            )
        };

        (Self {
            units,
            full_update_flag,
            designating_entity_id: EntityId::encode(&item.designating_entity_id),
            code_name,
            designated_entity_id,
            designator_code,
            designator_power,
            designator_wavelength,
            spot_wrt_designated_entity,
            designator_spot_location,
            dr_algorithm,
            dr_entity_linear_acceleration,
        }, state_result)
    }

    fn decode(&self, state: Option<&DecoderStateDesignator>, _options: &CodecOptions) -> (Counterpart, CodecStateResult) {
        let (
            designated_entity_id,
            code_name,
            designator_code,
            designator_power,
            designator_wavelength,
            spot_wrt_designated_entity,
            designator_spot_location,
            dr_algorithm,
            dr_entity_linear_acceleration,
            state_result,
        ) = if self.full_update_flag {
            (
                self.designated_entity_id.unwrap_or_default().decode(),
                self.code_name.unwrap_or_default(),
                DesignatorCode::from(self.designator_code.unwrap_or_default().value),
                f32::from_u32(self.designator_power.unwrap_or_default().value).unwrap_or_default(),
                f32::from_u32(self.designator_wavelength.unwrap_or_default().value).unwrap_or_default(),
                decode_entity_coordinate_vector(&self.spot_wrt_designated_entity.unwrap_or_default(), self.units.location_wrt_entity_units),
                decode_world_coordinates(&self.designator_spot_location.unwrap_or_default(), self.units.world_location_altitude),
                self.dr_algorithm.unwrap_or_default(),
                self.dr_entity_linear_acceleration.unwrap_or_default().decode(),
                CodecStateResult::StateUpdateDesignator,
            )
        } else {
            (
                self.designated_entity_id.map(|record| record.decode() )
                    .unwrap_or_else(|| if let Some(state) = state { state.designated_entity_id } else { Default::default() } ),
                self.code_name
                    .unwrap_or_else(|| if let Some(state) = state { state.code_name } else { Default::default() } ),
                self.designator_code.map(|record| DesignatorCode::from(record.value) )
                    .unwrap_or_else(|| if let Some(state) = state { state.designator_code } else { Default::default() }),
                self.designator_power.map(|record| f32::from_u32(record.value).unwrap_or_default() )
                    .unwrap_or_else(|| if let Some(state) = state { state.designator_power } else { Default::default() }),
                self.designator_wavelength.map(|record| f32::from_u32(record.value).unwrap_or_default() )
                    .unwrap_or_else(|| if let Some(state) = state { state.designator_wavelength } else { Default::default() }),
                self.spot_wrt_designated_entity.map(|record| decode_entity_coordinate_vector(&record, self.units.location_wrt_entity_units) )
                    .unwrap_or_else(|| if let Some(state) = state { state.spot_wrt_designated_entity } else { Default::default() }),
                self.designator_spot_location.map(|record| decode_world_coordinates(&record, self.units.world_location_altitude) )
                    .unwrap_or_else(|| if let Some(state) = state { state.designator_spot_location } else { Default::default() }),
                self.dr_algorithm.unwrap_or_else(|| if let Some(state) = state { state.dr_algorithm } else { Default::default() }),
                self.dr_entity_linear_acceleration.map(|record| record.decode() )
                    .unwrap_or_else(|| if let Some(state) = state { state.dr_entity_linear_acceleration } else { Default::default() }),
                CodecStateResult::StateUnaffected,
            )
        };

        (Counterpart::builder()
             .with_designating_entity_id(self.designating_entity_id.decode())
             .with_system_name(code_name)
             .with_designated_entity_id(designated_entity_id)
             .with_code(designator_code)
             .with_power(designator_power)
             .with_wavelength(designator_wavelength)
             .with_spot_wrt_designated_entity(spot_wrt_designated_entity)
             .with_spot_location(designator_spot_location)
             .with_dead_reckoning_algorithm(dr_algorithm)
             .with_linear_acceleration(dr_entity_linear_acceleration)
             .build(), state_result)
    }
}

fn evaluate_timeout_for_designator(last_heartbeat: &Instant, options: &CodecOptions) -> bool {
    let elapsed = last_heartbeat.elapsed().as_secs_f32();
    elapsed > (options.federation_parameters.HBT_PDU_DESIGNATOR * options.hbt_cdis_full_update_mplier)
}