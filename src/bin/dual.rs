extern crate peroxide;
use peroxide::*;

fn main() {
    let a = Dual::new(3, 1);
    (a * a).print();

    let b = Dual::new(0, 1);
    b.cos().print();
}