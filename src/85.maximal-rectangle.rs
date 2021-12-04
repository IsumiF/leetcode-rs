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
            Some(first_row) => {
                let mut histogram = vec![0; first_row.len()];
                let max_area = matrix
                    .into_iter()
                    .map(|row| {
                        for (idx, cell) in row.into_iter().enumerate() {
                            if cell == '1' {
                                histogram[idx] += 1;
                            } else {
                                histogram[idx] = 0;
                            }
                        }
                        Self::largest_rectangle_area(&histogram)
                    })
                    .max()
                    .unwrap_or(0);
                max_area
            }
            None => 0,
        }
    }

    fn largest_rectangle_area(heights: &Vec<i32>) -> i32 {
        let mut stack = Vec::<usize>::new();
        stack.reserve_exact(heights.len());
        let mut max_area: i32 = i32::MIN;
        let mut pop = |stack: &mut Vec<usize>, right_index: usize| -> bool {
            if let Some(cur_idx) = stack.pop() {
                let left_index = stack.last().map(|&left_index| left_index + 1).unwrap_or(0);
                let cur_area = (right_index - left_index + 1) as i32 * heights[cur_idx];
                if cur_area > max_area {
                    max_area = cur_area;
                }
                true
            } else {
                false
            }
        };

        for (idx, &height) in heights.iter().enumerate() {
            loop {
                if !stack
                    .last()
                    .map(|&prev_idx| heights[prev_idx] < height)
                    .unwrap_or(true)
                {
                    pop(&mut stack, idx - 1);
                } else {
                    break;
                }
            }
            stack.push(idx);
        }
        while !stack.is_empty() {
            pop(&mut stack, heights.len() - 1);
        }
        max_area
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
    #[test_case(vec![vec![0, 1], vec![1, 0]] => 1)]
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
