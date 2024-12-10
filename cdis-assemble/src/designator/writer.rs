use crate::constants::{FOUR_BITS, ONE_BIT, SIXTEEN_BITS};
use crate::designator::model::Designator;
use crate::writing::{serialize_when_present, write_value_unsigned, SerializeCdis};
use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
use dis_rs::enumerations::DesignatorSystemName;

impl SerializeCdisPdu for Designator {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();

        let cursor =
            write_value_unsigned(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            ONE_BIT,
            self.units.location_wrt_entity_units.into(),
        );
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            ONE_BIT,
            self.units.world_location_altitude.into(),
        );
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.full_update_flag.into());
        let cursor = self.designating_entity_id.serialize(buf, cursor);

        let cursor = serialize_when_present(&self.code_name, buf, cursor);
        let cursor = serialize_when_present(&self.designated_entity_id, buf, cursor);
        let cursor = serialize_when_present(&self.designator_code, buf, cursor);
        let cursor = serialize_when_present(&self.designator_power, buf, cursor);
        let cursor = serialize_when_present(&self.designator_wavelength, buf, cursor);
        let cursor = serialize_when_present(&self.spot_wrt_designated_entity, buf, cursor);
        let cursor = serialize_when_present(&self.designator_spot_location, buf, cursor);

        let cursor = if let Some(algo) = self.dr_algorithm {
            write_value_unsigned::<u8>(buf, cursor, FOUR_BITS, algo.into())
        } else {
            cursor
        };
        let cursor = serialize_when_present(&self.dr_entity_linear_acceleration, buf, cursor);

        cursor
    }
}

impl SerializeCdis for DesignatorSystemName {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u16>(buf, cursor, SIXTEEN_BITS, (*self).into());

        cursor
    }
}
