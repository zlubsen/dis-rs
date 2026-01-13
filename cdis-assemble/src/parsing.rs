use crate::acknowledge::parser::acknowledge_body;
use crate::action_request::parser::action_request_body;
use crate::action_response::parser::action_response_body;
use crate::collision::parser::collision_body;
use crate::comment::parser::comment_body;
use crate::constants::ONE_BIT;
use crate::create_entity::parser::create_entity_body;
use crate::data::parser::data_body;
use crate::data_query::parser::data_query_body;
use crate::designator::parser::designator_body;
use crate::detonation::parser::detonation_body;
use crate::electromagnetic_emission::parser::electromagnetic_emission_body;
use crate::entity_state::parser::entity_state_body;
use crate::event_report::parser::event_report_body;
use crate::fire::parser::fire_body;
use crate::iff::parser::iff_body;
use crate::receiver::parser::receiver_body;
use crate::records::model::{CdisHeader, CdisProtocolVersion};
use crate::records::parser::cdis_header;
use crate::remove_entity::parser::remove_entity_body;
use crate::set_data::parser::set_data_body;
use crate::signal::parser::signal_body;
use crate::start_resume::parser::start_resume_body;
use crate::stop_freeze::parser::stop_freeze_body;
use crate::transmitter::parser::transmitter_body;
use crate::types::model::VarInt;
use crate::unsupported::Unsupported;
use crate::{CdisBody, CdisError, CdisPdu};
use dis_rs::enumerations::PduType;
use nom::bits::complete::take;
use nom::error::ErrorKind;
use nom::IResult;
use std::ops::BitAnd;

/// Attempts to parse the provided buffer for CDIS PDUs
///
/// # Errors
/// Returns a `CdisError` when parsing fails.
pub fn parse(input: &[u8]) -> Result<Vec<CdisPdu>, CdisError> {
    parse_multiple_cdis_pdu(input)
}

pub(crate) fn parse_multiple_cdis_pdu(input: &[u8]) -> Result<Vec<CdisPdu>, CdisError> {
    let mut vec = vec![];
    let mut bit_input = (input, 0);
    loop {
        match cdis_pdu(bit_input) {
            Ok((next_input, pdu)) => {
                bit_input = next_input;
                if pdu.header.protocol_version != CdisProtocolVersion::StandardDis {
                    vec.push(pdu);
                }
            }
            Err(nom::Err::Error(e)) => {
                if e.code == ErrorKind::Eof {
                    break;
                }
                return Err(CdisError::ParseError("crack".to_string()));
            }
            Err(err) => {
                return Err(CdisError::ParseError(err.to_string()));
            }
        }
    }
    Ok(vec)
    // FIXME With nom v8 the many1 combinator does not accept bit-input anymore
    // match many1(cdis_pdu)((input, 0)) {
    //     Ok((_, pdus)) => Ok(pdus),
    //     Err(err) => Err(CdisError::ParseError(err.to_string())), // TODO not very descriptive / error means we can not match any PDUs
    // }
}

pub(crate) fn cdis_pdu(input: BitInput) -> IResult<BitInput, CdisPdu> {
    let (input, header) = cdis_header(input)?;
    let (input, body) = cdis_body(&header)(input)?;

    Ok((input, CdisPdu { header, body }))
}

