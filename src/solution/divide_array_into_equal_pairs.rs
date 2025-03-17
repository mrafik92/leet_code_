/**
LeetCode Problem 2308: Divide Array Into Equal Pairs
Link: https://leetcode.com/problems/divide-array-into-equal-pairs
Difficulty: Easy
**/

pub fn divide_array(nums: Vec<i32>) -> bool {
  nums
    .iter()
    .fold(std::collections::HashMap::new(), |mut map, x| {
      *map.entry(x).or_insert(0) += 1;
      map
    })
    .into_values()
    .all(|x| x % 2 == 0)
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_divide_array() {
    assert_eq!(divide_array(vec![3, 2, 3, 2, 2, 2]), true);
    assert_eq!(divide_array(vec![1, 2, 3, 4]), false);
  }
}
