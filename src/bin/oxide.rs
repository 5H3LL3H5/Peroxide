extern crate peroxide;

use peroxide::*;

fn main() {
    let a = Matrix::new(vec![1,2,3,4], 2, 2, Row);
    println!("{}", a);
    let b = Matrix::new(vec![1,2,3,4], 2, 2, Col);
    println!("{}", b);
    // println!("{}", a.clone() + b.clone());
    // println!("{}", a.clone() - b.clone());
    println!("{}", a.clone() * b.clone());
    // println!("{}", a.fmap(|x| x + 1f64));
    // println!("{}", a.reduce(1, |x,y| x*y));
    println!("{}", a.zip_with(|x,y| x * y, &b));
    // println!("{}", a.clone() + 1f64);
    println!("{}", a.clone() % b.clone());

    let c = seq(1,10,1);
    println!("{}", c);
}