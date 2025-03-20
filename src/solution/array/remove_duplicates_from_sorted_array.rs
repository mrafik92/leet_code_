/**
LeetCode Problem 26: Remove Duplicates from Sorted Array
Link: https://leetcode.com/problems/remove-duplicates-from-sorted-array
Difficulty: Easy
**/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = 0;
  let mut unique = 0;
  let mut current = 0;

  println!("{:?}", nums);

  while right < nums.len() {
    while nums[right] == nums[left] {
      right +=1;
    }
    println!("update {} to {}", left, current);
    nums[current] = nums[left];
    current+=1;
    unique+=1;
    left = right;
    println!("right: {}", right);

    right += 1;

    if nums.len() == right && nums[current] != nums[right - 1] {
      nums[current] = nums[right - 1];
      unique += 1;
    }

    println!("{:?}", nums);
  }

  unique
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_remove_dupliates() {
    let mut v = vec![1,1,2];
    assert_eq!(remove_duplicates(&mut v), 2);
    assert_eq!(v, vec![1,2,2]);
  }

  #[test]
  fn test_remove_dupliates_1() {
    let mut v = vec![0,0,1,1,1,2,2,3,3,4];
    assert_eq!(remove_duplicates(&mut v), 5);
    assert_eq!(v, vec![0, 1, 2, 3, 4,  2, 2, 3, 3, 4]);
  }

  #[test]
  fn test_remove_dupliates_1() {
    let mut v = vec![0,0,1,1,1,2,2,3,3,4];
    assert_eq!(remove_duplicates(&mut v), 5);
    assert_eq!(v, vec![0, 1, 2, 3, 4,  2, 2, 3, 3, 4]);
  }
}