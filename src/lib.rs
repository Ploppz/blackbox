#[macro_use]
extern crate itertools;

pub mod bayesian;

use bayesian::*;


pub trait BlackboxInput: Sized + Clone + std::fmt::Debug {
    fn evaluate(&self) -> f64;
    /// Sample randomly from the domain
    fn random() -> Self;
    fn n_variables() -> usize;

    fn get_domains() -> Vec<Variable>;

    fn to_numbers(&self) -> Vec<f64>;

    fn bayesian_search(init_samples: usize, max_iter: usize) -> Self {
        use rusty_machine::linalg::Matrix;
        assert!(init_samples < max_iter);

        let to_matrix = |source: &[Self]| {
            let flat: Vec<f64> = source.iter().map(|x| x.to_numbers()).flatten().collect();
            Matrix::new(flat.len() / Self::n_variables(), Self::n_variables(), flat)
        };

        // println!("= Initial samples =");
        let mut best_x = None;
        let mut best_y = std::f64::NEG_INFINITY;

        let mut x = Vec::<Self>::new();
        let mut y = Vec::<f64>::new();
        for i in 0..init_samples {
            let sample_x = Self::random();
            let sample_y = sample_x.evaluate();
            if sample_y > best_y {
                best_x = Some(sample_x.clone());
                best_y = sample_y;
            }
            x.push(sample_x);
            y.push(sample_y);

        }
        
        for i in init_samples..max_iter {
            // println!("= Iter {}/{} =", i+1, max_iter-init_samples+1);
            let surrogate = GPSurrogate::<Self>::new(&to_matrix(&x), &y.clone().into());
            let sample_x = surrogate.maximize(best_y);
            let sample_y = sample_x.evaluate();

            if sample_y > best_y {
                best_x = Some(sample_x.clone());
                best_y = sample_y;
            }
            x.push(sample_x);
            y.push(sample_y);
        }
        best_x.unwrap()
    }

    fn grid_search(max_iter: Option<usize>) -> Self {
        let config = Self::random();
        unimplemented!()
    }
    fn random_search(max_iter: usize) -> Self {
        let mut config = Self::random();

        let mut best_score = std::f64::NEG_INFINITY;
        let mut best_config = config.clone();
        let mut i = 0;
        loop {
            // Sample random configuration
            config = Self::random();
            // Evaluate
            let score = config.evaluate();
            if score > best_score {
                best_score = score;
                best_config = config.clone();
            }

            i += 1;
            if i >= max_iter {
                break;
            }
        }

        best_config
    }
}



pub struct Variable {
    pub domain: Domain,
    // TODO: distribution
}

pub enum Domain {
    Real {low: f64, high: f64},
    Discrete {low: i32, high: i32},
}

pub enum Value {
    Real (f64),
    Discrete (i32),
}
impl Value {
    pub fn as_num(&self) -> f64 {
        match *self {
            Value::Real(x) => x,
            Value::Discrete(n) => n as f64,
        }
    }
}
