use geohash::interval::*;
use geohash::*;

pub fn decode(hash:&str) -> Option<Geohash> {
  let mut lat      = Interval{ lo:-MAX_LAT, hi:MAX_LAT };
  let mut lon      = Interval{ lo:-MAX_LON, hi:MAX_LON };

  let mut bits : ~[bool] = ~[];
  // TODO: implement an iterator over the packed bits, which would
  // return a stream of bools. should simplify the algorthm when I learn 
  // how to do write it :)
  //
  for ch in hash.iter() {
    match base32::decode_vec(ch) {
      None => return None,
      Some(char_bits) => { bits.push_all(char_bits) },
    }
  };

  let ops = bits.map(|&bit| if bit {UpperHalf} else {LowerHalf} );

  for (i, &op) in ops.iter().enumerate() {
    let comp = if i.is_even() {&mut lon} else {&mut lat};
    comp.contract(op);
  }

  Some(Geohash{ lat:lat, lon:lon })
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

