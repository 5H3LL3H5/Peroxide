use operation::extra_ops::{ExpLogOps, PowOps, TrigOps};
use std::convert::Into;
use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};
#[allow(unused_imports)]
use structure::vector::*;
use structure::dual::dual;
use Real;

/// Hyper Dual number
///
/// # Description
///
/// For second order differentiation
#[derive(Debug, Copy, Clone, Default)]
pub struct HyperDual {
    x: f64,
    dx: f64,
    ddx: f64,
}

impl fmt::Display for HyperDual {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s1 = format!(
            "value: {}\nslope: {}\naccel: {}",
            self.x,
            self.dx,
            self.ddx
        );
        write!(f, "{}", s1)
    }
}

impl HyperDual {
    pub fn new<T: Into<f64> + Copy>(x: T, dx: T, ddx: T) -> Self {
        Self {
            x: x.into(),
            dx: dx.into(),
            ddx: ddx.into(),
        }
    }

    pub fn value(&self) -> f64 {
        self.x
    }

    pub fn slope(&self) -> f64 {
        self.dx
    }

    pub fn accel(&self) -> f64 {
        self.ddx
    }

    pub fn extract(&self) -> (f64, f64, f64) {
        (self.x, self.dx, self.ddx)
    }
}

pub fn hyper_dual<T: Into<f64> + Copy>(x: T, dx: T, ddx: T) -> HyperDual {
    HyperDual::new(x, dx, ddx)
}

impl Neg for HyperDual {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.dx, -self.ddx)
    }
}

impl Add<HyperDual> for HyperDual {
    type Output = Self;

    fn add(self, rhs: HyperDual) -> Self::Output {
        Self::new(
            self.x + rhs.x,
            self.dx + rhs.dx,
            self.ddx + rhs.ddx,
        )
    }
}

impl Sub<HyperDual> for HyperDual {
    type Output = Self;

    fn sub(self, rhs: HyperDual) -> Self::Output {
        Self::new(
            self.x - rhs.x,
            self.dx - rhs.dx,
            self.ddx - rhs.ddx,
        )
    }
}

impl Mul<HyperDual> for HyperDual {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let (x, dx, ddx) = self.extract();
        let (y, dy, ddy) = rhs.extract();

        Self::new(
            x * y,
            dx*y + x*dy,
            ddx*y + 2f64*dx*dy + x*ddy
        )
    }
}

impl Div<HyperDual> for HyperDual {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        assert_ne!(rhs.x, 0f64);
        let (x, dx, ddx) = self.extract();
        let (y, dy, ddy) = rhs.extract();

        let dual_x = dual(x, dx);
        let dual_y = dual(y, dy);

        let x_div_y = (dual_x / dual_y).slope();

        Self::new(
            x/y,
            (dx*y - x*dy)/(y*y),
            (ddx - 2f64*x_div_y*dy - x/y*ddy)/y
        )
    }
}

impl Add<f64> for HyperDual {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        self + Self::new(rhs, 0., 0.)
    }
}

impl Sub<f64> for HyperDual {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Self::new(rhs, 0., 0.)
    }
}

impl Mul<f64> for HyperDual {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Self::new(rhs, 0., 0.)
    }
}

impl Div<f64> for HyperDual {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / Self::new(rhs, 0., 0.)
    }
}

impl Add<HyperDual> for f64 {
    type Output = HyperDual;

    fn add(self, rhs: HyperDual) -> Self::Output {
        rhs.add(self)
    }
}

impl Sub<HyperDual> for f64 {
    type Output = HyperDual;

    fn sub(self, rhs: HyperDual) -> Self::Output {
        -rhs.sub(self)
    }
}

impl Mul<HyperDual> for f64 {
    type Output = HyperDual;

    fn mul(self, rhs: HyperDual) -> Self::Output {
        rhs.mul(self)
    }
}

impl Div<HyperDual> for f64 {
    type Output = HyperDual;

    fn div(self, rhs: HyperDual) -> Self::Output {
        hyper_dual(self, 0., 0.) / rhs
    }
}

impl TrigOps for HyperDual {
    type Output = Self;

    fn sin(&self) -> Self::Output {
        let x = self.x.sin();
        let dx = self.dx * self.x.cos();
        let ddx = self.ddx * self.x.cos() - self.dx.powi(2) * x;
        Self::new(x, dx, ddx)
    }

    fn cos(&self) -> Self::Output {
        let x = self.x.cos();
        let dx = - self.dx * self.x.sin();
        let ddx =  - self.ddx * self.x.sin() - self.dx.powi(2) * x;
        Self::new(x, dx, ddx)
    }

    fn tan(&self) -> Self::Output {
        let x = self.x.tan();
        let dx = self.dx * (1f64 + x.powi(2));
        let ddx = self.ddx * (1f64 + x.powi(2)) + dx * self.dx * 2f64*x;
        Self::new(x, dx, ddx)
    }

    fn asin(&self) -> Self::Output {
        unimplemented!()
    }

    fn acos(&self) -> Self::Output {
        unimplemented!()
    }

    fn atan(&self) -> Self::Output {
        unimplemented!()
    }

    fn sinh(&self) -> Self::Output {
        unimplemented!()
    }

    fn cosh(&self) -> Self::Output {
        unimplemented!()
    }

    fn tanh(&self) -> Self::Output {
        unimplemented!()
    }

    fn asinh(&self) -> Self::Output {
        unimplemented!()
    }

    fn acosh(&self) -> Self::Output {
        unimplemented!()
    }

    fn atanh(&self) -> Self::Output {
        unimplemented!()
    }

    fn sin_cos(&self) -> (Self::Output, Self::Output) {
        unimplemented!()
    }
}

impl ExpLogOps for HyperDual {
    type Output = Self;

    fn exp(&self) -> Self::Output {
        let x = self.x.exp();
        let dx = self.dx * x;
        let ddx = self.ddx * x + self.dx.powi(2) * x;
        Self::new(x, dx, ddx)
    }

    fn ln(&self) -> Self::Output {
        assert!(self.x > 0f64, "Logarithm Domain Error");
        let x = self.x.ln();
        let dx = self.dx / self.x;
        let ddx = self.ddx / self.x - self.dx.powi(2)/self.x.powi(2);
        Self::new(x, dx, ddx)
    }

    fn log(&self, base: f64) -> Self::Output {
        self.ln() / base.ln()
    }

    fn log2(&self) -> Self::Output {
        unimplemented!()
    }

    fn log10(&self) -> Self::Output {
        unimplemented!()
    }
}

impl PowOps for HyperDual {
    type Output = Self;

    fn powi(&self, n: i32) -> Self::Output {
        let mut s = self.clone();
        for _i in 1 .. n {
            s = s * s;
        }
        s
    }

    fn powf(&self, f: f64) -> Self::Output {
        let x = self.x.powf(f);
        let dx = self.dx * f * self.x.powf(f - 1f64);
        let ddx = self.ddx * f * self.x.powf(f - 1f64)
            + self.dx.powi(2) * f * (f - 1f64) * self.x.powf(f - 2f64);
        Self::new(x, dx, ddx)
    }

    fn sqrt(&self) -> Self::Output {
        self.powf(0.5)
    }
}

impl Real for HyperDual {
    fn to_f64(&self) -> f64 {
        self.x
    }

    fn from_f64(f: f64) -> Self {
        HyperDual::new(f, 0f64, 0f64)
    }
}