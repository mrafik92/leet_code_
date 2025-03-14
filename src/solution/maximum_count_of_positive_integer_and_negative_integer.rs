/*!
LeetCode Problem 2614: Maximum Count of Positive Integer and Negative Integer
Link: https://leetcode.com/problems/maximum-count-of-positive-integer-and-negative-integer
Difficulty: Easy

Description:
Given an array  nums  sorted in  non-decreasing  order, return  the maximum between the number of positive integers and the number of negative integers.


   In other words, if the number of positive integers in  nums  is  pos  and the number of negative integers is  neg , then return the maximum of  pos  and  neg .


 Note  that  0  is neither positive nor negative.


 Example 1:

 Input:  nums = [-2,-1,-1,1,2,3]
 Output:  3
 Explanation:  There are 3 positive integers and 3 negative integers. The maximum count among them is 3.


 Example 2:

 Input:  nums = [-3,-2,-1,0,0,1,2]
 Output:  3
 Explanation:  There are 2 positive integers and 3 negative integers. The maximum count among them is 3.


 Example 3:

 Input:  nums = [5,20,66,1314]
 Output:  4
 Explanation:  There are 4 positive integers and 0 negative integers. The maximum count among them is 4.



 Constraints:


   1 <= nums.length <= 2000
   -2000 <= nums[i] <= 2000
   nums  is sorted in a  non-decreasing order .



 Follow up:  Can you solve the problem in  O(log(n))  time complexity?


Examples:

**/

pub fn maximum_count(nums: Vec<i32>) -> i32 {
  let pos: i32 = match nums.iter().enumerate().find(|(_i, x)| **x > 0) {
    Some((i, _)) => (nums.len() - i) as i32,
    None => 0,
  };
  let neg: i32 = match nums.iter().enumerate().rfind(|(_i, x)| **x < 0) {
    Some((i, _)) => (i + 1usize) as i32,
    None => -1,
  };
  pos.max(neg)
}

pub fn find_lower_upper_bound(nums: &Vec<i32>) -> (i32, i32) {
  println!("checking for upper bound: {:?}", nums);
  let mut left = 0;
  let mut right: i32 = (nums.len() - 1) as i32;
  let mut upper_bound = -1;
  let mut lower_bound = -1;

  while left <= right {
    let mid = ((left + right) / 2) as usize;
    print!("left: {}, right: {}, mid: {}", left, right, mid);
    if nums[mid] < 0 {
      left = (mid + 1) as i32;
      lower_bound = mid as i32;
      println!(" -> lower_bound: {}", lower_bound);
    } else {
      // nums[mid] >= 0
      right = mid as i32 - 1;
      upper_bound = mid as i32;
      println!(" -> upper_bound: {}", upper_bound);
    }
  }

  (lower_bound, upper_bound)
}

pub fn find_first_positive_traditional(nums: &Vec<i32>) -> i32 {
  let mut left = 0;
  let mut right = (nums.len() - 1) as i32;
  let mut first_positive = -1;
  println!("checking for first positive: {:?}", nums);
  while left <= right {
    print!("left: {}, right: {}", left, right);
    let mid = (left + right) / 2;
    if nums[mid as usize] > 0 {
      right = mid - 1;
      first_positive = mid;
      println!(" -> first_positive: {}", first_positive);
    } else {
      left = mid + 1;
      println!()
    }
  }
  first_positive
}

pub fn find_first_positive_meugel(nums: &Vec<i32>, n: i32) -> i32 {

  if nums.last().unwrap() < &n {
    return -1;
  }

  let mut ok = nums.len() as i32;
  let mut ng = -1;

  while i32::abs(ok - ng) > 1 {
    let mid = (ok + ng) / 2;
    if nums[mid as usize] > n {
      ok = mid;
    } else {
      ng = mid;
    }
  }

  ok
}

pub fn find_first_positive(nums: &Vec<i32>) -> i32 {
  find_first_positive_meugel(nums, 0)
}

