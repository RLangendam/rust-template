//! # My Project
//!
//! A brief explanation of the library for `rustdoc`.

/// Adds two numbers.
/// 
/// # Examples
/// ```
/// let res = my_project_lib::add(2, 2);
/// assert_eq!(res, 4);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}