# Peroxide

[![On crates.io](https://img.shields.io/crates/v/peroxide.svg)](https://crates.io/crates/peroxide)
[![On docs.rs](https://docs.rs/peroxide/badge.svg)](https://peroxide.surge.sh)

[![builds.sr.ht status](https://builds.sr.ht/~axect/Peroxide/.build.yml.svg)](https://builds.sr.ht/~axect/Peroxide/.build.yml?)
[![travis](https://api.travis-ci.org/Axect/Peroxide.svg?branch=master)](https://travis-ci.org/Axect/Peroxide)
![github](https://github.com/Axect/Peroxide/workflows/Github/badge.svg) 

![maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

Rust numeric library contains linear algebra, numerical analysis, statistics and machine learning tools with R, MATLAB, Python like macros.

## Why Peroxide?

### 1. Customize features

Peroxide provides various features.

* `default` - Pure Rust (No dependencies of architecture - Perfect cross compilation)
* `O3` - SIMD + OpenBLAS (Perfect performance but hard to set-up - Strongly recommend to read [OpenBLAS for Rust](https://github.com/Axect/Issues/tree/master/Rust))
* `plot` - With matplotlib of python, we can draw any plots.
* `dataframe` - Dataframe & netcdf
* `serde` - serialization with [Serde](https://serde.rs/).

If you want to do high performance computation, then choose openblas feature.
If you don't want to depend C/C++ or Fortran libraries, then choose default feature.
If you want to draw plot with some great templates, then choose plot feature.

You can choose any features simultaneously.
 
### 2. Easy to optimize

Peroxide uses 1D data structure to describe matrix. So, it's too easy to integrate BLAS & SIMD.
It means peroxide guarantees perfect performance for linear algebraic computations.

### 3. Friendly syntax

Rust is so strange for Numpy, MATLAB, R users. Thus, it's harder to learn the more rusty libraries.
With peroxide, you can do heavy computations with R, Numpy, MATLAB like syntax.

For example,

```rust
extern crate peroxide;
use peroxide::*;

fn main() {
    // MATLAB like matrix constructor
    let a = ml_matrix("1 2;3 4");

    // R like matrix constructor (default)
    let b = matrix(c!(1,2,3,4), 2, 2, Row);

    // Or use zeros
    let mut z = zeros(2, 2);
    z[(0,0)] = 1.0;
    z[(0,1)] = 2.0;
    z[(1,0)] = 3.0;
    z[(1,1)] = 4.0;
    
    // Simple but effective operations
    let c = a * b; // Matrix multiplication (BLAS integrated)

    // Easy to pretty print
    c.print();
    //       c[0] c[1]
    // r[0]     1    3
    // r[1]     2    4

    // Easy to do linear algebra
    c.det().print();
    c.inv().unwrap().print();

    // and etc.
}
```

### 4. Batteries included

Peroxide can do many things. 

* Linear Algebra
    * Effective Matrix structure
    * Transpose, Determinant, Diagonal
    * LU Decomposition, Inverse matrix, Block partitioning
    * QR Decomposition
    * Reduced Row Echelon form
    * Column, Row operations
    * Eigenvalue, Eigenvector
* Functional Programming
    * More easy functional programming with `Vec<f64>`
    * For matrix, there are three maps
        * `fmap` : map for all elements
        * `col_map` : map for column vectors
        * `row_map` : map for row vectors
* Automatic Differentiation
    * Dual number system - for 1st order AD
    * Hyper dual number system - for 2nd order AD
    * Exact jacobian
    * `Real` trait to constrain for `f64` and `Dual`
    * `Number` structure to unify `f64` and `Dual`
* Numerical Analysis
    * Lagrange interpolation
    * Cubic spline
    * Non-linear regression
        * Gradient Descent
        * Gauss Newton
        * Levenberg Marquardt
    * Ordinary Differential Equation
        * Euler
        * Runge Kutta 4th order
        * Backward Euler
        * Gauss Legendre 4th order
    * Numerical Integration
        * Gauss-Legendre Quadrature
* Statistics
    * More easy random with `rand` crate
    * Ordered Statistics
        * Median
        * Quantile (Matched with R quantile)
    * Probability Distributions
        * Bernoulli
        * Uniform
        * Normal
        * Gamma
        * Beta
        * Student's-t
    * RNG algorithms
        * Acceptance Rejection
        * Marsaglia Polar
        * Ziggurat
        * Wrapper for `rand-dist` crate
* Special functions
    * Wrapper for `puruspe` crate (pure rust)
* Utils
    * R-like macro & functions
    * Matlab-like macro & functions
    * Numpy-like macro & functions
    * Julia-like macro & functions
* Plotting
    * With `pyo3` & `matplotlib`
* DataFrame
    * Convert with Matrix
    * Read & Write `csv` files
    * Read & Write `netcdf` files

### 5. Written in Rust

Rust & Cargo are awesome for scientific computations. 
You can use any external packages easily with Cargo, not make.
And default runtime performance of Rust is also great. If you use many iterations for computations,
then Rust become great choice.

## Latest README version

Corresponding to `0.21.7`

## Pre-requisite

* For `O3` feature - Need `OpenBLAS`
* For `plot` feature - Need `matplotlib` of python
* For `dataframe` feature - Need `netcdf`

## Install

* Add next block to your `cargo.toml`

1. Default
    ```toml
   [dependencies]
   peroxide = "0.21"
    ```
2. OpenBLAS + SIMD
    ```toml
   [dependencies.peroxide]
   version = "0.21"
   default-features = false
   features = ["O3"] 
   ```
3. Plot
    ```toml
   [dependencies.peroxide]
   version = "0.21"
   default-features = false
   features = ["plot"] 
   ```
4. DataFrame
    ```toml
   [dependencies.peroxide]
   version = "0.21"
   default-features = false
   features = ["dataframe"]
   ```
4. OpenBLAS + SIMD & Plot & DataFrame
    ```toml
   [dependencies.peroxide]
   version = "0.21"
   default-features = false
   features = ["O3", "plot", "dataframe"] 
   ```

## Useful tips for features

* After `0.21.4`, if size of matrix is smaller than `1000 x 1000`, default is more effective than `O3` feature.
* To plot, use `dataframe` to export data as netcdf format and use python to draw plot.
    * `plot` feature has limited plot abilities.
    * There is a template of python code. - [Socialst](https://github.com/Axect/Socialst/blob/master/Templates/PyPlot_Template/nc_plot.py)

## Module Structure

- __src__
  - [lib.rs](src/lib.rs) : `mod` and `re-export`
  - __macros__ : Macro files
    - [julia_macro.rs](src/macros/julia_macro.rs) : Julia like macro
    - [matlab_macro.rs](src/macros/matlab_macro.rs) : MATLAB like macro
    - [mod.rs](src/macros/mod.rs)
    - [r_macro.rs](src/macros/r_macro.rs) : R like macro
  - __ml__ : For machine learning (*Beta*)
      - [mod.rs](src/ml/mod.rs)
      - [reg.rs](src/ml/reg.rs) : Regression tools
  - __numerical__ : To do numerical things
    - [bdf.rs](src/numerical/bdf.rs) : Backward Differentiation Formula (deprecated)
    - [eigen.rs](src/numerica;/eigen.rs) : Eigenvalue, Eigenvector algorithm
    - [interp.rs](src/numerical/interp.rs) : Interpolation
    - [mod.rs](src/numerical/mod.rs)
    - [newton.rs](src/numerical/newton.rs) : Newton's Method
    - [ode.rs](src/numerical/ode.rs) : Main ODE solver with various algorithms
    - [optimize.rs](src/numerical/optimize.rs) : Non-linear regression
    - [spline.rs](src/numerical/spline.rs) : Natural Spline
    - [utils.rs](src/numerical/utils.rs) : Utils to do numerical things (e.g. jacobian)
  - __operation__ : To define general operations
    - [extra_ops.rs](src/operation/extra_ops.rs) : Missing operations & Real Trait
    - [mut_ops.rs](src/operation/mut_ops.rs) : Mutable operations
    - [mod.rs](src/operation/mod.rs)
    - [number.rs](src/operation/number.rs) : Number type (include `f64`, `Dual`)
  - __special__ : Special functions written in pure Rust
    - [mod.rs](src/special/mod.rs)
    - [function.rs](src/special/function.rs) : Special functions
  - __statistics__ : Statistical Tools
    - [mod.rs](src/statistics/mod.rs)
    - [dist.rs](src/statistics/dist.rs) : Probability distributions
    - [ops.rs](src/statistics/ops.rs) : Some probabilistic operations
    - [rand.rs](src/statistics/rand.rs) : Wrapper for `rand` crate
    - [stat.rs](src/statistics/stat.rs) : Statistical tools
  - __structure__ : Fundamental data structures
    - [dataframe.rs](src/structure/dataframe.rs) : Dataframe
    - [dual.rs](src/structure/dual.rs) : Dual number system for automatic differentiation
    - [hyper_dual.rs](src/structure/hyper_dual.rs) : Hyper dual number system for automatic differentiation
    - [matrix.rs](src/structure/matrix.rs) : Matrix
    - [multinomial.rs](src/structure/multinomial.rs) : For multinomial (*Beta*)
    - [mod.rs](src/structure/mod.rs)
    - [polynomial.rs](src/structure/polynomial.rs) : Polynomial
    - [vector.rs](src/structure/vector.rs) : Extra tools for `Vec<f64>`
  - __util__
    - [mod.rs](src/util/mod.rs)
    - [api.rs](src/util/api.rs) : Matrix constructor for various language style 
    - [non_macro.rs](src/util/non_macro.rs) : Primordial version of macros
    - [plot.rs](src/util/plot.rs) : To draw plot (using `pyo3`)
    - [print.rs](src/util/print.rs) : To print conveniently
    - [useful.rs](src/util/useful.rs) : Useful utils to implement library
    - [wrapper.rs](src/util/wrapper.rs) : Wrapper for other crates (e.g. rand)
    - [writer.rs](src/util/writer.rs) : More convenient write system


## Documentation

* [![On docs.rs](https://docs.rs/peroxide/badge.svg)](https://peroxide.surge.sh)

## Example

[Peroxide Gallery](https://github.com/Axect/Peroxide_Gallery)

## Version Info

To see [RELEASES.md](./RELEASES.md)

## Contributes Guide

See [CONTRIBUTES.md](./CONTRIBUTES.md)

## TODO

To see [TODO.md](./TODO.md)
