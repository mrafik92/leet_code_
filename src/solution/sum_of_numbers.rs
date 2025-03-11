pub fn sum_of_numbers(n: i32) -> i64 {
  n as i64 * (n as i64 + 1) / 2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_sum_of_numbers() {
    fn sum_of_numbers_test(n: i32) -> i64 {
      let mut sum = 0;
      for n in 0..=n {
        sum += n as i64;
      }
      sum
    }

    for n in 0..=100000 {
      assert_eq!(sum_of_numbers(n), sum_of_numbers_test(n));
    }
  }
}
