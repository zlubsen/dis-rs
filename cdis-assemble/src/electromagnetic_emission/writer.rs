use crate::constants::{EIGHT_BITS, FIVE_BITS, FOUR_BITS, ONE_BIT, SIX_BITS, SIXTEEN_BITS};
use crate::electromagnetic_emission::model::{
    ElectromagneticEmission, EmitterBeam, EmitterSystem, FundamentalParameter, SiteAppPair,
    TrackJam,
};
use crate::types::model::{CdisFloat, UVINT8};
use crate::writing::{BitBuffer, SerializeCdis, serialize_when_present, write_integer_bits};

impl SerializeCdis for ElectromagneticEmission {
    #[allow(clippy::let_and_return)]
    #[allow(clippy::cast_possible_truncation)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.full_update_flag));
        let cursor = write_integer_bits(buf, cursor, FIVE_BITS, self.fundamental_params.len());
        let cursor = write_integer_bits(buf, cursor, FIVE_BITS, self.beam_data.len());
        let cursor = write_integer_bits(buf, cursor, SIX_BITS, self.site_app_pairs.len());

        let cursor = self.emitting_id.serialize(buf, cursor);
        let cursor = self.event_id.serialize(buf, cursor);

        let cursor =
            write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.state_update_indicator));

        let cursor = UVINT8::from(self.emitter_systems.len() as u8).serialize(buf, cursor);

        let cursor = self
            .fundamental_params
            .iter()
            .fold(cursor, |cursor, param| param.serialize(buf, cursor));
        let cursor = self
            .beam_data
            .iter()
            .fold(cursor, |cursor, beam_data| beam_data.serialize(buf, cursor));
        let cursor = self
            .site_app_pairs
            .iter()
            .fold(cursor, |cursor, pair| pair.serialize(buf, cursor));

        let cursor = self
            .emitter_systems
            .iter()
            .fold(cursor, |cursor, system| system.serialize(buf, cursor));

        cursor
    }
}

impl SerializeCdis for FundamentalParameter {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.frequency.serialize(buf, cursor);
        let cursor = self.frequency_range.serialize(buf, cursor);
        let cursor = write_integer_bits(buf, cursor, EIGHT_BITS, self.erp);
        let cursor = self.prf.serialize(buf, cursor);
        let cursor = self.pulse_width.serialize(buf, cursor);

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
    #[allow(clippy::cast_possible_truncation)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        // System Field Present Flags: Emitter System Details: true when both name and function are present.
        let emitter_details_present = self.name.is_some() && self.function.is_some();
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(emitter_details_present));

        // System Field Present Flags: Location with Respect to Entity
        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(self.location_with_respect_to_entity.is_some()),
        );

        // Number of Beams
        let cursor = write_integer_bits(buf, cursor, FIVE_BITS, self.emitter_beams.len() as u8);

        let cursor = if emitter_details_present {
            // Emitter Name
            let cursor = if let Some(name) = self.name {
                write_integer_bits(buf, cursor, SIXTEEN_BITS, u16::from(name))
            } else {
                cursor
            };
            // Emitter Function
            let cursor = if let Some(function) = self.function {
                write_integer_bits(buf, cursor, EIGHT_BITS, u8::from(function))
            } else {
                cursor
            };
            cursor
        } else {
            cursor
        };

        // Emitter Number
        let cursor = self.number.serialize(buf, cursor);

        // Location with Respect to Entity
        let cursor = serialize_when_present(&self.location_with_respect_to_entity, buf, cursor);

        // Emitter Beams
        let cursor = self
            .emitter_beams
            .iter()
            .fold(cursor, |cursor, beam| beam.serialize(buf, cursor));

        cursor
    }
}

impl SerializeCdis for EmitterBeam {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        // Beam Field Present Flags: Fundamental Parameters
        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(self.fundamental_params_index.is_some()),
        );

        // Beam Field Present Flags: Beam Data Details
        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(self.beam_data_index.is_some()),
        );

        // Beam Field Present Flags: Jamming Technique Flag
        let jamming_technique_flag = self.jamming_technique_kind.is_some()
            && self.jamming_technique_category.is_some()
            && self.jamming_technique_subcategory.is_some()
            && self.jamming_technique_specific.is_some();
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(jamming_technique_flag));

        // Beam Field Present Flags: Jammer Track Flag
        let track_jam_flag = self
            .track_jam
            .iter()
            .all(|track_jam| track_jam.beam_number.is_some() && track_jam.emitter_number.is_some());
        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(track_jam_flag && !self.track_jam.is_empty()),
        );

        let cursor = self.beam_id.serialize(buf, cursor);
        let cursor = write_integer_bits(buf, cursor, SIXTEEN_BITS, self.beam_parameter_index);

        let cursor = if let Some(index) = self.fundamental_params_index {
            write_integer_bits(buf, cursor, FIVE_BITS, index)
        } else {
            cursor
        };
        let cursor = if let Some(index) = self.beam_data_index {
            write_integer_bits(buf, cursor, FIVE_BITS, index)
        } else {
            cursor
        };

        let cursor = write_integer_bits(buf, cursor, FIVE_BITS, u8::from(self.beam_function));
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, self.track_jam.len());
        let cursor =
            write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.high_density_track_jam));
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.beam_status));

        let cursor = if jamming_technique_flag {
            let cursor = serialize_when_present(&self.jamming_technique_kind, buf, cursor);
            let cursor = serialize_when_present(&self.jamming_technique_category, buf, cursor);
            let cursor = serialize_when_present(&self.jamming_technique_subcategory, buf, cursor);
            let cursor = serialize_when_present(&self.jamming_technique_specific, buf, cursor);
            cursor
        } else {
            cursor
        };

        let cursor = self
            .track_jam
            .iter()
            .fold(cursor, |cursor, track_jam| track_jam.serialize(buf, cursor));

        cursor
    }
}

impl SerializeCdis for TrackJam {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, SIX_BITS, self.site_app_pair_index);
        let cursor = self.entity_id.serialize(buf, cursor);

        // Both fields have to be present at the same time (see Tracker Jam Flag)
        let cursor = if self.beam_number.is_some() && self.emitter_number.is_some() {
            let cursor = serialize_when_present(&self.emitter_number, buf, cursor);
            let cursor = serialize_when_present(&self.beam_number, buf, cursor);
            cursor
        } else {
            cursor
        };

        cursor
    }
}
