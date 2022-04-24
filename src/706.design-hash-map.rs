/*
 * @lc app=leetcode id=706 lang=rust
 *
 * [706] Design HashMap
 */

// @lc code=start
struct MyHashMap {
    values: Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MyHashMap {

    fn new() -> Self {
        Self {
            values: vec![-1; 1000001],
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        self.values[key as usize] = value;
    }
    
    fn get(&self, key: i32) -> i32 {
        self.values[key as usize]
    }
    
    fn remove(&mut self, key: i32) {
        self.values[key as usize] = -1;
    }
}

// @lc code=end

