pub struct Interval {
  lo: f64,
  hi: f64,
}

pub enum IntervalContraction {
  UpperHalf,
  LowerHalf,
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

  pub fn contract(&mut self, base: IntervalContraction) {
    match base {
      UpperHalf => { self.lo = self.median() },
      LowerHalf => { self.hi = self.median() },
    }
  }
}