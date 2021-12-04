/*
 * @lc app=leetcode id=1032 lang=rust
 *
 * [1032] Stream of Characters
 */

// @lc code=start
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
};

struct StreamChecker {
    trie: Trie,
    consumed_string: RefCell<Vec<u8>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StreamChecker {
    fn new(words: Vec<String>) -> Self {
        let trie = Trie::new(
            &words
                .into_iter()
                .map(|w| w.chars().rev().collect())
                .collect(),
        );
        Self {
            trie,
            consumed_string: RefCell::new(Vec::new()),
        }
    }

    fn query(&self, letter: char) -> bool {
        self.consumed_string.borrow_mut().push(letter as u8);
        let mut trie: &Trie = &self.trie;
        for &ch in self.consumed_string.borrow().iter().rev() {
            if let Some(next_trie) = trie.lookup(ch) {
                trie = next_trie;
                if trie.is_acceptable() {
                    return true;
                }
            } else {
                break;
            }
        }
        false
    }
}

#[derive(Debug)]
struct Trie {
    dict: Option<Box<HashMap<u8, Trie>>>,
    acceptable: bool,
}

impl Trie {
    pub fn new<'a>(words: &Vec<String>) -> Self {
        Self::build_trie_rec(words.iter().map(|s| s.as_bytes()).collect())
    }

    fn build_trie_rec<'a>(words: Vec<&'a [u8]>) -> Self {
        if words.is_empty() {
            Self::new_empty()
        } else {
            let mut first_letter_to_remainings: HashMap<u8, Vec<&[u8]>> = HashMap::new();
            let mut acceptable_letters: HashSet<u8> = HashSet::new();
            for word in words {
                if let Some((first_letter, remaining)) = word.split_first() {
                    let mut remainings = first_letter_to_remainings
                        .remove(first_letter)
                        .unwrap_or(vec![]);
                    if remaining.len() > 0 {
                        remainings.push(remaining);
                    } else {
                        acceptable_letters.insert(*first_letter);
                    }
                    first_letter_to_remainings.insert(*first_letter, remainings);
                }
            }
            if first_letter_to_remainings.is_empty() {
                Self::new_empty()
            } else {
                let mut trie: HashMap<u8, Trie> = HashMap::new();
                for (k, v) in first_letter_to_remainings.into_iter() {
                    let mut subtrie = if v.is_empty() {
                        Self::new_empty()
                    } else {
                        Self::build_trie_rec(v)
                    };
                    subtrie.acceptable = acceptable_letters.get(&k).is_some();
                    trie.insert(k, subtrie);
                }
                Trie {
                    dict: Some(Box::new(trie)),
                    acceptable: false,
                }
            }
        }
    }

    fn new_empty() -> Self {
        Self {
            dict: None,
            acceptable: true,
        }
    }

    pub fn lookup(&self, key: u8) -> Option<&Trie> {
        self.dict.as_ref().and_then(|dict| dict.get(&key))
    }

    pub fn is_acceptable(&self) -> bool {
        self.acceptable
    }
}

/*
 * Your StreamChecker object will be instantiated and called as such:
 * let obj = StreamChecker::new(words);
 * let ret_1: bool = obj.query(letter);
 */
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let checker = StreamChecker::new(
            ["ab", "ba", "aaab", "abab", "baa"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        let got: Vec<bool> = "aaaaabababbbababbbbababaaabaaa"
            .to_string()
            .chars()
            .map(|c| checker.query(c))
            .collect();
        let want = vec![
            false, false, false, false, false, true, true, true, true, true, false, false, true,
            true, true, true, false, false, false, true, true, true, true, true, true, false, true,
            true, true, false,
        ];
        assert_eq!(got, want);
    }
}
