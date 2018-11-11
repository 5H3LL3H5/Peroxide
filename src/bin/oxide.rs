extern crate peroxide;

use peroxide::*;
#[allow(unused_imports)]
use std::process;

#[allow(unused_must_use)]
fn main() {
    let a = linspace!(0,1,11);
    a.print();
    let b = seq!(0,1,0.1);
    b.print();
    let r1 = rand!();
    let r2 = rand!(4, 2);
    println!("{}", r1);
    r2.print();
}