/**
LeetCode Problem 2665: Minimum Time to Repair Cars
Link: https://leetcode.com/problems/minimum-time-to-repair-cars
Difficulty: Medium
**/

pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
  let mut ng = *ranks.iter().min().unwrap() as i64;
  let mut ok = ranks[0] as i64 * cars as i64 * cars as i64;
  let cars = cars as i64;
  while i64::abs(ng - ok) > 1 {
    let mid = (ok + ng) / 2;
    let mut cnt = 0i64;
    let mut pass = false;
    for &rank in &ranks {
      cnt += ((mid / rank as i64) as f64).sqrt() as i64;
      if cnt >= cars {
        pass = true;
        break;
      }
    }
    if pass {
      ok = mid;
    } else {
      ng = mid;
    }
  }
  ok
}

// tests
#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_repair_cars() {
    assert_eq!(repair_cars(vec![4, 2, 3, 1], 10), 16);
    assert_eq!(repair_cars(vec![5, 1, 8], 6), 16);
  }
}
