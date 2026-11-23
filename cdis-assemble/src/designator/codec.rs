use crate::codec::Codec;
use crate::codec::{CodecOptions, CodecStateResult, CodecUpdateMode, DecoderState, EncoderState};
use crate::designator::model::{
    DesignatedEntityIdAndSpotLocationWrtEntity, Designator, DesignatorDetails, DesignatorUnits,
    EntityDrAndLinearAcceleration,
};
use crate::entity_state::codec::encode_dr_linear_acceleration;
use crate::records::codec::{
    decode_entity_coordinate_vector, decode_world_coordinates, encode_entity_coordinate_vector,
    encode_world_coordinates,
};
use crate::records::model::{EntityId, UnitsDekameters, UnitsMeters};
use crate::types::model::{UVINT16, UVINT32};
use crate::{BodyProperties, CdisBody};
use dis_rs::BodyRaw;
use dis_rs::enumerations::{DeadReckoningAlgorithm, DesignatorCode, DesignatorSystemName};
use dis_rs::model::{EntityId as DisEntityId, Location, PduBody, VectorF32};
use std::time::Instant;

type Counterpart = dis_rs::designator::model::Designator;

pub(crate) fn encode_designator_body_and_update_state(
    dis_body: &Counterpart,
    state: &mut EncoderState,
    options: &CodecOptions,
) -> (CdisBody, CodecStateResult) {
    let state_for_id = state.designator.get(&dis_body.designating_entity_id);

    let (cdis_body, state_result) = Designator::encode(dis_body, state_for_id, options);

    if state_result == CodecStateResult::StateUpdateDesignator {
        state
            .designator
            .entry(dis_body.designating_entity_id)
            .and_modify(|designator| designator.heartbeat = Instant::now())
            .or_default();
    }

    (cdis_body.into_cdis_body(), state_result)
}

