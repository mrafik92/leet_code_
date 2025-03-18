/**
LeetCode Problem 2478: Longest Nice Subarray
Link: https://leetcode.com/problems/longest-nice-subarray
Difficulty: Medium
**/

pub fn longest_nice_subarray(nums: Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = 0;
  let mut max_len = 1;

  let mut mask = 0;

  while right < nums.len() {
    if (mask & nums[right]) == 0 {
      mask |= nums[right];
      right += 1;
      max_len = max_len.max(right - left);
    } else {
      mask ^= nums[left];
      left += 1;
    }
  }

  max_len as i32
}

pub fn longest_nice_subarray_2(nums: Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = 0;
  let mut max_len = 1;

  while right < nums.len() {
    if nums[right] & nums[left..right].iter().sum::<i32>() == 0 {
      right += 1;
      max_len = max_len.max((right - left) as i32);
    } else {
      left += 1;
    }
  }

  max_len
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_longest_nice_subarray() {
    assert_eq!(longest_nice_subarray(vec![1, 3, 8, 48, 10]), 3);
    assert_eq!(longest_nice_subarray(vec![3, 1, 5, 11, 13]), 1);
  }

  #[test]
  fn test_longest_nice_subarray_2() {
    assert_eq!(longest_nice_subarray_2(vec![1, 3, 8, 48, 10]), 3);
    assert_eq!(longest_nice_subarray_2(vec![3, 1, 5, 11, 13]), 1);
  }

  #[test]
  fn test_longest_nice_subarray_1_2() {
    assert_eq!(
      longest_nice_subarray(vec![
        84139415, 693324769, 614626365, 497710833, 615598711, 264, 65552, 50331652, 1, 1048576,
        16384, 544, 270532608, 151813349, 221976871, 678178917, 845710321, 751376227, 331656525,
        739558112, 267703680
      ]),
      8
    );
  }
}
