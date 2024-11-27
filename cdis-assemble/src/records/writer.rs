use crate::constants::{
    EIGHT_BITS, ELEVEN_BITS, FIVE_BITS, FOURTEEN_BITS, FOUR_BITS, MAX_VARIABLE_DATUM_LENGTH_BITS,
    NINE_BITS, ONE_BIT, SIXTEEN_BITS, SIX_BITS, TEN_BITS, THIRTEEN_BITS, THIRTY_ONE_BITS,
    THIRTY_TWO_BITS, THREE_BITS, TWELVE_BITS, TWENTY_SIX_BITS, TWO_BITS,
};
use crate::records::model::{
    AngularVelocity, BeamData, CdisArticulatedPartVP, CdisAttachedPartVP, CdisEntityAssociationVP,
    CdisEntityMarking, CdisEntitySeparationVP, CdisEntityTypeVP, CdisHeader, CdisRecord,
    CdisVariableParameter, EncodingScheme, EntityCoordinateVector, EntityId, EntityType,
    LayerHeader, LinearAcceleration, LinearVelocity, Orientation, WorldCoordinates,
};
use crate::types::model::{CdisFloat, UVINT8};
use crate::writing::BitBuffer;
use crate::writing::{write_value_signed, write_value_unsigned, SerializeCdis};
use dis_rs::enumerations::VariableParameterRecordType;
use dis_rs::model::{FixedDatum, VariableDatum};
use num_traits::FromPrimitive;

impl SerializeCdis for CdisHeader {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, TWO_BITS, self.protocol_version.into());
        let cursor = self.exercise_id.serialize(buf, cursor);
        let cursor = write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.pdu_type.into());
        let cursor =
            write_value_unsigned::<u32>(buf, cursor, TWENTY_SIX_BITS, self.timestamp.raw_timestamp);
        let cursor = write_value_unsigned(buf, cursor, FOURTEEN_BITS, self.length);
        let cursor = write_value_unsigned(
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
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_signed(buf, cursor, THIRTY_ONE_BITS, self.latitude as i32);
        let cursor = write_value_signed(buf, cursor, THIRTY_TWO_BITS, self.longitude as i32);
        let cursor = self.altitude_msl.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for Orientation {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_signed(buf, cursor, THIRTEEN_BITS, self.psi);
        let cursor = write_value_signed(buf, cursor, THIRTEEN_BITS, self.theta);
        let cursor = write_value_signed(buf, cursor, THIRTEEN_BITS, self.phi);

        cursor
    }
}

impl SerializeCdis for CdisEntityMarking {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, self.marking.len());
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, self.char_encoding.encoding());
        let codes: Vec<u8> = self
            .marking
            .chars()
            .map(|char| self.char_encoding.u8_from_char(char))
            .collect();
        let cursor = codes.iter().fold(cursor, |cur, code| {
            write_value_unsigned(buf, cur, self.char_encoding.bit_size(), *code)
        });

        cursor
    }
}

impl SerializeCdis for CdisVariableParameter {
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        const COMPRESSED_FLAG_TRUE: u8 = 1; // FIXME currently only writes compressed Variable Parameters; where is it decided/configured that normal VPs should be processed?
        const RECORD_TYPE_BIT_LENGTH: usize = 3;
        let cursor = write_value_unsigned(buf, cursor, ONE_BIT, COMPRESSED_FLAG_TRUE);

