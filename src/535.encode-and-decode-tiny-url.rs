/*
 * @lc app=leetcode id=535 lang=rust
 *
 * [535] Encode and Decode TinyURL
 */

// @lc code=start
use std::collections::HashMap;
use std::rc::Rc;

struct Codec {
    long_to_short: HashMap<Rc<String>, usize>,
    short_to_long: Vec<Rc<String>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Codec {
    fn new() -> Self {
        Codec {
            long_to_short: HashMap::new(),
            short_to_long: Vec::new(),
        }
    }

    // Encodes a URL to a shortened URL.
    fn encode(&mut self, long_url: String) -> String {
        let long_url = Rc::new(long_url);
        let existing = self.long_to_short.get(&long_url).cloned();
        existing
            .unwrap_or_else(|| {
                let idx = self.short_to_long.len();
                self.short_to_long.push(long_url.clone());
                self.long_to_short.insert(long_url, idx);
                idx
            })
            .to_string()
    }

    // Decodes a shortened URL to its original URL.
    fn decode(&self, short_url: String) -> String {
        let idx: usize = short_url.parse().unwrap();
        (*self.short_to_long[idx]).clone()
    }
}

// @lc code=end
