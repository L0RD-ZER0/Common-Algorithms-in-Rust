extern crate common;
use common::prints;

/// Linear Search, also known as Sequential Search, is used to search through an array sequentially.
///
/// Time Complexity:
/// * Best-Case: O(1)
/// * Average-Case: O(N)
/// * Worst-Case: O(N)
///
/// Space Complexity: O(1)
pub fn linear<T: std::cmp::PartialEq>(arr: &[T], target: T) -> i32 {
    for (index, value) in arr.iter().enumerate() {
        if *value == target {
            return index as i32;
        }
    }
    return -1;
}

pub fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let val1 = 9;
    let val2 = 2;
    let val3 = 11;
    println!("Array:");
    prints::print_array(&arr);
    println!("Index for value `{}` is `{}`", val1, linear(&arr, val1));
    println!("Index for value `{}` is `{}`", val2, linear(&arr, val2));
    println!("Index for value `{}` is `{}`", val3, linear(&arr, val3));
}
