/*
 * @lc app=leetcode id=85 lang=rust
 *
 * [85] Maximal Rectangle
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        match matrix.first() {
            None => 0,
            Some(first_row) => {
                let row_count = matrix.len();
                let col_count = first_row.len();
                let mut cache = vec![vec![0; col_count]; row_count];
                for (row_idx, row) in matrix.iter().enumerate() {
                    for (col_idx, &cell) in row.iter().enumerate() {
                        if cell == '1' {
                            cache[row_idx][col_idx] = if row_idx == 0 || col_idx == 0 {
                                1
                            } else {
                                *[
                                    cache[row_idx - 1][col_idx],
                                    cache[row_idx][col_idx - 1],
                                    cache[row_idx - 1][col_idx - 1],
                                ]
                                .iter()
                                .min()
                                .unwrap()
                                    + 1
                            }
                        }
                    }
                }
                println!("cache: {:?}", cache);
                *cache
                    .iter()
                    .map(|row| row.iter().max().unwrap())
                    .max()
                    .unwrap_or(&0)
            }
        }
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1,0,1,0,0],vec![1,0,1,1,1],vec![1,1,1,1,1],vec![1,0,0,1,0]] => 6; "simple")]
    #[test_case(vec![] => 0; "empty")]
    #[test_case(vec![vec![0]] => 0; "zero")]
    #[test_case(vec![vec![1]] => 1; "one")]
    #[test_case(vec![vec![0, 0]] => 0; "one row")]
    fn test(matrix: Vec<Vec<i32>>) -> i32 {
        Solution::maximal_rectangle(
            matrix
                .into_iter()
                .map(|r| {
                    r.into_iter()
                        .map(|c| if c == 1 { '1' } else { '0' })
                        .collect()
                })
                .collect(),
        )
    }
}
