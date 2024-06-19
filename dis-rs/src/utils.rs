struct EcefToGeoConstants;

#[allow(clippy::excessive_precision)]
impl EcefToGeoConstants {
    const WGS_84_SEMI_MAJOR_AXIS: f64 = 6378137.0;  //WGS-84 semi-major axis
    const E2: f64 = 6.694_379_990_137_799_7e-3;          // WGS-84 first eccentricity squared
    const A1: f64 = 4.269_767_270_715_753_5e+4;          //a1 = a*e2
    const A2: f64 = 1.823_091_254_607_545_5e+9;          //a2 = a1*a1
    const A3: f64 = 1.429_172_228_981_241_3e+2;          //a3 = a1*e2/2
    const A4: f64 = 4.557_728_136_518_863_7e+9;          //a4 = 2.5*a2
    const A5: f64 = 4.284_058_993_005_565_9e+4;          //a5 = a1+a3
    const A6: f64 = 9.933_056_200_098_622_0e-1;          //a6 = 1-e2
}

/// Applies Geocentric (ECEF) to Geodetic (LLA) conversion
///
/// ECEF input parameters are in meters.
/// Return value of consists of a tuple `(lat, lon, alt)`, where the ``lat`` and ``lon`` are in radians, ``altitude`` is in meters (MSL).
///
/// Adapted from https://danceswithcode.net/engineeringnotes/geodetic_to_ecef/geodetic_to_ecef.html
pub fn ecef_to_geodetic_lla(ecef_x: f64, ecef_y: f64, ecef_z: f64) -> (f64, f64, f64) {
    let zp = ecef_z.abs();
    let w2 = ecef_x * ecef_x + ecef_y * ecef_y;
    let w = w2.sqrt();
    let r2 = w2 + ecef_z * ecef_z;
    let r = r2.sqrt();
    let longitude = ecef_y.atan2(ecef_x);

    let s2 = ecef_z * ecef_z /r2;
    let c2 = w2/r2;
    let u = EcefToGeoConstants::A2/r;
    let v = EcefToGeoConstants::A3 - EcefToGeoConstants::A4/r;
    let (latitude, s, ss, c) = if c2 > 0.3 {
        let s = ( zp/r )*( 1.0 + c2*( EcefToGeoConstants::A1 + u + s2*v )/r );
        let latitude = s.asin();      //Lat
        let ss = s*s;
        let c = (1.0 - ss).sqrt();
        (latitude, s, ss, c)
    } else {
        let c = ( w/r )*( 1.0 - s2*( EcefToGeoConstants::A5 - u - c2*v )/r );
        let latitude = c.acos();      //Lat
        let ss = 1.0 - c*c;
        let s = ss.sqrt();
        (latitude, s, ss, c)
    };

    let g = 1.0 - EcefToGeoConstants::E2*ss;
    let rg = EcefToGeoConstants::WGS_84_SEMI_MAJOR_AXIS/g.sqrt();
    let rf = EcefToGeoConstants::A6*rg;
    let u = w - rg*c;
    let v = zp - rf*s;
    let f = c*u + s*v;
    let m = c*v - s*u;
    let p = m/( rf/g + f );
    let latitude = latitude + p;    //Lat
    let altitude = f + m*p/2.0;     //Altitude
    let latitude = if ecef_z < 0.0 {
        latitude * -1.0                  //Lat
    } else { latitude };

    (latitude, longitude, altitude)
}

/// Applies Geodetic (LLA) to Geocentric (ECEF) conversion
///
/// Geodetic input parameters are in meters.
/// Return value of consists of a tuple `(lat, lon, alt)`, where the ``lat`` and ``lon`` are in _radians_, ``altitude`` is in _meters_ (MSL).
///
/// Adapted from https://danceswithcode.net/engineeringnotes/geodetic_to_ecef/geodetic_to_ecef.html
pub fn geodetic_lla_to_ecef(latitude: f64, longitude: f64, altitude_msl: f64) -> (f64, f64, f64) {
    let n = EcefToGeoConstants::WGS_84_SEMI_MAJOR_AXIS
        / ( 1.0 - EcefToGeoConstants::E2 * latitude.sin() * latitude.sin() ).sqrt();
    let ecef_x = ( n + altitude_msl ) * latitude.cos() * longitude.cos();               //ECEF x
    let ecef_y = ( n + altitude_msl ) * latitude.cos() * longitude.sin();               //ECEF y
    let ecef_z = ( n * ( 1.0 - EcefToGeoConstants::E2 ) + altitude_msl ) * latitude.sin();    //ECEF z

    (ecef_x, ecef_y, ecef_z)
}