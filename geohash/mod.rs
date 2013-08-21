use self::interval::Interval;

pub mod base32;
mod interval;
mod decode;
mod encode;
 
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

  pub fn encode(location:(f64,f64), precision:u8) -> ~str {
    self::encode::encode(location, precision)
  }
}