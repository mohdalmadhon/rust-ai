use crate::errors::{Result, StatsErrors};
use crate::utils::num::Num;
use crate::utils::sort::merge_sort;

fn to_sorted_f64<T: Num>(data: &[T]) -> Vec<f64> {
    let mut v: Vec<f64> = data.iter().map(|x| x.to_f64()).collect();
    merge_sort(&mut v);
    v
}

pub fn mean<T: Num>(data: &[T]) -> Result<f64> {
    let n: usize = data.len();
    if n == 0 {
        return Err(StatsErrors::EmptyInput);
    }
    let sum: f64 = data.iter().map(|x| x.to_f64()).sum();
    Ok(sum / n as f64)
}

pub fn median<T: Num>(data: &[T]) -> Result<f64> {
    let n: usize = data.len();
    if n == 0 {
        return Err(StatsErrors::EmptyInput);
    }
    let v = to_sorted_f64(data);

    if n % 2 == 0 {
        Ok((v[n / 2 - 1] + v[n / 2]) / 2.0)
    } else {
        Ok(v[n / 2])
    }
}

pub fn mode<T: Num>(data: &[T]) -> Result<Vec<f64>> {
    if data.is_empty() {
        return Err(StatsErrors::EmptyInput);
    }

    let v = to_sorted_f64(data);

    let mut modes: Vec<f64> = Vec::new();
    let mut best = 0;
    let mut i = 0;

    while i < v.len() {
        let mut j = i;
        while j < v.len() && v[j] == v[i] {
            j += 1;
        }
        let count = j - i;

        if count > best {
            best = count;
            modes.clear();
            modes.push(v[i]);
        } else if count == best {
            modes.push(v[i]);
        }

        i = j;
    }

    Ok(modes)
}