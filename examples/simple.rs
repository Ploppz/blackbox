#[macro_use]
extern crate blackbox_derive;
extern crate blackbox;
use slog::o;

use blackbox::BlackboxInput;


fn blackbox(c: f64, d: usize) -> f64 {
    c * d as f64
}

make_optimizer! {
    Configuration {
        c: f64 = 0.0 .. 1.0,
        d: usize = 0 .. 5,
        // e: String = ["a", "b", "c"],
    }
    
    // Arbitrary code follows, which should return f64 (the 'score' to optimize)
    blackbox(c, d)
}

fn main() {
    let drain = slog::Discard;
    let log = slog::Logger::root(drain, o!());

    const MAX_SCORE: f64 = 4.0;
    println!(" = Random search =");
    let config = Configuration::random_search(10, log.clone());
    println!("{:?}", config);
    println!("Score: {} (max: {})", config.evaluate(log.clone()), MAX_SCORE);

    println!(" = Bayesian optimization =");
    let config = Configuration::bayesian_search(3, 10, log.clone());
    println!("{:?}", config);
    println!("Score: {} (max: {})", config.evaluate(log), MAX_SCORE);
}
