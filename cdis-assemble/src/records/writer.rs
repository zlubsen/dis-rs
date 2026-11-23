use crate::constants::{
    EIGHT_BITS, ELEVEN_BITS, FIVE_BITS, FOUR_BITS, FOURTEEN_BITS, MAX_VARIABLE_DATUM_LENGTH_BITS,
    NINE_BITS, ONE_BIT, SIX_BITS, SIXTEEN_BITS, TEN_BITS, THIRTEEN_BITS, THIRTY_ONE_BITS,
    THIRTY_TWO_BITS, THREE_BITS, TWELVE_BITS, TWO_BITS,
};
use crate::records::model::{
    AngularVelocity, BeamData, CdisArticulatedPartVP, CdisAttachedPartVP, CdisEntityAssociationVP,
    CdisEntityMarking, CdisEntitySeparationVP, CdisEntityTypeVP, CdisHeader, CdisRecord,
    CdisVariableParameter, DescriptorQuantityRate, DescriptorWarheadFuse, EncodingScheme,
    EntityCoordinateVector, EntityId, EntityType, LayerHeader, LinearAcceleration, LinearVelocity,
    Orientation, WorldCoordinates,
};
use crate::types::model::{CdisFloat, UVINT8};
use crate::writing::{BitBuffer, SerializeCdis, write_integer_bits};
use dis_rs::enumerations::VariableParameterRecordType;
use dis_rs::model::{FixedDatum, VariableDatum};
use num_traits::FromPrimitive;

impl SerializeCdis for CdisHeader {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, TWO_BITS, u8::from(self.protocol_version));
        let cursor = self.exercise_id.serialize(buf, cursor);
        let cursor = write_integer_bits(buf, cursor, EIGHT_BITS, u8::from(self.pdu_type));
        let cursor = self.timestamp.serialize(buf, cursor);
        let cursor = write_integer_bits(buf, cursor, FOURTEEN_BITS, self.length);
        let cursor = write_integer_bits(
            buf,
            cursor,
            EIGHT_BITS,
            dis_rs::serialize_pdu_status(&self.pdu_status, &self.pdu_type),
        );

        cursor
    }
}

impl SerializeCdis for AngularVelocity {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for LinearAcceleration {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityCoordinateVector {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityId {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.site.serialize(buf, cursor);
        let cursor = self.application.serialize(buf, cursor);
        let cursor = self.entity.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for EntityType {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, self.kind);
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, self.domain);
        let cursor = write_integer_bits(buf, cursor, NINE_BITS, self.country);

        let cursor = self.category.serialize(buf, cursor);
        let cursor = self.subcategory.serialize(buf, cursor);
        let cursor = self.specific.serialize(buf, cursor);
        let cursor = self.extra.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for LinearVelocity {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.x.serialize(buf, cursor);
        let cursor = self.y.serialize(buf, cursor);
        let cursor = self.z.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for WorldCoordinates {
    #[allow(clippy::let_and_return)]
    #[allow(clippy::cast_possible_truncation)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, THIRTY_ONE_BITS, self.latitude);
        let cursor = write_integer_bits(buf, cursor, THIRTY_TWO_BITS, self.longitude);
        let cursor = self.altitude_msl.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for Orientation {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.psi);
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.theta);
        let cursor = write_integer_bits(buf, cursor, THIRTEEN_BITS, self.phi);

        cursor
    }
}

impl SerializeCdis for CdisEntityMarking {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, self.marking.len());
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, self.char_encoding.encoding());
        let codes: Vec<u8> = self
            .marking
            .chars()
            .map(|char| self.char_encoding.u8_from_char(char))
            .collect();
        let cursor = codes.iter().fold(cursor, |cur, code| {
            write_integer_bits(buf, cur, self.char_encoding.bit_size(), *code)
        });

        cursor
    }
}

