/**
LeetCode Problem 3475: Minimum Operations to Make Binary Array Elements Equal to One I
Link: https://leetcode.com/problems/minimum-operations-to-make-binary-array-elements-equal-to-one-i
Difficulty: Medium
**/

pub fn min_operations(mut nums: Vec<i32>) -> i32 {

  use std::cell::Cell;
  let mut operations = 0;

  let slice = &mut nums[..];
  let slice_of_cells = Cell::from_mut(slice).as_slice_of_cells();
  for w in slice_of_cells.windows(3) {
    if Cell::get(&w[0]) == 0 {
      operations += 1;
      Cell::set(&w[1], 1 - Cell::get(&w[1]));
      Cell::set(&w[2], 1 - Cell::get(&w[2]));
    }
  }

  match nums.iter().rev().take(2).any(|&x| { x == 0 }) {
    true => -1,
    false => operations
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_min_operations() {
    assert_eq!(min_operations(vec![0,1,1,1,0,0]), 3);
    assert_eq!(min_operations(vec![0,1,1,1]), -1);
  }
}