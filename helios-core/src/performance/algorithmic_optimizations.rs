//! Algorithmic Performance Optimizations
//!
//! This module provides optimized algorithms for common data visualization tasks,
//! including sorting, searching, and mathematical operations.

use std::cmp::Ordering;

/// Optimized quicksort implementation with early termination for nearly sorted data
pub fn optimized_quicksort<T, F>(arr: &mut [T], compare: F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.len() <= 1 {
        return;
    }
    
    // Use insertion sort for small arrays
    if arr.len() <= 10 {
        insertion_sort(arr, &compare);
        return;
    }
    
    // Check if array is already sorted
    if is_sorted(arr, &compare) {
        return;
    }
    
    let pivot_index = partition(arr, &compare);
    optimized_quicksort(&mut arr[..pivot_index], &compare);
    optimized_quicksort(&mut arr[pivot_index + 1..], &compare);
}

fn insertion_sort<T, F>(arr: &mut [T], compare: &F)
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        let key = arr[i].clone();
        let mut j = i;
        
        while j > 0 && compare(&arr[j - 1], &key) == Ordering::Greater {
            arr[j] = arr[j - 1].clone();
            j -= 1;
        }
        
        arr[j] = key;
    }
}

fn is_sorted<T, F>(arr: &[T], compare: &F) -> bool
where
    F: Fn(&T, &T) -> Ordering,
{
    for i in 1..arr.len() {
        if compare(&arr[i - 1], &arr[i]) == Ordering::Greater {
            return false;
        }
    }
    true
}

fn partition<T, F>(arr: &mut [T], compare: &F) -> usize
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    let len = arr.len();
    let pivot = arr[len - 1].clone();
    let mut i = 0;
    
    for j in 0..len - 1 {
        if compare(&arr[j], &pivot) != Ordering::Greater {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len - 1);
    i
}

/// Optimized binary search with early termination
pub fn optimized_binary_search<T, F>(arr: &[T], target: &T, compare: F) -> Option<usize>
where
    F: Fn(&T, &T) -> Ordering,
{
    let mut left = 0;
    let mut right = arr.len();
    
    while left < right {
        let mid = left + (right - left) / 2;
        
        match compare(&arr[mid], target) {
            Ordering::Equal => return Some(mid),
            Ordering::Less => left = mid + 1,
            Ordering::Greater => right = mid,
        }
    }
    
    None
}

/// Fast interpolation search for uniformly distributed data
pub fn interpolation_search<T, F>(arr: &[T], target: &T, compare: F) -> Option<usize>
where
    T: Clone,
    F: Fn(&T, &T) -> Ordering,
{
    if arr.is_empty() {
        return None;
    }
    
    let mut left = 0;
    let mut right = arr.len() - 1;
    
    while left <= right {
        // Calculate interpolation position
        let left_val = &arr[left];
        let right_val = &arr[right];
        
        if compare(left_val, right_val) == Ordering::Equal {
            if compare(left_val, target) == Ordering::Equal {
                return Some(left);
            } else {
                return None;
            }
        }
        
        let pos = left + ((right - left) as f64 * 
            (compare(target, left_val) as i32 as f64) / 
            (compare(right_val, left_val) as i32 as f64)) as usize;
        
        if pos > right || pos < left {
            break;
        }
        
        match compare(&arr[pos], target) {
            Ordering::Equal => return Some(pos),
            Ordering::Less => left = pos + 1,
            Ordering::Greater => right = pos - 1,
        }
    }
    
    None
}

/// Optimized matrix multiplication using blocking for cache efficiency
pub fn optimized_matrix_multiply(a: &[Vec<f64>], b: &[Vec<f64>]) -> Vec<Vec<f64>> {
    let rows_a = a.len();
    let cols_a = a[0].len();
    let cols_b = b[0].len();
    
    let mut result = vec![vec![0.0; cols_b]; rows_a];
    
    // Block size for cache optimization
    const BLOCK_SIZE: usize = 64;
    
    for i in (0..rows_a).step_by(BLOCK_SIZE) {
        for j in (0..cols_b).step_by(BLOCK_SIZE) {
            for k in (0..cols_a).step_by(BLOCK_SIZE) {
                // Process block
                let i_end = (i + BLOCK_SIZE).min(rows_a);
                let j_end = (j + BLOCK_SIZE).min(cols_b);
                let k_end = (k + BLOCK_SIZE).min(cols_a);
                
                for ii in i..i_end {
                    for jj in j..j_end {
                        let mut sum = 0.0;
                        for kk in k..k_end {
                            sum += a[ii][kk] * b[kk][jj];
                        }
                        result[ii][jj] += sum;
                    }
                }
            }
        }
    }
    
    result
}

