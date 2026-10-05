# Exercism Rust Homework

## Author

Trevor Thompson

## Description

This submission contains seven solved Exercism Rust track exercises, each as
its own Cargo library crate:

| Exercise | What it does |
|---|---|
| `reverse-string` | Reverses the characters of a UTF-8 string. |
| `difference-of-squares` | Computes the difference between the square of the sum and the sum of the squares of the first `n` natural numbers. |
| `prime-factors` | Computes the prime factorization of a positive integer. |
| `matching-brackets` | Checks whether brackets (`()[]{}`) in a string are balanced and correctly nested. |
| `collatz-conjecture` | Computes the number of steps to reach `1` from `n` using the Collatz conjecture rules. |
| `bob` | A lackadaisical teenager who replies to questions, shouting, silence, and anything else with a fixed set of responses. |
| `robot-simulator` | Simulates a robot on an infinite grid that can turn left/right and advance in the direction it faces. |

Each exercise is a standalone library crate (`src/lib.rs`) with its official
Exercism test suite under `tests/`, plus Rustdoc comments (including runnable
doctest examples) on every public item.

## Build and run

Each exercise directory is an independent Cargo project. From inside any
exercise directory (e.g. `cd reverse-string`):

```sh
cargo build          # compile the crate
cargo test           # run the unit tests, integration tests, and doctests
cargo doc --open     # build and view the Rustdoc documentation
```

To check formatting and linting (what was used to verify this submission):

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
```

All seven crates build cleanly with current stable Rust, produce no compiler
or `cargo clippy` warnings, are formatted with `cargo fmt`, and pass all
tests (`cargo test`), including doctests.

## Issues encountered

I didn't run into any real issues — this was a fairly simple assignment.
The main thing I needed to work on was basic Rust syntax and conventions
(data types, `if`/loops, `match`, ownership/borrowing, etc.), since I'm new
to the language. The compiler's own error messages and suggestions
(`rustc`/`cargo`) were more than enough to work through those, along with
`cargo clippy` for idiomatic style.

## AI disclosure

I used AI (GitHub Copilot CLI) in two limited ways while completing this
assignment:

- As a reference for basic Rust syntax (data types, `if`/`else`, loops,
  `match`, etc.) while I was learning the language.
- To write this README file, under my guidance (I specified what it should
  contain and reviewed/edited the result).

AI did not write or answer any of the 7 exercises themselves; those
solutions are my own work.

## Notes

- No crate required third-party dependencies; all solutions use only the
  Rust standard library.
- Nothing has been omitted via `#[allow(...)]` — all warnings were fixed at
  the source rather than silenced.
- The `tests/` directory in each exercise contains Exercism's official test
  suite for that exercise (with `#[ignore]` removed so `cargo test` runs
  every test by default).
- `reverse-string` reverses by Unicode scalar value (`char`), not by
  grapheme cluster; the optional grapheme-cluster tests, which require the
  external `unicode-segmentation` crate, are not included.
