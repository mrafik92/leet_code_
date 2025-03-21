use std::collections::HashMap;

pub fn count_of_constrained_substrings(chars: &[char], k: i32) -> i64 {
  let mut left = 0;
  let mut right = 0;
  let mut constants = 0;
  let mut substrings = 0;

  let mut vowels_map = HashMap::new();

  while right < chars.len() {
    if "aeiou".contains(chars[right]) {
      vowels_map.insert(
        chars[right],
        1 + vowels_map.get(&chars[right]).unwrap_or(&0),
      );
    } else {
      constants += 1;
    }

    while vowels_map.len() == 5 && constants >= k {
      substrings += (chars.len() - right) as i64;

      if "aeiou".contains(chars[left]) {
        let count = vowels_map.get_mut(&chars[left]).unwrap();
        *count -= 1;
        if *count == 0 {
          vowels_map.remove(&chars[left]);
        }
      } else {
        constants -= 1;
      }

      left += 1;
    }
    right += 1;
  }

  substrings
}

pub fn count_of_super_constrained(word: String, k: i32) -> i64 {
  // sliding window with at most k constants - at most k + 1 constants
  let chars = word.chars().collect::<Vec<char>>();
  count_of_constrained_substrings(&chars, k) - count_of_constrained_substrings(&chars, k + 1)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_count_of_constrained_substrings() {
    assert_eq!(count_of_super_constrained("aoaiuefi".to_string(), 1), 4);

    assert_eq!(count_of_super_constrained("aadieuoh".to_string(), 1), 2);

    assert_eq!(count_of_super_constrained("iqeaouqi".to_string(), 2), 3);

    assert_eq!(
      count_of_super_constrained("ieaouqqieaouqq".to_string(), 1),
      3
    );
    assert_eq!(count_of_super_constrained("buoeia".to_string(), 0), 1);
  }
}
