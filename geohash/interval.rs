pub struct Interval(f64,f64);

impl Interval {
  pub fn median(&self) -> f64 {
    let half = (self.high() - self.low()) / 2.0;
    self.low() + half
  }

  #[inline]
  pub fn low(&self) -> f64 {
    match *self {
      Interval(lo, _) => lo
    }
  }

  #[inline]
  pub fn high(&self) -> f64 {
    match *self {
      Interval(_, hi) => hi
    }
  }

  pub fn upper_half(&self) -> Interval { Interval(self.median(), self.high()) }
  pub fn lower_half(&self) -> Interval { Interval(self.low(), self.median()) }
}

pub fn contract_interval(interval:Interval, contract_hi:bool) -> Interval {
  if (contract_hi) {
    interval.upper_half()
  } else {
    interval.lower_half()
  }
}
