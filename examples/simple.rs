#[macro_use]
extern crate blackbox_derive;
extern crate blackbox;

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
    const MAX_SCORE: f64 = 4.0;
    let config = Configuration::random_search(100);
    println!("{:?}", config);
    println!("Score: {} (max: {})", config.evaluate(), MAX_SCORE);
}
