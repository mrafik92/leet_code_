/*
LeetCode Problem 2649: Count Total Number of Colored Cells
Link: https://leetcode.com/problems/count-total-number-of-colored-cells/
Difficulty: Medium

Description:
There exists an infinitely large two-dimensional grid of uncolored unit cells. You are given a positive integer n, indicating that you must do the following routine for n minutes:

At the first minute, color any arbitrary unit cell blue.
Every minute thereafter, color blue every uncolored cell that touches a blue cell.

Below is a pictorial representation of the state of the grid after minutes 1, 2, and 3.

Return the number of colored cells at the end of n minutes.
 
Example 1:

Input: n = 1
Output: 1
Explanation: After 1 minute, there is only 1 blue cell, so we return 1.

Example 2:

Input: n = 2
Output: 5
Explanation: After 2 minutes, there are 4 colored cells on the boundary and 1 in the center, so we return 5. 

 
Constraints:

1 <= n <= 105

Examples:
1
2
*/

use super::sum_of_numbers;

pub fn colored_cells(n: i32) -> i64 {
  match n {
    0 => 0,
    1 => 1,
    2 => 5,
    _ => {
      let sum_ofn_2 = sum_of_numbers::sum_of_numbers(n - 2);
      let sum_ofn_1 = sum_ofn_2 + (n as i64) - 1i64;
      let sum_ofn = sum_ofn_1 + n as i64;
      sum_ofn + 2 * sum_ofn_1 + sum_ofn_2
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_colored_cells() {
    assert_eq!(colored_cells(0), 0);
    assert_eq!(colored_cells(1), 1);
    assert_eq!(colored_cells(2), 5);
    assert_eq!(colored_cells(3), 13);
    assert_eq!(colored_cells(4), 25);
    assert_eq!(colored_cells(5), 41);
  }
}
