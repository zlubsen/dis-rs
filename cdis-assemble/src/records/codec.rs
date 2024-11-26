use num_traits::Signed;
use num_traits::FromPrimitive;
use dis_rs::enumerations::{ChangeIndicator, Country, EntityKind, PlatformDomain};
use dis_rs::model::{ArticulatedPart, AttachedPart, DisTimeStamp, EntityAssociationParameter, EntityTypeParameter, Location, PduHeader, SeparationParameter, TimeStamp, VariableParameter, VectorF32};
use dis_rs::utils::{ecef_to_geodetic_lla, geodetic_lla_to_ecef};
use crate::codec::Codec;
use crate::constants::{ALTITUDE_CM_THRESHOLD, CENTER_OF_EARTH_ALTITUDE, CENTIMETER_PER_METER, DECIMETERS_IN_METER, RADIANS_SEC_TO_DEGREES_SEC};
use crate::records::model::{BeamData, EncodingScheme, LayerHeader, UnitsMeters};
use crate::records::model::{AngularVelocity, CdisArticulatedPartVP, CdisAttachedPartVP, CdisEntityAssociationVP, CdisEntitySeparationVP, CdisEntityTypeVP, CdisHeader, CdisProtocolVersion, CdisTimeStamp, CdisVariableParameter, EntityCoordinateVector, EntityId, EntityType, LinearAcceleration, LinearVelocity, Orientation, ParameterValueFloat, UnitsDekameters, WorldCoordinates};
use crate::types::model::{CdisFloat, SVINT12, SVINT13, SVINT14, SVINT16, SVINT24, UVINT16, UVINT8};

