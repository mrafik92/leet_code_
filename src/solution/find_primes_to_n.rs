pub fn find_primes_to_n(g: i32) -> Vec<i32> {
  // Early return for small values
  if g <= 1 {
    return vec![];
  }

  let g_usize = g as usize;
  // using the sieve of eratosthenes
  let mut primes = vec![];
  let mut is_prime = vec![true; g_usize + 1];
  is_prime[0] = false;
  is_prime[1] = false;

  // Only need to check up to sqrt(g)
  let sqrt_g = (g as f64).sqrt() as usize;

  for i in 2..=sqrt_g {
    if is_prime[i] {
      // Mark all multiples as non-prime
      let mut j = i * i;
      while j <= g_usize {
        is_prime[j] = false;
        j += i;
      }
    }
  }

  for i in 2..=g_usize {
    if is_prime[i] {
      primes.push(i as i32);
    }
  }

  primes
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_find_primes_to_n() {
    assert_eq!(find_primes_to_n(10), vec![2, 3, 5, 7]);
    assert_eq!(find_primes_to_n(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
  }
}
