
pub fn merge_sort(data: &mut [f64]) {
    let n: usize = data.len();
    
    if n <= 1 {return}

    let middle: usize = n / 2;

    let mut right: Vec<f64> = Vec::new();
    let mut left: Vec<f64> = Vec::new();

    

    for i in 0..n {
        if i < middle {
            left.push(data[i]); 
        } else {
            right.push(data[i]);
        }
    }
    merge_sort(&mut left);
    merge_sort(&mut right);
    merge(&mut left, &mut right, data);
}

fn merge(left: &mut [f64], right: &mut [f64], arr: &mut [f64]) {
    let left_size: usize = arr.len() / 2;
    let right_size: usize = arr.len() - left_size;
    
    let mut i = 0;
    let mut l = 0;
    let mut r = 0;

    while l < left_size && r < right_size {
        if left[l] < right[r] {
            arr[i] = left[l];
            i += 1;
            l += 1;
        } else {
            arr[i] = right[r];
            i += 1;
            r += 1;
        }
    }
    while l < left_size {
        arr[i] = left[l];
        i += 1;
        l += 1;
    }
    while r < right_size {
        arr[i] = right[r];
        i += 1;
        r += 1;
    }
}