/// Macro used for reusing the example for sorting. Uses a function which implements an inplace sort
/// and takes a single mutable array reference as an argument. Alternatively, an array may be used
/// for customising the array used.
#[macro_export]
macro_rules! sort_example {
    ($func: ident) => {
        let mut arr1 = [10, 9, 8, 7, 6, 5, 4, 3, 2, 1];
        let mut arr2 = [2, 6, 9, 7, 3, 10, 5, 8, 1, 4];
        let mut arr3 = [6, 10, 2, 7, 3, 9, 8, 4, 5, 1];
        sort_example!($func, arr1);
        sort_example!($func, arr2);
        sort_example!($func, arr3);
    };
    ($func: ident, $arr: ident) => {
        print!("Unsorted Array: ");
        prints::print_array(&$arr);
        $func(&mut $arr);
        print!("After Sorting: ");
        prints::print_array(&$arr);
    };
}
pub use sort_example;
