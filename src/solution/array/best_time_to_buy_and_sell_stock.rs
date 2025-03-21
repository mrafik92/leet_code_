/**
LeetCode Problem 121: Best Time to Buy and Sell Stock
Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock
Difficulty: Easy
**/

pub fn max_profit(prices: Vec<i32>) -> i32 {
  let mut min_price = prices[0];
  let mut max_profit = 0;

  for &price in &prices[1..] {
    min_price = min_price.min(price);
    max_profit = max_profit.max(price - min_price);
  }

  max_profit
}

#[cfg(test)]
mod tests {
  use super::*;
  use test_case::test_case;

  #[test_case(vec![7, 1, 5, 3, 6, 4], 5)]
  #[test_case(vec![7, 6, 4, 3, 1], 0)]
  #[test_case(vec![1, 2], 1)]
  #[test_case(vec![2, 1], 0)]
  #[test_case(vec![1, 2, 3, 4, 5], 4)]
  #[test_case(vec![5, 4, 3, 2, 1], 0)]
  #[test_case(vec![1, 2, 3, 4, 5, 6], 5)]
  #[test_case(vec![6, 5, 4, 3, 2, 1], 0)]
  #[test_case(vec![1, 2, 3, 4, 5, 6, 7], 6)]
  #[test_case(vec![7, 6, 5, 4, 3, 2, 1], 0)]
  #[test_case(vec![1, 2, 3, 4, 5, 6, 7, 8], 7)]
  #[test_case(vec![8, 7, 6, 5, 4, 3, 2, 1], 0)]
  fn test_max_profit(prices: Vec<i32>, expected: i32) {
    assert_eq!(max_profit(prices), expected);
  }
}
