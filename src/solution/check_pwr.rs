pub fn check_pwr(mut n: i32) -> bool {
  let mut s = String::new();

  while n > 0 {
    s.push(char::from_digit((n % 3) as u32, 10).unwrap());
    n /= 3;
  }

  s.chars().all(|c| c == '1' || c == '0')
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_check_pwr() {
    assert!(check_pwr(27));
    assert!(check_pwr(9));
    assert!(!check_pwr(45));
  }
}
