/**
LeetCode Problem 80: Remove Duplicates from Sorted Array II
Link: https://leetcode.com/problems/remove-duplicates-from-sorted-array-ii
Difficulty: Medium
**/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = 0;
  let mut current = 0;

  while right < nums.len() {
    while right < nums.len() && nums[right] == nums[left] {
      right += 1;
    }

    nums[current] = nums[left];
    if right - left > 1 {
      nums[current + 1] = nums[left];
      current += 1;
    }
    current += 1;

    left = right;
    right += 1;
    if nums.len() == right
      && (nums[current] != nums[right - 1] || nums[current] != nums[current - 1])
    {
      nums[current] = nums[right - 1];
      current += 1;
    }
  }

  current as i32
}

pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
  let mut count = 0;
  let mut current_val = nums[0];
  nums.retain(|x| {
    if *x == current_val {
      count += 1;
      count <= 2
    } else {
      current_val = *x;
      count = 1;
      true
    }
  });
  nums.len() as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_remove_duplicates() {
    let mut v = vec![1, 2];
    assert_eq!(remove_duplicates(&mut v), 2);
    assert_eq!(v, vec![1, 2]);
  }

  #[test]
  fn test_remove_duplicates_2() {
    let mut v = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    assert_eq!(remove_duplicates(&mut v), 7);
    assert_eq!(v, vec![0, 0, 1, 1, 2, 3, 3, 3, 3]);
  }
}
