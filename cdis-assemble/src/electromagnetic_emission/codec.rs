use std::time::Instant;
use dis_rs::model::SimulationAddress;
use crate::codec::{CodecOptions, CodecStateResult};
use crate::electromagnetic_emission::model::{ElectromagneticEmission, FundamentalParameter, SiteAppPair};
use crate::types::model::UVINT16;

/// Decoder to maintain state:
/// - Fundamental params for a given beam id
/// - Beam Data for a given beam id


type Counterpart = dis_rs::electromagnetic_emission::model::ElectromagneticEmission;

#[derive(Debug)]
pub struct EncoderStateElectromagneticEmission {
    pub last_send: Instant,
}

impl Default for EncoderStateElectromagneticEmission {
    fn default() -> Self {
        Self {
            last_send: Instant::now()
        }
    }
}

#[derive(Debug)]
pub struct DecoderStateElectromagneticEmission {
    pub last_received: Instant,
    // pub fundamental_params: HashMap<EntityId, >,
    // pub beam_data: ,
}

impl DecoderStateElectromagneticEmission {
    pub fn new() -> Self {
        Self {
            last_received: Instant::now()
        }
    }
}

impl Default for DecoderStateElectromagneticEmission {
    fn default() -> Self {
        Self {
            last_received: Instant::now()
        }
    }
}

impl ElectromagneticEmission {
    pub fn encode(item: &Counterpart, state: Option<&EncoderStateElectromagneticEmission>, options: &CodecOptions)
        -> (Self, CodecStateResult) {

        // let fundamental_param_list = ...;
        // let beam_data_list = ...;
        // let site_app_pairs_list = construct_site_app_pairs_list(item);
        //
        // let (
        //     fundamental_params,
        //     beam_data,
        //     jamming_technique_kind,
        //     jamming_technique_category,
        //     jamming_technique_subcategory,
        //     jamming_technique_specific,
        //     location,
        //     system_details_name,
        //     system_details_function,
        // ) = if options.update_mode == CodecUpdateMode::PartialUpdate
        //     && state.is_some()
        //     // TODO Refactor timeout evaluation to take the instant instead of the state object, relocate.
        //     && !evaluate_timeout_for_ee(state.unwrap(), options) {
        //     // Do not update stateful fields when a full update is not required
        //     (None, None, None, None, None, None, None, CodecStateResult::StateUnaffected)
        // } else {
        //     // full update mode, or partial with a (state) timeout on the entity
        //     (
        //         if options.update_mode == CodecUpdateMode::PartialUpdate {
        //             CodecStateResult::StateUpdateElectromagneticEmission
        //         } else { CodecStateResult::StateUnaffected }
        //     )
        // }
        (Self::default(), CodecStateResult::StateUnaffected)
    }

    pub fn decode(&self, state: Option<&DecoderStateElectromagneticEmission>, options: &CodecOptions)
        -> (Counterpart, CodecStateResult) {
        (Counterpart::default(), CodecStateResult::StateUnaffected)
    }
}

fn evaluate_timeout_for_ee(state: &EncoderStateElectromagneticEmission, options: &CodecOptions) -> bool {
    let elapsed = state.last_send.elapsed().as_secs_f32();
    elapsed > (options.federation_parameters.HBT_PDU_EE * options.hbt_cdis_full_update_mplier)
}

fn construct_fundamental_params_list(item: &Counterpart) -> Vec<FundamentalParameter> {
    // let mut list = item.emitter_systems.iter()
    //     .map(|emitter| emitter.beams.iter()
    //         .map(|beam| beam.parameter_data))
    todo!()
}

fn construct_site_app_pairs_list(item: &Counterpart) -> Vec<SiteAppPair> {
    let mut pairs: Vec<SimulationAddress> = item.emitter_systems.iter()
        .map(|emitter| emitter.beams.iter()
            .map(|beam| beam.track_jam_data.iter()
                .map(|tj| tj.entity_id.simulation_address )
                .collect::<Vec<SimulationAddress>>())
            .flatten().collect::<Vec<SimulationAddress>>())
        .flatten().collect::<Vec<SimulationAddress>>();

    pairs.sort();
    pairs.dedup();

    pairs.iter()
        .map(|sa| SiteAppPair {
            site: UVINT16::from(sa.site_id),
            application: UVINT16::from(sa.application_id) } )
        .collect()
}

#[cfg(test)]
mod tests {
    use dis_rs::electromagnetic_emission::model::{Beam, ElectromagneticEmission, EmitterSystem, TrackJam};
    use dis_rs::model::{EntityId};
    use crate::electromagnetic_emission::codec::construct_site_app_pairs_list;

    #[test]
    fn site_app_pairs_list() {
        let track_1 = TrackJam::default().with_entity_id(EntityId::new(1, 1, 1));
        let track_2 = TrackJam::default().with_entity_id(EntityId::new(1, 1, 2));
        let track_3 = TrackJam::default().with_entity_id(EntityId::new(2, 2, 2));
        let mut list_1 = vec![track_1, track_2];

        let body = ElectromagneticEmission::builder()
            .with_emitter_system(EmitterSystem::default()
                .with_beam(Beam::default()
                    .with_track_jams(&mut list_1))
                .with_beam(Beam::default()
                    .with_track_jam(track_3)))
            .build();

        let pairs = construct_site_app_pairs_list(&body);

        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs.get(0).unwrap().site.value, 1);
        assert_eq!(pairs.get(0).unwrap().application.value, 1);
        assert_eq!(pairs.get(1).unwrap().site.value, 2);
        assert_eq!(pairs.get(1).unwrap().application.value, 2);
    }
}