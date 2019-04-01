# Release 0.9.0 (2019-04-01) (Candidates)

* Modify `Pickle` trait

# Release 0.8.13 (2019-04-01)

* `HyperDual` for 2nd order Automatic Differentiation

# Release 0.8.12 (2019-03-31)

* Implement Tri-Diagonal Matrix Algorithm
    * Add `tdma` to `numerical/utils.rs`

# Release 0.8.11 (2019-03-31)

* Modify `matrix/lu` to correct doolittle algorithm

# Release 0.8.10 (2019-03-25)

* Add two dependencies in `Cargo.toml`
    * `serde`
    * `serde_pickle`
* Add `pickle.rs` to `util`
    * Write `Vec<f64>` to pickle file easily
    * Write `Matrix` to pickle file (Caution: It depends on shape of matrix)
* Fix all warnings from compiler
    
# Release 0.8.9 (2019-03-24)

* New constructor in `matrix.rs`
    * Matrix from index operations - `from_index<F>(F, (usize, usize))`
* Update print of Matrix
    * Extend limit from `10x10` to `100x10` or `10x100` or `20x20`
* Fix bug of `take` of `FPMatrix`
    * Early return if size is smaller than row or column

# Release 0.8.8 (2019-03-23)

* Add `ops.rs` to `statistics`
    * Factorial: `factorial(n)`
    * Permutation: `P(n,r)`
    * Combination: `C(n,r)`
    * Combination with Repetition: `H(n,r)`

# Release 0.8.7 (2019-03-08)