impl Codec for CdisHeader {
    type Counterpart = PduHeader;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            protocol_version: CdisProtocolVersion::SISO_023_2023,
            exercise_id: UVINT8::from(item.exercise_id),
            pdu_type: item.pdu_type,
            timestamp: TimeStamp::from(CdisTimeStamp::from(DisTimeStamp::from(item.time_stamp))),
            length: 0,
            pdu_status: item.pdu_status.unwrap_or_default(),
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

/// Encode a DIS LayerHeader into CDIS LayerHeader, providing the new layer length in CDIS bits.
pub(crate) fn encode_layer_header_with_length(header: &dis_rs::iff::model::LayerHeader, layer_length_bits: u16) -> LayerHeader {
    LayerHeader {
        layer_number: header.layer_number,
        layer_specific_information: header.layer_specific_information,
        length: layer_length_bits,
    }
}

/// Decode a CDIS LayerHeader into DIS LayerHeader, providing the new layer length in DIS bytes.
pub(crate) fn decode_layer_header_with_length(header: &LayerHeader, layer_length_bytes: u16) -> dis_rs::iff::model::LayerHeader {
    dis_rs::iff::model::LayerHeader::builder()
        .with_layer_number(header.layer_number)
        .with_layer_specific_information(header.layer_specific_information)
        .with_length(layer_length_bytes)
        .build()
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
    const SCALING: f32 = ((2^11) - 1) as f32 / (4.0 * std::f32::consts::PI);
    const CONVERSION: f32 = RADIANS_SEC_TO_DEGREES_SEC;

    // FIXME: possibly the rounding is off from the spec, as the example at page 27 of the standard uses +0.5 for positive numbers, and -0.5 for negative numbers
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

impl Codec for EncodingScheme {
    type Counterpart = dis_rs::signal::model::EncodingScheme;

    fn encode(item: &Self::Counterpart) -> Self {
        match item {
            Self::Counterpart::EncodedAudio { encoding_class, encoding_type } => {
                Self::EncodedAudio { encoding_class: *encoding_class, encoding_type: *encoding_type }
            }
            Self::Counterpart::RawBinaryData { encoding_class, nr_of_messages } => {
                Self::RawBinaryData { encoding_class: *encoding_class, nr_of_messages: *nr_of_messages as u8 }
            }
            Self::Counterpart::ApplicationSpecificData { encoding_class, .. } => {
                Self::Unspecified { encoding_class: *encoding_class, encoding_type: 0 }
            }
            Self::Counterpart::DatabaseIndex { encoding_class, .. } => {
                Self::Unspecified { encoding_class: *encoding_class, encoding_type: 0 }
            }
            Self::Counterpart::Unspecified { encoding_class } => {
                Self::Unspecified { encoding_class: *encoding_class, encoding_type: 0 }
            }
        }
    }

    fn decode(&self) -> Self::Counterpart {
        match self {
            EncodingScheme::EncodedAudio { encoding_class, encoding_type } => {
                Self::Counterpart::EncodedAudio { encoding_class: *encoding_class, encoding_type: *encoding_type }
            }
            EncodingScheme::RawBinaryData { encoding_class, nr_of_messages } => {
                Self::Counterpart::RawBinaryData { encoding_class: *encoding_class, nr_of_messages: *nr_of_messages as u16 }
            }
            EncodingScheme::Unspecified { encoding_class, .. } => {
                Self::Counterpart::Unspecified { encoding_class: *encoding_class }
            }
        }
    }
}

/// Encode DIS `VectorF32` representing an Entity Coordinate Vector (DIS 6.2.96a)
/// to C-DIS `EntityCoordinateVector` (11.10).
///
/// The VectorF32 is in meters.
/// The encoded `EntityCoordinateVector` will be in centimeters, or meters when
/// _at least one encoded component value of the vector_ cannot fit in +32 767, -32 768 cm.
/// Values outside of those bounds are placed at the boundary.
///
/// Returns the encoded value and a `UnitsMeters` enum indicating if the value is in _Centimeters_ (default) or _Meters_.
pub(crate) fn encode_entity_coordinate_vector(entity_coordinate_vector: &VectorF32) -> (EntityCoordinateVector, UnitsMeters) {
    let cm_range = f32::from(i16::MIN)..=f32::from(i16::MAX);
    if !cm_range.contains(&entity_coordinate_vector.first_vector_component)
        || !cm_range.contains(&entity_coordinate_vector.second_vector_component)
        || !cm_range.contains(&entity_coordinate_vector.third_vector_component) {
        // at least one vector component is larger than the possible range for centimeters.
        (encode_entity_coordinate_vector_meters(entity_coordinate_vector), UnitsMeters::Meter)
    } else {
        // all vector components can be expressed in centimeters
        (encode_entity_coordinate_vector_centimeters(entity_coordinate_vector), UnitsMeters::Centimeter)
    }
}

pub(crate) fn encode_entity_coordinate_vector_centimeters(entity_coordinate_vector: &VectorF32) -> EntityCoordinateVector {
    EntityCoordinateVector {
        x: SVINT16::from(f32_to_i16_without_overflow(entity_coordinate_vector.first_vector_component * CENTIMETER_PER_METER)),
        y: SVINT16::from(f32_to_i16_without_overflow(entity_coordinate_vector.second_vector_component * CENTIMETER_PER_METER)),
        z: SVINT16::from(f32_to_i16_without_overflow(entity_coordinate_vector.third_vector_component * CENTIMETER_PER_METER)),
    }
}

pub(crate) fn encode_entity_coordinate_vector_meters(entity_coordinate_vector: &VectorF32) -> EntityCoordinateVector {
    EntityCoordinateVector {
        x: SVINT16::from(f32_to_i16_without_overflow(entity_coordinate_vector.first_vector_component)),
        y: SVINT16::from(f32_to_i16_without_overflow(entity_coordinate_vector.second_vector_component)),
        z: SVINT16::from(f32_to_i16_without_overflow(entity_coordinate_vector.third_vector_component)),
    }
}

/// Helper function to convert `f32` values to `i16`, taking the max and min of `i16` when the `f32` value is out of `i16`'s range.
fn f32_to_i16_without_overflow(value: f32) -> i16 {
    i16::from_f32(value).unwrap_or_else(|| if value.is_positive() {i16::MAX} else {i16::MIN})
}

/// Decode C-DIS `EntityCoordinateVector` (11.10) to DIS `VectorF32` representing an Entity Coordinate Vector (DIS 6.2.96a).
/// The units parameter indicates whether the entity coordinate vector's value are in centimeters or meters.
///
/// The resulting `VectorF32` value are in meters.
pub(crate) fn decode_entity_coordinate_vector(entity_coordinate_vector: &EntityCoordinateVector, units: UnitsMeters) -> VectorF32 {
    const CENTIMETER_PER_METER: f32 = 100f32;

    let (x, y, z) = match units {
        UnitsMeters::Centimeter => {
            (f32::from(entity_coordinate_vector.x.value) / CENTIMETER_PER_METER,
             f32::from(entity_coordinate_vector.y.value) / CENTIMETER_PER_METER,
             f32::from(entity_coordinate_vector.z.value) / CENTIMETER_PER_METER)
        }
        UnitsMeters::Meter => {
            (f32::from(entity_coordinate_vector.x.value),
             f32::from(entity_coordinate_vector.y.value),
             f32::from(entity_coordinate_vector.z.value))
        }
    };

    VectorF32::new(x, y, z)
}

/// Encode DIS geocentric (ECEF) ``Location`` to C-DIS geodetic (LLA) ``WorldCoordinates``.
/// DIS ECEF is in meters
/// C-DIS LLA is in radians (lat/lon angles) and centimeters or dekameters depending on the Unit flag
pub(crate) fn encode_world_coordinates(ecef_location: &Location) -> (WorldCoordinates, UnitsDekameters) {
    const CENTIMETER_PER_METER: f64 = 100f64;
    const METER_PER_DEKAMETER: f64 = 10f64;

    if ecef_location.x_coordinate == 0.0 &&
        ecef_location.y_coordinate == 0.0 &&
        ecef_location.z_coordinate == 0.0 {
        (WorldCoordinates {
            latitude: 0.0,
            longitude: 0.0,
            altitude_msl: SVINT24::from(CENTER_OF_EARTH_ALTITUDE)
        }, UnitsDekameters::Dekameter)
    } else {
        let (lat, lon, alt_meters) = ecef_to_geodetic_lla(
            ecef_location.x_coordinate, ecef_location.y_coordinate, ecef_location.z_coordinate);

        // Scale: (2^30 - 1) / (PI/2)
        let lat = lat * ((2.0_f64.powi(30) - 1.0) / std::f64::consts::FRAC_PI_2);
        // Scale: (2^31 - 1) / PI
        let lon = lon * ((2.0_f64.powi(31) - 1.0) / std::f64::consts::PI);

        let alt_cm = alt_meters * CENTIMETER_PER_METER;
        let (alt, units) = if (alt_cm) <= ALTITUDE_CM_THRESHOLD {
            (alt_cm, UnitsDekameters::Centimeter)
        } else { (alt_meters / METER_PER_DEKAMETER, UnitsDekameters::Dekameter) };

        let world_coordinates = WorldCoordinates::new(
            lat as f32, lon as f32, SVINT24::from(alt as i32));

        (world_coordinates, units)
    }
}

/// Decode C-DIS geodetic (LLA) ``WorldCoordinates`` to DIS geocentric (ECEF) ``Location``.
/// DIS ECEF is in meters
/// C-DIS LLA is in radians (lat/lon angles) and centimeters or dekameters depending on the Unit flag
pub(crate) fn decode_world_coordinates(lla_location: &WorldCoordinates, units: UnitsDekameters) -> Location {
    const CENTIMETER_PER_METER: f32 = 100f32;
    const METER_PER_DEKAMETER: f32 = 10f32;

    let alt = match units {
        UnitsDekameters::Centimeter => { lla_location.altitude_msl.value as f32 / CENTIMETER_PER_METER }
        UnitsDekameters::Dekameter => { lla_location.altitude_msl.value as f32 * METER_PER_DEKAMETER }
    };

    let lat = lla_location.latitude / ((2.0_f32.powi(30) - 1.0) / std::f32::consts::FRAC_PI_2);
    let lon = lla_location.longitude / ((2.0_f32.powi(31) - 1.0) / std::f32::consts::PI);

    let (x, y, z) = geodetic_lla_to_ecef(lat as f64, lon as f64, alt as f64);
    Location::new(x, y, z)
}

impl Codec for CdisVariableParameter {
    type Counterpart = VariableParameter;

    fn encode(item: &Self::Counterpart) -> Self {
        match item {
            VariableParameter::Articulated(vp) => {
                CdisVariableParameter::ArticulatedPart(CdisArticulatedPartVP::encode(vp)) }
            VariableParameter::Attached(vp) => {
                CdisVariableParameter::AttachedPart(CdisAttachedPartVP::encode(vp)) }
            VariableParameter::Separation(vp) => {
                CdisVariableParameter::EntitySeparation(CdisEntitySeparationVP::encode(vp)) }
            VariableParameter::EntityType(vp) => {
                CdisVariableParameter::EntityType(CdisEntityTypeVP::encode(vp)) }
            VariableParameter::EntityAssociation(vp) => {
                CdisVariableParameter::EntityAssociation(CdisEntityAssociationVP::encode(vp)) }
            VariableParameter::Unspecified(_, _) => { CdisVariableParameter::Unspecified }
        }
    }

    fn decode(&self) -> Self::Counterpart {
        match self {
            CdisVariableParameter::ArticulatedPart(vp) => { VariableParameter::Articulated(vp.decode()) }
            CdisVariableParameter::AttachedPart(vp) => { VariableParameter::Attached(vp.decode()) }
            CdisVariableParameter::EntitySeparation(vp) => { VariableParameter::Separation(vp.decode()) }
            CdisVariableParameter::EntityType(vp) => { VariableParameter::EntityType(vp.decode()) }
            CdisVariableParameter::EntityAssociation(vp) => { VariableParameter::EntityAssociation(vp.decode()) }
            CdisVariableParameter::Unspecified => { VariableParameter::Unspecified(0u8, [0u8; 15]) }
        }
    }
}

impl Codec for CdisArticulatedPartVP {
    type Counterpart = ArticulatedPart;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            change_indicator: item.change_indicator.into(),
            attachment_id: item.attachment_id,
            type_class: item.type_class,
            type_metric: item.type_metric,
            parameter_value: ParameterValueFloat::from_float(item.parameter_value),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        ArticulatedPart::default()
            .with_change_indicator(ChangeIndicator::from(self.change_indicator))
            .with_attachment_id(self.attachment_id)
            .with_type_class(self.type_class)
            .with_type_metric(self.type_metric)
            .with_parameter_value(self.parameter_value.to_float())
    }
}

impl Codec for CdisAttachedPartVP {
    type Counterpart = AttachedPart;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            detached_indicator: item.detached_indicator,
            attachment_id: item.attachment_id,
            parameter_type: item.parameter_type,
            attached_part_type: EntityType::encode(&item.attached_part_type),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_detached_indicator(self.detached_indicator)
            .with_attachment_id(self.attachment_id)
            .with_parameter_type(self.parameter_type)
            .with_attached_part_type(self.attached_part_type.decode())
    }
}

