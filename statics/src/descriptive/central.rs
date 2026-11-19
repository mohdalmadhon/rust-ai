use crate::Sort;
use crate::errors::{Result, StatsErrors};
use crate::utils::sort::merge_sort;

pub fn mean<T>(data: &[f64]) -> Result<f64> 
where T: Copy + Into<f64>
{
    let n: usize = data.len();
    if n == 0 {
        return Err(StatsErrors::EmptyInput);
    }
    Ok(data.iter().sum::<f64>() / n as f64)
}

pub fn median(data: &[f64]) -> Result<f64> {
    let n: usize = data.len();
    if n == 0 {
        return Err(StatsErrors::EmptyInput);
    }
    let mut vec: Vec<f64> = data.to_vec();
    vec.merge_sort();

    if n % 2 == 0 {
        Ok((vec[n / 2 - 1] + vec[n / 2]) / 2.0)
    } else {
        Ok(vec[n / 2])
    }
} 

pub fn mode(data: &[f64]) -> Result<Vec<f64>> {
    if data.is_empty() {
        return Err(StatsErrors::EmptyInput);
    }

    let mut v = data.to_vec();
    merge_sort(&mut v);

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