/// Fast Fourier Transform for signal processing
pub fn fft(input: &[f64]) -> Vec<f64> {
    let n = input.len();
    if n <= 1 {
        return input.to_vec();
    }
    
    // Ensure n is a power of 2
    let next_power_of_2 = n.next_power_of_two();
    let mut padded_input = input.to_vec();
    padded_input.resize(next_power_of_2, 0.0);
    
    // Cooley-Tukey FFT algorithm
    fft_recursive(&mut padded_input)
}

fn fft_recursive(data: &mut [f64]) -> Vec<f64> {
    let n = data.len();
    if n <= 1 {
        return data.to_vec();
    }
    
    // Divide
    let mut even = Vec::new();
    let mut odd = Vec::new();
    
    for i in (0..n).step_by(2) {
        even.push(data[i]);
        if i + 1 < n {
            odd.push(data[i + 1]);
        }
    }
    
    // Conquer
    let even_fft = fft_recursive(&mut even);
    let odd_fft = fft_recursive(&mut odd);
    
    // Combine
    let mut result = vec![0.0; n];
    for i in 0..n/2 {
        let angle = -2.0 * std::f64::consts::PI * i as f64 / n as f64;
        let cos_val = angle.cos();
        let sin_val = angle.sin();
        
        let t_real = cos_val * odd_fft[i] - sin_val * 0.0; // Assuming real input
        let t_imag = sin_val * odd_fft[i] + cos_val * 0.0;
        
        result[i] = even_fft[i] + t_real;
        result[i + n/2] = even_fft[i] - t_real;
    }
    
    result
}

/// Optimized moving average calculation
pub fn optimized_moving_average(data: &[f64], window_size: usize) -> Vec<f64> {
    if data.is_empty() || window_size == 0 {
        return Vec::new();
    }
    
    let mut result = Vec::with_capacity(data.len());
    let mut sum = 0.0;
    
    // Calculate initial window
    for i in 0..window_size.min(data.len()) {
        sum += data[i];
    }
    
    // First result
    if window_size <= data.len() {
        result.push(sum / window_size as f64);
    }
    
    // Sliding window
    for i in window_size..data.len() {
        sum = sum - data[i - window_size] + data[i];
        result.push(sum / window_size as f64);
    }
    
    result
}

/// Fast percentile calculation using quickselect
pub fn fast_percentile(data: &mut [f64], percentile: f64) -> f64 {
    if data.is_empty() {
        return 0.0;
    }
    
    let k = ((data.len() - 1) as f64 * percentile / 100.0) as usize;
    quickselect(data, k)
}

fn quickselect(arr: &mut [f64], k: usize) -> f64 {
    if arr.len() == 1 {
        return arr[0];
    }
    
    let pivot_index = partition_float(arr);
    
    match k.cmp(&pivot_index) {
        Ordering::Equal => arr[pivot_index],
        Ordering::Less => quickselect(&mut arr[..pivot_index], k),
        Ordering::Greater => quickselect(&mut arr[pivot_index + 1..], k - pivot_index - 1),
    }
}

fn partition_float(arr: &mut [f64]) -> usize {
    let len = arr.len();
    let pivot = arr[len - 1];
    let mut i = 0;
    
    for j in 0..len - 1 {
        if arr[j] <= pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    
    arr.swap(i, len - 1);
    i
}

/// Optimized data sampling for large datasets
pub fn optimized_sampling<T>(data: &[T], sample_size: usize) -> Vec<T>
where
    T: Clone,
{
    if data.len() <= sample_size {
        return data.to_vec();
    }
    
    let mut result = Vec::with_capacity(sample_size);
    let step = data.len() as f64 / sample_size as f64;
    
    for i in 0..sample_size {
        let index = (i as f64 * step) as usize;
        result.push(data[index].clone());
    }
    
    result
}

/// Fast correlation calculation using optimized algorithms
pub fn fast_correlation(x: &[f64], y: &[f64]) -> f64 {
    if x.len() != y.len() || x.is_empty() {
        return 0.0;
    }
    
    let n = x.len() as f64;
    
    // Calculate means
    let mean_x = x.iter().sum::<f64>() / n;
    let mean_y = y.iter().sum::<f64>() / n;
    
    // Calculate correlation
    let mut numerator = 0.0;
    let mut sum_x_sq = 0.0;
    let mut sum_y_sq = 0.0;
    
    for i in 0..x.len() {
        let dx = x[i] - mean_x;
        let dy = y[i] - mean_y;
        
        numerator += dx * dy;
        sum_x_sq += dx * dx;
        sum_y_sq += dy * dy;
    }
    
    let denominator = (sum_x_sq * sum_y_sq).sqrt();
    
    if denominator == 0.0 {
        0.0
    } else {
        numerator / denominator
    }
}