impl Codec for CdisEntitySeparationVP {
    type Counterpart = SeparationParameter;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            reason_for_separation: item.reason,
            pre_entity_indicator: item.pre_entity_indicator,
            parent_entity_id: EntityId::encode(&item.parent_entity_id),
            station_name: item.station_name,
            station_number: item.station_number,
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_reason(self.reason_for_separation)
            .with_pre_entity_indicator(self.pre_entity_indicator)
            .with_parent_entity_id(self.parent_entity_id.decode())
            .with_station_name(self.station_name)
            .with_station_number(self.station_number)
    }
}

impl Codec for CdisEntityTypeVP {
    type Counterpart = EntityTypeParameter;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            change_indicator: item.change_indicator,
            attached_part_type: EntityType::encode(&item.entity_type),
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_change_indicator(self.change_indicator)
            .with_entity_type(self.attached_part_type.decode())
    }
}

impl Codec for CdisEntityAssociationVP {
    type Counterpart = EntityAssociationParameter;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            change_indicator: item.change_indicator,
            association_status: item.association_status,
            association_type: item.association_type,
            entity_id: EntityId::encode(&item.entity_id),
            own_station_location: item.own_station_location,
            physical_connection_type: item.physical_connection_type,
            group_member_type: item.group_member_type,
            group_number: item.group_number,
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_change_indicator(self.change_indicator)
            .with_association_status(self.association_status)
            .with_association_type(self.association_type)
            .with_entity_id(self.entity_id.decode())
            .with_own_station_location(self.own_station_location)
            .with_physical_connection_type(self.physical_connection_type)
            .with_group_member_type(self.group_member_type)
            .with_group_number(self.group_number)
    }
}

