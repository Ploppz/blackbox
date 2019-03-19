#[macro_use]
extern crate blackbox_derive;
extern crate blackbox;

use blackbox::BlackboxInput;


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
    let MAX_SCORE: f64 = Configuration {c:0.0001, d:-1.0001}.evaluate();
    let N_SAMPLES: usize = 40;
    let N_EXP: usize = 30;
    let mut mean = 0.0;
    println!(" = Random search =");
    for i in 0..N_EXP {
        let config = Configuration::random_search(N_SAMPLES);
        mean += config.evaluate();
    }
    mean /= N_EXP as f64;
    println!("Score: {} (max: {})", mean, MAX_SCORE);


    mean = 0.0;
    println!(" = Bayesian optimization =");
    for i in 0..N_EXP {
        let config = Configuration::bayesian_search(2, N_SAMPLES - 2);
        mean += config.evaluate();
    }
    mean /= N_EXP as f64;
    println!("Score: {} (max: {})", mean, MAX_SCORE);
}
