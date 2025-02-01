pub mod builder;
pub mod model;
pub mod parser;
pub mod writer;

#[cfg(test)]
mod tests {
    use crate::common::iff::model::{
        FundamentalOperationalData, Iff, IffLayer2, InformationLayers, LayerHeader,
        LayersPresenceApplicability, SystemId,
    };
    use crate::common::model::{EntityId, EventId, Pdu, PduHeader, SimulationAddress};
    use crate::common::parser::parse_pdu;
    use crate::enumerations::{
        ActiveInterrogationIndicator, CoupledExtensionIndicator, IffSimulationMode, IffSystemType,
        LvcIndicator, PduType, TransferredEntityIndicator,
    };
    use crate::v7::model::PduStatus;
    use bytes::BytesMut;

    #[test]
    fn internal_consistency() {
        let header = PduHeader::new_v7(1, PduType::IFF).with_pdu_status(
            PduStatus::default()
                .with_transferred_entity_indicator(TransferredEntityIndicator::NoDifference)
                .with_lvc_indicator(LvcIndicator::NoStatement)
                .with_coupled_extension_indicator(CoupledExtensionIndicator::NotCoupled)
                .with_iff_simulation_mode(IffSimulationMode::Regeneration)
                .with_active_interrogation_indicator(ActiveInterrogationIndicator::NotActive),
        );
        let iff_body = Iff::builder()
            .with_emitting_entity_id(EntityId::new(1, 1, 1))
            .with_event_id(EventId::new(SimulationAddress::new(15, 15), 15))
            .with_fundamental_operational_data(
                FundamentalOperationalData::builder()
                    .with_parameter_1(1)
                    .with_parameter_2(2)
                    .with_parameter_3(3)
                    .with_parameter_4(4)
                    .with_parameter_5(5)
                    .with_parameter_6(6)
                    .with_information_layers(
                        InformationLayers::builder()
                            .with_layer_1(LayersPresenceApplicability::PresentApplicable)
                            .with_layer_2(LayersPresenceApplicability::PresentApplicable)
                            .build(),
                    )
                    .build(),
            )
            .with_system_specific_data(8)
            .with_system_id(
                SystemId::builder()
                    .with_system_type(IffSystemType::Mode5Interrogator)
                    .build(),
            )
            .with_layer_2(
                IffLayer2::builder()
                    .with_header(
                        LayerHeader::builder()
                            .with_layer_number(2)
                            .with_length(8)
                            .build(),
                    )
                    .build(),
            )
            .build()
            .into_pdu_body();
        let original_pdu = Pdu::finalize_from_parts(header, iff_body, 1);
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
                assert!(false);
            }
        }
    }
}