* Add constraint to uniform distribution
* Reduce & modify `README.md`
    * Add missing modules to module structure
    * Remove `Usage` section (Move to [Peroxide Gitbook](https://axect.github.io/Peroxide_Gitbook))
    

# Release 0.8.6 (2019-03-05)

* Add [Peroxide Gitbook](https://axect.github.io/Peroxide_Gitbook) link to `README`
* Fix `statistics/rand.rs`, `statistics/dist.rs`
    * pub use rand crate to private use
    * Now you can use `rand(usize, usize)` function in `util/non_macro.rs`

# Release 0.8.5 (2019-02-17)

* Fix bugs of `cbind`, `rbind`
* Add Linear Discriminant (Least Square) example

# Release 0.8.4 (2019-02-14)

* Fix complete pivoting
* Add `det` test in `tests`
* Add `tov` in `bin` - Tolman-Oppenheimer-Volkoff equation

# Release 0.8.3 (2019-02-13)

* Fix error of `Sub<Dual> for f64`
* Fix error of `Div<Dual> for f64`

# Release 0.8.2 (2019-02-12)

* Bump `rand` dependency to 0.6 (Thanks to [koute](https://github.com/koute))
* Fix error of `powf` operation of `dual`
    * Now, it works fine. 

# Release 0.8.1 (2019-02-04)

* Fix errors of test

# Release 0.8.0 (2019-02-04) (Yanked)

* Fix `write`, `write_with_header`
    * Move `round` parameter to `write_round`, `write_with_header_round`
* Add `solve_with_condition` to `ode.rs`
    * Now, you can give stop condition to ode solver.

# Release 0.7.7 (2019-01-27)

* Add various distributions in `dist.rs`
    * `Bernoulli(mu)`
    * `Beta(a, b)`

# Release 0.7.6 (2019-01-21)

* Modify `write`, `write_with_header`
    * Now there is round option

# Release 0.7.5 (2019-01-21)

* Fix error of `bdf.rs`

# Release 0.7.4 (2019-01-21)

* Modify `bdf.rs`
    * Put `max_iter = 10`
    * Simplify non-autonomous jacobian
    
# Release 0.7.3 (2019-01-19)

* Move distributions(`Uniform`, `Normal`) from `rand.rs` to `dist.rs`
    * Now `Uniform` & `Normal` are enums
    * Remove `Uniform::new` & `Normal::new`
* Add `special/function.rs`
    * Add `gaussian`
    

# Release 0.7.2 (2019-01-18)

* Implement `GL4` - Gauss-Legendre 4th order
    * Add `GL4(f64)` to `ODEMethod`

# Release 0.7.1 (2019-01-17)

* Add `take` to `FP` trait for Matrix
* Add `skip` to `FP` trait for Matrix
* Fix `fmt::Display` of `Matrix`
    * If larger than 10x10 -> Only print 10x10 part
* Add `ode.rs` to `numeric`
    * Add `solve` - numerical solve ODE
    * Now you can choose two methods
        * `RK4`
        * `BDF1`
* Change `rk4`
    * All functions should have form
        * `f(Dual, Vec<Dual>) -> Vec<Dual>`
* Fix error of `spread`

# Release 0.7.0 (2019-01-15)

* Modify matrix declaration
    * `p_matrix` -> `py_matrix`
    * `m_matrix` -> `ml_matrix`
    * Add `r_matrix` (same as `matrix`)
* Add `util/api.rs`
    * Can choose various coding style
        * `MATLAB`
        * `PYTHON`
        * `R`
* Remove `CreateMatrix`
    * Deprecated `Matrix::new` -> Use `matrix` instead

# Release 0.6.15 (2019-01-12)

* Update `matrix.rs`
    * Add `p_matrix`, `m_matrix`
        * Pythonic matrix
        * MATLAB matrix
    * Add `write_with_header` for `matrix.rs`
        * Now, can write matrix with header

# Release 0.6.14 (2019-01-05)

* Add `runge_kutta.rs`
    * Implement RK4 algorithm for Non-autonomous equation

# Release 0.6.13 (2019-01-03)

* Add `grave`
* Move `rok4a.rs` to `grave`

# Release 0.6.12 (2019-01-03)

* Fix error of `Div` for `Dual`

# Release 0.6.11 (2018-12-31)

* Add `rok4a.rs`
    * Now, deprecated
* Add `non_auto_jacobian` to utils
    * TODO: Generalize jacobian
* Add `bdf.rs`
    * Implement Backward Euler Method

# Release 0.6.10 (2018-12-27)

* Add comfortable tools for `Vec<Dual>`
* Add `jacobian` in `numerical/utils`
* Add `newton` in `numerical`
    * Newton-Raphson Method 

# Release 0.6.9 (2018-12-26)

* **Fix error of `inv`**
    * Reverse order of permutations

# Release 0.6.8 (2018-12-25)

* Update `Dual`
    * Also add `Ops<Dual> for f64`

# Release 0.6.7 (2018-12-24)

* Add `multinomial.rs`
    * Implement `print`, `eval`
    * TODO: Partial eval?
* Update `Dual`
    * Add `Add<f64> for Dual`
    * Add `Sub<f64> for Dual`
    * Add `Mul<f64> for Dual`
    * Add `Div<f64> for Dual`

# Release 0.6.6 (2018-11-30)

* Update `FPVector`
    * Add `filter, take, drop`

# Release 0.6.5 (2018-11-29)

* Update `read`
    * Move `read` to `Matrix::read`
    * Can set `delimiter`

# Release 0.6.4 (2018-11-28)

* Add `pseudo_inv` method for `Matrix`
* New `useful.rs` in util
    * Move `tab, quot_rem, nearly_eq` from `matrix.rs` to `useful.rs`
    * Move `choose_*` from `polynomial.rs` to `useful.rs` 
* Fix error of `VectorOps` - `dot`

# Release 0.6.3 (2018-11-28)

* Fix typo of `fmt::Display` for `Polynomial` 
* Fix module structures - Thanks to md-file-tree

# Release 0.6.2 (2018-11-26)

* Implement Horner's Algorithm - `Div` for Polynomial
* TODO: Fix `print` of Polynomial (Fixed!)
* Add `Dual` for Automatic Differentiation to structure
    * structure
        * matrix
        * vector
        * polynomial
        * dual
* Make `operation` directory & add `extra_ops.rs`

# Release 0.6.1 (2018-11-18)

* Fix `README`

# Release 0.6.0 (2018-11-18)

* Implement `Calculus` for `Polynomial`
* Re-construct all module structures
    * structure
        * matrix.rs
        * vector.rs
        * polynomial.rs
    * statistics
        * stat.rs
        * rand.rs
    * macros
        * r_macro.rs
        * matlab_macro.rs
    * util
        * print.rs
* Add `numerical` directory
    * Add interp.rs
        * Lagrange Polynomial (`lagrange_polynomial`)
        * Chebyshev Nodes (`chebyshev_nodes`)
    * Add spline.rs
        * Natural Cubic Spline (`cubic_spline`)
* Impl `pow` for Polynomial
* Fixed `fmt::Display` for Polynomial

# Release 0.5.8 (2018-11-16)

* Add `print.rs` for print any values conveniently
    * Implement `print` for Vector
    * Implement `print` for Matrix
    * Implement `print` for `f32, f64, usize, u32, u64, i32, i64`
* Add `poly.rs` to deal `Polynomial`
    * Implement `fmt::Display` for Polynomial
    * Add `new`
    * Implement `eval` for Polynomial
    * Implement `Neg, Add, Sub, Mul<T>` for Polynomial
    * Implement `Mul, Add<T>, Sub<T>, Div<T>` for Polynomial

# Release 0.5.7 (2018-11-13)

* Change gaussian generate method
    * `marsaglia_polar` to `ziggurat`
* Add comments and examples to `rand.rs`

# Release 0.5.6 (2018-11-13)

* Add `linspace` to `matlab_macro`
* Fixed `linspace` export error
* Add `rand.rs`
    * Generic `Rand` structure
    * `sample` method
    * Marsaglia Polar
    * `Rand` to `Uniform` and `Normal`

# Release 0.5.5 (2018-11-06)

* Extend `matrix` macro to single valued matrix
* Make `lm`
* And also make `lm` macro - `lm!(y ~ x)`
* Make `LinearOps` Trait - But not necessary

# Release 0.5.4 (2018-11-05)

* Add badges to README
* Fix README - add cargo.toml
* Modify `std::ops` for Matrix
    * `f64` to generic
    * Add comments
    * Matmul for `Matrix` vs `Vector` vice versa

# Release 0.5.3 (2018-11-05)

* Add `eye` to `matlab_macro`
* Extend `zeros` to matrix
* Fix `cov` for `Vec<f64>` - not consume anymore
* Add `cor`
* Update `README`

# Release 0.5.2 (2018-11-03)

* Add `matlab_macro`

# Release 0.5.1 (2018-11-02)

* Add `read`
    * Can read matrix from csv
* Add comment to `write`, `read`
* Fix all README

# Release 0.5.0 (2018-11-01)

* Add `write` for `Matrix`
    * Can write matrix to csv!

# Release 0.4.9 (2018-10-30)

* Modify `block`, `inv_u`, `combine`
    * Just change code syntax

# Release 0.4.8 (2018-10-30)

* Modify `lu`
    * Just change code syntax

# Release 0.4.7 (2018-10-30)

* Add `IndexMut` implementation for `Matrix`
* Modify `Rem`
    * Just using `IndexMut`
    * Very fast!

# Release 0.4.6 (2018-10-29)

* Fixed `block` & `combine`
    * Only squared matrices -> Every matrices

# Release 0.4.5 (2018-10-26)

* More add R-like macro
    * `cbind`
    * `rbind`
* README update

# Release 0.4.4 (2018-10-26)

* Refactor structure
    * Move all macro to `r_macro.rs`

# Release 0.4.3 (2018-10-24)

* Add `stat.rs`
    * `mean, var, sd`
* Modify `spread`
    * Fix bugs of all cases
    * Use modern syntax

# Release 0.4.2 (2018-10-24)

* Fix `Cargo.toml`

# Release 0.4.1 (2018-10-24)

* Replace `lu` with `plu`
* Make Algorithm trait for Vector
    * `rank, sign, arg_max`
* Change `PartialEq` for `Matrix`
    * Add `nearly_eq` function
    * Use `nearly_eq` for `Matrix::eq`
* Add `swap`
* Make `PQLU` structure
* Remove `pivot`, `to_perm`
* Replace `plu` with `lu`
    * `lu` returns `Option<PQLU>`
* Enhance error handling with `lu, det, inv`
* Complete Pivoting LU Decomposition
* Fix error of `lu` - initialize `u`

# Release 0.4.0 (2018-10-23)

* Remove non-necessary comments
* Remove `vec2mat, mat2vec`
* Change `col`, `row` functions
    * `col, row` returns `Vec<f64>`
* Add `diag`
* Add `det`
* Add `reduce` in `vector_macro`
* Add `inv_l`, `inv_u`
* Add `block`, `combine`
* Fix error of `block`, `combine`
* Fix error of `inv_l`
* Add `inv`

# Release 0.3.1 (2018-10-21)

* Remove `Vector` struct
    * Replace with `vector_macro`
    * `c!` & `seq!`
* Make R-like matrix macro
    * `matrix!(1;4;1, 2, 2, Row)`

# Release 0.3.0 (2018-10-20)

* Vector
* `seq` : moved from matrix to vector
* Rename `Generic` trait - `CreateMatrix`

# Release 0.2.5 (2018-10-19)

* LU Decomposition

# Release 0.2.4 (2018-10-19)

* `matrix` function - Same as R
* Fix `README.md`
* More documentation

# Release 0.2.3 (2018-10-18)

* `seq` function - Same as R
* Extract Col & Row
    * `a.col(1)` : Extract 1st column of `a` as Column matrix
    * `a.row(1)` : Extract 1st row of `a` as Row matrix

# Release 0.2.1 (2018-10-04)

* Update Documentation
    * Update README

# Release 0.2.0 (2018-10-04)

* Change structure
    * remove `ozone`
