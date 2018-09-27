extern crate peroxide;

use peroxide::*;

fn main() {
    let mut a = Matrix::new(vec![1,2,3,4], 2, 2, Row);
    println!("{}", a);
    let b = Matrix::new(vec![1,2,3,4], 2, 2, Col);
    println!("{}", b);
    println!("{}", a.clone() + b.clone());
    println!("{}", a.clone() - b.clone());
}