//! Sequential Model-Based Optimization
//! This is a type of bayesian optimization that uses the history of samples to pick good new
//! sample points.
//! In general, we use `x` to denote parameters, and `y` to denote score.

use kernel_density::{density::Density, kde};
use crate::*;

// IMPLEMENTATION NOTE: the algorithm tries to minimize the output y. So we take y = -score.


/// Returns all points that have been tried, with respective scores
pub fn optimize<T: BlackboxInput>(n_iter: usize) -> Vec<(T, f64)> {
    let domains = T::get_domains();
    let mut history = Vec::new();
    // Start with random evaluation
    {
        let x = T::random();
        let y = x.evaluate();
        history.push((x, y));
    }


    for _ in 0..n_iter {
        history.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        let params = select_candidate(&history, &domains);
    }
    history
}

/// history needs to be sorted, and have at least one element
fn select_candidate<T: BlackboxInput>(history: &Vec<(T, f64)>, domains: &Vec<Variable>) -> T {
    const BANDWIDTH: f64 = 0.5;
    // mu = p(y < y_target). In other words, it's the fraction of scores from the history which should be
    // better than `y_target`, when selectind `y_target`.
    const MU: f64 = 0.2;

    let y_target = history[(MU * history.len() as f64) as usize].1;

    // TODO: rewrite kernel_density so that we don't have to filter, then collect, then pass
    // reference then (internally) clone

    // Construct g(x) and l(x) for each variable x
    let g: Vec<Box<dyn Density>> = domains.iter().enumerate().map(|(i, domain)| {
        kde::normal(history.iter()
                                        .filter(|h| h.1 > MU)
                                        .map(|h| h.0.get(i).as_num())
                                        .collect(),
                                        BANDWIDTH)
    }).collect();
    let l: Vec<Box<dyn Density>> = domains.iter().enumerate().map(|(i, domain)| {
        kde::normal(history.iter()
                                        .filter(|h| h.1 < MU)
                                        .map(|h| h.0.get(i).as_num())
                                        .collect(),
                                        BANDWIDTH)
    }).collect();
                       

    for (domain, g, l) in izip!(domains, &g, &l) {
    }
    unimplemented!()
}
