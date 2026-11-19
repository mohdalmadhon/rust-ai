use statics::Stats;

pub fn main() {
    let data: [f64; 3] = [2.0, 1.0, 3.0];

    match data.mean() {
        Ok(value) => println!("{}", value),
        Err(e) => println!("Error: {}", e),
    }
}