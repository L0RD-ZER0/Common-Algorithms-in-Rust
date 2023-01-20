extern crate common;
use common::macros::sort_example;
use common::prints;

/// Helper function to partition the array into sub-arrays for [quick].
fn _partition<T>(arr: &mut [T], start: usize, end: usize) -> usize
where
    T: std::cmp::PartialOrd,
{
    let pivot_index = end;
    let mut last_greatest = start;
    for index in start..end {
        if arr[index] < arr[pivot_index] {
            arr.swap(last_greatest, index);
            last_greatest += 1;
        }
    }
    arr.swap(last_greatest, pivot_index);
    last_greatest
}

/// Recursive function used in [quick] function to sort using divide and conquer.
fn _sort<T>(arr: &mut [T], start: usize, end: usize)
where
    T: std::cmp::PartialOrd,
{
    if start >= end { return; }
    let pivot = _partition(arr, start, end);
    if pivot != 0 {
        _sort(arr, start, pivot - 1);
    }
    _sort(arr, pivot + 1, end);
}

/// This algorithm is a Divide and Conquer sorting algorithm that picks an element as a pivot before
/// partitioning the array before placing all the elements lesser than the pivot to its left and all
/// the greater elements to its right, after which it divides the array into two sub-arrays, placing
/// the pivot in betweem the sub-arrays, i.e. at it's appropriate location. This process is repeated
/// until the array is sorted.
///
/// Time Complexity:
/// * Best-Case: O(N*log(N))
/// * Average-Case: O(N*log(N))
/// * Worst-Case: O(N^2)
///
/// Space Complexity: O(N)
pub fn quick<T>(arr: &mut [T])
    where
        T: std::cmp::PartialOrd,
{
    _sort(arr, 0, arr.len() - 1);
}

fn main() {
    sort_example!(quick);
}
