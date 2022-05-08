/*
 * @lc app=leetcode id=341 lang=rust
 *
 * [341] Flatten Nested List Iterator
 */

#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}

// @lc code=start
use std::iter::Peekable;

struct NestedIterator {
    iter: Peekable<<Vec<i32> as IntoIterator>::IntoIter>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        let nums = to_vec(NestedInteger::List(nested_list));
        Self {
            iter: nums.into_iter().peekable(),
        }
    }

    fn next(&mut self) -> i32 {
        self.iter.next().unwrap()
    }

    fn has_next(&mut self) -> bool {
        self.iter.peek().is_some()
    }
}

fn to_vec(nestd_int: NestedInteger) -> Vec<i32> {
    match nestd_int {
        NestedInteger::Int(x) => vec![x],
        NestedInteger::List(xs) => xs.into_iter().flat_map(to_vec).collect(),
    }
}

// @lc code=end

