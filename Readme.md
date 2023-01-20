Common-Algorithms
=================

This repository contains my implementations for common algorithms in [Rust] 
programming language. This repository is created to practice [Rust] by implementing
various algorithms and solutions for common programming problems.

Each solution has a rust file. In case of a solution with multiple implementations,
various solutions will be included in the corresponding file or module.

Contents
--------
### Search Algorithms:
* [Linear-Search](searches/linear.rs)
* [Binary-Search](searches/binary.rs)

### Sorting Algorithms:
* [Insertion-Sort](sorts/insertion.rs)
* [Bubble-Sort](sorts/bubble.rs)
* [Selection-Sort](sorts/selection.rs)

How to Run
----------
To run these programs, use the following command:
```commandline
cargo run --bin <binary-name>
```
For available binary names, consider checking `[[bin]]` sections within the [Cargo.toml] 
file. Due to presence of multiple binaries, using `cargo run` results in an error.


[Rust]: https://www.rust-lang.org/
[Cargo.toml]: ./Cargo.toml