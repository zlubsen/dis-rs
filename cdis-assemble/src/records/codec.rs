use dis_rs::enumerations::{Country, EntityKind, PlatformDomain};
use dis_rs::model::{DisTimeStamp, Location, PduHeader, TimeStamp, VectorF32};
use dis_rs::utils::{ecef_to_geodetic_lla, geodetic_lla_to_ecef};
use crate::codec::Codec;
use crate::constants::{ALTITUDE_CM_THRESHOLD, CENTER_OF_EARTH_ALTITUDE, DECIMETERS_IN_METER, RADIANS_SEC_TO_DEGREES_SEC};
use crate::records::model::{AngularVelocity, CdisHeader, CdisProtocolVersion, CdisTimeStamp, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, Units, WorldCoordinates};
use crate::types::model::{SVINT12, SVINT14, SVINT16, SVINT24, UVINT16, UVINT8};

impl Codec for CdisHeader {
    type Counterpart = PduHeader;

    fn encode(item: &Self::Counterpart) -> Self {
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

    fn encode(item: &Self::Counterpart) -> Self {
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

    fn encode(item: &Self::Counterpart) -> Self {
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
    const CONVERSION: f32 = DECIMETERS_IN_METER;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            x: SVINT16::from((item.first_vector_component * Self::CONVERSION) as i16),
            y: SVINT16::from((item.second_vector_component * Self::CONVERSION) as i16),
            z: SVINT16::from((item.third_vector_component * Self::CONVERSION) as i16),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_first(self.x.value as f32 / Self::CONVERSION)
            .with_second(self.y.value as f32 / Self::CONVERSION)
            .with_third(self.z.value as f32 / Self::CONVERSION)
    }
}

/// DIS specifies Euler Angles (Orientation) in radians.
/// CDIS specifies Euler Angles in degrees
///
/// This field shall specify a geocentric orientation using Euler angles as specified in DIS. The values shall be
/// scaled signed integer units up to +-pi (180 degrees). Scale = (2^12 - 1) / pi.
/// Angles shall be reduced to within the +-pi (180 degrees) range before scaling to get accurate values.
impl Codec for Orientation {
    type Counterpart = dis_rs::model::Orientation;
    const SCALING: f32 = 4095f32 / std::f32::consts::PI; // (2^12 - 1) = 4095

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            psi: (normalize_radians_to_plusminus_pi(item.psi) * Self::SCALING) as i16,
            theta: (normalize_radians_to_plusminus_pi(item.theta) * Self::SCALING) as i16,
            phi: (normalize_radians_to_plusminus_pi(item.phi) * Self::SCALING) as i16,
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::new(
            self.psi as f32 / Self::SCALING,
            self.theta as f32 / Self::SCALING,
            self.phi as f32 / Self::SCALING)
    }
}

fn normalize_radians_to_plusminus_pi(radians: f32) -> f32 {
    const TWO_PI: f32 = 2.0 * std::f32::consts::PI;
    let radians = radians % TWO_PI;
    let radians = (radians + TWO_PI) % TWO_PI;
    if radians > std::f32::consts::PI {
        radians - TWO_PI
    } else { radians }
}

/// Encode/Decode a ``VectorF32`` to ``LinearAcceleration``.
/// DIS Lin. Acc. is in meters/sec/sec (as ``VectorF32``).
/// CDIS Lin. Acc. is in decimeters/sec/sec (as ``LinearAcceleration``).
///
/// +8191, -8192 decimeters/sec/sec (Aprox 83.5 g)
impl Codec for LinearAcceleration {
    type Counterpart = VectorF32;
    const CONVERSION: f32 = DECIMETERS_IN_METER;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            x: SVINT14::from((item.first_vector_component * Self::CONVERSION) as i16),
            y: SVINT14::from((item.second_vector_component * Self::CONVERSION) as i16),
            z: SVINT14::from((item.third_vector_component * Self::CONVERSION) as i16),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::new(
            self.x.value as f32 / Self::CONVERSION,
            self.y.value as f32 / Self::CONVERSION,
            self.z.value as f32 / Self::CONVERSION,
        )
    }
}