pub(crate) fn cdis_body(
    header: &CdisHeader,
) -> impl Fn(BitInput) -> IResult<BitInput, CdisBody> + '_ {
    move |input: BitInput| {
        let (input, body): (BitInput, CdisBody) = match header.pdu_type {
            PduType::EntityState => entity_state_body(input)?,
            PduType::Fire => fire_body(input)?,
            PduType::Detonation => detonation_body(input)?,
            PduType::Collision => collision_body(input)?,
            PduType::CreateEntity => create_entity_body(input)?,
            PduType::RemoveEntity => remove_entity_body(input)?,
            PduType::StartResume => start_resume_body(input)?,
            PduType::StopFreeze => stop_freeze_body(input)?,
            PduType::Acknowledge => acknowledge_body(input)?,
            PduType::ActionRequest => action_request_body(input)?,
            PduType::ActionResponse => action_response_body(input)?,
            PduType::DataQuery => data_query_body(input)?,
            PduType::SetData => set_data_body(input)?,
            PduType::Data => data_body(input)?,
            PduType::EventReport => event_report_body(input)?,
            PduType::Comment => comment_body(input)?,
            PduType::ElectromagneticEmission => electromagnetic_emission_body(input)?,
            PduType::Designator => designator_body(input)?,
            PduType::Transmitter => transmitter_body(input)?,
            PduType::Signal => signal_body(input)?,
            PduType::Receiver => receiver_body(input)?,
            PduType::IFF => iff_body(input)?,
            // Unsupported PDUs in CDIS v1
            PduType::Other => (input, CdisBody::Unsupported(Unsupported)),
            PduType::Unspecified(_val) => (input, CdisBody::Unsupported(Unsupported)),
            _val => (input, CdisBody::Unsupported(Unsupported)),
        };

        Ok((input, body))
    }
}

pub(crate) type BitInput<'a> = (&'a [u8], usize);

