extern crate common;
use common::macros::sort_example;
use common::prints;

/// A simple sorting algorithm which works by repeatedly selecting the minimum element from a given
/// subsection of the array until all elements are exhausted. The selected elements are placed into
/// a sorted subsection into the array, essentially  dividing the array into two parts, one sorted
/// and the other unsorted.
///
/// Time Complexity:
/// * Best-Case: O(N^2)
/// * Average-Case: O(N^2)
/// * Worst-Case: O(N^2)
///
/// Space Complexity: O(1)
pub fn min_selection<T>(arr: &mut [T])
where
    T: std::cmp::PartialOrd,
{
    let mut min_index: usize;
    for iteration in 0..(arr.len() - 1) {
        min_index = iteration
            + arr[iteration..]
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap(); // Won't panic because slice will never be empty
        arr.swap(iteration, min_index)
    }
}

/// A simple sorting algorithm which works by repeatedly selecting the minimum element from a given
/// subsection of the array until all elements are exhausted. The selected elements are placed into
/// a sorted subsection into the array, essentially  dividing the array into two parts, one sorted
/// and the other unsorted.
///
/// Time Complexity:
/// * Best-Case: O(N^2)
/// * Average-Case: O(N^2)
/// * Worst-Case: O(N^2)
///
/// Space Complexity: O(1)
pub fn max_selection<T>(arr: &mut [T])
where
    T: std::cmp::PartialOrd,
{
    let mut max_index: usize;
    let size = arr.len();
    for iteration in 0..(size - 1) {
        max_index = arr[..(size - iteration)]
            .iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap(); // Won't panic because slice will never be empty
        arr.swap(size - iteration - 1, max_index)
    }
}

fn main() {
    println!("Minimum Selection Sort:");
    sort_example!(min_selection);
    println!("Maximum Selection Sort:");
    sort_example!(max_selection);
}
