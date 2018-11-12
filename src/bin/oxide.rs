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

    let v_u32 = Rand::new((1u32, 11),  Uniform);
    v_u32.sample(10).print();
    let v_f64 = Rand::new((1f64, 11f64), Uniform);
    v_f64.sample(10).print();

    println!("{}", erf(1.0));

    let v_n = Rand::new((1f64, 2f64), Normal {m: 0., s: 1.});
    println!("{}", v_n.sample(1000).mean());
    println!("{}", v_n.sample(1000).sd());
}