impl SerializeCdis for CdisVariableParameter {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        const COMPRESSED_FLAG_TRUE: u8 = 1; // FIXME currently only writes compressed Variable Parameters; where is it decided/configured that normal VPs should be processed?
        const RECORD_TYPE_BIT_LENGTH: usize = 3;
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, COMPRESSED_FLAG_TRUE);

        match self {
            CdisVariableParameter::ArticulatedPart(vp) => {
                let cursor = write_integer_bits(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    u8::from(VariableParameterRecordType::ArticulatedPart),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::AttachedPart(vp) => {
                let cursor = write_integer_bits(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    u8::from(VariableParameterRecordType::AttachedPart),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntitySeparation(vp) => {
                let cursor = write_integer_bits(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    u8::from(VariableParameterRecordType::Separation),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntityType(vp) => {
                let cursor = write_integer_bits(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    u8::from(VariableParameterRecordType::EntityType),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntityAssociation(vp) => {
                let cursor = write_integer_bits(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    u8::from(VariableParameterRecordType::EntityAssociation),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::Unspecified => cursor,
        }
    }
}

impl SerializeCdis for CdisArticulatedPartVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, EIGHT_BITS, self.change_indicator);
        let cursor = write_integer_bits(buf, cursor, TEN_BITS, self.attachment_id);
        let type_metric: u32 = self.type_metric.into();
        let type_class: u32 = self.type_class.into();
        let parameter_type = type_metric + type_class;
        let cursor = write_integer_bits(buf, cursor, FOURTEEN_BITS, parameter_type);

        let cursor = self.parameter_value.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisAttachedPartVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.detached_indicator));
        let cursor = write_integer_bits(buf, cursor, TEN_BITS, self.attachment_id);
        let cursor = write_integer_bits(buf, cursor, ELEVEN_BITS, u32::from(self.parameter_type));
        let cursor = self.attached_part_type.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisEntitySeparationVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(
            buf,
            cursor,
            THREE_BITS,
            u8::from(self.reason_for_separation),
        );
        let cursor =
            write_integer_bits(buf, cursor, THREE_BITS, u8::from(self.pre_entity_indicator));
        let cursor = self.parent_entity_id.serialize(buf, cursor);
        let cursor = write_integer_bits(buf, cursor, SIX_BITS, u16::from(self.station_name));
        let cursor = write_integer_bits(buf, cursor, TWELVE_BITS, self.station_number);

        cursor
    }
}

impl SerializeCdis for CdisEntityTypeVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.change_indicator));
        let cursor = self.attached_part_type.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisEntityAssociationVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, ONE_BIT, u8::from(self.change_indicator));
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, u8::from(self.association_status));
        let cursor = write_integer_bits(buf, cursor, EIGHT_BITS, u8::from(self.association_type));
        let cursor = self.entity_id.serialize(buf, cursor);
        let cursor =
            write_integer_bits(buf, cursor, SIX_BITS, u16::from(self.own_station_location));
        let cursor = write_integer_bits(
            buf,
            cursor,
            FIVE_BITS,
            u8::from(self.physical_connection_type),
        );
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, u8::from(self.group_member_type));
        let cursor = write_integer_bits(buf, cursor, SIXTEEN_BITS, self.group_number);

        cursor
    }
}

impl SerializeCdis for FixedDatum {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, THIRTY_TWO_BITS, u32::from(self.datum_id));
        let cursor = write_integer_bits(buf, cursor, THIRTY_TWO_BITS, self.datum_value);

        cursor
    }
}

impl SerializeCdis for VariableDatum {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_integer_bits(buf, cursor, THIRTY_TWO_BITS, u32::from(self.datum_id));
        let cursor = write_integer_bits(
            buf,
            cursor,
            FOURTEEN_BITS,
            u16::from_usize(self.datum_value.len() * EIGHT_BITS)
                .unwrap_or(MAX_VARIABLE_DATUM_LENGTH_BITS),
        );

        let cursor = self
            .datum_value
            .iter()
            .fold(cursor, |cursor, vp| vp.serialize(buf, cursor));

        cursor
    }
}

