/*
LeetCode Problem 1460: Number of Substrings Containing All Three Characters
Link: https://leetcode.com/problems/number-of-substrings-containing-all-three-characters
Difficulty: Medium

Description:
Given a string s consisting only of characters a, b and c.
Return the number of substrings containing at least one occurrence of all these characters a, b and c.
 
Example 1:

Input: s = "abcabc"
Output: 10
Explanation: The substrings containing at least one occurrence of the characters a, b and c are "abc", "abca", "abcab", "abcabc", "bca", "bcab", "bcabc", "cab", "cabc" and "abc" (again). 

Example 2:

Input: s = "aaacb"
Output: 3
Explanation: The substrings containing at least one occurrence of the characters a, b and c are "aaacb", "aacb" and "acb". 

Example 3:

Input: s = "abc"
Output: 1

 
Constraints:

3 <= s.length <= 5 x 10^4
s only consists of a, b or c characters.

Examples:
"abcabc"
"aaacb"
"abc"
*/

pub fn count_of_every_minimum_substring(s: String) -> i32 {
  let mut left = 0;
  let mut map = [0; 3];
  let mut count = 0;
  let len_s = s.len();
  for (r, c) in s.bytes().enumerate() {
    map[(c - b'a') as usize] += 1;
    while map[0] > 0 && map[1] > 0 && map[2] > 0 {
      count += len_s - r;
      map[(s.as_bytes()[left] - b'a') as usize] -= 1;
      left += 1;
    }
  }
  count as i32
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_count_of_every_minimum_substring() {
    assert_eq!(count_of_every_minimum_substring("aababc".to_string()), 4);
  }
}