/// Encode/Decode a ``VectorF32`` to ``AngularVelocity``.
/// DIS Lin. Acc. is in radians/sec.
/// CDIS Lin. Acc. is in degrees/sec.
///
/// +-720 degrees per second max 0.35 degrees/sec resolution
/// Scale = (2^11 - 1) / (4 * pi)
impl Codec for AngularVelocity {
    type Counterpart = VectorF32;
    const SCALING: f32 = (2^11 - 1) as f32 / (4.0 * std::f32::consts::PI);
    const CONVERSION: f32 = RADIANS_SEC_TO_DEGREES_SEC;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            x: SVINT12::from((item.first_vector_component * Self::CONVERSION * Self::SCALING) as i16),
            y: SVINT12::from((item.second_vector_component * Self::CONVERSION * Self::SCALING) as i16),
            z: SVINT12::from((item.third_vector_component * Self::CONVERSION * Self::SCALING) as i16),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::new(
            self.x.value as f32 / Self::SCALING / Self::CONVERSION,
            self.y.value as f32 / Self::SCALING / Self::CONVERSION,
            self.z.value as f32 / Self::SCALING / Self::CONVERSION,
        )
    }
}

/// Encode/decode DIS geocentric (ECEF) ``Location`` to C-DIS geodetic (LLA) ``WorldCoordinates``.
/// DIS ECEF is in meters
/// C-DIS LLA is in radians (lat/lon angles) and centimeters or dekameters depending on the Unit flag
// impl Codec for WorldCoordinates {
//     type Counterpart = dis_rs::model::Location;
//     // TODO account for the scaling of lat
//     // TODO account for the scaling of lon
//     // TODO use of the Units flag - correct calculation of Altitude MSL
//     fn encode(item: &Self::Counterpart) -> Self {
//         let (latitude, longitude, altitude_msl) = ecef_to_geodetic_lla(
//             item.x_coordinate,
//             item.y_coordinate,
//             item.z_coordinate);
//         println!("{} - {} - {}", latitude, longitude, altitude_msl);
//         Self {
//             latitude: latitude as f32,
//             longitude: longitude as f32,
//             altitude_msl: SVINT24::from(altitude_msl as i32),
//         }
//     }
//
//     fn decode(&self) -> Self::Counterpart {
//         let (x, y, z) = geodetic_lla_to_ecef(
//             self.latitude as f64, self.longitude as f64, self.altitude_msl.value as f64);
//         Self::Counterpart::new(x, y, z)
//     }
// }

/// Encode DIS geocentric (ECEF) ``Location`` to C-DIS geodetic (LLA) ``WorldCoordinates``.
/// DIS ECEF is in meters
/// C-DIS LLA is in radians (lat/lon angles) and centimeters or dekameters depending on the Unit flag
pub(crate) fn encode_world_coordinates(ecef_location: &Location) -> (WorldCoordinates, Units) {
    const CENTIMETER_PER_METER: f64 = 100f64;
    const METER_PER_DEKAMETER: f64 = 10f64;

    if ecef_location.x_coordinate == 0.0 &&
        ecef_location.y_coordinate == 0.0 &&
        ecef_location.z_coordinate == 0.0 {
        (WorldCoordinates {
            latitude: 0.0,
            longitude: 0.0,
            altitude_msl: SVINT24::from(CENTER_OF_EARTH_ALTITUDE)
        }, Units::Dekameter)
    } else {
        let (lat, lon, alt_meters) = ecef_to_geodetic_lla(
            ecef_location.x_coordinate, ecef_location.y_coordinate, ecef_location.z_coordinate);

        // Scale: (2^30 - 1) / (PI/2)
        let lat = lat * ((2.0_f64.powi(30) - 1.0) / std::f64::consts::FRAC_PI_2);
        // Scale: (2^31 - 1) / PI
        let lon = lon * ((2.0_f64.powi(31) - 1.0) / std::f64::consts::PI);

        let alt_cm = alt_meters * CENTIMETER_PER_METER;
        let (alt, units) = if (alt_cm) <= ALTITUDE_CM_THRESHOLD {
            (alt_cm, Units::Centimeter)
        } else { (alt_meters / METER_PER_DEKAMETER, Units::Dekameter) };

        let world_coordinates = WorldCoordinates::new(
            lat as f32, lon as f32, SVINT24::from(alt as i32));

        (world_coordinates, units)
    }
}

