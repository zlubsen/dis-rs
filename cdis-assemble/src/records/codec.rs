use dis_rs::enumerations::{Country, EntityKind, PlatformDomain};
use dis_rs::model::{DisTimeStamp, PduHeader, TimeStamp, VectorF32};
use crate::codec::Codec;
use crate::constants::DECIMETERS_TO_METERS;
use crate::records::model::{CdisHeader, CdisProtocolVersion, CdisTimeStamp, EntityId, EntityType, LinearVelocity};
use crate::types::model::{SVINT16, UVINT16, UVINT8};

impl Codec for CdisHeader {
    type Counterpart = PduHeader;

    fn encode(item: Self::Counterpart) -> Self {
        Self {
            protocol_version: CdisProtocolVersion::SISO_023_2023,
            exercise_id: UVINT8::from(item.exercise_id),
            pdu_type: item.pdu_type,
            timestamp: TimeStamp::from(CdisTimeStamp::from(DisTimeStamp::from(item.time_stamp))),
            length: 0,
            pdu_status: if let Some(status) = item.pdu_status { status } else { Default::default() },
        }
    }

    fn decode(&self) -> Self::Counterpart {
        PduHeader::new_v7(self.exercise_id.value, self.pdu_type)
            .with_time_stamp(DisTimeStamp::from(CdisTimeStamp::from(self.timestamp)))
            .with_pdu_status(self.pdu_status)
    }
}

impl Codec for EntityId {
    type Counterpart = dis_rs::model::EntityId;

    fn encode(item: Self::Counterpart) -> Self {
        EntityId::new(
            UVINT16::from(item.simulation_address.site_id),
            UVINT16::from(item.simulation_address.application_id),
            UVINT16::from(item.entity_id)
        )
    }

    fn decode(&self) -> Self::Counterpart {
        dis_rs::model::EntityId::new(
            self.site.value,
            self.application.value,
            self.entity.value
        )
    }
}

impl Codec for EntityType {
    type Counterpart = dis_rs::model::EntityType;

    fn encode(item: Self::Counterpart) -> Self {
        EntityType::new(
            item.kind.into(),
            item.domain.into(),
            item.country.into(),
            UVINT8::from(item.category),
            UVINT8::from(item.subcategory),
            UVINT8::from(item.specific),
            UVINT8::from(item.extra),
        )
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_kind(EntityKind::from(self.kind))
            .with_domain(PlatformDomain::from(self.domain))
            .with_country(Country::from(self.country))
            .with_category(self.category.value)
            .with_subcategory(self.subcategory.value)
            .with_specific(self.specific.value)
            .with_extra(self.extra.value)
    }
}

/// DIS specifies linear velocity in meters/sec.
/// C-DIS specifies linear velocity in decimeters/sec
impl Codec for LinearVelocity {
    type Counterpart = VectorF32;

    fn encode(item: Self::Counterpart) -> Self {
        Self {
            x: SVINT16::from((item.first_vector_component * DECIMETERS_TO_METERS) as i16),
            y: SVINT16::from((item.second_vector_component * DECIMETERS_TO_METERS) as i16),
            z: SVINT16::from((item.third_vector_component * DECIMETERS_TO_METERS) as i16),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_first(self.x.value as f32 / DECIMETERS_TO_METERS)
            .with_second(self.y.value as f32 / DECIMETERS_TO_METERS)
            .with_third(self.z.value as f32 / DECIMETERS_TO_METERS)
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::model::VectorF32;
    use crate::codec::Codec;
    use crate::records::model::LinearVelocity;
    use crate::types::model::SVINT16;

    #[test]
    fn encode_linear_velocity() {
        let dis = VectorF32::new(11.1f32, -22.2f32, 33.3f32);
        let cdis = LinearVelocity::encode(dis);

        assert_eq!(cdis.x.value, 111);
        assert_eq!(cdis.y.value, -222);
        assert_eq!(cdis.z.value, 333);
    }

    #[test]
    fn decode_linear_velocity() {
        let cdis = LinearVelocity::new(
            SVINT16::from(111),
            SVINT16::from(-222),
            SVINT16::from(333));
        let dis = cdis.decode();
        //VectorF32::new(11.1f32, -22.2f32, 33.3f32);

        assert_eq!(dis.first_vector_component, 11.1f32);
        assert_eq!(dis.second_vector_component, -22.2f32);
        assert_eq!(dis.third_vector_component, 33.3f32);
    }
}