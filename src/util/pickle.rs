extern crate serde;
extern crate serde_pickle;

use structure::matrix::*;
use structure::vector::*;
use std::fs::File;
use std::io::Write;
use std::process::exit;

/// Pickle trait
///
/// # Description
///
/// Use python pickle to export vector or matrix
pub trait Pickle {
    fn write_pickle(&self, path: &str) -> serde_pickle::Result<()>;
}

impl Pickle for Vector {
    fn write_pickle(&self, path: &str) -> serde_pickle::Result<()> {
        let mut writer: Box<Write>;

        match File::create(path) {
            Ok(p) => writer = Box::new(p),
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        }

        serde_pickle::to_writer(&mut writer, &self, true)
    }
}

impl Pickle for Matrix {
    fn write_pickle(&self, path: &str) -> serde_pickle::Result<()> {
        let mut writer: Box<Write>;

        match File::create(path) {
            Ok(p) => writer = Box::new(p),
            Err(e) => {
                println!("{:?}", e);
                exit(1);
            }
        }

        let mut container: Vec<Vec<f64>> = Vec::new();;

        match self.shape {
            Row => {
                for i in 0 .. self.row {
                    container.push(self.row(i));
                }
            },
            Col => {
                for i in 0 .. self.col {
                    container.push(self.col(i));
                }
            }
        }

        serde_pickle::to_writer(&mut writer, &container, true)
    }
}