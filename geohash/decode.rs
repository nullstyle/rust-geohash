use geohash::base32::decode_char;
use geohash::interval::Interval;
use geohash::interval::contract_interval;
use geohash::Geohash;

//
// constants
// 
static MAX_LAT: f64 = 90.0;
static MAX_LON: f64 = 180.0;

static BIT1: u8 = 1 << 0;
static BIT2: u8 = 1 << 1;
static BIT3: u8 = 1 << 2;
static BIT4: u8 = 1 << 3;
static BIT5: u8 = 1 << 4;


pub fn decode(hash:&str) -> Option<Geohash> {
  let mut lat = Interval(-MAX_LAT, MAX_LAT);
  let mut lon = Interval(-MAX_LON, MAX_LON);
  let mut is_odd = true;
  //TODO: implement an iterator over the packed bits

  for ch in hash.iter() {
    match decode_char(ch) {
      None => return None,
      Some(i) => {
        // work on the lon
        if (is_odd) {
          lon = contract_interval(lon, is_bit_set(i, BIT5));
          lat = contract_interval(lat, is_bit_set(i, BIT4));
          lon = contract_interval(lon, is_bit_set(i, BIT3));
          lat = contract_interval(lat, is_bit_set(i, BIT2));
          lon = contract_interval(lon, is_bit_set(i, BIT1));
        } else {
          lat = contract_interval(lat, is_bit_set(i, BIT5));
          lon = contract_interval(lon, is_bit_set(i, BIT4));
          lat = contract_interval(lat, is_bit_set(i, BIT3));
          lon = contract_interval(lon, is_bit_set(i, BIT2));
          lat = contract_interval(lat, is_bit_set(i, BIT1));
        }
      },
    }
    is_odd = !is_odd;
  };

  Some(Geohash{ lat:lat, lon:lon })
}

fn is_bit_set(byte:u8, position:u8) -> bool {
  (byte & position) > 0
}


#[test]
fn test_decode_good() {
  match Geohash::decode("zz") {
    Some(hash) => {
      assert!( hash.lat.low()  == 84.375 );
      assert!( hash.lat.high() == 90.0 );
      assert!( hash.lon.low()  == 168.75 );
      assert!( hash.lon.high() == 180.0 );
    },
    None => fail!("No geohash returned")
  }


  match Geohash::decode("u4pruydqqvj") {
    Some(hash) => {
      assert!( hash.lat.low()  == 57.649109959602356 );
      assert!( hash.lat.high() == 57.64911130070686 );
      assert!( hash.lon.low()  == 10.407439023256302 );
      assert!( hash.lon.high() == 10.40744036436081 );
    },
    None => fail!("No geohash returned")
  }

  match Geohash::decode("") {
    Some(hash) => {
      assert!( hash.lat.low()  == -MAX_LAT );
      assert!( hash.lat.high() == MAX_LAT );
      assert!( hash.lon.low()  == -MAX_LON );
      assert!( hash.lon.high() == MAX_LON );
    },
    None => fail!("No geohash returned")
  }
}


#[test]
fn test_decode_bad() {
  match Geohash::decode("  ") {
    Some(_) => { fail!("A geohash was returned") },
    None => ()
  }

  match Geohash::decode("zza") {
    Some(_) => { fail!("A geohash was returned") },
    None => ()
  }
}

