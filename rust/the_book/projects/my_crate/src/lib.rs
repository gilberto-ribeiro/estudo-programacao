//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Example
/// 
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

pub fn square_plus_two(x: i32) -> i32 {
    //! Take the square an adds two.
    //! 
    //! # Example
    //! ```
    //! let a = 5;
    //! assert_eq!(27, my_crate::square_plus_two(a));
    //! ```
    x*x + 2
}