#[macro_use]
extern crate blackbox_derive;
extern crate blackbox;

use blackbox::BlackboxInput;
use slog::{o, info, Drain};


fn blackbox(c: f64, d: f64) -> f64 {
    c.sin()/c + (d+1.0).sin()/(d+1.0)
}

make_optimizer! {
    Configuration {
        c: f64 = -10.0 .. 10.0,
        d: f64 = -10.0 .. 10.0,
        // e: String = ["a", "b", "c"],
    }
    
    // Arbitrary code follows, which should return f64 (the 'score' to optimize)
    blackbox(c, d)
}

fn main() {
    let decorator = slog_term::TermDecorator::new().build();

    let drain = slog_term::CompactFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();
    let log = slog::Logger::root(drain, o!());

    let max_score: f64 = Configuration {c:0.0001, d:-1.0001}.evaluate(log.clone());
    const N_SAMPLES: usize = 40;
    const N_EXP: usize = 30;
    let mut mean = 0.0;
    for i in 0..N_EXP {
        let log = log.new(o!("Algorithm" => "Random search", "run" => i+1, "total runs" => N_EXP));
        let config = Configuration::random_search(N_SAMPLES, log.clone());
        mean += config.evaluate(log.clone());
    }
    mean /= N_EXP as f64;
    info!(log, "Random searches complete"; "mean score" => mean, "max score" => max_score);


    mean = 0.0;
    for i in 0..N_EXP {
        let log = log.new(o!("Algorithm" => "Bayesian optimization", "run" => i+1, "total runs" => N_EXP));
        let config = Configuration::bayesian_search(2, N_SAMPLES - 2, log.clone());
        mean += config.evaluate(log.clone());
    }
    mean /= N_EXP as f64;
    println!("Score: {} (max: {})", mean, max_score);
}
