# Release 0.4.1 (2018-10-23)

* Replace `lu` with `plu`
* Make Algorithm trait for Vector
    * `rank, sign, arg_max`
* Change `PartialEq` for `Matrix`
    * Add `nearly_eq` function
    * Use `nearly_eq` for `Matrix::eq`

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