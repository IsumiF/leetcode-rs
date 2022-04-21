/*
 * @lc app=leetcode id=705 lang=rust
 *
 * [705] Design HashSet
 */

// @lc code=start
struct MyHashSet {
    buckets: Vec<Vec<i32>>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashSet {
    fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 10000],
        }
    }

    fn add(&mut self, key: i32) {
        let bucket = self.bucket_mut(key);
        if bucket.iter().find(|&v| *v == key).is_none() {
            bucket.push(key);
        }
    }

    fn remove(&mut self, key: i32) {
        let bucket = self.bucket_mut(key);
        if let Some((idx, _)) = bucket.iter().enumerate().find(|&(_, v)| *v == key) {
            bucket.remove(idx);
        }
    }

    fn contains(&self, key: i32) -> bool {
        self.bucket(key).iter().find(|&v| *v == key).is_some()
    }

    fn bucket_mut(&mut self, key: i32) -> &mut Vec<i32> {
        &mut self.buckets[(key % 10000) as usize]
    }

    fn bucket(&self, key: i32) -> &Vec<i32> {
        &self.buckets[(key % 10000) as usize]
    }
}

// @lc code=end
