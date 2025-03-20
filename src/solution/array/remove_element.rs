// LeetCode Problem 27: Remove Element
// Link: https://leetcode.com/problems/remove-element
// Difficulty: Easy

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
  // let mut j = 0;
  // let mut occurence = nums.iter().filter(|x| **x == val).count();
  //
  // println!("{:?}", nums);
  //
  // for l in 0..nums.len()-occurence {
  //   if nums[l] == val {
  //     while (nums[j] == val || l == j) && j < nums.len() {
  //       j+=1;
  //       println!("increase j: {}", j);
  //     }
  //     println!("adding {} to {}", j, l);
  //     nums[l] = nums[j];
  //   }
  //   println!("{:?}", nums);
  // }
  //
  // occurence as i32

  nums.retain(|x| *x != val);
  nums.len() as i32
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn test_remove_element() {
    let mut v = vec![0, 1, 2, 2, 3, 0, 4, 2];
    assert_eq!(remove_element(&mut v, 2), 5);
    assert_eq!(v, vec![0, 1, 3, 0, 4]);
  }
}
