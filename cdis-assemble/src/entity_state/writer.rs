use crate::entity_state::model::{CdisEntityAppearance, CdisEntityCapabilities, EntityState};
use crate::BodyProperties;
use crate::constants::{FOUR_BITS, HUNDRED_TWENTY_BITS, ONE_BIT, THIRTY_TWO_BITS};
use crate::types::model::UVINT8;
use crate::writing::{BitBuffer, serialize_when_present, SerializeCdis, SerializeCdisPdu, write_value_unsigned};

impl SerializeCdisPdu for EntityState {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let fields_present = self.fields_present_field();

        let cursor = write_value_unsigned(buf, cursor, self.fields_present_length(), fields_present);
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.units.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.full_update_flag.into());
        let cursor = self.entity_id.serialize(buf, cursor);
        let cursor = serialize_when_present(&self.force_id, buf, cursor);
        let cursor = if self.full_update_flag | !self.variable_parameters.is_empty() {
            UVINT8::from(self.variable_parameters.len() as u8 ).serialize(buf, cursor)
        } else { cursor };
        let cursor = serialize_when_present(&self.entity_type, buf, cursor);
        let cursor = serialize_when_present(&self.alternate_entity_type, buf, cursor);
        let cursor = serialize_when_present(&self.entity_linear_velocity, buf, cursor);
        let cursor = serialize_when_present(&self.entity_location, buf, cursor);
        let cursor = serialize_when_present(&self.entity_orientation, buf, cursor);
        let cursor = serialize_when_present(&self.entity_appearance, buf, cursor);

        let cursor = write_value_unsigned::<u8>(buf, cursor, FOUR_BITS, self.dr_algorithm.into());
        let cursor = if let Some(other) = &self.dr_params_other {
            write_value_unsigned(buf, cursor, HUNDRED_TWENTY_BITS, other.0)
        } else { cursor };
        let cursor = serialize_when_present(&self.dr_params_entity_linear_acceleration, buf, cursor);
        let cursor = serialize_when_present(&self.dr_params_entity_angular_velocity, buf, cursor);

        let cursor = serialize_when_present(&self.entity_marking, buf, cursor);
        let cursor = serialize_when_present(&self.capabilities, buf, cursor);

        let cursor = self.variable_parameters.iter()
            .fold(cursor, |cursor, vp| vp.serialize(buf, cursor) );

        cursor
    }
}

impl SerializeCdis for CdisEntityAppearance {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, self.0)
    }
}

impl SerializeCdis for CdisEntityCapabilities {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        self.0.serialize(buf, cursor)
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::BitArray;
    use dis_rs::enumerations::{Country, DeadReckoningAlgorithm, EntityKind, ForceId, PlatformDomain};
    use crate::{BitBuffer, BodyProperties, SerializeCdisPdu};
    use crate::entity_state::model::{CdisEntityCapabilities, EntityState};
    use crate::records::model::{CdisEntityMarking, EntityId, LinearVelocity, Orientation, Units, WorldCoordinates};
    use crate::types::model::{SVINT16, SVINT24, UVINT16, UVINT32, UVINT8};

    #[test]
    fn serialize_entity_state_no_fields_present() {
        let cdis_body = EntityState {
            units: Units::Dekameter,
            full_update_flag: false,
            entity_id: EntityId::new(UVINT16::from(10), UVINT16::from(10), UVINT16::from(10)),
            force_id: Some(UVINT8::from(u8::from(ForceId::Friendly))),
            entity_type: Some(crate::records::model::EntityType::new(u8::from(EntityKind::Platform), u8::from(PlatformDomain::Air), u16::from(Country::from(1)), UVINT8::from(1), UVINT8::from(1), UVINT8::from(1), UVINT8::from(1))),
            alternate_entity_type: None,
            entity_linear_velocity: Some(LinearVelocity::new(SVINT16::from(5), SVINT16::from(5),SVINT16::from(-5))),
            entity_location: Some(WorldCoordinates::new(52.0, 5.0, SVINT24::from(1000))),
            entity_orientation: Some(Orientation::new(4, 3, 2)),
            entity_appearance: None,
            dr_algorithm: DeadReckoningAlgorithm::DRM_FPW_ConstantVelocityLowAccelerationLinearMotionEntity,
            dr_params_other: None,
            dr_params_entity_linear_acceleration: None,
            dr_params_entity_angular_velocity: None,
            entity_marking: Some(CdisEntityMarking::new("TEST".to_string())),
            capabilities: Some(CdisEntityCapabilities(UVINT32::from(0xABC00000))),
            variable_parameters: vec![],
        }.into_cdis_body();

        let mut buf: BitBuffer = BitArray::ZERO;
        let cursor = cdis_body.serialize(&mut buf, 0);

        assert_eq!(cursor, cdis_body.body_length());
        assert_eq!(buf.data[..5], [0b1010_1110, 0b0001_1100, 0b0000_0101, 0b0000_0001, 0b0100_0000]);
    }
}