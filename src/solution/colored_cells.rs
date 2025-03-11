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
