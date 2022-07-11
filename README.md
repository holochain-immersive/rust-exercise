# Rust Exercises

This is a Rust library that contains multiple utility functions, to get introduced into the fundamentals of rust.

## Your Goals

Clone this repository, and go to `src/lib.rs` to see them.

**Your goal is to implement all the `unimplemented!()` functions in this library, making all the tests pass.** 

Each function has some tests associated with it. After you implement a function, run the tests to see whether the implementation was correct. 

It's much better for you to go in order, implementing the functions from top to bottom.

## Running the tests

> If you haven't yet, [install Rust](https://www.rust-lang.org/tools/install).

To run the tests, run this from a terminal in the root folder of this repository:

```bash
cargo test
```

Which should give an output with the summary of the tests like this one: 

```
test result: FAILED. 0 passed; 17 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

## What's in this repository

- `src/lib.rs`: main file with the source code for the library. Go to this file and implement its functions to complete the exercise.
- `tests/tests.rs`: tests for each function defined in `src/lib.rs`.
- `Cargo.toml`: cargo file which declares project as a Rust package.