impl SerializeCdis for EncodingScheme {
    #[allow(clippy::let_and_return)]
    #[allow(clippy::cast_possible_truncation)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let (encoding_class, encoding_type) = match self {
            EncodingScheme::EncodedAudio {
                encoding_class,
                encoding_type,
            } => (encoding_class, u16::from(*encoding_type) as u8),
            EncodingScheme::RawBinaryData {
                encoding_class,
                nr_of_messages,
            } => (encoding_class, *nr_of_messages),
            EncodingScheme::Unspecified {
                encoding_class,
                encoding_type,
            } => (encoding_class, *encoding_type),
        };

        let cursor = write_integer_bits(buf, cursor, TWO_BITS, u16::from(*encoding_class));
        let cursor = UVINT8::from(encoding_type).serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for BeamData {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.az_center.serialize(buf, cursor);
        let cursor = self.az_sweep.serialize(buf, cursor);
        let cursor = self.el_center.serialize(buf, cursor);
        let cursor = self.el_sweep.serialize(buf, cursor);
        let cursor = write_integer_bits(buf, cursor, TEN_BITS, self.sweep_sync);

        cursor
    }
}

impl SerializeCdis for DescriptorWarheadFuse {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.warhead.serialize(buf, cursor);
        let cursor = self.fuse.serialize(buf, cursor);
        cursor
    }
}

impl SerializeCdis for DescriptorQuantityRate {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = self.quantity.serialize(buf, cursor);
        let cursor = self.rate.serialize(buf, cursor);
        cursor
    }
}

impl LayerHeader {
    #[allow(clippy::let_and_return)]
    pub fn serialize_with_length(
        &self,
        body_length: usize,
        buf: &mut BitBuffer,
        cursor: usize,
    ) -> usize {
        let cursor = write_integer_bits(buf, cursor, FOUR_BITS, self.layer_number);
        let cursor = write_integer_bits(buf, cursor, EIGHT_BITS, self.layer_specific_information);
        let cursor = write_integer_bits(
            buf,
            cursor,
            FOURTEEN_BITS,
            self.record_length() + body_length,
        );

        cursor
    }
}

#[cfg(test)]
mod tests {
    use crate::records::model::{
        CdisEntityMarking, CdisHeader, CdisProtocolVersion, CdisRecord, CdisTimestamp,
    };
    use crate::types::model::UVINT8;
    use crate::writing::BitBuffer;
    use crate::writing::SerializeCdis;
    use bitvec::prelude::BitArray;
    use dis_rs::enumerations::PduType;
    use dis_rs::model::PduStatus;

    const FOUR_BYTES: usize = 4;

    #[test]
    fn serialize_marking_five_bit_encoding() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = CdisEntityMarking::from("ABCDE");
        let expected: [u8; FOUR_BYTES] = [0b0101_0000, 0b0100_0100, 0b0011_0010, 0b0001_0100];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..FOUR_BYTES], buf.as_raw_slice()[..FOUR_BYTES]);
    }

    #[test]
    fn serialize_marking_six_bit_encoding() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let input = CdisEntityMarking::from("AAJJ");
        let expected: [u8; FOUR_BYTES] = [0b0100_1000, 0b0010_0000, 0b1001_0100, 0b0101_0000];
        let _next_cursor = input.serialize(&mut buf, 0);

        assert_eq!(expected[..FOUR_BYTES], buf.as_raw_slice()[..FOUR_BYTES]);
    }

    #[test]
    fn serialize_cdis_header() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let header = CdisHeader {
            protocol_version: CdisProtocolVersion::SISO_023_2023,
            exercise_id: UVINT8::from(7),
            pdu_type: PduType::EntityState,
            timestamp: CdisTimestamp::default(),
            length: 0,
            pdu_status: PduStatus::default(),
        };

        let expected: [u8; 8] = [
            0b0100_1110,
            0b0000_0010,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
            0b0000_0000,
        ];
        let next_cursor = header.serialize(&mut buf, 0);

        assert_eq!(next_cursor, header.record_length());
        assert_eq!(buf.data[..64][..8], expected);
    }
}