impl Codec for BeamData {
    type Counterpart = dis_rs::model::BeamData;

    const SCALING: f32 = ((2^12) - 1) as f32 / std::f32::consts::PI;
    const SCALING_2: f32 = 1023f32 / 100.0;

    fn encode(item: &Self::Counterpart) -> Self {
        Self {
            az_center: SVINT13::from((item.azimuth_center * Self::SCALING).round() as i16),
            az_sweep: SVINT13::from((item.azimuth_sweep * Self::SCALING).round() as i16),
            el_center: SVINT13::from((item.elevation_center * Self::SCALING).round() as i16),
            el_sweep: SVINT13::from((item.elevation_sweep * Self::SCALING).round() as i16),
            sweep_sync: (item.sweep_sync * Self::SCALING_2).round() as u16,
        }
    }

    fn decode(&self) -> Self::Counterpart {
        Self::Counterpart::default()
            .with_azimuth_center(self.az_center.value as f32 / Self::SCALING)
            .with_azimuth_sweep(self.az_sweep.value as f32 / Self::SCALING)
            .with_elevation_center(self.el_center.value as f32 / Self::SCALING)
            .with_elevation_sweep(self.el_sweep.value as f32 / Self::SCALING)
            .with_sweep_sync(self.sweep_sync as f32 / Self::SCALING_2)
    }
}