/// This is a 'conditional parser', which applies the provided parser `f` when either a full update is needed (indicated by the `full_update` flag)
/// or when `mask` applied (bitwise OR) to the `fields_present` flags yields a none-zero value.
///
/// The function returns the output of parser `f` as an `Option`.
pub(crate) fn parse_field_when_present<'a, O, T, F>(
    fields_present: T,
    mask: T,
    f: F,
) -> impl Fn(BitInput<'a>) -> IResult<BitInput<'a>, Option<O>>
where
    O: std::fmt::Debug,
    T: Copy + BitAnd + PartialEq + Default,
    <T as BitAnd>::Output: PartialEq<T>,
    F: Fn(BitInput<'a>) -> IResult<BitInput<'a>, O>,
{
    move |input: BitInput<'a>| {
        if field_present(fields_present, mask) {
            let result = f(input);
            match result {
                Ok((input, result)) => Ok((input, Some(result))),
                Err(err) => Err(err),
            }
        } else {
            Ok((input, None))
        }
    }
}

/// Helper function to match presents of a bit position in a bitfield.
///
/// Returns `true` when `fields_present` OR `mask` yields a non-zero value.
/// Works with the basic numerical types (u8, u16, u32, i..).
pub(crate) fn field_present<T>(fields_present: T, mask: T) -> bool
where
    T: BitAnd + PartialEq + Default,
    <T as BitAnd>::Output: PartialEq<T>,
{
    (fields_present & mask) != Default::default()
}

/// Conversion function to convert the inner type of an `Option<T>` as
/// returned by a conditional parser to another type.
/// Useful for transforming a `VarInt` to a standard Rust type such as `u8`.
///
/// Returns `None` or `Some` with the converted type
pub(crate) fn varint_to_type<V, I, T>(enum_value: Option<V>) -> Option<T>
where
    V: VarInt<InnerType = I>,
    T: From<I>,
{
    if let Some(value) = enum_value {
        let inner = value.value();
        Some(T::from(inner))
    } else {
        None
    }
}

/// Parse a signed value from the bit stream, formatted in `count` bits.
/// MSB is the sign bit, the remaining bits form the value.
/// This function then converts these two components to a signed value of type `isize`.
#[allow(clippy::cast_possible_truncation)]
#[allow(clippy::cast_possible_wrap)]
pub(crate) fn take_signed(count: usize) -> impl Fn(BitInput) -> IResult<BitInput, isize> {
    move |input| {
        let (input, sign_bit): (BitInput, isize) = take(ONE_BIT)(input)?;
        let (input, value_bits): (BitInput, isize) = take(count - ONE_BIT)(input)?;

        let max_value = 2usize.pow((count - 1) as u32) - 1;
        let min_value = -(max_value as isize + 1);
        let value = if sign_bit != 0 {
            min_value + value_bits
        } else {
            value_bits
        };

        Ok((input, value))
    }
}

#[cfg(test)]
mod tests {
    use crate::codec::{CodecOptions, EncoderState};
    use crate::constants::THREE_BITS;
    use crate::parsing::{field_present, parse_field_when_present, take_signed};
    use crate::records::model::dis_to_cdis_u32_timestamp;
    use crate::records::parser::entity_identification;
    use crate::{parse, CdisPdu, SerializeCdisPdu};
    use dis_rs::enumerations::{
        EntityKind, FireTypeIndicator, MunitionDescriptorFuse, MunitionDescriptorWarhead, PduType,
        PlatformDomain,
    };
    use dis_rs::fire::model::{Fire, FireDescriptor};
    use dis_rs::model::{
        EntityId, EntityType, EventId, Location, MunitionDescriptor, Pdu, PduBody, PduHeader,
        PduStatus, TimeStamp,
    };
    use dis_rs::BodyRaw;

    fn build_default_fire_header() -> PduHeader {
        PduHeader::new_v7(7, PduType::Fire).with_pdu_status(
            PduStatus::default().with_fire_type_indicator(FireTypeIndicator::Munition),
        )
    }

    fn build_default_fire_body() -> PduBody {
        Fire::builder()
            .with_firing_entity_id(EntityId::new(10, 10, 10))
            .with_target_entity_id(EntityId::new(20, 20, 20))
            .with_entity_id(EntityId::new(10, 10, 500))
            .with_event_id(EventId::new(10, 10, 1))
            .with_location_in_world(Location::new(0.0, 0.0, 20000.0))
            .with_descriptor(FireDescriptor::Munition(
                MunitionDescriptor::default()
                    .with_entity_type(
                        EntityType::default()
                            .with_kind(EntityKind::Munition)
                            .with_domain(PlatformDomain::Air),
                    )
                    .with_warhead(MunitionDescriptorWarhead::Dummy)
                    .with_fuse(MunitionDescriptorFuse::Dummy_8110)
                    .with_quantity(1)
                    .with_rate(1),
            ))
            .with_range(10000.0)
            .build()
            .into_pdu_body()
    }

    #[test]
    fn parse_single_cdis_pdu() {
        let mut encoder_state = EncoderState::new();
        let codec_options = CodecOptions::new_full_update();

        let dis_header = build_default_fire_header();
        let dis_body = build_default_fire_body();
        let dis_pdu_in = Pdu::finalize_from_parts(dis_header, dis_body, 0);

        let (cdis_pdu, _state_result) =
            CdisPdu::encode(&dis_pdu_in, &mut encoder_state, &codec_options);

        let mut bit_buf = crate::create_bit_buffer();
        let cursor = cdis_pdu.serialize(&mut bit_buf, 0);
        assert_ne!(cursor, 0);

        let parsed_cdis_pdu = parse(bit_buf.as_raw_slice()).unwrap();
        assert_eq!(parsed_cdis_pdu.len(), 1);
        assert_eq!(
            parsed_cdis_pdu.first().unwrap().header.pdu_type,
            PduType::Fire
        );
    }

    #[test]
    fn parse_multiple_cdis_pdu() {
        let mut encoder_state = EncoderState::new();
        let codec_options = CodecOptions::new_full_update();

        let dis_header = build_default_fire_header();
        let dis_body = build_default_fire_body();
        let dis_pdu_in_1 =
            Pdu::finalize_from_parts(dis_header, dis_body.clone(), TimeStamp::new(0));
        let dis_pdu_in_2 = Pdu::finalize_from_parts(dis_header, dis_body, TimeStamp::new(50));

        let (cdis_pdu_1, _state_result) =
            CdisPdu::encode(&dis_pdu_in_1, &mut encoder_state, &codec_options);
        let (cdis_pdu_2, _state_result) =
            CdisPdu::encode(&dis_pdu_in_2, &mut encoder_state, &codec_options);

        let mut bit_buf = crate::create_bit_buffer();
        let cursor_1 = cdis_pdu_1.serialize(&mut bit_buf, 0);
        assert_ne!(cursor_1, 0);
        let cursor_2 = cdis_pdu_2.serialize(&mut bit_buf, cursor_1);
        assert_ne!(cursor_2, cursor_1);

        let parsed_cdis_pdus = parse(bit_buf.as_raw_slice()).unwrap();
        assert_eq!(parsed_cdis_pdus.len(), 2);
        #[allow(clippy::get_first)]
        if let Some(parsed_pdu_1) = parsed_cdis_pdus.get(0) {
            assert_eq!(parsed_pdu_1.header.pdu_type, PduType::Fire);
            assert_eq!(parsed_pdu_1.header.timestamp, TimeStamp::new(0));
        } else {
            panic!("Failed to get parsed PDU from vec.")
        }
        if let Some(parsed_pdu_2) = parsed_cdis_pdus.get(1) {
            assert_eq!(parsed_pdu_2.header.pdu_type, PduType::Fire);
            assert_eq!(
                parsed_pdu_2.header.timestamp,
                TimeStamp::new(dis_to_cdis_u32_timestamp(50))
            );
        } else {
            panic!("Failed to get parsed PDU from vec.")
        }
    }

    #[test]
    fn take_signed_positive_min() {
        let input = [0b0000_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(0, value);
    }

    #[test]
    fn take_signed_positive_max() {
        let input = [0b0110_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(3, value);
    }

    #[test]
    fn take_signed_negative_min() {
        let input = [0b1000_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(-4, value);
    }

    #[test]
    fn take_signed_negative_max() {
        let input = [0b1110_0000];
        let (_input, value) = take_signed(THREE_BITS)((&input, 0)).unwrap();

        assert_eq!(-1, value);
    }

    #[test]
    fn field_present_u8_true() {
        let fields = 0b0000_0010_u8;
        let mask = 0x2u8;

        assert!(field_present(fields, mask));
    }

    #[test]
    fn field_present_u32_true() {
        let fields = 0x0200_4010_u32;
        let mask = 0x10u32;

        assert!(field_present(fields, mask));
    }

    #[test]
    fn parse_when_present_entity_id() {
        let fields = 0b0000_0001_u8;
        let mask = 0x01u8;
        let input: [u8; 4] = [0b0000_0000, 0b0100_0000, 0b0001_0000, 0b0000_0100];

        // entity_identification is in reality always present, but is an easy example for a test.
        let actual = parse_field_when_present(fields, mask, entity_identification)((&input, 0));

        assert!(actual.is_ok());
        let entity = actual.unwrap().1;
        assert!(entity.is_some());
        let entity = entity.unwrap();
        assert_eq!(1u16, entity.site.value);
        assert_eq!(1u16, entity.application.value);
        assert_eq!(1u16, entity.entity.value);
    }

    #[test]
    fn parse_when_present_entity_id_not_present() {
        let fields = 0b0001_0000_u8;
        let mask = 0x01u8;
        let input: [u8; 4] = [0b0000_0000, 0b0100_0000, 0b0001_0000, 0b0000_0100];

        // entity_identification is in reality always present, but is an easy example for a test.
        let actual = parse_field_when_present(fields, mask, entity_identification)((&input, 0));

        assert!(actual.is_ok());
        assert!(actual.unwrap().1.is_none());
    }

    #[test]
    fn field_present_u32_false() {
        let fields = 0x0200_4010_u32;
        let mask = 0x01u32;

        assert!(!field_present(fields, mask));
    }

    #[test]
    fn field_present_u8_false() {
        let fields = 0b0000_0100_u8;
        let mask = 0x2u8;

        assert!(!field_present(fields, mask));
    }
}
