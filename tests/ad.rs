extern crate peroxide;
#[allow(unused_imports)]
use peroxide::prelude::*;

#[test]
fn add_test() {
    let a = AD1(2f64, 1f64);
    let b = AD2(4f64, 4f64, 2f64);
    assert_eq!(a + b, AD2(6f64, 5f64, 2f64));
}
