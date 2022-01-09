/*
 * @lc app=leetcode id=1041 lang=rust
 *
 * [1041] Robot Bounded In Circle
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn is_robot_bounded(instructions: String) -> bool {
        let mut pos = (0i64, 0i64);
        let mut dir = (0i64, 1i64);
        for ch in instructions.chars() {
            match ch {
                'G' => {
                    pos.0 += dir.0;
                    pos.1 += dir.1;
                }
                // (0, 1) -> (-1, 0) -> (0, -1) -> (1, 0) -> (0, 1)
                'L' => {
                    dir = match dir {
                        (0, 1) => (-1, 0),
                        (-1, 0) => (0, -1),
                        (0, -1) => (1, 0),
                        (1, 0) => (0, 1),
                        _ => panic!("invalid direction"),
                    };
                }
                'R' => {
                    dir = match dir {
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        (-1, 0) => (0, 1),
                        _ => panic!("invalid direction"),
                    };
                }
                _ => {
                    panic!("invalid input");
                }
            }
        }
        pos == (0i64, 0i64) || dir != (0i64, 1i64)
    }
}

// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("GGLLGG" => true)]
    #[test_case("GG" => false)]
    #[test_case("GL" => true)]
    fn test(instructions: &'static str) -> bool {
        Solution::is_robot_bounded(instructions.to_string())
    }
}
