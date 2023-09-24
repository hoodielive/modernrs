extern crate serde;
#[macro_use]
extern crate serde_derive;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::File;
use std::vec::Vec;
use std::error::Error;
use rand::thread_rng;
use rand::seq::SliceRandom;
use supervised_learning::datasets::get_boston_records_file;
use rusty_machine;
use rusty_machine::linalg::Matrix;
use rusty_machine::linalg::Vector;

pub fn run() -> Result<(), Box<dyn Error>> {
    let fl = data/housing.csv;
    let mut data = get_boston_records_file(&fl);

    data.shuffle(&mut thread_rng());

    // Separate to train and test datasets.

    let test_size: f64 = 0.2;
    let test_size: f64 = data.len() as f64 * test_size;
    let test_size = test_size.round() as usize;
    let (test_data, train_data) = data.split_at(test_size);
    let train_size = train_data.len();
    let test_size = test_data.len();

    // Differentiate the predictors and the targets.
    
    let boston_x_train: Vec<f64> = train_data.iter()
        .flat_map(|r| r.into_feature_vector())
        .collect();
    
    let boston_y_train: Vec<f64> = train_data.iter()
        .map(|r| r.into_targets()).collect();
  
    let boston_x_test: Vec<f64> = test_data.iter()
        .flat_map(|r| r.into_feature_vector()).collect();

    let boston_y_test: Vec<f64> = test_data.iter()
        .map(|r| r.into_targets()).collect();

    let boston_x_train = Matrix::new(train_size, 13, boston_x_train);
    let boston_y_train = Vector::new(boston_y_train);
    let boston_x_test = Matrix::new(test_size, 13, boston_x_test);
    let boston_y_test = Matrix::new(test_size, 1, boston_y_test);
}


