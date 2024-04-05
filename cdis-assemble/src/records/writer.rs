use dis_rs::enumerations::VariableParameterRecordType;
use crate::records::model::{AngularVelocity, CdisArticulatedPartVP, CdisAttachedPartVP, CdisEntityAssociationVP, CdisEntityMarking, CdisEntitySeparationVP, CdisEntityTypeVP, CdisHeader, CdisVariableParameter, EntityCoordinateVector, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, ParameterValueFloat, WorldCoordinates};
use crate::writing::{SerializeCdis, write_value_signed, write_value_unsigned};
use crate::constants::{EIGHT_BITS, ELEVEN_BITS, FIVE_BITS, FOUR_BITS, FOURTEEN_BITS, NINE_BITS, ONE_BIT, SIX_BITS, SIXTEEN_BITS, TEN_BITS, THIRTEEN_BITS, THREE_BITS, TWELVE_BITS, TWENTY_SIX_BITS, TWO_BITS};
use crate::types::writer::serialize_cdis_float;
use crate::writing::BitBuffer;

impl SerializeCdis for CdisHeader {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, TWO_BITS, self.protocol_version.into());
        let cursor = self.exercise_id.serialize(buf, cursor);
        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.pdu_type.into());
        let cursor = write_value_unsigned::<u32>(buf, cursor, TWENTY_SIX_BITS, self.timestamp.raw_timestamp.into());
        let cursor = write_value_unsigned(buf, cursor, FOURTEEN_BITS, self.length);
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, dis_rs::serialize_pdu_status(&self.pdu_status, &self.pdu_type));

        cursor
    }
}

impl SerializeCdis for AngularVelocity {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for LinearAcceleration {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityCoordinateVector {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityId {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.site.serialize(buf, cursor);
        let cursor = self.application.serialize(buf, cursor);
        let cursor = self.entity.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityType {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, self.kind);
        let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, self.domain);
        let cursor = write_value_unsigned(buf, cursor, NINE_BITS, self.country);

        let cursor = self.category.serialize(buf, cursor);
        let cursor = self.subcategory.serialize(buf, cursor);
        let cursor = self.specific.serialize(buf, cursor);
        let cursor = self.extra.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for LinearVelocity {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for WorldCoordinates {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        // TODO float to signed integer - apply scaling
        // self.latitude
        // self.longitude
        todo!();
        let cursor = self.altitude_msl.serialize(buf, cursor);
        cursor
    }
}

impl SerializeCdis for Orientation {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_signed(buf, cursor, THIRTEEN_BITS, self.psi);
        let cursor = write_value_signed(buf, cursor, THIRTEEN_BITS, self.theta);
        let cursor = write_value_signed(buf, cursor, THIRTEEN_BITS, self.phi);
        cursor
    }
}

impl SerializeCdis for CdisEntityMarking {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, self.marking.len());
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, self.char_encoding.encoding());
        let codes: Vec<u8> = self.marking.chars()
            .map(|char| self.char_encoding.u8_from_char(char) )
            .collect();
        let cursor = codes.iter().fold(cursor, |cur, code| {
            write_value_unsigned(buf, cur, self.char_encoding.bit_size(), *code)
        });
        cursor
    }
}

impl SerializeCdis for ParameterValueFloat {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        serialize_cdis_float(*self, buf, cursor)
    }
}

impl SerializeCdis for CdisVariableParameter {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        const COMPRESSED_FLAG_TRUE: u8 = 1; // FIXME currently only writes compressed Variable Parameters; where is it decided/configured that normal VPs should be processed?
        const RECORD_TYPE_BIT_LENGTH: usize = 3;
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, COMPRESSED_FLAG_TRUE);

        match self {
            CdisVariableParameter::ArticulatedPart(vp) => {
                let cursor = write_value_unsigned::<u8>(buf, cursor, RECORD_TYPE_BIT_LENGTH, VariableParameterRecordType::ArticulatedPart.into());
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::AttachedPart(vp) => {
                let cursor = write_value_unsigned::<u8>(buf, cursor, RECORD_TYPE_BIT_LENGTH, VariableParameterRecordType::AttachedPart.into());
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntitySeparation(vp) => {
                let cursor = write_value_unsigned::<u8>(buf, cursor, RECORD_TYPE_BIT_LENGTH, VariableParameterRecordType::Separation.into());
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntityType(vp) => {
                let cursor = write_value_unsigned::<u8>(buf, cursor, RECORD_TYPE_BIT_LENGTH, VariableParameterRecordType::EntityType.into());
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntityAssociation(vp) => {
                let cursor = write_value_unsigned::<u8>(buf, cursor, RECORD_TYPE_BIT_LENGTH, VariableParameterRecordType::EntityAssociation.into());
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::Unspecified => { cursor }
        }
    }
}

impl SerializeCdis for CdisArticulatedPartVP {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.change_indicator);
        let cursor = write_value_unsigned(buf, cursor, TEN_BITS, self.attachment_id);
        let type_metric: u32 = self.type_metric.into();
        let type_class: u32 = self.type_class.into();
        let parameter_type = type_metric + type_class;
        let cursor = write_value_unsigned(buf, cursor, FOURTEEN_BITS, parameter_type);

        let cursor = self.parameter_value.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisAttachedPartVP {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.detached_indicator.into());
        let cursor = write_value_unsigned(buf, cursor, TEN_BITS, self.attachment_id);
        let cursor = write_value_unsigned::<u32>(buf, cursor, ELEVEN_BITS, self.parameter_type.into());
        let cursor = self.attached_part_type.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisEntitySeparationVP {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.reason_for_separation.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.pre_entity_indicator.into());
        let cursor = self.parent_entity_id.serialize(buf, cursor);
        let cursor = write_value_unsigned::<u16>(buf, cursor, SIX_BITS, self.station_name.into());
        let cursor = write_value_unsigned::<u16>(buf, cursor, TWELVE_BITS, self.station_number);

        cursor
    }
}

impl SerializeCdis for CdisEntityTypeVP {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.change_indicator.into());
        let cursor = self.attached_part_type.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisEntityAssociationVP {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.change_indicator.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, FOUR_BITS, self.association_status.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.association_type.into());
        let cursor = self.entity_id.serialize(buf, cursor);
        let cursor = write_value_unsigned::<u16>(buf, cursor, SIX_BITS, self.own_station_location.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, FIVE_BITS, self.physical_connection_type.into());
        let cursor = write_value_unsigned::<u8>(buf, cursor, FOUR_BITS, self.group_member_type.into());
        let cursor = write_value_unsigned::<u16>(buf, cursor, SIXTEEN_BITS, self.group_number);

        cursor
    }
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::BitArray;
    use crate::writing::SerializeCdis;
    use crate::records::model::CdisEntityMarking;
    use crate::writing::BitBuffer;

    const FOUR_BYTES: usize = 4;

    #[test]
    fn serialize_marking_five_bit_encoding() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = CdisEntityMarking::from("ABCDE");
        let expected: [u8; FOUR_BYTES] = [0b01010000, 0b01000100, 0b00110010, 0b00010100];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..FOUR_BYTES], buf.as_raw_slice()[..FOUR_BYTES]);
    }

    #[test]
    fn serialize_marking_six_bit_encoding() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = CdisEntityMarking::from("AAJJ");
        let expected: [u8; FOUR_BYTES] = [0b01001000, 0b00100000, 0b10010100, 0b01010000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..FOUR_BYTES], buf.as_raw_slice()[..FOUR_BYTES]);
    }
}