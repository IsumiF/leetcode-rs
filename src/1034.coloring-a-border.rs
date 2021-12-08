/*
 * @lc app=leetcode id=1034 lang=rust
 *
 * [1034] Coloring A Border
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn color_border(
        mut grid: Vec<Vec<i32>>,
        row_i32: i32,
        col_i32: i32,
        dest_color: i32,
    ) -> Vec<Vec<i32>> {
        let row = row_i32 as usize;
        let col = col_i32 as usize;
        let border: Vec<(usize, usize)> = vec![(row, col)];
        let src_color = grid[row][col];
        let mut visited: Vec<Vec<bool>> = vec![vec![false; grid[0].len()]; grid.len()];
        std::iter::successors(Some(border), |cur_points| {
            if cur_points.is_empty() {
                None
            } else {
                cur_points.iter().for_each(|&(row, col)| {
                    visited[row][col] = true;
                });
                let mut next_points: Vec<(usize, usize)> = cur_points
                    .iter()
                    .flat_map(|(r, c)| {
                        adjacent_non_zero(*r, *c)
                            .filter(|(r, c)| {
                                grid.get(*r)
                                    .and_then(|row_data| row_data.get(*c))
                                    .map(|&x| x == src_color)
                                    .unwrap_or(false)
                                    && !visited[*r][*c]
                            })
                            .map(|(r, c)| (r, c))
                    })
                    .collect();
                next_points.sort();
                next_points.dedup();
                Some(next_points)
            }
        })
        .last();

        let mut should_color: Vec<(usize, usize)> = Vec::new();
        for (r, row) in visited.iter().enumerate() {
            for (c, _) in row.iter().enumerate() {
                if visited[r][c] && Self::is_border(&grid, r, c) {
                    should_color.push((r, c));
                }
            }
        }
        for (r, c) in should_color.into_iter() {
            grid[r][c] = dest_color;
        }
        grid
    }

    fn is_border(grid: &Vec<Vec<i32>>, row: usize, col: usize) -> bool {
        let color = grid[row][col];
        let flags: Vec<bool> = adjacent_non_zero(row, col)
            .map(|(row, col)| {
                grid.get(row)
                    .and_then(|r| r.get(col))
                    .map(|&c| c == color)
                    .unwrap_or(false)
            })
            .collect();
        flags.len() < 4 || !flags.into_iter().all(std::convert::identity)
    }
}

fn adjacent_non_zero(r: usize, c: usize) -> impl Iterator<Item = (usize, usize)> {
    vec![
        (r as i64 - 1, c as i64),
        (r as i64 + 1, c as i64),
        (r as i64, c as i64 - 1),
        (r as i64, c as i64 + 1),
    ]
    .into_iter()
    .filter(|&(r, c)| r >= 0 && c >= 0)
    .map(|(r, c)| (r as usize, c as usize))
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case(vec![vec![1, 1], vec![1, 2]], 0, 0, 3 => vec![vec![3, 3], vec![3, 2]]; "simple")]
    #[test_case(vec![vec![1,1,1],vec![1,1,1],vec![1,1,1]], 1, 1, 2 => vec![vec![2,2,2], vec![2,1,2], vec![2,2,2]]; "inside")]
    fn test(grid: Vec<Vec<i32>>, row: i32, col: i32, color: i32) -> Vec<Vec<i32>> {
        Solution::color_border(grid, row, col, color)
    }
}
