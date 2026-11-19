use crate::utils::num::Num;

pub fn linear_regression<T: Num>(x: &[T], y: &[T]) -> Option<(f64, f64)> {
    let n = x.len();
    if n < 2 || n != y.len() {
        return None;
    }
    let n_f = n as f64;

    // Do all the math in f64
    let mean_x = x.iter().map(|&v| v.to_f64()).sum::<f64>() / n_f;
    let mean_y = y.iter().map(|&v| v.to_f64()).sum::<f64>() / n_f;

    let (mut num, mut den) = (0.0, 0.0);
    for (&xi, &yi) in x.iter().zip(y) {
        let dx = xi.to_f64() - mean_x;
        num += dx * (yi.to_f64() - mean_y);
        den += dx * dx;
    }
    if den == 0.0 {
        return None;
    }
    let slope = num / den;
    Some((slope, mean_y - slope * mean_x))
}