#[cfg(test)]
mod tests {
    use dis_rs::enumerations::{PduType, ProtocolVersion};
    use dis_rs::model::{PduHeader, TimeStamp, VectorF32};
    use crate::codec::Codec;
    use crate::records::codec::{decode_world_coordinates, encode_world_coordinates, normalize_radians_to_plusminus_pi};
    use crate::types::model::{SVINT12, SVINT14, SVINT16, SVINT24, UVINT8};
    use crate::records::model::{cdis_to_dis_u32_timestamp, dis_to_cdis_u32_timestamp, AngularVelocity, CdisHeader, CdisProtocolVersion, LinearAcceleration, LinearVelocity, Orientation, UnitsDekameters, WorldCoordinates};

    #[test]
    fn test_normalize_radians_to_plusminus_pi() {
        assert_eq!(normalize_radians_to_plusminus_pi(std::f32::consts::PI), 3.1415925f32); // approx std::f32::consts::PI
        assert_eq!(normalize_radians_to_plusminus_pi(-std::f32::consts::PI), std::f32::consts::PI);
        assert_eq!(normalize_radians_to_plusminus_pi(0.5 * std::f32::consts::PI), 1.5707965); // approx std::f32::consts::FRAC_PI_2
        assert_eq!(normalize_radians_to_plusminus_pi(3.5f32 * std::f32::consts::PI), -1.570796); // approx -std::f32::consts::FRAC_PI_2
    }

    #[test]
    fn cdis_header_encode() {
        let dis = PduHeader::new_v7(7, PduType::EntityState)
            .with_length(140)
            .with_time_stamp(20000u32);
        let cdis = CdisHeader::encode(&dis);

        assert_eq!(dis.protocol_version, ProtocolVersion::IEEE1278_12012);
        assert_eq!(cdis.protocol_version, CdisProtocolVersion::SISO_023_2023);
        assert_eq!(dis.exercise_id, cdis.exercise_id.value);
        assert_eq!(dis.pdu_type, cdis.pdu_type);
        assert_eq!(dis_to_cdis_u32_timestamp(dis.time_stamp), cdis.timestamp.raw_timestamp);
    }