/// Decode C-DIS geodetic (LLA) ``WorldCoordinates`` to DIS geocentric (ECEF) ``Location``.
/// DIS ECEF is in meters
/// C-DIS LLA is in radians (lat/lon angles) and centimeters or dekameters depending on the Unit flag
pub(crate) fn decode_world_coordinates(lla_location: &WorldCoordinates, units: Units) -> Location {
    const CENTIMETER_PER_METER: f32 = 100f32;
    const METER_PER_DEKAMETER: f32 = 10f32;

    let alt = match units {
        Units::Centimeter => { lla_location.altitude_msl.value as f32 / CENTIMETER_PER_METER }
        Units::Dekameter => { lla_location.altitude_msl.value as f32 * METER_PER_DEKAMETER }
    };

    let lat = lla_location.latitude / ((2.0_f32.powi(30) - 1.0) / std::f32::consts::FRAC_PI_2);
    let lon = lla_location.longitude / ((2.0_f32.powi(31) - 1.0) / std::f32::consts::PI);

    let (x, y, z) = geodetic_lla_to_ecef(lat as f64, lon as f64, alt as f64);
    Location::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use dis_rs::model::{VectorF32};
    use crate::codec::Codec;
    use crate::records::codec::{decode_world_coordinates, encode_world_coordinates, normalize_radians_to_plusminus_pi};
    use crate::types::model::{SVINT12, SVINT14, SVINT16};
    use crate::records::model::{AngularVelocity, LinearAcceleration, LinearVelocity, Orientation};

    #[test]
    fn test_normalize_radians_to_plusminus_pi() {
        assert_eq!(normalize_radians_to_plusminus_pi(std::f32::consts::PI), 3.1415925f32); // approx std::f32::consts::PI
        assert_eq!(normalize_radians_to_plusminus_pi(-std::f32::consts::PI), std::f32::consts::PI);
        assert_eq!(normalize_radians_to_plusminus_pi(0.5 * std::f32::consts::PI), 1.5707965); // approx std::f32::consts::FRAC_PI_2
        assert_eq!(normalize_radians_to_plusminus_pi(3.5f32 * std::f32::consts::PI), -1.570796); // approx -std::f32::consts::FRAC_PI_2
    }


    #[test]
    fn linear_velocity_encode() {
        let dis = VectorF32::new(11.1f32, -22.2f32, 33.3f32);
        let cdis = LinearVelocity::encode(&dis);

        assert_eq!(cdis.x.value, 111);
        assert_eq!(cdis.y.value, -222);
        assert_eq!(cdis.z.value, 333);
    }

    #[test]
    fn linear_velocity_decode() {
        let cdis = LinearVelocity::new(
            SVINT16::from(111),
            SVINT16::from(-222),
            SVINT16::from(333));
        let dis = cdis.decode();

        assert_eq!(dis.first_vector_component, 11.1f32);
        assert_eq!(dis.second_vector_component, -22.2f32);
        assert_eq!(dis.third_vector_component, 33.3f32);
    }

    #[test]
    fn linear_acceleration_encode() {
        let dis = VectorF32::new(1.0, -819.2, 0.0);
        let cdis = LinearAcceleration::encode(&dis);

        assert_eq!(cdis.x.value, 10);
        assert_eq!(cdis.y.value, -8192);
        assert_eq!(cdis.z.value, 0)
    }

    #[test]
    fn linear_acceleration_decode() {
        let cdis = LinearAcceleration::new(
            SVINT14::from(10),
            SVINT14::from(-8192),
            SVINT14::from(0));
        let expected_dis = VectorF32::new(1.0, -819.2, 0.0);
        let dis = cdis.decode();

        assert_eq!(dis, expected_dis);
    }

    #[test]
    fn angular_velocity_encode() {
        const ANGULAR_VELOCITY_SCALE: f32 = (2^11 - 1) as f32 / (4.0 * std::f32::consts::PI);
        let dis = VectorF32::new(1.0, 4.0 * std::f32::consts::PI, -std::f32::consts::PI);
        let cdis = AngularVelocity::encode(&dis);

        assert_eq!(cdis.x.value, (57f32 * ANGULAR_VELOCITY_SCALE) as i16);
        assert_eq!(cdis.y.value, (720f32 * ANGULAR_VELOCITY_SCALE) as i16);
        assert_eq!(cdis.z.value, (-180f32 * ANGULAR_VELOCITY_SCALE) as i16);

        assert!((56.5f32..57.0f32).contains(&(cdis.x.value as f32 / AngularVelocity::SCALING)));
        assert!((719.4f32..720.0f32).contains(&(cdis.y.value as f32 / AngularVelocity::SCALING)));
        assert!((-180.35f32..-179.0f32).contains(&(cdis.z.value as f32 / AngularVelocity::SCALING)));

        let back_to_dis = cdis.decode();
        assert!((0.95f32..1.0f32).contains(&back_to_dis.first_vector_component));
        assert!((12.5f32..12.6f32).contains(&back_to_dis.second_vector_component));
        assert!((-3.14f32..-3.11f32).contains(&back_to_dis.third_vector_component));
    }

    #[test]
    fn angular_velocity_decode() {
        let cdis = AngularVelocity::new(
            SVINT12::from((57f32 * AngularVelocity::SCALING) as i16),
            SVINT12::from((720f32 * AngularVelocity::SCALING) as i16),
            SVINT12::from((-180f32 * AngularVelocity::SCALING) as i16));
        let dis = cdis.decode();

        assert!((0.95f32..1.0f32).contains(&dis.first_vector_component));
        assert!((12.5f32..12.6f32).contains(&dis.second_vector_component));
        assert!((-3.14f32..-3.11f32).contains(&dis.third_vector_component));
    }

    #[test]
    fn entity_location_dis_to_cdis() {
        let dis = dis_rs::model::Location::new(3_919_999.0, 342_955.0, 5_002_819.0);
        // println!("ECEF in - {:?}", dis);
        // let cdis = WorldCoordinates::encode(&dis);
        let (cdis, units) = encode_world_coordinates(&dis);

        // println!("{:?} - {:?} - alt in {:?}", cdis, cdis.latitude.to_degrees(), units);
        assert!(false);
        let dis = decode_world_coordinates(&cdis, units);

        // let cdis = WorldCoordinates::new(90.0, 0.0, SVINT24::from(-6356752));
        // let dis = decode_world_coordinates(cdis, Units::Dekameter);
        // println!("ECEF out - {:?}", dis);
        assert!(false);
    }

    #[test]
    fn entity_location_center_of_earth() {
        assert!(false);
    }

    #[test]
    fn entity_location_alt_in_centimeters() {
        assert!(false);
    }

    #[test]
    fn entity_orientation_dis_to_cdis() {
        let dis = dis_rs::model::Orientation::new(std::f32::consts::PI, 0.0, 0.0);
        let cdis = Orientation::encode(&dis);
        let dis_2 = cdis.decode();

        assert_eq!(cdis.psi, 4094);
        assert_eq!(cdis.theta, 0);

        assert_eq!(dis_2.psi, 3.1408255);
        assert_eq!(dis_2.theta, 0.0);

        let cdis = Orientation::new(1, 4095, -4096);
        let dis = cdis.decode();

        assert_eq!(dis.psi * Orientation::CONVERSION, 0.04395604); // in degrees, the resolution of the field
    }
}