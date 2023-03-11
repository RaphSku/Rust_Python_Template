use pyo3::prelude::*;
use std::thread::available_parallelism;
use crossbeam_utils::thread::{self};


/// Custom Function
#[pyfunction]
fn parallelized_sum_values_in_array(array: Vec<i32>) -> i32 {
    let num_logical_cores: usize = available_parallelism().unwrap().get();
    let array_size: usize = array.len();
    let elements_per_partition: usize = (array_size / num_logical_cores) + (array_size % num_logical_cores != 0) as usize;

    let mut partitions: Vec<usize> = Vec::with_capacity(num_logical_cores + 1);
    partitions.push(0);
    let mut diff: usize = array_size;
    for i in 0..num_logical_cores {
        if diff < elements_per_partition {
            partitions.push(i * elements_per_partition + diff);
            break;
        }

        partitions.push((i + 1) * elements_per_partition);

        diff -= elements_per_partition;
    }

    println!("partitions: {:?}", partitions);

    let mut subresults: Vec<i32> = Vec::with_capacity(num_logical_cores as usize);
    thread::scope(|s| {
        for partition_index in 0..num_logical_cores {
            let start = &partitions[partition_index];
            let end = &partitions[partition_index + 1];
            let scoped_join_handle = s.spawn(|_| {
                let mut subresult: i32 = 0;
                for i in *start..*end {
                    subresult += array[i];
                }
                return subresult;
            });
            subresults.push(scoped_join_handle.join().unwrap());
        }
    }).unwrap();

    let mut result: i32 = 0;
    for handle in subresults {
        let subresult = handle;
        result += subresult;
    }

    return result;
}


/// Custom Function
#[pyfunction]
fn sum_values_in_array(array: Vec<i32>) -> i32 {
    let array_size: i32 = array.len().try_into().unwrap();
    let mut result: i32 = 0;
    for index in 0..array_size as usize {
        result += array[index];
    }

    return result
}


/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn example_project(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(sum_values_in_array, m)?)?;
    m.add_function(wrap_pyfunction!(parallelized_sum_values_in_array, m)?)?;
    Ok(())
}