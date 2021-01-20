use crate::numerical::{
    eigen,
    eigen::{Eigen, EigenMethod::Jacobi},
    integral,
    integral::Integral::GaussLegendre,
};
use crate::structure::matrix::{self, Matrix};
use crate::structure::polynomial;
use crate::traits::math::{Norm, Normed};

/// Simple Norm
pub trait SimpleNorm: Normed {
    fn norm(&self) -> Self::Scalar;
    fn normalize(&self) -> Self;
}

/// Simple integrate
pub fn integrate<F: Fn(f64) -> f64>(f: F, (a, b): (f64, f64)) -> f64 {
    integral::integrate(f, (a, b), GaussLegendre(15))
}

/// Simple Linear algebra
pub trait SimplerLinearAlgebra {
    fn back_subs(&self, b: &Vec<f64>) -> Vec<f64>;
    fn forward_subs(&self, b: &Vec<f64>) -> Vec<f64>;
    fn lu(&self) -> matrix::PQLU;
    fn waz_diag(&self) -> Option<matrix::WAZD>;
    fn waz(&self) -> Option<matrix::WAZD>;
    fn qr(&self) -> matrix::QR;
    fn rref(&self) -> Matrix;
    fn det(&self) -> f64;
    fn block(&self) -> (Matrix, Matrix, Matrix, Matrix);
    fn inv(&self) -> Matrix;
    fn pseudo_inv(&self) -> Matrix;
    fn solve(&self, b: &Vec<f64>) -> Vec<f64>;
    fn solve_mat(&self, m: &Matrix) -> Matrix;
}

/// Simple Eigenpair
pub fn eigen(m: &Matrix) -> Eigen {
    eigen::eigen(m, Jacobi)
}

/// Simple L2 norm
impl SimpleNorm for Vec<f64> {
    fn norm(&self) -> Self::Scalar {
        Normed::norm(self, Norm::L2)
    }

    fn normalize(&self) -> Self {
        Normed::normalize(self, Norm::L2)
    }
}

/// Simple Frobenius norm
impl SimpleNorm for Matrix {
    fn norm(&self) -> Self::Scalar {
        Normed::norm(self, Norm::F)
    }

    fn normalize(&self) -> Self {
        unimplemented!()
    }
}

impl SimplerLinearAlgebra for Matrix {
    fn back_subs(&self, b: &Vec<f64>) -> Vec<f64> {
        matrix::LinearAlgebra::back_subs(self, b)
    }

    fn forward_subs(&self, b: &Vec<f64>) -> Vec<f64> {
        matrix::LinearAlgebra::forward_subs(self, b)
    }

    fn lu(&self) -> matrix::PQLU {
        matrix::LinearAlgebra::lu(self)
    }

    fn waz(&self) -> Option<matrix::WAZD> {
        matrix::LinearAlgebra::waz(self, matrix::Form::Identity)
    }

    fn waz_diag(&self) -> Option<matrix::WAZD> {
        matrix::LinearAlgebra::waz(self, matrix::Form::Diagonal)
    }

    fn qr(&self) -> matrix::QR {
        matrix::LinearAlgebra::qr(self)
    }

    fn rref(&self) -> Matrix {
        matrix::LinearAlgebra::rref(self)
    }

    fn det(&self) -> f64 {
        matrix::LinearAlgebra::det(self)
    }

    fn block(&self) -> (Matrix, Matrix, Matrix, Matrix) {
        matrix::LinearAlgebra::block(self)
    }

    fn inv(&self) -> Matrix {
        matrix::LinearAlgebra::inv(self)
    }

    fn pseudo_inv(&self) -> Matrix {
        matrix::LinearAlgebra::pseudo_inv(self)
    }

    fn solve(&self, b: &Vec<f64>) -> Vec<f64> {
        matrix::LinearAlgebra::solve(self, b, matrix::SolveKind::LU)
    }

    fn solve_mat(&self, m: &Matrix) -> Matrix {
        matrix::LinearAlgebra::solve_mat(self, m, matrix::SolveKind::LU)
    }
}

/// Simple solve
#[allow(non_snake_case)]
pub fn solve(A: &Matrix, m: &Matrix) -> Matrix {
    matrix::solve(A, m, matrix::SolveKind::LU)
}

/// Simple Chebyshev Polynomial (First Kind)
pub fn chebyshev_polynomial(n: usize) -> polynomial::Polynomial {
    polynomial::chebyshev_polynomial(n, polynomial::SpecialKind::First)
}
