use self::interval::Interval;

pub mod base32;
pub mod interval;
pub mod decode;
pub mod encode;
 
pub struct Geohash {
  lat: Interval,
  lon: Interval
}

impl Geohash {
  pub fn lat(&self) -> f64 { self.lat.median() }
  pub fn lon(&self) -> f64 { self.lon.median() }

  pub fn decode(hash:&str) -> Option<Geohash> {
    self::decode::decode(hash)
  }

  pub fn encode(lat:f64, lon:f64, precision:uint) -> ~str {
    self::encode::encode(lat, lon, precision)
  }
}


pub static MAX_LAT: f64 = 90.0;
pub static MAX_LON: f64 = 180.0;