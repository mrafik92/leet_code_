use super::find_primes_to_n;

pub fn closest_primes(left: i32, right: i32) -> Vec<i32> {
  let primes = find_primes_to_n::find_primes_to_n(right)
    .iter()
    .filter(|&x| *x >= left)
    .copied()
    .collect::<Vec<i32>>();
  let diff = primes
    .windows(2)
    .map(|w| w[1] - w[0])
    .enumerate()
    .min_by_key(|(_, x)| *x)
    .unwrap_or((0usize, -1));

  if diff.1 == -1 {
    return vec![-1, -1];
  }
  vec![primes[diff.0], primes[diff.0 + 1]]
}

// multi case test
#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(10, 19, vec![11, 13]; "primes between 10 and 19")]
  #[test_case(4, 6, vec![-1, -1]; "no primes available between 4 and 6")]
  #[test_case(1, 1000000, vec![2, 3]; "large range returns first two primes")]
  #[test_case(14, 17, vec![-1, -1]; "edge case with primes close but not adjacent in range")]
  #[test_case(100, 1000, vec![101, 103]; "large range with primes close but not adjacent in range")]
  fn test_closest_primes(left: i32, right: i32, expected: Vec<i32>) {
    // measure performance on closes_proimes invocation
    let start = std::time::Instant::now();
    let result = closest_primes(left, right);
    let duration = start.elapsed();
    println!("closest_primes took {} seconds", duration.as_secs_f64());
    assert_eq!(result, expected);
  }
}
//
