#![allow(dead_code)]

#[path = "62.unique-paths.rs"]
mod unique_paths;
#[path = "448.find-all-numbers-disappeared-in-an-array.rs"]
mod find_all_numbers_disappeared_in_an_array;
#[path = "540.single-element-in-a-sorted-array.rs"]
mod p540;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
