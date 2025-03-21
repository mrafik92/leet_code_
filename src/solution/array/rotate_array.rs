/**
LeetCode Problem 189: Rotate Array
Link: https://leetcode.com/problems/rotate-array
Difficulty: Medium
**/

pub fn rotate_v1(nums: &mut Vec<i32>, k: i32) {
  let k = k as usize % nums.len();
  nums.rotate_right(k);
}

pub fn rotate_v2(nums: &mut Vec<i32>, k: i32) {
  let k = k as usize % nums.len();

  for _ in 0..k {
    let last = nums.pop().unwrap();
    nums.insert(0, last);
  }
}

pub fn rotate_v3(nums: &mut Vec<i32>, k: i32) {
  let k = k as usize % nums.len();
  // get a ref split to first k elements of nums
  let (left, right) = nums.split_at(nums.len() - k);
  *nums = [right, left].concat();
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rotate_v1() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_v1(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
  }

  #[test]
  fn test_rotate_v2() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_v2(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
  }

  #[test]
  fn test_rotate_v3() {
    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];
    rotate_v3(&mut nums, 3);
    assert_eq!(nums, vec![5, 6, 7, 1, 2, 3, 4]);
  }
}
