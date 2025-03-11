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

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_closest_primes() {
    // assert_eq!(Solution::closest_primes(2, 10), vec![2, 3]);
    // assert_eq!(Solution::closest_primes(8, 20), vec![11, 13]);
    // assert_eq!(Solution::closest_primes(11, 20), vec![11, 13]);
    assert_eq!(closest_primes(12854, 130337), vec![12917, 12919]);
    assert_eq!(closest_primes(1, 1000000), vec![2, 3]);
    assert_eq!(closest_primes(1000, 5000), vec![1019, 1021]);
    assert_eq!(closest_primes(20000, 30000), vec![20021, 20023]);
    assert_eq!(closest_primes(100000, 200000), vec![100151, 100153]);
    assert_eq!(closest_primes(999000, 1000000), vec![999329, 999331]);
  }
}
