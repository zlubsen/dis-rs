use crate::electromagnetic_emission::model::{BeamData, ElectromagneticEmission, EmitterBeam, EmitterSystem, FundamentalParameter, SiteAppPair, TrackJam};
use crate::{BitBuffer, SerializeCdisPdu};
use crate::constants::{EIGHT_BITS, FIVE_BITS, ONE_BIT, TEN_BITS};
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
        todo!()
    }
}

impl SerializeCdis for EmitterBeam {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        todo!()
    }
}

impl SerializeCdis for TrackJam {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        todo!()
    }
}