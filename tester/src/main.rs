use statics::Stats;

pub fn main() {
    let data: [f64; 3] = [1.0, 1.0, 3.0];

    match data.percentile() {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error: {}", e),
    }
}