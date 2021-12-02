/*
 * @lc app=leetcode id=328 lang=rust
 *
 * [328] Odd Even Linked List
 */

struct Solution {}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// @lc code=start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let vec = Self::list_to_vec(head);
        let part = |rem: usize| {
            vec.iter()
                .enumerate()
                .rev()
                .filter(move |&(idx, _)| idx % 2 == rem)
                .map(|(_, val)| *val)
        };
        let result: Option<Box<ListNode>> = part(1)
            .chain(part(0))
            .fold(None, |acc, val| Some(Box::new(ListNode { val, next: acc })));
        result
    }

    fn list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
        Self::list_to_vec_rec(head, vec![])
    }

    fn list_to_vec_rec(head: Option<Box<ListNode>>, mut acc: Vec<i32>) -> Vec<i32> {
        match head {
            None => acc,
            Some(node) => {
                acc.push(node.val);
                Self::list_to_vec_rec(node.next, acc)
            }
        }
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let head = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 2,
                next: Some(Box::new(ListNode {
                    val: 3,
                    next: Some(Box::new(ListNode {
                        val: 4,
                        next: Some(Box::new(ListNode { val: 5, next: None })),
                    })),
                })),
            })),
        }));
        println!("{:?}", Solution::odd_even_list(head));
    }
}
