use crate::common::model::PduBody;
use crate::common::parser::entity_id;
use crate::common::signal::model::{EncodingScheme, Signal};
use crate::constants::{FOUR_OCTETS, ONE_BYTE_IN_BITS};
use crate::enumerations::{
    SignalEncodingClass, SignalEncodingType, SignalTdlType, SignalUserProtocolIdentificationNumber,
};
use crate::model::length_padded_to_num;
use nom::number::complete::{be_u16, be_u32};
use nom::IResult;

pub(crate) fn signal_body(input: &[u8]) -> IResult<&[u8], PduBody> {
    let (input, radio_reference_id) = entity_id(input)?;
    let (input, radio_number) = be_u16(input)?;
    let (input, encoding_scheme) = be_u16(input)?;
    let (input, tdl_type) = be_u16(input)?;
    let tdl_type = SignalTdlType::from(tdl_type);
    let (input, sample_rate) = be_u32(input)?;

    let (input, data_length_in_bits) = be_u16(input)?;
    let (input, samples) = be_u16(input)?;
    let (input, data) =
        nom::bytes::complete::take(data_length_in_bits / ONE_BYTE_IN_BITS as u16)(input)?;

    let data_length_in_bytes = data_length_in_bits / ONE_BYTE_IN_BITS as u16;
    let padded_record = length_padded_to_num(data_length_in_bytes as usize, FOUR_OCTETS);
    let (input, _padding) = nom::bytes::complete::take(padded_record.padding_length)(input)?;

    let encoding_scheme = parse_encoding_scheme(encoding_scheme, data);

    let body = Signal::builder()
        .with_radio_reference_id(radio_reference_id)
        .with_radio_number(radio_number)
        .with_encoding_scheme(encoding_scheme)
        .with_tdl_type(tdl_type)
        .with_sample_rate(sample_rate)
        .with_samples(samples)
        .with_data(data.to_vec())
        .build();

    Ok((input, body.into_pdu_body()))
}

fn parse_encoding_scheme(encoding_scheme_bytes: u16, data: &[u8]) -> EncodingScheme {
    let encoding_class = encoding_scheme_bytes >> 14;
    let low_bits = encoding_scheme_bytes & 0x3FFF;
    let encoding_class = SignalEncodingClass::from(encoding_class);

    match encoding_class {
        SignalEncodingClass::EncodedAudio => {
            let encoding_type = SignalEncodingType::from(low_bits);
            EncodingScheme::EncodedAudio {
                encoding_class,
                encoding_type,
            }
        }
        SignalEncodingClass::RawBinaryData => EncodingScheme::RawBinaryData {
            encoding_class,
            nr_of_messages: low_bits,
        },
        SignalEncodingClass::ApplicationSpecificData => {
            let mut bytes = [0u8, 0u8, 0u8, 0u8];
            bytes.clone_from_slice(&data[0..4]);
            let user_protocol_id = u32::from_be_bytes(bytes);
            let user_protocol_id = SignalUserProtocolIdentificationNumber::from(user_protocol_id);
            EncodingScheme::ApplicationSpecificData {
                encoding_class,
                user_protocol_id,
            }
        }
        SignalEncodingClass::DatabaseIndex => {
            let mut index_bytes = [0u8, 0u8, 0u8, 0u8];
            index_bytes.clone_from_slice(&data[0..4]);
            let mut offset_bytes = [0u8, 0u8, 0u8, 0u8];
            offset_bytes.clone_from_slice(&data[4..8]);
            let mut duration_bytes = [0u8, 0u8, 0u8, 0u8];
            duration_bytes.clone_from_slice(&data[8..12]);
            EncodingScheme::DatabaseIndex {
                encoding_class,
                index: u32::from_be_bytes(index_bytes),
                offset_milli_secs: u32::from_be_bytes(offset_bytes),
                duration_milli_secs: u32::from_be_bytes(duration_bytes),
            }
        }
        SignalEncodingClass::Unspecified(_) => {
            // 2-bit _value can only contain values 0-3 decimal, so SignalEncodingClass::Unspecified should never be possible.
            // For completeness sake and possible debugging the contained value is returned as EncodingScheme::Unspecified
            EncodingScheme::Unspecified { encoding_class }
        }
    }
}
