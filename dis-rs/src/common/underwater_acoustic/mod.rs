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
        PduType, UAPassiveParameterIndex, UAPropulsionPlantConfiguration,
        UAStateChangeUpdateIndicator,
    };
    use crate::model::{EntityId, EventId, SimulationAddress};
    use crate::underwater_acoustic::model::{
        PropulsionPlantConfiguration, Shaft, UABeam, UAEmitterSystem, UAFundamentalParameterData,
        UnderwaterAcoustic, APA,
    };
    use bytes::BytesMut;

    #[test]
    fn underwater_acoustic_internal_consistency() {
        let header = PduHeader::new_v6(1, PduType::UnderwaterAcoustic);

        let body = UnderwaterAcoustic::builder()
            .with_emitting_entity_id(EntityId::new(10, 10, 10))
            .with_event_id(EventId::new(SimulationAddress::new(10, 10), 38))
            .with_state_change_update_indicator(UAStateChangeUpdateIndicator::StateUpdate)
            .with_passive_parameter_index(UAPassiveParameterIndex::Other)
            .with_propulsion_plant_configuration(
                PropulsionPlantConfiguration::default()
                    .with_configuration(UAPropulsionPlantConfiguration::Battery)
                    .with_hull_mounted_masker(false),
            )
            .with_shafts(vec![
                Shaft::default()
                    .with_current_rpm(150)
                    .with_ordered_rpm(100)
                    .with_rpm_rate_of_change(-25),
                Shaft::default()
                    .with_current_rpm(50)
                    .with_ordered_rpm(100)
                    .with_rpm_rate_of_change(25),
            ])
            .with_apa(APA::default())
            .with_emitter_system(
                UAEmitterSystem::default().with_beam(
                    UABeam::default()
                        .with_beam_data_length(24)
                        .with_beam_id_number(1)
                        .with_fundamental_parameters(UAFundamentalParameterData::default()),
                ),
            )
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
}
