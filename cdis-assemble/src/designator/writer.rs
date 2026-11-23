use crate::BodyProperties;
use crate::constants::{FOUR_BITS, ONE_BIT, SIXTEEN_BITS};
use crate::designator::model::{Designator, EntityDrAndLinearAcceleration};
use crate::writing::{BitBuffer, SerializeCdis, serialize_when_present, write_integer_bits};
use dis_rs::enumerations::DesignatorSystemName;

impl SerializeCdis for Designator {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();

        let cursor = write_integer_bits(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(self.units.location_wrt_entity),
        );
        let cursor = write_integer_bits(
            buf,
            cursor,
            ONE_BIT,
            u8::from(self.units.world_location_altitude),
        );
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.full_update_flag));
        let cursor = self.designating_entity_id.serialize(buf, cursor);

        let code_name = self.designator_details.map(|s| s.code_name);
        let cursor = serialize_when_present(&code_name, buf, cursor);

        let designated_entity_id = self
            .designated_entity_id_and_spot_location_wrt_entity
            .map(|s| s.designated_entity_id);
        let cursor = serialize_when_present(&designated_entity_id, buf, cursor);

        let designator_code = self.designator_details.map(|s| s.designator_code);
        let cursor = serialize_when_present(&designator_code, buf, cursor);

        let designator_power = self.designator_details.map(|s| s.designator_power);
        let cursor = serialize_when_present(&designator_power, buf, cursor);

        let designator_wavelength = self.designator_details.map(|s| s.designator_wavelength);
        let cursor = serialize_when_present(&designator_wavelength, buf, cursor);

        let designator_spot_location_wrt_designated_entity = self
            .designated_entity_id_and_spot_location_wrt_entity
            .map(|s| s.designator_spot_location_wrt_designated_entity);
        let cursor =
            serialize_when_present(&designator_spot_location_wrt_designated_entity, buf, cursor);

        let cursor = serialize_when_present(&self.designator_spot_location, buf, cursor);
        let cursor = serialize_when_present(&self.entity_dr_and_linear_acceleration, buf, cursor);

        cursor
    }
}

impl SerializeCdis for DesignatorSystemName {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, SIXTEEN_BITS, u16::from(*self));
        cursor
    }
}

impl SerializeCdis for EntityDrAndLinearAcceleration {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, u8::from(self.dr_algorithm));
        let cursor = self.dr_entity_linear_acceleration.serialize(buf, cursor);
        cursor
    }
}
