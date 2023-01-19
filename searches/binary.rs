extern crate common;
use common::prints;

/// Helper function for `recursive_binary` function which is used for recursion.
fn _recursive_binary<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
    arr: &[T],
    target: T,
    start: usize,
    end: usize,
) -> i32 {
    let mid = (start + end) / 2;
    if end >= start {
        // Segment is empty
        if arr[mid] == target {
            // Middle element is equal to target
            return mid as i32;
        } else if arr[mid] > target {
            // Middle Element is Greater
            return _recursive_binary(arr, target, start, mid - 1);
        } else if arr[mid] < target {
            // Middle element is less
            return _recursive_binary(arr, target, mid + 1, end);
        }
    }
    return -1; // Element is not found.
}

/// Recursive Binary Search Algorithm which searches through a sorted array by repeatedly dividing
/// the array into half. It uses a divide and conquer approach to search the array quickly.
/// Internally uses recursive calls to search through the array.
///
/// Notes: Requires a sorted array
///
/// Time Complexity:
/// * Best-Case: O(1)
/// * Average-Case: O(log(N))
/// * Worst-Case: O(log(N))
///
/// Space-Complexity: O(log(N))
pub fn recursive_binary<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
    arr: &[T],
    target: T,
) -> i32 {
    return _recursive_binary(arr, target, 0, arr.len() - 1);
}

/// Iterative Binary Search Algorithm which searches through a sorted array by repeatedly dividing
/// the array into half. It uses a divide and conquer approach to search the array quickly.
/// Internally uses a loop to iterate continually until solution is found.
///
/// Notes: Requires a sorted array
///
/// Time Complexity:
/// * Best-Case: O(1)
/// * Average-Case: O(log(N))
/// * Worst-Case: O(log(N))
///
/// Space-Complexity: O(1)
pub fn iterative_binary<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
    arr: &[T],
    target: T,
) -> i32 {
    let (mut start, mut end) = (0, arr.len() - 1);
    let mut mid: usize;
    loop {
        mid = (start + end) / 2;
        if end >= start {
            // Segment is empty
            if arr[mid] == target {
                // Middle element is equal to target
                return mid as i32;
            } else if arr[mid] > target {
                // Middle Element is Greater
                end = mid - 1;
            } else if arr[mid] < target {
                // Middle element is less
                start = mid + 1;
            }
        } else {
            break;
        }
    }
    return -1;
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let val1 = 9;
    let val2 = 2;
    let val3 = 11;
    println!("Array:");
    prints::print_array(&arr);
    println!("Recursive Binary Search:");
    println!(
        "Index for value `{}` is `{}`",
        val1,
        recursive_binary(&arr, val1)
    );
    println!(
        "Index for value `{}` is `{}`",
        val2,
        recursive_binary(&arr, val2)
    );
    println!(
        "Index for value `{}` is `{}`",
        val3,
        recursive_binary(&arr, val3)
    );
    println!("Iterative Binary Search:");
    println!(
        "Index for value `{}` is `{}`",
        val1,
        iterative_binary(&arr, val1)
    );
    println!(
        "Index for value `{}` is `{}`",
        val2,
        iterative_binary(&arr, val2)
    );
    println!(
        "Index for value `{}` is `{}`",
        val3,
        iterative_binary(&arr, val3)
    );
}
