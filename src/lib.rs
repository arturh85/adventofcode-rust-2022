//! # Advent of Code 2022 Solutions in Rust
//!
//! - ❔ [About Advent of Code](https://adventofcode.com/about)
//! - 📆 [List of Problems](https://adventofcode.com/2022)

#[allow(unused_imports)]
#[macro_use]
extern crate anyhow;
#[allow(unused_imports)]
// #[macro_use]
// extern crate serde_json;
#[macro_use]
extern crate aoc_runner_derive;
extern crate aoc_runner;

mod day1;
mod util;
mod day2;

aoc_lib! { year = 2022 }
