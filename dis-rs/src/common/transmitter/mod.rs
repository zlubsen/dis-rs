pub mod parser;
pub mod model;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{PduType, TransmitterTransmitState};
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::Serialize;
    use crate::common::model::{DisTimeStamp};
    use crate::model::{Location, VectorF32};
    use crate::transmitter::model::{Transmitter, VariableTransmitterParameter};

    #[test]
    fn transmitter_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::Transmitter);

        let body = Transmitter::builder()
            .with_power(45.0)
            .with_frequency(10215)
            .with_transmit_state(TransmitterTransmitState::Onbutnottransmitting)
            .with_antenna_location(Location::new(0.0, 0.0, 0.0))
            .with_relative_antenna_location(VectorF32::default())
            .with_variable_transmitter_parameter(VariableTransmitterParameter::default())
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf);

        let parsed = parse_pdu(&buf);
        match parsed {
            Ok(ref pdu) => {
                assert_eq!(&original_pdu, pdu);
            }
            Err(ref _err) => {
                println!("{_err}");
                assert!(false);
            }
        }
    }
}