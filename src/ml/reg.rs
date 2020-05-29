#[allow(unused_imports)]
use statistics::stat::*;
#[allow(unused_imports)]
use structure::matrix::*;
#[allow(unused_imports)]
use structure::polynomial::*;
#[allow(unused_imports)]
use structure::vector::*;

/// Simple Least Square 2D
///
/// # Type
///
/// (Vec<f64>, Vec<f64>) -> Polynomial
///
/// # Examples
/// ```
/// extern crate peroxide;
/// use peroxide::*;
///
/// let a = c!(1,2,3,4,5);
/// let b = c!(1.2, 1.8, 3.2, 3.8, 5.0);
/// let ls = least_square(a, b);
/// ls.print(); // 0.96x + 0.1200
/// ```
pub fn least_square(node_x: Vec<f64>, node_y: Vec<f64>) -> Polynomial {
    let l = node_x.len();
    assert_eq!(l, node_y.len());

    let mut x_bar = 0f64;
    let mut t_bar = 0f64;
    let mut xt_bar = 0f64;
    let mut x_sq_bar = 0f64;
    for i in 0..l {
        let x = node_x[i];
        let t = node_y[i];

        x_bar += x;
        t_bar += t;
        xt_bar += x * t;
        x_sq_bar += x * x;
    }
    x_bar /= l as f64;
    t_bar /= l as f64;
    xt_bar /= l as f64;
    x_sq_bar /= l as f64;

    let x_bar_sq = x_bar.clone() * x_bar.clone();
    let x_bar_t_bar = x_bar.clone() * t_bar.clone();

    let w1 = (xt_bar - x_bar_t_bar) / (x_sq_bar - x_bar_sq);
    let w0 = t_bar - w1 * x_bar;

    Polynomial::new(vec![w1, w0])
}

// Polynomial Regression
//pub fn poly_reg(node_x: Vec<f64>, node_y: Vec<f64>) -> Polynomial {
//    let n = node_x.len();
//    assert_eq!(n, node_y.len());
//    let a = matrix(vec![1f64; n], n, 1, Col);
//
//}
