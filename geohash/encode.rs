use geohash::interval::*;
use geohash::*;
use std::str;


pub fn encode(lat:f64, lon:f64, precision:uint) -> ~str {
  let mut ilat = Interval{ lo:-MAX_LAT, hi:MAX_LAT };
  let mut ilon = Interval{ lo:-MAX_LON, hi:MAX_LON };

  let bit_count : uint    = 5 * precision;
  let mut bits: ~[bool]   = ~[];
  let mut result: ~[char] = ~[];

  for i in range(0, bit_count) {
    if i.is_even() {
      let is_higher = lon > ilon.median();
      bits.push(is_higher);
      ilon.contract(if is_higher {UpperHalf} else {LowerHalf});
    } else {
      let is_higher = lat > ilat.median();
      bits.push(is_higher);
      ilat.contract(if is_higher {UpperHalf} else {LowerHalf});
    }
  }

  for char_bits in bits.chunk_iter(5) {
    let result_char = base32::encode_vec(char_bits).unwrap();
    result.push(result_char);
  }

  str::from_chars(result)
}

#[test]
fn test_encode_good() {
  let lat = 57.649109959602356;
  let lon = 10.407439023256302;

  assert!( Geohash::encode(lat, lon, 2)  == ~"u4" );
  assert!( Geohash::encode(lat, lon, 4)  == ~"u4pr" );
  assert!( Geohash::encode(lat, lon, 6)  == ~"u4pruy" );
  assert!( Geohash::encode(lat, lon, 8)  == ~"u4pruydq" );
}