pub struct SplitMix64 {
  state: u64,
}

impl SplitMix64 {
  pub fn new(seed: u64) -> Self {
    Self {
      state: seed,
    }
  }

  pub fn next_u64(&mut self) -> u64 {
    self.state = self.state.wrapping_add(0x9e3779b97f4a7c15);

    let mut result = self.state;
    result = (result ^ (result >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    result = (result ^ (result >> 27)).wrapping_mul(0x94d049bb133111eb);
    result ^ (result >> 31)
  }
}