pub fn maximum_count_with_binary_search(nums: Vec<i32>) -> i32 {
  let (lower_bound, _upper_bound) = find_lower_upper_bound(&nums);
  let first_positive = find_first_positive(&nums);
  let positive_count = if first_positive == -1 {
    0
  } else {
    nums.len() as i32 - first_positive
  };
  let negative_count = if lower_bound == -1 {
    0
  } else {
    lower_bound + 1
  };
  positive_count.max(negative_count)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_maximum_count() {
    assert_eq!(maximum_count(vec![-2, -1, -1, 1, 2, 3]), 3);
    assert_eq!(maximum_count(vec![-3, -2, -1, 0, 0, 1, 2]), 3);
    assert_eq!(maximum_count(vec![5, 20, 66, 1314]), 4);
  }

  #[test]
  fn test_maximum_count_with_binray() {
    assert_eq!(
      maximum_count_with_binary_search(vec![-2, -1, -1, 1, 2, 3]),
      3
    );
    assert_eq!(
      maximum_count_with_binary_search(vec![-3, -2, -1, 0, 0, 1, 2]),
      3
    );
    assert_eq!(maximum_count_with_binary_search(vec![5, 20, 66, 1314]), 4);
  }

  #[test]
  fn test_maximum_count_with_empty_input() {
    assert_eq!(
      maximum_count(vec![
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0
      ]),
      0
    );
  }

  #[test]
  fn test_find_lower_bound() {
    assert_eq!(find_lower_upper_bound(&vec![-2, -1, -1, 1, 2, 3]), (2, 3));
    assert_eq!(
      find_lower_upper_bound(&vec![-3, -2, -1, 0, 0, 1, 2]),
      (2, 3)
    );
    assert_eq!(find_lower_upper_bound(&vec![5, 20, 66, 1314]), (-1, 0));
    assert_eq!(
      find_lower_upper_bound(&vec![-1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
      (0, 1)
    );
    assert_eq!(
      find_lower_upper_bound(&vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]),
      (-1, 0)
    );
    assert_eq!(find_lower_upper_bound(&vec![-4, -3, -2, -1]), (3, -1));
  }

  #[test]
  fn test_find_first_positive() {
    assert_eq!(find_first_positive(&vec![1, 2, 3]), 0);
    assert_eq!(find_first_positive(&vec![-1, -2, -3]), -1);
    assert_eq!(find_first_positive(&vec![-1, 0, 1]), 2);
    assert_eq!(find_first_positive(&vec![-1, 0,0,0,0,0,0,0,0, 1]), 9);
    assert_eq!(find_first_positive(&vec![-1, 0,0,0,0,0,0,0,0, 1,1,1,1,1,1,2,3,4,5]), 9);
  }

  #[test]
  fn test_find_first_zero() {
    assert_eq!(find_first_positive_meugel(&vec![-1, 0, 1], -1), 1);
    assert_eq!(find_first_positive_meugel(&vec![-1, 0, 0, 0, 0, 1], -1), 1);
    assert_eq!(find_first_positive_meugel(&vec![-3, -2, -1, 0, 0, 0, 0, 1], -1), 3);
    assert_eq!(find_first_positive_meugel(&vec![-5, -4], -1), -1);
    assert_eq!(find_first_positive_meugel(&vec![1, 2, 3], -1), 0);
    assert_eq!(find_first_positive_meugel(&vec![-1, -2, -3], -1), -1);
    assert_eq!(find_first_positive_meugel(&vec![-1, -2, -3, 5], -1), 3);
    assert_eq!(find_first_positive_meugel(&vec![-1, -1, -1, -1, -1, 0, 0, 0, 0, 1], -1), 5);
    assert_eq!(find_first_positive_meugel(&vec![-1, -1, -1, -1, -1, 0, 0, 0, 0, 1], 0), 9);
  }
}
