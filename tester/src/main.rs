use statics::*;

use std::time::Instant;

fn main() {
    let start = Instant::now();

    let array = statics::arange(0, 100000000, 1);

    let elapsed = start.elapsed();

    
    println!("done!");
    println!("Time taken: {:.6} seconds", elapsed.as_secs_f64());
}