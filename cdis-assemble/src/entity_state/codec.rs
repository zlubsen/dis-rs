use dis_rs::entity_state::model::{DrParameters, EntityAppearance, EntityMarking};
use dis_rs::enumerations::{EntityCapabilities, EntityMarkingCharacterSet, ForceId};
use crate::codec::Codec;
use crate::entity_state::model::{CdisDRParametersOther, CdisEntityCapabilities, EntityState};
use crate::records::codec::{decode_world_coordinates, encode_world_coordinates};
use crate::records::model::{AngularVelocity, CdisEntityMarking, CdisVariableParameter, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation};
use crate::types::model::{UVINT32, UVINT8};

impl Codec for EntityState {
    type Counterpart = dis_rs::entity_state::model::EntityState;
    const SCALING: f32 = 0.0;

    fn encode(item: &Self::Counterpart) -> Self {
        // Covers full update mode
        let (entity_location, units) = encode_world_coordinates(&item.entity_location);
        let entity_location = Some(entity_location);
        Self {
            units,
            full_update_flag: true,
            entity_id: EntityId::encode(&item.entity_id),
            force_id: Some(UVINT8::from(u8::from(item.force_id))),
            entity_type: Some(EntityType::encode(&item.entity_type)),
            alternate_entity_type: Some(EntityType::encode(&item.alternative_entity_type)),
            entity_linear_velocity: Some(LinearVelocity::encode(&item.entity_linear_velocity)),
            entity_location,
            entity_orientation: Some(Orientation::encode(&item.entity_orientation)),
            entity_appearance: Some((&item.entity_appearance).into()),
            dr_algorithm: item.dead_reckoning_parameters.algorithm,
            dr_params_other: Some(CdisDRParametersOther::from(&item.dead_reckoning_parameters.other_parameters)),
            dr_params_entity_linear_acceleration: Some(LinearAcceleration::encode(&item.dead_reckoning_parameters.linear_acceleration)),
            dr_params_entity_angular_velocity: Some(AngularVelocity::encode(&item.dead_reckoning_parameters.angular_velocity)),
            entity_marking: Some(CdisEntityMarking::new(item.entity_marking.marking_string.clone())),
            capabilities: Some(CdisEntityCapabilities(UVINT32::from(u32::from(item.entity_capabilities)))),
            variable_parameters: item.variable_parameters.iter()
                .map(CdisVariableParameter::encode )
                .collect(),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        // pre-compute, decoded value is needed two times
        let entity_type = self.entity_type.unwrap_or_default().decode();

        // Covers full update mode
        Self::Counterpart::builder()
            .with_entity_id(self.entity_id.decode())
            .with_force_id(ForceId::from(self.force_id.unwrap_or_default().value))
            .with_entity_type(entity_type)
            .with_alternative_entity_type(self.alternate_entity_type.unwrap_or_default().decode())
            .with_velocity(self.entity_linear_velocity.unwrap_or_default().decode())
            .with_location(self.entity_location
                .map(| world_coordinates | decode_world_coordinates(&world_coordinates, self.units) )
                .unwrap_or_default())
            .with_orientation(self.entity_orientation.unwrap_or_default().decode())
            .with_appearance(self.entity_appearance.as_ref()
                .map(|cdis| EntityAppearance::from_bytes(cdis.0, &entity_type))
                .unwrap_or_default())
            .with_dead_reckoning_parameters(DrParameters::default()
                .with_algorithm(self.dr_algorithm)
                .with_parameters(self.dr_params_other.clone().unwrap_or_default().decode(self.dr_algorithm))
                .with_linear_acceleration(self.dr_params_entity_linear_acceleration.unwrap_or_default().decode())
                .with_angular_velocity(self.dr_params_entity_angular_velocity.unwrap_or_default().decode()))
            .with_marking(EntityMarking::new(self.entity_marking.clone().unwrap_or_default().marking, EntityMarkingCharacterSet::ASCII))
            .with_capabilities(EntityCapabilities::from(self.capabilities.clone().unwrap_or_default().0.value))
            .with_variable_parameters(self.variable_parameters.iter()
                .map(|vp| vp.decode() )
                .collect())
            .build()
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::entity_state::model::{EntityMarking, EntityState};
    use dis_rs::enumerations::{Country, EntityKind, EntityMarkingCharacterSet, ForceId, PduType, PlatformDomain};
    use dis_rs::model::{EntityId, EntityType, Pdu, PduBody, PduHeader};
    use crate::{CdisBody, CdisPdu};
    use crate::Codec;
    use crate::records::model::CdisProtocolVersion;

    #[test]
    fn cdis_entity_state_body_encode() {
        let dis_header = PduHeader::new_v7(7, PduType::EntityState);
        let dis_body = EntityState::builder()
            .with_entity_id(EntityId::new(7, 127, 255))
            .with_entity_type(EntityType::default()
                .with_domain(PlatformDomain::Air)
                .with_country(Country::Netherlands_NLD_)
                .with_kind(EntityKind::Platform))
            .with_force_id(ForceId::Friendly8)
            .with_marking(EntityMarking::new("TEST", EntityMarkingCharacterSet::ASCII))
            .build()
            .into_pdu_body();
        let dis_pdu = Pdu::finalize_from_parts(dis_header, dis_body, 1000);

        let cdis_pdu = CdisPdu::encode(&dis_pdu);

        let dis_body = if let PduBody::EntityState(es) = dis_pdu.body {
            es
        } else { assert!(false); dis_rs::entity_state::model::EntityState::default() };
        let cdis_body = if let CdisBody::EntityState(es) = cdis_pdu.body {
            es
        } else { assert!(false); crate::EntityState::default() };

        assert_eq!(dis_pdu.header.exercise_id, cdis_pdu.header.exercise_id.value);
        assert_eq!(dis_pdu.header.pdu_type, cdis_pdu.header.pdu_type);
        assert_eq!(cdis_pdu.header.protocol_version, CdisProtocolVersion::SISO_023_2023);
        assert_eq!(dis_body.force_id, ForceId::from(cdis_body.force_id.unwrap().value));
        assert_eq!(dis_body.entity_id.simulation_address.site_id, cdis_body.entity_id.site.value);
        assert_eq!(dis_body.entity_id.simulation_address.application_id, cdis_body.entity_id.application.value);
        assert_eq!(dis_body.entity_id.entity_id, cdis_body.entity_id.entity.value);
        assert_eq!(dis_body.entity_type.domain, PlatformDomain::from(cdis_body.entity_type.unwrap().domain));
        assert_eq!(dis_body.entity_type.kind, EntityKind::from(cdis_body.entity_type.unwrap().kind));
        assert_eq!(dis_body.entity_type.country, Country::from(cdis_body.entity_type.unwrap().country));
        assert_eq!(dis_body.entity_marking.marking_string, cdis_body.entity_marking.unwrap().marking);
    }

    #[test]
    fn cdis_entity_state_body_decode() {
        assert!(false)
    }
}