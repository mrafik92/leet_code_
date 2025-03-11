pub fn find_repeated_and_missing(g: Vec<Vec<i32>>) -> Vec<i32> {
  // bitfield to store the values
  let total_nums = g[0].len() * g[0].len();
  let mut occurence: Vec<u32> = vec![0; total_nums / 32 + 1];
  let mut repeated = 0;
  let mut missing = 0;
  // iterate over the grid

  for i in 0..g.len() {
    for j in 0..g[i].len() {
      let num = g[i][j] as usize;
      let idx = num / 32;
      let bit = num % 32;
      if (occurence[idx] >> bit) & 1 == 1 {
        repeated = num as i32;
      } else {
        occurence[idx] |= 1 << bit;
      }
    }
  }

  let mut i = 0;
  for num in occurence {
    for j in 0..32 {
      if i == 0 && j == 0 {
        continue;
      }
      if (num >> j) & 1 == 0 {
        missing = (i * 32 + j) as i32;
        break;
      }
    }
    if missing != 0 {
      break;
    }
    i += 1;
  }

  vec![repeated, missing]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  pub fn test_find_repeated_and_missing() {
    assert_eq!(
      find_repeated_and_missing(vec![
        vec![1, 18, 33, 26, 20, 30],
        vec![9, 17, 21, 7, 10, 23],
        vec![35, 14, 28, 22, 2, 36],
        vec![4, 12, 8, 15, 11, 19],
        vec![27, 16, 5, 34, 8, 31],
        vec![6, 13, 32, 29, 25, 3]
      ]),
      vec![8, 24]
    );
  }
}
