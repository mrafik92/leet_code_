/**
LeetCode Problem 169: Majority Element
Link: https://leetcode.com/problems/majority-element
Difficulty: Easy
**/

pub fn majority_element(nums: Vec<i32>) -> i32 {
  *nums
    .iter()
    .fold(std::collections::HashMap::new(), |mut map, x| {
      *map.entry(x).or_insert(0) += 1;
      map
    })
    .into_iter()
    .max_by_key(|&(_, count)| count)
    .unwrap()
    .0
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(&[3, 2, 3].to_vec() => 3; "Test case 1")]
  #[test_case(&[2, 2, 1, 1, 1, 2, 2].to_vec() => 2; "Test case 2")]
  #[test_case(&[1].to_vec() => 1; "Test case 3")]
  #[test_case(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 1, 1, 1].to_vec() => 1; "Test case 4")]
  fn test_majority_element(nums: &Vec<i32>) -> i32 {
    majority_element(nums.to_vec())
  }
}
