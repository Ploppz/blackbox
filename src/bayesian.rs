use rusty_machine::{
    learning::{gp::*, toolkit::kernel::*, SupModel},
    linalg::{Matrix, Vector}
};
use crate::{Variable, Domain, BlackboxInput};

use probability::distribution::{Gaussian, Continuous, Distribution};

use std::cmp::max;


/// Gaussian Process with a kernel that handles discrete variables
pub struct GPSurrogate<T: BlackboxInput> {
    gp: GaussianProcess<CustomSquaredExp, ConstMean>,
    _phantom: std::marker::PhantomData<T>,
}
impl<T: BlackboxInput> GPSurrogate<T> {
    pub fn new(inputs: &Matrix<f64>, outputs: &Vector<f64>)
            -> GPSurrogate<T> {
        let kernel = CustomSquaredExp::new(1.0, 1.0, T::get_domains());
        let mut gp = GaussianProcess::new(kernel, ConstMean::default(), 0.0);
        gp.train(inputs, outputs).unwrap();
        GPSurrogate {
            gp,
            _phantom: std::marker::PhantomData,
        }
    }

    /// Maximize EI in this surrogate. For now a naive search. Return input that maximizes it.
    pub fn maximize(&self, best_score: f64) -> T {
        // Take N random samples...
        // Page 7 (https://arxiv.org/pdf/1807.02811.pdf) mentions better approaches
        const N: usize = 400;
        let mut best_input = None;
        let mut best_surrogate_output = std::f64::NEG_INFINITY;
        for i in 0..N {
            let input = T::random();
            let output = self.expected_improvement(&input.to_numbers(), best_score);
            if output > best_surrogate_output {
                best_surrogate_output = output;
                best_input = Some(input);
            }
        }
        best_input.unwrap()
    }

    /// Evaluates the expected improvement (EI) in a single point
    pub fn expected_improvement(&self, x: &[f64], best_score: f64) -> f64 {
        // page 7, equation 8 https://arxiv.org/pdf/1807.02811.pdf
        let std_gaussian = Gaussian::new(0.0, 1.0);
        let (mean, variance) = self.evaluate_in(x);
        let delta = mean - best_score;
        delta.max(0.0) + variance * std_gaussian.density(delta/variance)
                      - delta.abs() * std_gaussian.distribution(delta/variance)

    }

    /// Evaluates the posterior of the GP in a single point. Returns (mean, variance)
    pub fn evaluate_in(&self, x: &[f64]) -> (f64, f64) {
        // TODO should I take posterior or prior??
        let (mean, variance) = self.gp.get_posterior(&Matrix::new(1, x.len(), x)).unwrap();

        (mean[0], variance[[0,0]]) // because we only sample one point
    }

}

/// Takes special care of discrete variables (https://arxiv.org/pdf/1706.03673.pdf).
/// Is also "Automatic Relevance Determination" (ARD) (https://stats.stackexchange.com/a/362537/182715) (TODO)
pub struct CustomSquaredExp {
    /// length scale
    l: f64,
    ampl: f64,
    domains: Vec<Variable>,
    /// Which input variables are discrete
    discrete: Vec<bool>,

}
impl CustomSquaredExp {
    pub fn new(l: f64, ampl: f64, domains: Vec<Variable>) -> CustomSquaredExp {
        let discrete = domains.iter()
            .map(|x| if let Domain::Discrete{low:_, high:_} = x.domain { true } else { false }).collect();
        CustomSquaredExp {
            l,
            ampl,
            domains,
            discrete,
        }
    }
}
impl Kernel for CustomSquaredExp {
    fn kernel(&self, x1: &[f64], x2: &[f64]) -> f64 {
        let mse = x1.iter().zip(x2).enumerate().map(|(i, (x1, x2))| {
            if self.discrete[i] {
                (x1.round() - x2.round()).powf(2.0)
            } else {
                (x1-x2).powf(2.0)
            }
        }).sum::<f64>();
        self.ampl * (-mse / (2.0 * self.l.powf(2.0))).exp()
    }
}
