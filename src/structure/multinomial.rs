#[allow(unused_imports)]
use structure::matrix::*;
use structure::vector::*;
use util::useful::*;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Multinomial {
    coef: Vector
}

impl fmt::Display for Multinomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        let l = self.coef.len();

        if l == 1 {
            let value = self.coef[0];
            let target = choose_shorter_string(
                format!("{}x_0", value),
                format!("{:.4}x_0", value),
            );
            return write!(f, "{}", target);
        }

        let first_value = self.coef[0];
        result.push_str(&choose_shorter_string(
            format!("{}x_0", first_value),
            format!("{:.4}x_0", first_value),
        ));

        for i in 1 .. l {
            let value = self.coef[i];
            if value > 0. {
                let target = choose_shorter_string(
                    format!(" + {}x_{}", value, i),
                    format!(" + {:.4}x_{}", value, i),
                );
                result.push_str(&target);
            } else if value < 0. {
                let target = choose_shorter_string(
                    format!(" - {}x_{}", value, i),
                    format!(" - {:.4}x_{}", value, i),
                );
                result.push_str(&target);
            }
        }
        write!(f, "{}", result)
    }
}

impl Multinomial {
    pub fn new(coef: Vector) -> Self {
        Self { coef }
    }

    pub fn eval(&self, values: &Vector) -> f64 {
        self.coef.dot(values)
    }
}
