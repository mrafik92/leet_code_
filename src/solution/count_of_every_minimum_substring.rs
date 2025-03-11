pub fn count_of_every_minimum_substring(s: String) -> i32 {
  let mut left = 0;
  let mut map = [0; 3];
  let mut count = 0;
  let len_s = s.len();
  for (r, c) in s.bytes().enumerate() {
    map[(c - b'a') as usize] += 1;
    while map[0] > 0 && map[1] > 0 && map[2] > 0 {
      count += len_s - r;
      map[(s.as_bytes()[left] - b'a') as usize] -= 1;
      left += 1;
    }
  }
  count as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_count_of_every_minimum_substring() {
    assert_eq!(count_of_every_minimum_substring("aababc".to_string()), 4);
  }
}
