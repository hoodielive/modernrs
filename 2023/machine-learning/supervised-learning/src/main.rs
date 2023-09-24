extern crate serde;
#[macro_use]
extern create serde_derive;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::vec::Vec;
use std::error::Error;
use rand::thread_rng;
use rand::seq::SliceRandom;

// Machine learning is the science of getting computers to act
// without being specifically programmed. This is done by
// implementing special algorithms that have the ability to detect
// patterns in data. From a developer point of view, this means
// creating a system that has access to relevant data, is able
// to feed the data to machine learning algorithms, and is able to
// take the output and redirect it to downstream processes and tasks.

pub struct BostonHousing {
    crim: f64,
    zn: f64,
    indus: f64,
    chas: f64, 
    nox: f64,
    rm: f64,
    age: f64,
    dis: f64,
    rad: f64,
    tax: f64,
    ptratio: f64,
    black: f64,
    lstat: f64,
    medv: f64,
}

impl BostonHousing {
    pub fn new(v: Vec<&str>) -> BostonHousing {
        let f64_formatted: Vec<f64> = v.iter().map(
            |s| s.parse().unwrap()).collect();

        BostonHousing {
           crim: f64_formatted[0],
           zn: f64_formatted[1],
           indus: f64_formatted[2],
           chas: f64_formatted[3],
           nox: f64_formatted[4],
           rm: f64_formatted[5],
           age: f64_formatted[6],
           dis: f64_formatted[7],
           rad: f64_formatted[8],
           tax: f64_formatted[9],
           ptratio: f64_formatted[10],
           black: f64_formatted[11],
           lstat: f64_formatted[12],
           medv: f64_formatted[13],
        }

        pub fn into_feature_vector(&self) -> Vec<f64> {
            vec![self.crim, self.zn, self.indus, self.chas, self.nox,
                 self.rm, self.age, self.dis, self.rad, self.tax, self.ptratio,
                 self.black, self.lstat]
        }

        pub fn into_targets(&self) -> f64 {
            self.medv
        }
    }
}

fn main() {

}
