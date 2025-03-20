/**
LeetCode Problem 88: Merge Sorted Array
Link: https://leetcode.com/problems/merge-sorted-array
Difficulty: Easy
**/

pub fn merge(nums1: &mut Vec<i32>, m: u32, nums2: &mut Vec<i32>, n: u32) {
  let mut i = (m as i32 - 1) as i32;
  let mut j = (n as i32 - 1) as i32;
  let mut k = ((m + n) - 1) as i32;

  while i >= 0 && j >= 0 {
    if nums1[i as usize] > nums2[j as usize] {
      nums1[k as usize] = nums1[i as usize];
      i -= 1;
      k -= 1;
    } else {
      nums1[k as usize] = nums2[j as usize];
      j -= 1;
      k -= 1;
    }
  }

  while j >= 0 {
    nums1[k as usize] = nums2[j as usize];
    j -= 1;
    k -= 1;
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_merge() {
    let mut nums1 = vec![1, 2, 3, 0, 0, 0];
    let mut nums2 = vec![2, 5, 6];
    merge(&mut nums1, 3, &mut nums2, 3);
    assert_eq!(nums1, vec![1, 2, 2, 3, 5, 6]);
  }

  #[test]
  fn test_merge_empty_nums2() {
    let mut nums1 = vec![0];
    let mut nums2 = vec![1];
    merge(&mut nums1, 0, &mut nums2, 1);
    assert_eq!(nums1, vec![1]);
  }
}
