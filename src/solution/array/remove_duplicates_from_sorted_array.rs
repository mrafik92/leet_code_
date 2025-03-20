/**
LeetCode Problem 26: Remove Duplicates from Sorted Array
Link: https://leetcode.com/problems/remove-duplicates-from-sorted-array
Difficulty: Easy
**/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = 0;
  let mut current = 0;

  println!("{:?}", nums);

  while right < nums.len() {
    while right < nums.len() && nums[right] == nums[left] {
      right += 1;
    }
    right += 1;

    nums[current] = nums[left];
    current += 1;

    left = right;

    if nums.len() == right && nums[current] != nums[right - 1] {
      nums[current] = nums[right - 1];
      current += 1;
    }
  }

  current as i32
}

pub fn remove_duplicates_2(nums: &mut Vec<i32>) -> i32 {
  let mut write_idx = 0;

  for read_idx in 0..nums.len() {
    if nums[read_idx] != nums[write_idx] {
      write_idx += 1;
      nums[write_idx] = nums[read_idx];
    }
  }
  write_idx as i32 + 1
}

pub fn remove_duplicates_3(nums: &mut Vec<i32>) -> i32 {
  let mut current_val = nums[0];
  let mut first = true;
  nums.retain(|x| {
    if *x == current_val {
      if first {
        first = false;
        true
      } else {
        false
      }
    } else {
      current_val = *x;
      true
    }
  });
  println!("{:?}", nums);
  nums.len() as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_remove_duplicates() {
    let mut v = vec![1, 1, 2];
    assert_eq!(remove_duplicates(&mut v), 2);
    assert_eq!(v, vec![1, 2, 2]);
  }

  #[test]
  fn test_remove_duplicates_1() {
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates(&mut v), 5);
    assert_eq!(v, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
  }

  #[test]
  fn test_remove_duplicates_33() {
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates_3(&mut v), 5);
    assert_eq!(v, vec![0, 1, 2, 3, 4]);
  }

  #[test]
  fn test_remove_dupliates_2() {
    let mut v = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
    assert_eq!(remove_duplicates_2(&mut v), 5);
    assert_eq!(v, vec![0, 1, 2, 3, 4, 2, 2, 3, 3, 4]);
  }

  #[test]
  fn test_remove_duplicates_3() {
    let mut v = vec![1, 1, 2];
    assert_eq!(remove_duplicates_2(&mut v), 2);
    assert_eq!(v, vec![1, 2, 2]);
  }
}