        match self {
            CdisVariableParameter::ArticulatedPart(vp) => {
                let cursor = write_value_unsigned::<u8>(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    VariableParameterRecordType::ArticulatedPart.into(),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::AttachedPart(vp) => {
                let cursor = write_value_unsigned::<u8>(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    VariableParameterRecordType::AttachedPart.into(),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntitySeparation(vp) => {
                let cursor = write_value_unsigned::<u8>(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    VariableParameterRecordType::Separation.into(),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntityType(vp) => {
                let cursor = write_value_unsigned::<u8>(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    VariableParameterRecordType::EntityType.into(),
                );
                vp.serialize(buf, cursor)
            }
            CdisVariableParameter::EntityAssociation(vp) => {
                let cursor = write_value_unsigned::<u8>(
                    buf,
                    cursor,
                    RECORD_TYPE_BIT_LENGTH,
                    VariableParameterRecordType::EntityAssociation.into(),
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
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.detached_indicator.into());
        let cursor = write_value_unsigned(buf, cursor, TEN_BITS, self.attachment_id);
        let cursor =
            write_value_unsigned::<u32>(buf, cursor, ELEVEN_BITS, self.parameter_type.into());
        let cursor = self.attached_part_type.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisEntitySeparationVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.reason_for_separation.into());
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, THREE_BITS, self.pre_entity_indicator.into());
        let cursor = self.parent_entity_id.serialize(buf, cursor);
        let cursor = write_value_unsigned::<u16>(buf, cursor, SIX_BITS, self.station_name.into());
        let cursor = write_value_unsigned::<u16>(buf, cursor, TWELVE_BITS, self.station_number);

        cursor
    }
}

impl SerializeCdis for CdisEntityTypeVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.change_indicator.into());
        let cursor = self.attached_part_type.serialize(buf, cursor);

        cursor
    }
}

impl SerializeCdis for CdisEntityAssociationVP {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor = write_value_unsigned::<u8>(buf, cursor, ONE_BIT, self.change_indicator.into());
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, FOUR_BITS, self.association_status.into());
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, EIGHT_BITS, self.association_type.into());
        let cursor = self.entity_id.serialize(buf, cursor);
        let cursor =
            write_value_unsigned::<u16>(buf, cursor, SIX_BITS, self.own_station_location.into());
        let cursor = write_value_unsigned::<u8>(
            buf,
            cursor,
            FIVE_BITS,
            self.physical_connection_type.into(),
        );
        let cursor =
            write_value_unsigned::<u8>(buf, cursor, FOUR_BITS, self.group_member_type.into());
        let cursor = write_value_unsigned::<u16>(buf, cursor, SIXTEEN_BITS, self.group_number);

        cursor
    }
}

impl SerializeCdis for FixedDatum {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned::<u32>(buf, cursor, THIRTY_TWO_BITS, self.datum_id.into());
        let cursor = write_value_unsigned(buf, cursor, THIRTY_TWO_BITS, self.datum_value);

        cursor
    }
}

impl SerializeCdis for VariableDatum {
    #[allow(clippy::let_and_return)]
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let cursor =
            write_value_unsigned::<u32>(buf, cursor, THIRTY_TWO_BITS, self.datum_id.into());
        let cursor = write_value_unsigned(
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
    fn serialize(&self, buf: &mut BitBuffer, cursor: usize) -> usize {
        let (encoding_class, encoding_type) = match self {
            EncodingScheme::EncodedAudio {
                encoding_class,
                encoding_type,
            } => {
                let encoding_type: u16 = (*encoding_type).into();
                (encoding_class, UVINT8::from(encoding_type as u8))
            }
            EncodingScheme::RawBinaryData {
                encoding_class,
                nr_of_messages,
            } => (encoding_class, UVINT8::from(*nr_of_messages)),
            EncodingScheme::Unspecified {
                encoding_class,
                encoding_type,
            } => (encoding_class, UVINT8::from(*encoding_type)),
        };

        let encoding_class: u16 = (*encoding_class).into();
        let cursor = write_value_unsigned(buf, cursor, TWO_BITS, encoding_class);
        let cursor = encoding_type.serialize(buf, cursor);

        cursor
    }
}

#[cfg(test)]
mod tests {
    use crate::records::model::{CdisEntityMarking, CdisHeader, CdisProtocolVersion, CdisRecord};
    use crate::types::model::UVINT8;
    use crate::writing::BitBuffer;
    use crate::writing::SerializeCdis;
    use bitvec::prelude::BitArray;
    use dis_rs::enumerations::PduType;

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

    #[test]
    fn serialize_cdis_header() {
        let mut buf: BitBuffer = BitArray::ZERO;

        let header = CdisHeader {
            protocol_version: CdisProtocolVersion::SISO_023_2023,
            exercise_id: UVINT8::from(7),
            pdu_type: PduType::EntityState,
            timestamp: Default::default(),
            length: 0,
            pdu_status: Default::default(),
        };

        let expected: [u8; 8] = [
            0b01001110, 0b00000010, 0b00000000, 0b00000000, 0b00000000, 0b00000000, 0b00000000,
            0b00000000,
        ];
        let next_cursor = header.serialize(&mut buf, 0);

        assert_eq!(next_cursor, header.record_length());
        assert_eq!(buf.data[..64][..8], expected);
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

impl LayerHeader {
    pub fn serialize_with_length(
        &self,
        body_length: usize,
        buf: &mut BitBuffer,
        cursor: usize,
    ) -> usize {
        let cursor = write_value_unsigned(buf, cursor, FOUR_BITS, self.layer_number);
        let cursor = write_value_unsigned(buf, cursor, EIGHT_BITS, self.layer_specific_information);
        let cursor = write_value_unsigned(
            buf,
            cursor,
            FOURTEEN_BITS,
            self.record_length() + body_length,
        );

        cursor
    }
}
