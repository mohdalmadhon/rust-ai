use crate::utils::num::Num;


pub fn arange<T: Num, U: Num, Y: Num>(start: T, end: Y, step: U) -> Option<Vec<f64>> {
    let start = start.to_f64();
    let end = end.to_f64();
    let step = step.to_f64();

    if start == end || !(step > 0.0) {
        return None;
    }

    let n = ((end - start).abs() / step).ceil() as usize;
    let dir = if end > start { step } else { -step };

    Some((0..n).map(|i| start + i as f64 * dir).collect())
}