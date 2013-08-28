#[link(name = "geohash",
       vers   = "0.1",
       author = "nullstyle")];
#[crate_type = "lib"];

pub use geohash::Geohash;
pub mod geohash;
