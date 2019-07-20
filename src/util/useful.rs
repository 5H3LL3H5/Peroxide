//! Useful missing tools

use std::convert;
use std::f64::MAX;

type Vector = Vec<f64>;

// =============================================================================
// Fundamental Utils
// =============================================================================

/// Near equal
///
/// # Examples
/// ```
/// extern crate peroxide;
/// use peroxide::*;
///
/// assert!(nearly_eq(1.0/3.0 * 3.0, 1));
/// ```
pub fn nearly_eq<S, T>(x: S, y: T) -> bool
where
    S: convert::Into<f64>,
    T: convert::Into<f64>,
{
    let mut b: bool = false;
    let e = 1e-7;
    let p: f64 = x.into().abs();
    let q: f64 = y.into().abs();
    if (p - q).abs() < e {
        b = true;
    } else if (p - q).abs() / (p + q).min(MAX) < e {
        b = true;
    }
    b
}

#[allow(unused_comparisons)]
pub fn tab(s: &str, space: usize) -> String {
    let l = s.len();
    let mut m: String = String::new();
    let fs = format!("{}{}", " ".repeat(space - l), s);
    m.push_str(&fs);
    return m;
}

pub fn quot_rem(x: usize, y: usize) -> (i32, i32) {
    ((x / y) as i32, (x % y) as i32)
}

// =============================================================================
// Choose Utils
// =============================================================================
pub fn choose_shorter_string(x1: String, x2: String) -> String {
    if x1.len() > x2.len() {
        x2
    } else {
        x1
    }
}

pub fn choose_shorter_vec(x1: &Vector, x2: &Vector) -> Vector {
    if x1.len() > x2.len() {
        x2.clone()
    } else {
        x1.clone()
    }
}

pub fn choose_longer_vec(x1: &Vector, x2: &Vector) -> Vector {
    if x1.len() <= x2.len() {
        x2.clone()
    } else {
        x1.clone()
    }
}
