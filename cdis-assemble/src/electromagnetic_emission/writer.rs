use crate::electromagnetic_emission::model::{BeamData, ElectromagneticEmission, EmitterBeam, EmitterSystem, FundamentalParameter, SiteAppPair, TrackJam};
use crate::{BitBuffer, SerializeCdisPdu};
use crate::constants::{EIGHT_BITS, FIVE_BITS, FOUR_BITS, ONE_BIT, SIX_BITS, SIXTEEN_BITS, TEN_BITS};
use crate::types::model::UVINT8;
use crate::types::writer::{serialize_cdis_float_signed, serialize_cdis_float_unsigned};
use crate::writing::{SerializeCdis, write_value_unsigned};

impl SerializeCdisPdu for ElectromagneticEmission {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.full_update_flag.into());
        let cursor = write_value_unsigned(buf, cursor, FIVE_BITS, self.fundamental_params.len());
        let cursor = write_value_unsigned(buf, cursor, FIVE_BITS, self.beam_data.len());
        let cursor = write_value_unsigned(buf, cursor, FIVE_BITS, self.site_app_pairs.len());

        let cursor = self.emitting_id.serialize(buf, cursor);
        let cursor = self.event_id.serialize(buf, cursor);

        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.state_update_indicator.into());

        let cursor = UVINT8::from(self.emitter_systems.len() as u8).serialize(buf, cursor);

        let cursor = self.fundamental_params.iter()
            .fold(cursor, |cursor, param| param.serialize(buf, cursor));
        let cursor = self.beam_data.iter()
            .fold(cursor, |cursor, beam_data| beam_data.serialize(buf, cursor));
        let cursor = self.site_app_pairs.iter()
            .fold(cursor, |cursor, pair| pair.serialize(buf, cursor));

        let cursor = self.emitter_systems.iter()
            .fold(cursor, |cursor, system| system.serialize(buf, cursor));

        cursor
    }
}

impl SerializeCdis for FundamentalParameter {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = serialize_cdis_float_unsigned(buf, cursor, &self.frequency);
        let cursor = serialize_cdis_float_unsigned(buf, cursor, &self.frequency_range);
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.erp);
        let cursor = self.prf.serialize(buf, cursor);
        let cursor = serialize_cdis_float_signed(buf, cursor, &self.pulse_width);

        cursor
    }
}

impl SerializeCdis for BeamData {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.az_center.serialize(buf, cursor);
        let cursor = self.az_center.serialize(buf, cursor);
        let cursor = self.az_center.serialize(buf, cursor);
        let cursor = self.az_center.serialize(buf, cursor);
        let cursor = write_value_unsigned(buf, cursor, TEN_BITS, self.sweep_sync);

        cursor
    }
}

impl SerializeCdis for SiteAppPair {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.site.serialize(buf, cursor);
        let cursor = self.application.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EmitterSystem {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        // TODO only checks on `name`, not taking into account the presence of `function`.
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, self.name.is_some() as u8);
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, self.location_with_respect_to_entity.is_some() as u8);

        let cursor = write_value_unsigned(buf, cursor, FIVE_BITS, self.emitter_beams.len() as u8);

        let cursor = if let Some(name) = self.name {
            write_value_unsigned::<u16>(buf, cursor, SIXTEEN_BITS, name.into())
        } else { cursor };
        let cursor = if let Some(function) = self.function {
            write_value_unsigned::<u8>(buf, cursor, SIXTEEN_BITS, function.into())
        } else { cursor };
        let cursor = if let Some(location) = self.location_with_respect_to_entity {
            location.serialize(buf, cursor)
        } else { cursor };

        let cursor = self.emitter_beams.iter()
            .fold(cursor, |cursor, beam| beam.serialize(buf, cursor) );

        cursor
    }
}

impl SerializeCdis for EmitterBeam {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, self.fundamental_params_index.is_some() as u8);
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, self.beam_data_index.is_some() as u8);
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, self.jamming_technique_kind.is_some() as u8); // TODO only considers jamming_technique_kind
        let track_jam_flag = if let Some(track_jam) = self.track_jam.first() {
            track_jam.beam_number.is_some() & track_jam.emitter_number.is_some()
        } else { false };
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, track_jam_flag as u8);

        let cursor = self.beam_id.serialize(buf, cursor);
        let cursor = write_value_unsigned(buf, cursor, SIXTEEN_BITS, self.beam_parameter_index);

        let cursor = if let Some(index) = self.fundamental_params_index {
            write_value_unsigned(buf, cursor, FIVE_BITS, index)
        } else { cursor };
        let cursor = if let Some(index) = self.beam_data_index {
            write_value_unsigned(buf, cursor, FIVE_BITS, index)
        } else { cursor };
        let cursor = write_value_unsigned::<u8>(buf, cursor, FIVE_BITS, self.beam_function.into());

        let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, self.track_jam.len());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.high_density_track_jam.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.beam_status as u8);

        let cursor = if let Some(kind) = self.jamming_technique_kind {
            kind.serialize(buf, cursor)
        } else { cursor };
        let cursor = if let Some(category) = self.jamming_technique_category {
            category.serialize(buf, cursor)
        } else { cursor };
        let cursor = if let Some(subcategory) = self.jamming_technique_subcategory {
            subcategory.serialize(buf, cursor)
        } else { cursor };
        let cursor = if let Some(specific) = self.jamming_technique_specific {
            specific.serialize(buf, cursor)
        } else { cursor };

        let cursor = self.track_jam.iter()
            .fold(cursor, |cursor, track_jam | track_jam.serialize(buf, cursor) );

        cursor
    }
}

impl SerializeCdis for TrackJam {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, SIX_BITS, self.site_app_pair_index);
        let cursor = self.entity_id.serialize(buf, cursor);

        let cursor = if let Some(number) = self.emitter_number {
            number.serialize(buf, cursor)
        } else { cursor };
        let cursor = if let Some(number) = self.beam_number {
            number.serialize(buf, cursor)
        } else { cursor };

        cursor
    }
}