/**
LeetCode Problem 3643: Zero Array Transformation II
Link: https://leetcode.com/problems/zero-array-transformation-ii
Difficulty: Medium
**/

pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
  fn can_from_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>, k: usize) -> bool {
    let n = nums.len();
    let mut sum = 0;
    let mut difference_array = vec![0; n + 1];

    for i in queries.iter().take(k) {
      let (s, e, v) = (i[0], i[1], i[2]);
      difference_array[s as usize] += v;
      difference_array[e as usize + 1usize] -= v;
    }

    for i in 0..n {
      sum += difference_array[i];
      if sum < nums[i] {
        return false;
      }
    }
    true
  }

  if !can_from_zero_array(nums.clone(), queries.clone(), queries.len()) {
    return -1;
  }

  let mut left = 0;
  let mut right = queries.len() as i32;

  while left <= right {
    let mid = ((left + right) / 2) as usize;
    if can_from_zero_array(nums.clone(), queries.clone(), mid) {
      right = (mid - 1) as i32;
    } else {
      left = (mid + 1) as i32;
    }
  }

  left
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_solution() {
    assert_eq!(
      min_zero_array(
        vec![2, 0, 2],
        vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]]
      ),
      2
    );
  }
}
