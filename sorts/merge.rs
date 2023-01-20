extern crate common;
use common::macros::sort_example;
use common::prints;

/// Helper function to merge two sub-arrays used in [merge] function.
fn _merge<T>(arr: &mut [T], start: usize, mid: usize, end: usize)
where
    T: std::cmp::PartialOrd + Clone,
{
    let (s1, s2) = (mid - start + 1, end - mid);
    let arr1 = arr[start..=mid].to_vec();
    let arr2 = arr[mid + 1..=end].to_vec();
    let (mut i1, mut i2, mut im): (usize, usize, usize) = (0, 0, start);
    while i1 < s1 && i2 < s2 {
        if arr1[i1] < arr2[i2] {
            arr[im] = arr1[i1].clone();
            i1 += 1;
        } else {
            arr[im] = arr2[i2].clone();
            i2 += 1;
        }
        im += 1;
    }
    while i1 < s1 {
        arr[im] = arr1[i1].clone();
        i1 += 1;
        im += 1;
    }
    while i2 < s2 {
        arr[im] = arr2[i2].clone();
        i2 += 1;
        im += 1;
    }
}

/// Recursive function used in [merge] function to divide and conquer the problem.
fn _sort<T>(arr: &mut [T], start: usize, end: usize)
where
    T: std::cmp::PartialOrd + Clone,
{
    if start >= end {
        return;
    }
    let mid = (start + end) / 2;
    _sort(arr, start, mid);
    _sort(arr, mid + 1, end);
    _merge(arr, start, mid, end);
}

/// A sorting algorithm that works by dividing an array into smaller sub-arrays and sorting them
/// individually before combining them all together to produce the final sorted array, using a
/// Divide and Conquer approach to sort the array.
///
/// Time Complexity:
/// * Best-Case: O(N*log(N))
/// * Average-Case: O(N*log(N))
/// * Worst-Case: O(N*log(N))
///
/// Space Complexity: O(N)
pub fn merge<T>(arr: &mut [T])
where
    T: std::cmp::PartialOrd + Clone,
{
    _sort(arr, 0, arr.len() - 1);
}

fn main() {
    sort_example!(merge);
}
