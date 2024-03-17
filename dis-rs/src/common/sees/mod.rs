pub mod model;
pub mod parser;
pub mod writer;
pub mod builder;

#[cfg(test)]
mod tests {
    use bytes::BytesMut;
    use crate::enumerations::{PduType};
    use crate::common::model::{Pdu, PduHeader};
    use crate::common::parser::parse_pdu;
    use crate::common::model::{DisTimeStamp};
    use crate::model::{EntityId};
    use crate::sees::model::{PropulsionSystemData, SEES, VectoringNozzleSystemData};

    #[test]
    fn sees_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::SupplementalEmissionEntityState);

        let body = SEES::builder()
            .with_originating_entity_id(EntityId::new(10,10,10))
            .with_infrared_signature_representation_index(8)
            .with_acoustic_signature_representation_index(4)
            .with_radar_cross_section_representation_index(2)
            .with_propulsion_system(PropulsionSystemData::default().with_power_setting(11.0).with_engine_rpm(12000.0))
            .with_vectoring_nozzle_system(VectoringNozzleSystemData::default().with_horizontal_deflection_angle(32.3).with_vertical_deflection_angle(67.7))
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, body, DisTimeStamp::new_absolute_from_secs(100));
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
