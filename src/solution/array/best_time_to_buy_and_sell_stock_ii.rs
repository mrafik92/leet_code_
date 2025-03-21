/**
LeetCode Problem 122: Best Time to Buy and Sell Stock II
Link: https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii
Difficulty: Medium
**/

pub fn max_profit(prices: Vec<i32>) -> i32 {
  let mut total_profit = 0;

  prices.windows(2).for_each(|window| {
    total_profit += i32::max(0, window[1] - window[0]);
  });

  total_profit
}