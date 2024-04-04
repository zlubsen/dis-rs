struct EcefToGeoConstants;

impl EcefToGeoConstants {
    const WGS_84_SEMI_MAJOR_AXIS: f32 = 6378137.0;  //WGS-84 semi-major axis
    const E2: f32 = 6.6943799901377997e-3;          // WGS-84 first eccentricity squared
    const A1: f32 = 4.2697672707157535e+4;          //a1 = a*e2
    const A2: f32 = 1.8230912546075455e+9;          //a2 = a1*a1
    const A3: f32 = 1.4291722289812413e+2;          //a3 = a1*e2/2
    const A4: f32 = 4.5577281365188637e+9;          //a4 = 2.5*a2
    const A5: f32 = 4.2840589930055659e+4;          //a5 = a1+a3
    const A6: f32 = 9.9330562000986220e-1;          //a6 = 1-e2
}

/// Applies ECEF to Geo conversion
///
/// Adapted from https://danceswithcode.net/engineeringnotes/geodetic_to_ecef/geodetic_to_ecef.html
pub fn ecef_to_geo_location(ecef_x: f32, ecef_y: f32, ecef_z: f32) -> (f32, f32, f32) {
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

/// Applies Geo to ECEF conversion
///
/// Adapted from https://danceswithcode.net/engineeringnotes/geodetic_to_ecef/geodetic_to_ecef.html
pub fn geo_location_to_ecef(latitude: f32, longitude: f32, altitude_msl: f32) -> (f32, f32, f32) {
    // TODO account for the scaling of lat
    // TODO account for the scaling of lon
    // TODO use of the Units flag - correct calculation of Altitude MSL
    let n = EcefToGeoConstants::WGS_84_SEMI_MAJOR_AXIS
        / ( 1.0 - EcefToGeoConstants::E2 * latitude.sin() * latitude.sin() ).sqrt();
    let ecef_x = ( n + altitude_msl ) * latitude.cos() * longitude.cos();               //ECEF x
    let ecef_y = ( n + altitude_msl ) * latitude.cos() * longitude.sin();               //ECEF y
    let ecef_z = ( n * ( 1.0 - EcefToGeoConstants::E2 ) + altitude_msl ) * latitude;    //ECEF z

    (ecef_x, ecef_y, ecef_z)
}