    #[test]
    fn cdis_header_decode() {
        let cdis = CdisHeader {
            protocol_version: CdisProtocolVersion::SISO_023_2023,
            exercise_id: UVINT8::from(5),
            pdu_type: PduType::Acknowledge,
            timestamp: TimeStamp::from(20000),
            length: 140,
            pdu_status: Default::default(),
        };
        let dis = cdis.decode();

        assert_eq!(dis.protocol_version, ProtocolVersion::IEEE1278_12012);
        assert_eq!(dis.exercise_id, 5);
        assert_eq!(dis.pdu_type, PduType::Acknowledge);
        assert_eq!(dis.time_stamp, cdis_to_dis_u32_timestamp(20000));
        assert!(dis.pdu_status.is_some());
        assert!(dis.pdu_status.unwrap().fire_type_indicator.is_none())
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
    fn entity_location_encode() {
        // Latitude  : 52   deg N
        // Longitude : 05   deg E
        // Height    : 20   m
        // === Equals ===
        // X : 3919999  m
        // Y : 342955   m
        // Z : 5002819  m
        // As per https://www.oc.nps.edu/oc2902w/coord/llhxyz.htm
        let dis = dis_rs::model::Location::new(3_919_999.0, 342_955.0, 5_002_819.0);
        let (cdis, units) = encode_world_coordinates(&dis);

        assert_eq!(units, UnitsDekameters::Centimeter);
        assert_eq!(cdis.latitude, 620384200f32);    // scaled value
        assert_eq!((cdis.latitude / ((2.0_f32.powi(30) - 1.0) / std::f32::consts::FRAC_PI_2)).to_degrees().round(), 52.0); // unscaled and in degrees
        assert_eq!(cdis.longitude, 59652240f32);    // scaled value
        assert_eq!(cdis.altitude_msl.value, 1987);  // ~ 19.87 meters
    }

    #[test]
    fn entity_location_decode() {
        let cdis = WorldCoordinates::new(620384200f32, 59652240f32, SVINT24::from(1987));
        let units = UnitsDekameters::Centimeter;
        let dis = decode_world_coordinates(&cdis, units);

        assert_eq!(dis.x_coordinate.round(), 3_919_998.0);
        assert_eq!(dis.y_coordinate.round(), 342_955.0);
        assert_eq!(dis.z_coordinate.round(), 5_002_819.0);
    }

    #[test]
    fn entity_location_center_of_earth() {
        let dis = dis_rs::model::Location::new(0.0, 0.0, 0.0);
        let (cdis, units) = encode_world_coordinates(&dis);

        assert_eq!(cdis.latitude, 0.0);
        assert_eq!(cdis.longitude, 0.0);
        assert_eq!(cdis.altitude_msl.value, -8_388_608);
        assert_eq!(units, UnitsDekameters::Dekameter);
    }

    #[test]
    fn entity_location_encode_and_decode_non_null() {
        let dis_in = dis_rs::model::Location::new(3_919_999.0, 342_955.0, 5_002_819.0);
        let (cdis, units) = encode_world_coordinates(&dis_in);
        let dis_out = decode_world_coordinates(&cdis, units);

        assert_eq!(dis_in.x_coordinate, dis_out.x_coordinate.ceil());
        assert_eq!(dis_in.y_coordinate, dis_out.y_coordinate.round());
        assert_eq!(dis_in.z_coordinate, dis_out.z_coordinate.round());
    }

    #[test]
    fn entity_location_encode_and_decode_null() {
        let dis_in = dis_rs::model::Location::new(0.0, 0.0, 5_002_819.0);
        let (cdis, units) = encode_world_coordinates(&dis_in);
        let dis_out = decode_world_coordinates(&cdis, units);

        assert_eq!(dis_in.x_coordinate, dis_out.x_coordinate.ceil());
        assert_eq!(dis_in.y_coordinate, dis_out.y_coordinate.round());
        assert_eq!(dis_in.z_coordinate, dis_out.z_coordinate.round());
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

        assert_eq!(dis.psi.to_degrees(), 0.04395604); // in degrees, the resolution of the field
    }
}
