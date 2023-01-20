extern crate common;
use common::macros::sort_example;
use common::prints;

/// Insertion sort is a simple sorting algorithm that virtually splits an array into sorted and
/// unsorted parts. Values are picked one after another from the unsorted part and are inserted into
/// the sorted portion at their appropriate locations.
///
/// Time Complexity:
/// * Best-Case: O(N)
/// * Average-Case: O(N^2)
/// * Worst-Case: O(N^2)
///
/// Space Complexity: O(1)
pub fn insertion<T>(arr: &mut [T])
where
    T: std::cmp::PartialOrd,
{
    for current_index in 1..arr.len() {
        let mut pos = current_index;
        while pos > 0 && arr[pos] < arr[pos - 1] {
            // Insert at the position where left is smaller or equal and right is bigger
            arr.swap(pos, pos - 1);
            pos -= 1;
        }
    }
}

fn main() {
    sort_example!(insertion);
}
