use dis_rs::enumerations::{Country, EntityKind, PlatformDomain};
use dis_rs::model::{DisTimeStamp, PduHeader, TimeStamp, VectorF32};
use crate::codec::Codec;
use crate::constants::{METERS_TO_DECIMETERS, RADIANS_SEC_TO_DEGREES_SEC};
use crate::records::model::{AngularVelocity, CdisHeader, CdisProtocolVersion, CdisTimeStamp, EntityId, EntityType, LinearAcceleration, LinearVelocity};
use crate::types::model::{SVINT12, SVINT14, SVINT16, UVINT16, UVINT8};

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
            x: SVINT16::from((item.first_vector_component * METERS_TO_DECIMETERS) as i16),
            y: SVINT16::from((item.second_vector_component * METERS_TO_DECIMETERS) as i16),
            z: SVINT16::from((item.third_vector_component * METERS_TO_DECIMETERS) as i16),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_first(self.x.value as f32 / METERS_TO_DECIMETERS)
            .with_second(self.y.value as f32 / METERS_TO_DECIMETERS)
            .with_third(self.z.value as f32 / METERS_TO_DECIMETERS)
    }
}

impl From<VectorF32> for LinearAcceleration {
    /// Convert a ``VectorF32`` to ``LinearAcceleration``.
    /// DIS Lin. Acc. is in meters/sec/sec.
    /// CDIS Lin. Acc. is in decimeters/sec/sec
    ///
    /// +8191, -8192 decimeters/sec/sec (Aprox 83.5 g)
    fn from(value: VectorF32) -> Self {
        Self {
            x: SVINT14::from((value.first_vector_component * METERS_TO_DECIMETERS) as i16),
            y: SVINT14::from((value.second_vector_component * METERS_TO_DECIMETERS) as i16),
            z: SVINT14::from((value.third_vector_component * METERS_TO_DECIMETERS) as i16),
        }
    }
}

impl From<LinearAcceleration> for VectorF32 {
    /// Convert a ``LinearAcceleration`` to ``VectorF32``.
    /// DIS Lin. Acc. is in meters/sec/sec.
    /// CDIS Lin. Acc. is in decimeters/sec/sec
    ///
    /// +8191, -8192 decimeters/sec/sec (Aprox 83.5 g)
    fn from(value: LinearAcceleration) -> Self {
        Self::new(
            value.x.value as f32 / METERS_TO_DECIMETERS,
            value.y.value as f32 / METERS_TO_DECIMETERS,
            value.z.value as f32 / METERS_TO_DECIMETERS,
        )
    }
}

impl From<VectorF32> for AngularVelocity {
    /// Convert a ``VectorF32`` to ``AngularVelocity``.
    /// DIS Lin. Acc. is in radians/sec.
    /// CDIS Lin. Acc. is in degrees/sec.
    ///
    /// +-720 degrees per second max 0.35 degrees/sec resolution
    /// Scale = (2^11 - 1) / (4 * pi)
    fn from(value: VectorF32) -> Self {
        Self {
            x: SVINT12::from((value.first_vector_component * RADIANS_SEC_TO_DEGREES_SEC * Self::SCALING) as i16),
            y: SVINT12::from((value.second_vector_component * RADIANS_SEC_TO_DEGREES_SEC * Self::SCALING) as i16),
            z: SVINT12::from((value.third_vector_component * RADIANS_SEC_TO_DEGREES_SEC * Self::SCALING) as i16),
        }
    }
}

impl From<AngularVelocity> for VectorF32 {
    /// Convert an ``AngularVelocity`` to ``VectorF32``.
    /// DIS Lin. Acc. is in radians/sec.
    /// CDIS Lin. Acc. is in degrees/sec.
    ///
    /// +-720 degrees per second max 0.35 degrees/sec resolution
    /// Scale = (2^11 - 1) / (4 * pi)
    fn from(value: AngularVelocity) -> Self {
        VectorF32::new(
            value.x.value as f32 / AngularVelocity::SCALING / RADIANS_SEC_TO_DEGREES_SEC,
            value.y.value as f32 / AngularVelocity::SCALING / RADIANS_SEC_TO_DEGREES_SEC,
            value.z.value as f32 / AngularVelocity::SCALING / RADIANS_SEC_TO_DEGREES_SEC,
        )
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::model::VectorF32;
    use crate::codec::Codec;
    use crate::types::model::SVINT16;
    use crate::records::model::{AngularVelocity, LinearAcceleration, LinearVelocity};

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

    #[test]
    fn linear_acceleration_dis_to_cdis() {
        let dis = VectorF32::new(1.0, -819.2, 0.0);
        let cdis = LinearAcceleration::from(dis);

        assert_eq!(cdis.x.value, 10);
        assert_eq!(cdis.y.value, -8192);
        assert_eq!(cdis.z.value, 0)
    }

    #[test]
    fn angular_velocity_dis_to_cdis() {
        const ANGULAR_VELOCITY_SCALE: f32 = (2^11 - 1) as f32 / (4.0 * std::f32::consts::PI);
        let dis = VectorF32::new(1.0, 4.0 * std::f32::consts::PI, -std::f32::consts::PI);
        let cdis = AngularVelocity::from(dis);

        assert_eq!(cdis.x.value, (57f32 * ANGULAR_VELOCITY_SCALE) as i16);
        assert_eq!(cdis.y.value, (720f32 * ANGULAR_VELOCITY_SCALE) as i16);
        assert_eq!(cdis.z.value, (-180f32 * ANGULAR_VELOCITY_SCALE) as i16);

        assert!((56.5f32..57.0f32).contains(&cdis.x_scaled()));
        assert!((719.4f32..720.0f32).contains(&cdis.y_scaled()));
        assert!((-180.35f32..-179.0f32).contains(&cdis.z_scaled()));

        let back_to_dis = VectorF32::from(cdis);
        assert!((0.95f32..1.0f32).contains(&back_to_dis.first_vector_component));
        assert!((12.5f32..12.6f32).contains(&back_to_dis.second_vector_component));
        assert!((-3.14f32..-3.11f32).contains(&back_to_dis.third_vector_component));
    }
}