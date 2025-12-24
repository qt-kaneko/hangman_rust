use std::time::SystemTime;

pub fn random(max: usize) -> usize {
  if max == 0 { return 0; }

  let nanos = SystemTime::UNIX_EPOCH
    .elapsed()
    .unwrap()
    .as_nanos();
  (nanos % max as u128) as usize
}