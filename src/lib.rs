//! # My Project
//!
//! A brief explanation of the library for `rustdoc`.

#![deny(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

/// Adds two numbers.
///
/// # Examples
/// ```
/// let res = my_project_lib::add(2, 2);
/// assert_eq!(res, 4);
/// ```
#[must_use]
pub const fn add(left: usize, right: usize) -> usize {
    left + right
}
