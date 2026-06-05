//! A reasonably fast factorisation library for unsigned integers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(fast_factor::factor(24_u32), vec![1,2,3,4,6,8,12,24]);
//! assert_eq!(fast_factor::proper_factor(24_u32), vec![1,2,3,4,6,8,12]);
//! assert_eq!(fast_factor::exclusive_factor(24_u32), vec![2,3,4,6,8,12]);
//! assert!(!fast_factor::is_prime(24_u32));
//! ```

use num_integer::Roots;
use num_traits::{PrimInt, Unsigned};

/// For non-zero be unsigned integer inputs, returns all factors of the number given, including one and the number itself, as a vector. Returns an empty vector for zero.
///
/// # Examples
///
/// ```
/// assert_eq!(fast_factor::factor(0_u32), vec![]);
/// assert_eq!(fast_factor::factor(1_u32), vec![1]);
/// assert_eq!(fast_factor::factor(7_u32), vec![1,7]);
/// assert_eq!(fast_factor::factor(12_u32), vec![1,2,3,4,6,12]);
/// assert_eq!(fast_factor::factor(36_u32), vec![1,2,3,4,6,9,12,18,36]);
/// ```
pub fn factor<T: PrimInt + Unsigned + Roots>(n: T) -> Vec<T> {
    let mut factors: Vec<T> = Vec::new();
    let zero = T::zero();
    let one = T::one();
    let two = one + one;
    if n == zero {
        return factors;
    }

    factors.push(one);
    let root_n = n.sqrt();
    let mut factor = two;
    while factor <= root_n {
        if n % factor == zero {
            factors.push(factor);
        }
        factor = factor + one;
    }
    let is_square = root_n * root_n == n;
    let mid = factors.len();
    for index in (0..mid).rev() {
        let factor = factors[index];
        if !(is_square && factor == root_n) {
            factors.push(n / factor);
        }
    }
    factors
}

/// For all unsigned integer inputs, returns all factors of the number, excluding the number
/// itself.
///
/// # Examples
///
/// ```
/// assert_eq!(fast_factor::proper_factor(0_u32), vec![]);
/// assert_eq!(fast_factor::proper_factor(1_u32), vec![]);
/// assert_eq!(fast_factor::proper_factor(7_u32), vec![1]);
/// assert_eq!(fast_factor::proper_factor(12_u32), vec![1,2,3,4,6]);
/// assert_eq!(fast_factor::proper_factor(36_u32), vec![1,2,3,4,6,9,12,18]);
/// ```
pub fn proper_factor<T: PrimInt + Unsigned + Roots>(n: T) -> Vec<T> {
    let mut factors: Vec<T> = Vec::new();

    let zero = T::zero();
    let one = T::one();
    let two = one + one;

    if n < two {
        return factors;
    }

    factors.push(one);
    let root_n = n.sqrt();

    let mut factor = two;
    while factor <= root_n {
        if n % factor == zero {
            factors.push(factor);
        }
        factor = factor + one;
    }
    let is_square = root_n * root_n == n;
    let mid = factors.len();
    for index in (1..mid).rev() {
        let factor = factors[index];
        if !(is_square && factor == root_n) {
            factors.push(n / factor);
        }
    }
    factors
}

/// For all unsigned integer inputs, returns all factors of the number, excluding one and the number
/// itself.
///
/// # Examples
///
/// ```
/// assert_eq!(fast_factor::exclusive_factor(0_u32), vec![]);
/// assert_eq!(fast_factor::exclusive_factor(1_u32), vec![]);
/// assert_eq!(fast_factor::exclusive_factor(7_u32), vec![]);
/// assert_eq!(fast_factor::exclusive_factor(12_u32), vec![2,3,4,6]);
/// assert_eq!(fast_factor::exclusive_factor(36_u32), vec![2,3,4,6,9,12,18]);
/// ```
pub fn exclusive_factor<T: PrimInt + Unsigned + Roots>(n: T) -> Vec<T> {
    let mut factors: Vec<T> = Vec::new();

    let zero = T::zero();
    let one = T::one();
    let two = one + one;

    if n < two + two {
        return factors;
    }

    let root_n = n.sqrt();

    let mut factor = two;
    while factor <= root_n {
        if n % factor == zero {
            factors.push(factor);
        }
        factor = factor + one;
    }
    let is_square = root_n * root_n == n;
    let mid = factors.len();
    for index in (0..mid).rev() {
        let factor = factors[index];
        if !(is_square && factor == root_n) {
            factors.push(n / factor);
        }
    }
    factors
}

/// For all unsigned integer inputs, returns whether the number is prime. 0 and 1 are not.
///
/// # Examples
///
/// ```
/// assert!(!fast_factor::is_prime(0_u32));
/// assert!(!fast_factor::is_prime(1_u32));
/// assert!(fast_factor::is_prime(7_u32));
/// assert!(!fast_factor::is_prime(12_u32));
/// assert!(!fast_factor::is_prime(36_u32));
/// ```
pub fn is_prime<T: PrimInt + Unsigned + Roots>(n: T) -> bool {
    let zero = T::zero();
    let one = T::one();
    let two = one + one;

    if n < two {
        return false;
    }

    let root_n = n.sqrt();

    let mut factor = two;
    while factor <= root_n {
        if n % factor == zero {
            return false;
        }
        factor = factor + one;
    }
    true
}
