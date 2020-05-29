//! `peroxide` is comprehensive numerical library for Rust.
//!
//! ## Components
//!
//! `peroxide` has various components for scientific computation.
//!
//! * Linear Algebra (with BLAS & LAPACK)
//!     * [Matrix](structure/matrix/index.html) operations
//!         * `+,-,*,/`
//!         * LU, Determinant, Inverse
//!         * QR Decomposition
//!         * Reduced Row Echelon Form
//!     * [Vector](structure/vector/index.html) operations
//!     * [Redox](redox/index.html) for convenient Vector operations
//!     * [Eigenvalue, Eigenvector](numerical/eigen/index.html) algorithms 
//! * Statistics
//!     * [Statistical operations](statistics/stat/index.html)
//!         * `mean, var, sd`
//!         * `factorial, P, C, H`
//!     * [Distributions](statistics/dist/index.html)
//!         * Bernoulli
//!         * Uniform
//!         * Normal
//!         * Gamma
//!         * Beta
//!         * Student's t
//! * [Special functions](special/function/index.html) (Using `puruspe` crate)
//!     * Gaussian
//!     * Gamma
//!     * Beta
//!     * Error
//!     * Incomplete Gamma
//!     * Incomplete Beta
//! * Automatic Differentiation
//!     * [Dual number system](structure/dual/index.html)
//!     * [Hyper dual number system](structure/hyper_dual/index.html)
//! * Numerical Utils
//!     * [Interpolation](numerical/interp/index.html)
//!     * [Spline](numerical/spline/index.html)
//!     * [Polynomial](structure/polynomial/index.html)
//!     * [Lanczos Approximation](special/lanczos/index.html)
//!     * [Numerical Integrations](numerical/integral/index.html)
//! * [Optimization](numerical/optimize/index.html)
//!     * Gradient Descent
//!     * Levenberg-Marquardt
//! * [Differential Equations](numerical/ode/index.html)
//!     * Explicit
//!         * Runge-Kutta 4th order
//!         * Euler methods
//!     * Implicit
//!         * Backward Euler
//!         * Gauss-Legendre 4th order
//! * Communication with Python
//!     * [Plot with `matplotlib`](util/plot/index.html)
//! * [DataFrame](structure/dataframe/index.html)
//!     * Read & Write with `netcdf` or `csv` format
//! * Macros
//!     * [R macros](macros/r_macro/index.html)
//!     * [Matlab macros](macros/matlab_macro/index.html)
//!     * [Julia macros](macros/julia_macro/index.html)
//!
//! ## Quick Start
//!
//! ### Cargo.toml
//!
//! * To use `peroxide`, you should edit `Cargo.toml`
//! * Current document version is corresponding to `0.21.4`
//!
//! 1. Default
//!     ```toml
//!     [dependencies]
//!     peroxide = "0.21"
//!     ```
//! 2. OpenBLAS & SIMD
//!     ```toml
//!     [dependencies.peroxide]
//!     version = "0.21"
//!     default-features = false
//!     features = ["O3"]
//!     ```
//! 3. Plot
//!     ```toml
//!     [dependencies.peroxide]
//!     version = "0.21"
//!     default-features = false
//!     features = ["plot"]
//!     ```
//! 4. DataFrame
//!     ```toml
//!     [dependencies.peroxide]
//!     version = "0.21"
//!     default-features = false
//!     features = ["dataframe"]
//!     ```
//! 5. Together
//!     ```toml
//!     [dependencies.peroxide]
//!     version = "0.21"
//!     default-features = false
//!     features = ["O3", "plot", "dataframe"]
//!     ```
//!
//! ## Import all at once
//!
//! * You can import all functions & structures at once
//!
//!     ```rust
//!     extern crate peroxide;
//!     use peroxide::*;
//!
//!     fn main() {
//!         //Some codes...
//!     }
//!     ```
//!
//! ## Useful tips for features
//!
//! * After `0.21.4`, if size of matrix is smaller than `1000 x 1000`, default is more effective than `O3` feature.
//! * To plot, use `dataframe` to export data as netcdf format and use python to draw plot.
//!     * `plot` feature has limited plot abilities.
//!     * There is a template of python code. - [Socialst](https://github.com/Axect/Socialst/blob/master/Templates/PyPlot_Template/nc_plot.py)

