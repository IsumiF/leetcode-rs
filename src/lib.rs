#![allow(dead_code)]

#[path = "62.unique-paths.rs"]
mod unique_paths;
#[path = "448.find-all-numbers-disappeared-in-an-array.rs"]
mod find_all_numbers_disappeared_in_an_array;
#[path = "540.single-element-in-a-sorted-array.rs"]
mod p540;
#[path = "952.largest-component-size-by-common-factor.rs"]
mod p952;
#[path = "986.interval-list-intersections.rs"]
mod p986;
#[path = "53.maximum-subarray.rs"]
mod p53;
#[path = "35.search-insert-position.rs"]
mod p35;
#[path = "238.product-of-array-except-self.rs"]
mod p238;
#[path = "797.all-paths-from-source-to-target.rs"]
mod p797;
#[path = "721.accounts-merge.rs"]
mod p721;
#[path = "206.reverse-linked-list.rs"]
mod p206;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
