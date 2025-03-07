use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn check_pwr(mut n: i32) -> bool {
        let mut s = String::new();

        while n > 0 {
            s.push(char::from_digit((n % 3) as u32, 10).unwrap());
            n /= 3;
        }

        s.chars().all(|c| c == '1' || c == '0')
    }

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

    pub fn colored_cells(n: i32) -> i64 {
        match n {
            0 => 0,
            1 => 1,
            2 => 5,
            _ => {
                let sum_ofn_2 = Solution::sum_of_numbers(n - 2);
                let sum_ofn_1 = sum_ofn_2 + (n as i64) - 1i64;
                let sum_ofn = sum_ofn_1 + n as i64;
                sum_ofn + 2 * sum_ofn_1 + sum_ofn_2
            }
        }
    }

    pub fn sum_of_numbers(n: i32) -> i64 {
        n as i64 * (n as i64 + 1) / 2
    }

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


}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_pwr() {
        assert_eq!(Solution::check_pwr(27), true);
        assert_eq!(Solution::check_pwr(9), true);
        assert_eq!(Solution::check_pwr(45), false);
    }

    #[test]
    fn test_merge_arrays() {
        assert_eq!(
            Solution::merge_arrays(vec![vec![1, 2], vec![2, 3]], vec![vec![1, 3], vec![2, 4]]),
            vec![vec![1, 5], vec![2, 7]]
        );
        assert_eq!(
            Solution::merge_arrays(
                vec![vec![1, 2], vec![2, 3]],
                vec![vec![1, 3], vec![2, 4], vec![3, 5]]
            ),
            vec![vec![1, 5], vec![2, 7], vec![3, 5]]
        );
    }

    #[test]
    fn test_colored_cells() {
        assert_eq!(Solution::colored_cells(0), 0);
        assert_eq!(Solution::colored_cells(1), 1);
        assert_eq!(Solution::colored_cells(2), 5);
        assert_eq!(Solution::colored_cells(3), 13);
        assert_eq!(Solution::colored_cells(4), 25);
        assert_eq!(Solution::colored_cells(5), 41);
    }

    #[test]
    fn test_sum_of_numbers() {
        fn sum_of_numbers_test(n: i32) -> i64 {
            let mut sum = 0;
            for n in 0..=n {
                sum += n as i64;
            }
            sum
        }

        for n in 0..=100000 {
            assert_eq!(Solution::sum_of_numbers(n), sum_of_numbers_test(n));
        }
    }

    #[test]
    fn test_find_repeated_and_missing() {
        assert_eq!(
            Solution::find_repeated_and_missing(vec![
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
