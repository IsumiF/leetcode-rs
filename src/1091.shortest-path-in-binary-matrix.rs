/*
 * @lc app=leetcode id=1091 lang=rust
 *
 * [1091] Shortest Path in Binary Matrix
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() {
            return 0;
        }
        if grid[0][0] == 1 {
            return -1;
        }
        let rowc = grid.len();
        let colc = grid[0].len();
        let mut dist_mat: Vec<Vec<i32>> = vec![vec![-1; colc]; rowc];
        dist_mat[0][0] = 1;
        let mut edge_points: Vec<(usize, usize)> = vec![(0, 0)];
        loop {
            if edge_points.is_empty() {
                break;
            }
            let mut next_edge_points = Vec::<(usize, usize)>::new();
            for (row, col) in edge_points {
                for (r, c) in neighbors(row, col, rowc, colc) {
                    if dist_mat[r][c] == -1 && grid[r][c] == 0 {
                        dist_mat[r][c] = dist_mat[row][col] + 1;
                        next_edge_points.push((r, c));
                    }
                }
            }
            edge_points = next_edge_points;
        }
        dist_mat[rowc - 1][colc - 1]
    }
}

fn neighbors(
    row: usize,
    col: usize,
    rowc: usize,
    colc: usize,
) -> impl Iterator<Item = (usize, usize)> {
    let row = row as i64;
    let col = col as i64;
    let rowc = rowc as i64;
    let colc = colc as i64;

    vec![
        (row - 1, col - 1),
        (row - 1, col),
        (row - 1, col + 1),
        (row, col - 1),
        (row, col + 1),
        (row + 1, col - 1),
        (row + 1, col),
        (row + 1, col + 1),
    ]
    .into_iter()
    .filter(move |&(r, c)| r >= 0 && r < rowc && c >= 0 && c < colc)
    .map(|(r, c)| (r as usize, c as usize))
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;

    #[test_case(vec![vec![1,0,0],vec![1,1,0],vec![1,1,0]] => -1)]
    #[test_case(vec![vec![0,0,0],vec![1,1,0],vec![1,1,0]] => 4)]
    fn test(grid: Vec<Vec<i32>>) -> i32 {
        Solution::shortest_path_binary_matrix(grid)
    }
}