#[cfg(feature = "O3")]
extern crate blas;

#[cfg(feature = "O3")]
extern crate lapack;

#[cfg(feature = "plot")]
extern crate pyo3;

#[cfg(feature = "simd")]
extern crate packed_simd;

#[cfg(feature = "serde")]
extern crate serde;

extern crate rand;

#[cfg(feature = "dataframe")]
extern crate indexmap;

#[cfg(feature = "dataframe")]
extern crate netcdf;

#[cfg(feature = "dataframe")]
extern crate json;

extern crate order_stat;

extern crate puruspe;

extern crate matrixmultiply;

pub mod statistics;
pub mod structure;
#[macro_use]
pub mod macros;
pub mod ml;
pub mod numerical;
pub mod operation;
pub mod redox;
pub mod special;
pub mod util;
pub mod algorithm;

#[allow(unused_imports)]
pub use macros::{julia_macro::*, matlab_macro::*, r_macro::*};

#[allow(unused_imports)]
pub use structure::matrix::*;

#[allow(unused_imports)]
pub use structure::vector::*;

#[allow(unused_imports)]
pub use statistics::stat::*;

#[allow(unused_imports)]
pub use macros::r_macro::*;

#[allow(unused_imports)]
pub use macros::matlab_macro::*;

#[allow(unused_imports)]
pub use macros::julia_macro::*;

#[allow(unused_imports)]
pub use statistics::rand::*;

#[allow(unused_imports)]
pub use util::print::*;

#[allow(unused_imports)]
pub use util::non_macro::*;

#[allow(unused_imports)]
pub use structure::polynomial::*;

#[allow(unused_imports)]
pub use numerical::interp::*;

#[allow(unused_imports)]
pub use numerical::spline::*;

#[allow(unused_imports)]
pub use ml::reg::*;

#[allow(unused_imports)]
pub use structure::dual::*;

#[allow(unused_imports)]
pub use operation::extra_ops::*;

#[allow(unused_imports)]
pub use util::useful::*;

#[allow(unused_imports)]
pub use structure::multinomial::*;

#[allow(unused_imports)]
pub use numerical::utils::*;

//#[allow(unused_imports)]
//pub use numerical::newton::*;

//#[allow(unused_imports)]
//pub use numerical::bdf::*;

#[allow(unused_imports)]
pub use util::api::*;

//#[allow(unused_imports)]
//pub use numerical::gauss_legendre::*;

#[allow(unused_imports)]
pub use statistics::dist::*;

#[allow(unused_imports)]
pub use special::function::*;

#[allow(unused_imports)]
pub use statistics::ops::*;

#[allow(unused_imports)]
pub use structure::hyper_dual::*;

#[allow(unused_imports)]
pub use util::writer::*;

#[allow(unused_imports)]
pub use operation::mut_ops::*;

#[allow(unused_imports)]
pub use numerical::ode::*;

#[allow(unused_imports)]
pub use operation::number::*;

#[allow(unused_imports)]
#[cfg(feature = "plot")]
pub use util::plot::*;

#[allow(unused_imports)]
pub use numerical::optimize::*;

#[allow(unused_imports)]
pub use redox::redoxable::*;

#[allow(unused_imports)]
pub use util::low_level::*;

#[allow(unused_imports)]
#[cfg(feature = "dataframe")]
pub use structure::dataframe::*;

pub use numerical::eigen::*;

pub use operation::raw_ops::*;

pub use util::wrapper::*;

pub use numerical::integral::*;
