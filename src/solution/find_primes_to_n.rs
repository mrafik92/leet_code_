pub fn find_primes_to_n(g: i32) -> Vec<i32> {
  // Early return for small values
  if g <= 1 {
    return vec![];
  }

  let g_usize = g as usize;
  // using the sieve of eratosthenes
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

  is_prime
    .iter()
    .enumerate()
    .skip(2)
    .filter_map(|(i, &b)| if b { Some(i as i32) } else { None })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_find_primes_to_n_10() {
    assert_eq!(find_primes_to_n(10), vec![2, 3, 5, 7]);
  }

  #[test]
  pub fn test_find_primes_to_n_20() {
    assert_eq!(find_primes_to_n(20), vec![2, 3, 5, 7, 11, 13, 17, 19]);
  }

  #[test]
  pub fn test_find_primes_to_n_30() {
    assert_eq!(
      find_primes_to_n(30),
      vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]
    );
  }

  #[test]
  pub fn test_find_primes_to_n_100() {
    assert_eq!(
      find_primes_to_n(100),
      vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97
      ]
    );
  }

  #[test]
  pub fn test_find_primes_to_n_1000() {
    assert_eq!(
      find_primes_to_n(1000),
      vec![
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97, 101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181,
        191, 193, 197, 199, 211, 223, 227, 229, 233, 239, 241, 251, 257, 263, 269, 271, 277, 281,
        283, 293, 307, 311, 313, 317, 331, 337, 347, 349, 353, 359, 367, 373, 379, 383, 389, 397,
        401, 409, 419, 421, 431, 433, 439, 443, 449, 457, 461, 463, 467, 479, 487, 491, 499, 503,
        509, 521, 523, 541, 547, 557, 563, 569, 571, 577, 587, 593, 599, 601, 607, 613, 617, 619,
        631, 641, 643, 647, 653, 659, 661, 673, 677, 683, 691, 701, 709, 719, 727, 733, 739, 743,
        751, 757, 761, 769, 773, 787, 797, 809, 811, 821, 823, 827, 829, 839, 853, 857, 859, 863,
        877, 881, 883, 887, 907, 911, 919, 929, 937, 941, 947, 953, 967, 971, 977, 983, 991, 997
      ]
    )
  }
}
