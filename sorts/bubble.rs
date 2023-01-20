extern crate common;
use common::macros::sort_example;
use common::prints;

/// Bubble sort algorithm compares two successive elements, essentially forming a 'bubble' and if
/// left element is larger than right, both elements are swapped. Comparisons are performed in
/// passes from left to right, sorting one element after another at a time.
///
/// Time Complexity:
/// * Best-Case: O(N)
/// * Average-Case: O(N^2)
/// * Worst-Case: O(N^2)
///
/// Space Complexity: O(1)
pub fn bubble<T>(arr: &mut [T])
where
    T: std::cmp::PartialOrd,
{
    let size = arr.len();
    for index in 0..size {
        // Last N elements would be in correct positions after N passes.
        for pass_position in 0..(size - 1 - index) {
            if arr[pass_position] > arr[pass_position + 1] {
                arr.swap(pass_position, pass_position + 1);
            }
        }
    }
}

fn main() {
    sort_example!(bubble);
}
