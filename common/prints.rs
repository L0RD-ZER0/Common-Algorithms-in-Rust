/// Function to print arrays whose members implement ``std::fmt::Display`` trait.
pub fn print_array<T:std::fmt::Display>(arr: &[T]) {
    print!("[ ");
    for i in arr {
        print!("{}, ", i);
    }
    println!("]");
}