/*
 * @lc app=leetcode id=1209 lang=rust
 *
 * [1209] Remove All Adjacent Duplicates in String II
 */

struct Solution {}

// @lc code=start
impl Solution {
    pub fn remove_duplicates(s: String, k: i32) -> String {
        let k = k as usize;
        let mut stack = Vec::<(char, usize)>::new();
        for ch in s.chars() {
            match stack.pop() {
                None => stack.push((ch, 1)),
                Some((top_ch, top_count)) => {
                    if ch == top_ch {
                        let top_count = top_count + 1;
                        if top_count < k {
                            stack.push((ch, top_count));
                        } else {
                        }
                    } else {
                        stack.push((top_ch, top_count));
                        stack.push((ch, 1));
                    }
                }
            }
        }
        stack
            .into_iter()
            .flat_map(|(ch, count)| std::iter::repeat(ch).take(count))
            .collect()
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::Solution;
    use test_case::test_case;
    // aa

    #[test_case("deeedbbcccbdaa", 3 => "aa")]
    fn test(s: &str, k: i32) -> String {
        Solution::remove_duplicates(s.to_string(), k)
    }
}
