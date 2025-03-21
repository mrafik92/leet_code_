use std::collections::HashMap;

pub fn merge_arrays(nums1: Vec<Vec<i32>>, nums2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
  let mut x = nums1
    .into_iter()
    .chain(nums2)
    .fold(HashMap::new(), |mut acc, v| {
      match acc.entry(v[0]) {
        std::collections::hash_map::Entry::Occupied(mut e) => {
          *e.get_mut() += v[1];
        }
        std::collections::hash_map::Entry::Vacant(e) => {
          e.insert(v[1]);
        }
      }
      acc
    })
    .iter()
    .map(|(k, v)| vec![*k, *v])
    .collect::<Vec<_>>();

  x.sort_by(|a, b| a[0].cmp(&b[0]));
  x
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_merge_arrays() {
    assert_eq!(
      merge_arrays(vec![vec![1, 2], vec![2, 3]], vec![vec![1, 3], vec![2, 4]]),
      vec![vec![1, 5], vec![2, 7]]
    );
    assert_eq!(
      merge_arrays(
        vec![vec![1, 2], vec![2, 3]],
        vec![vec![1, 3], vec![2, 4], vec![3, 5]]
      ),
      vec![vec![1, 5], vec![2, 7], vec![3, 5]]
    );
  }
}
