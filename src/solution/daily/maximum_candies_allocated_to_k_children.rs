/*
* LeetCode Problem 1335: Maximum Candies Allocated to K Children
* Link: https://leetcode.com/problems/maximum-candies-allocated-to-k-children
* Difficulty: Medium
*/

pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
  fn can_allocate(candies: &[i32], k: i64, mid: i64) -> bool {
    candies.iter().map(|&n| n as i64 / mid).sum::<i64>() >= k
  }

  let mut ok = 0;
  let mut ng = candies.iter().map(|&x| i64::from(x)).sum::<i64>() / k + 1;

  while i64::abs(ok - ng) > 1 {
    let mid = (ok + ng) / 2;
    if can_allocate(&candies, k, mid) {
      ok = mid;
    } else {
      ng = mid;
    }
  }

  ok as i32
}

// multi case test
#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(vec![5, 8, 6], 3, 5)]
  #[test_case(vec![2, 5], 11, 0)]

  fn test_maximum_candies(candies: Vec<i32>, k: i64, expected: i32) {
    assert_eq!(maximum_candies(candies, k), expected);
  }
}
