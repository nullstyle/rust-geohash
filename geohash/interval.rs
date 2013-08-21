pub struct Interval {
  lo: f64,
  hi: f64,
}

impl Interval {
  pub fn median(&self) -> f64 {
    let half = (self.hi - self.lo) / 2.0;
    self.lo + half
  }

  #[inline]
  pub fn low(&self) -> f64 {
    self.lo
  }

  #[inline]
  pub fn high(&self) -> f64 {
    self.hi
  }

  pub fn upper_half(&self) -> Interval { Interval{ lo:self.median(), hi:self.hi } }
  pub fn lower_half(&self) -> Interval { Interval{ lo:self.lo, hi:self.median() } }
}

pub fn contract_interval(interval:&mut Interval, contract_hi:bool) {
  if (contract_hi) {
    interval.lo = interval.median();
  } else {
    interval.hi = interval.median();
  }
}
