pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::model::DisTimeStamp;
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{
        CoupledExtensionIndicator, LvcIndicator, PduType, RadioAttachedIndicator,
        TransferredEntityIndicator, TransmitterTransmitState, VariableRecordType,
    };
    use crate::model::{Location, VectorF32};
    use crate::transmitter::model::{Transmitter, VariableTransmitterParameter};
    use crate::v7::model::PduStatus;
    use bytes::BytesMut;

    #[test]
    fn transmitter_internal_consistency_v6() {
        let header = PduHeader::new_v6(1, PduType::Transmitter);

        let body = Transmitter::builder()
            .with_power(45.0)
            .with_frequency(10215)
            .with_transmit_state(TransmitterTransmitState::OnButNotTransmitting)
            .with_antenna_location(Location::new(0.0, 0.0, 0.0))
            .with_relative_antenna_location(VectorF32::default())
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;
        let original_length = original_pdu.pdu_length();

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        let serialized_length = original_pdu.serialize(&mut buf).unwrap();

        assert_eq!(original_length, serialized_length);

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

    #[test]
    fn transmitter_internal_consistency_v7() {
        let header = PduHeader::new_v7(1, PduType::Transmitter).with_pdu_status(
            PduStatus::default()
                .with_transferred_entity_indicator(TransferredEntityIndicator::NoDifference)
                .with_lvc_indicator(LvcIndicator::NoStatement)
                .with_coupled_extension_indicator(CoupledExtensionIndicator::NotCoupled)
                .with_radio_attached_indicator(RadioAttachedIndicator::NoStatement),
        );

        let body = Transmitter::builder()
            .with_power(45.0)
            .with_frequency(10215)
            .with_transmit_state(TransmitterTransmitState::OnButNotTransmitting)
            .with_antenna_location(Location::new(0.0, 0.0, 0.0))
            .with_relative_antenna_location(VectorF32::default())
            .with_variable_transmitter_parameter(
                VariableTransmitterParameter::default()
                    .with_record_type(VariableRecordType::Age_34100)
                    .with_fields(vec![0xFF, 0xEE]),
            )
            .build()
            .into_pdu_body();
        let original_pdu =
            Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
        let pdu_length = original_pdu.header.pdu_length;

        let mut buf = BytesMut::with_capacity(pdu_length as usize);

        original_pdu.serialize(&mut buf).unwrap();

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