pub(crate) fn decode_designator_body_and_update_state(
    cdis_body: &Designator,
    state: &mut DecoderState,
    options: &CodecOptions,
) -> (PduBody, CodecStateResult) {
    let state_for_id = state
        .designator
        .get(&DisEntityId::from(&cdis_body.designating_entity_id));
    let (dis_body, state_result) = cdis_body.decode(state_for_id, options);

    if state_result == CodecStateResult::StateUpdateDesignator {
        state
            .designator
            .entry(DisEntityId::from(&cdis_body.designating_entity_id))
            .and_modify(|designator| {
                designator.heartbeat = Instant::now();
                designator.code_name = dis_body.system_name;
                designator.designator_code = dis_body.code;
                designator.designator_power = dis_body.power;
                designator.designator_wavelength = dis_body.wavelength;
                designator.designator_spot_location = dis_body.spot_location;
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
            heartbeat: Instant::now(),
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateDesignator {
    pub heartbeat: Instant,
    pub code_name: DesignatorSystemName,
    pub designator_code: DesignatorCode,
    pub designator_power: f32,
    pub designator_wavelength: f32,
    pub designator_spot_location: Location,
}

impl DecoderStateDesignator {
    #[must_use]
    pub fn new(body: &Counterpart) -> Self {
        Self {
            heartbeat: Instant::now(),
            code_name: body.system_name,
            designator_code: body.code,
            designator_power: body.power,
            designator_wavelength: body.wavelength,
            designator_spot_location: body.spot_location,
        }
    }
}

impl Default for DecoderStateDesignator {
    fn default() -> Self {
        Self {
            heartbeat: Instant::now(),
            code_name: DesignatorSystemName::default(),
            designator_code: DesignatorCode::default(),
            designator_power: 0.0,
            designator_wavelength: 0.0,
            designator_spot_location: Location::default(),
        }
    }
}

impl Designator {
    #[must_use]
    fn encode(
        item: &Counterpart,
        state: Option<&EncoderStateDesignator>,
        options: &CodecOptions,
    ) -> (Self, CodecStateResult) {
        let designating_entity_id = EntityId::encode(&item.designating_entity_id);

        // When no designated entity is provided, the spot location wrt entity reference frame
        // should also be discarded
        let (designated_entity_id_and_spot_location_wrt_entity, location_wrt_entity_units) =
            if item.designated_entity_id != DisEntityId::default() {
                let designated_entity_id = EntityId::encode(&item.designated_entity_id);
                let (designator_spot_location_wrt_designated_entity, location_wrt_entity_units) =
                    encode_entity_coordinate_vector(&item.spot_wrt_designated_entity);
                (
                    DesignatedEntityIdAndSpotLocationWrtEntity::new(
                        Some(designated_entity_id),
                        Some(designator_spot_location_wrt_designated_entity),
                    ),
                    location_wrt_entity_units,
                )
            } else {
                (None, UnitsMeters::default())
            };

        // When no DR algorithm is specified, the acceleration is not usable
        let entity_dr_and_linear_acceleration =
            if item.dead_reckoning_algorithm != DeadReckoningAlgorithm::Other {
                let dr_entity_linear_acceleration = encode_dr_linear_acceleration(
                    item.dead_reckoning_algorithm,
                    &item.linear_acceleration,
                );
                EntityDrAndLinearAcceleration::new(
                    Some(item.dead_reckoning_algorithm),
                    dr_entity_linear_acceleration,
                )
            } else {
                None
            };

        let (
            full_update_flag,
            designator_details,
            designator_spot_location,
            world_location_altitude_units,
            state_result,
        ) = if options.update_mode == CodecUpdateMode::PartialUpdate
            && state
                .is_some_and(|state| !evaluate_timeout_for_designator(&state.heartbeat, options))
        {
            // Do not update stateful fields when a full update is not required
            (
                false,
                None,
                None,
                UnitsDekameters::default(),
                CodecStateResult::StateUnaffected,
            )
        } else {
            // Full update mode, or heartbeat timeout has expired in partial update mode
            let code_name = Some(item.system_name);
            let designator_code = Some(UVINT16::from(u16::from(item.code)));
            let designator_power = Some(UVINT32::from(item.power.round() as u32));
            let designator_wavelength =
                Some(UVINT32::from((item.wavelength * 10f32).round() as u32)); // microns (DIS) -> nanometers (C-DIS)

            let designator_details = DesignatorDetails::new(
                code_name,
                designator_code,
                designator_power,
                designator_wavelength,
            );

            let (spot_location, world_location_altitude_units) =
                encode_world_coordinates(&item.spot_location);

            (
                true,
                designator_details,
                Some(spot_location),
                world_location_altitude_units,
                CodecStateResult::StateUpdateDesignator,
            )
        };

        (
            Self {
                units: DesignatorUnits {
                    location_wrt_entity: location_wrt_entity_units,
                    world_location_altitude: world_location_altitude_units,
                },
                full_update_flag,
                designating_entity_id,
                designated_entity_id_and_spot_location_wrt_entity,
                designator_details,
                designator_spot_location,
                entity_dr_and_linear_acceleration,
            },
            state_result,
        )
    }

    #[must_use]
    fn decode(
        &self,
        state: Option<&DecoderStateDesignator>,
        _options: &CodecOptions,
    ) -> (Counterpart, CodecStateResult) {
        let state_result = if self.full_update_flag {
            CodecStateResult::StateUpdateDesignator
        } else {
            CodecStateResult::StateUnaffected
        };

        // If an optional field is missing and the PDU is a partial update, then attempt to retrieve
        // the field from the decoder state
        let fallback = if self.full_update_flag { None } else { state };

        let designating_entity_id = self.designating_entity_id.decode();

        let (designated_entity_id, designator_spot_location_wrt_designated_entity) =
            if let Some(designated_entity_id_and_spot_location_wrt_entity) =
                self.designated_entity_id_and_spot_location_wrt_entity
            {
                let designated_entity_id = designated_entity_id_and_spot_location_wrt_entity
                    .designated_entity_id
                    .decode();
                let designator_spot_location_wrt_designated_entity =
                    decode_entity_coordinate_vector(
                        &designated_entity_id_and_spot_location_wrt_entity
                            .designator_spot_location_wrt_designated_entity,
                        self.units.location_wrt_entity,
                    );

                (
                    designated_entity_id,
                    designator_spot_location_wrt_designated_entity,
                )
            } else {
                (DisEntityId::default(), VectorF32::default())
            };

        let (code_name, designator_code, designator_power, designator_wavelength) =
            if let Some(designator_details) = self.designator_details {
                (
                    designator_details.code_name,
                    DesignatorCode::from(designator_details.designator_code.value),
                    designator_details.designator_power.value as f32,
                    designator_details.designator_wavelength.value as f32 / 10f32, // nanometers (C-DIS) -> microns (DIS)
                )
            } else if let Some(state) = fallback {
                (
                    state.code_name,
                    state.designator_code,
                    state.designator_power,
                    state.designator_wavelength,
                )
            } else {
                (
                    DesignatorSystemName::default(),
                    DesignatorCode::default(),
                    f32::default(),
                    f32::default(),
                )
            };

        let designator_spot_location = self.designator_spot_location.map_or_else(
            || {
                fallback
                    .map(|state| state.designator_spot_location)
                    .unwrap_or_default()
            },
            |spot_location| {
                decode_world_coordinates(&spot_location, self.units.world_location_altitude)
            },
        );

        let (dr_algorithm, dr_entity_linear_acceleration) =
            if let Some(entity_dr_and_linear_acceleration) = self.entity_dr_and_linear_acceleration
            {
                (
                    entity_dr_and_linear_acceleration.dr_algorithm,
                    entity_dr_and_linear_acceleration
                        .dr_entity_linear_acceleration
                        .decode(),
                )
            } else {
                (DeadReckoningAlgorithm::default(), VectorF32::default())
            };

        (
            Counterpart::builder()
                .with_designating_entity_id(designating_entity_id)
                .with_system_name(code_name)
                .with_designated_entity_id(designated_entity_id)
                .with_code(designator_code)
                .with_power(designator_power)
                .with_wavelength(designator_wavelength)
                .with_spot_wrt_designated_entity(designator_spot_location_wrt_designated_entity)
                .with_spot_location(designator_spot_location)
                .with_dead_reckoning_algorithm(dr_algorithm)
                .with_linear_acceleration(dr_entity_linear_acceleration)
                .build(),
            state_result,
        )
    }
}

fn evaluate_timeout_for_designator(last_heartbeat: &Instant, options: &CodecOptions) -> bool {
    let elapsed = last_heartbeat.elapsed().as_secs_f32();
    elapsed
        > (options.federation_parameters.HBT_PDU_DESIGNATOR * options.hbt_cdis_full_update_mplier)
}
