/*!
LeetCode Problem 3643: Zero Array Transformation II
Link: https://leetcode.com/problems/zero-array-transformation-ii
Difficulty: Medium

Description:
You are given an integer array  nums  of length  n  and a 2D array  queries  where  queries[i] = [l i , r i , val i ] .

 Each  queries[i]  represents the following action on  nums :


   Decrement the value at each index in the range  [l i , r i ]  in  nums  by  at most   val i .
   The amount by which each value is decremented  can be chosen  independently  for each index.


 A  Zero Array  is an array with all its elements equal to 0.

 Return the  minimum  possible  non-negative  value of  k , such that after processing the first  k  queries in  sequence ,  nums  becomes a  Zero Array . If no such  k  exists, return -1.


 Example 1:


 Input:   nums = [2,0,2], queries = [[0,2,1],[0,2,1],[1,1,3]]

 Output:   2

 Explanation:


   For i = 0 (l = 0, r = 2, val = 1):


     Decrement values at indices  [0, 1, 2]  by  [1, 0, 1]  respectively.
     The array will become  [1, 0, 1] .


   For i = 1 (l = 0, r = 2, val = 1):

     Decrement values at indices  [0, 1, 2]  by  [1, 0, 1]  respectively.
     The array will become  [0, 0, 0] , which is a Zero Array. Therefore, the minimum value of  k  is 2.





 Example 2:


 Input:   nums = [4,3,2,1], queries = [[1,3,2],[0,2,1]]

 Output:   -1

 Explanation:


   For i = 0 (l = 1, r = 3, val = 2):


     Decrement values at indices  [1, 2, 3]  by  [2, 2, 1]  respectively.
     The array will become  [4, 1, 0, 0] .


   For i = 1 (l = 0, r = 2, val   = 1):

     Decrement values at indices  [0, 1, 2]  by  [1, 1, 0]  respectively.
     The array will become  [3, 0, 0, 0] , which is not a Zero Array.






 Constraints:


   1 <= nums.length <= 10 5
   0 <= nums[i] <= 5 * 10 5
   1 <= queries.length <= 10 5
   queries[i].length == 3
   0 <= l i  <= r i  < nums.length
   1 <= val i  <= 5



Examples:

**/

pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
  fn can_from_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>, k: usize) -> bool {
    let n = nums.len();
    let mut sum = 0;
    let mut difference_array = vec![0; n + 1];

    for i in 0..k {
      let (s, e, v) = (queries[i][0], queries[i][1], queries[i][2]);
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
