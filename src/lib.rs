
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

use ndarray::{ArrayD, ArrayViewD};
// use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
use pyo3::{IntoPy, PyObject, ToPyObject};


pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

// This is a really bad adding function, its purpose is to fail in this
// example.
#[allow(dead_code)]
fn bad_add(a: i32, b: i32) -> i32 {
    a - b
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }

    #[test]
    fn test_bad_add() {
        // This assert would fire and test will fail.
        // Please note, that private functions can be tested too!
        assert_eq!(bad_add(1, 2), 3);
    }
}

fn cross(x: ArrayViewD<f64>, y: ArrayViewD<f64>) -> i32 {
    // x[0]
    32
}

#[pyfunction]
fn cross_py(
    py: Python,
    x: &PyArrayDyn<f64>,
    y: &PyArrayDyn<f64>,
) -> PyObject {
    let x = x.as_array();
    (x[0]).to_object(py)
}
// #[pyfunction]
// fn cross_py(
//     py: Python,
//     x: &PyArrayDyn<f64>,
//     y: &PyArrayDyn<f64>,
// ) -> Py<i32> {
//     let x = x.as_array();
//     let y = y.as_array();
//     // cross(x, y).into_py(py)
//     // cross(x, y)
//     cross(x, y).into_py(py)
// }



// @jit(nopython=True, fastmath=True, cache=True, parallel=False)
// def np_cross(a, b):
//     """
//     Simple numba compatible cross product of vectors.

//     Parameters
//     ----------
//     a : [type]
//         [description]
//     b : [type]
//         [description]

//     Returns
//     -------
//     [type]
//         [description]
//     """
//     x = a[1] * b[2] - a[2] * b[1]
//     y = a[2] * b[0] - a[0] * b[2]
//     z = a[0] * b[1] - a[1] * b[0]
//     return np.array([x, y, z])


fn axpy(a: f64, x: ArrayViewD<f64>, y: ArrayViewD<f64>) -> ArrayD<f64> {
    a * &x + &y
}

#[pyfunction]
fn axpy_py(
    py: Python,
    a: f64,
    x: &PyArrayDyn<f64>,
    y: &PyArrayDyn<f64>,
) -> Py<PyArrayDyn<f64>> {
    let x = x.as_array();
    let y = y.as_array();
    axpy(a, x, y).into_pyarray(py).to_owned()
}


#[pyfunction]
/// Formats the sum of two numbers as string
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
/// Junk
fn torres(a: f64, b: f64) -> PyResult<String> {
    Ok((a * b).to_string())
}



/// Represents a file that can be searched
#[pyclass(module = "word_count")]
struct WordCounter {
    path: PathBuf,
}

#[pymethods]
impl WordCounter {
    #[new]
    fn new(obj: &PyRawObject, path: String) {
        obj.init(WordCounter {
            path: PathBuf::from(path),
        });
    }

    /// Searches for the word, parallelized by rayon
    fn search(&self, py: Python<'_>, search: String) -> PyResult<usize> {
        let contents = fs::read_to_string(&self.path)?;

        let count = py.allow_threads(move || {
            contents
                .par_lines()
                .map(|line| count_line(line, &search))
                .sum()
        });
        Ok(count)
    }

    /// Searches for a word in a classic sequential fashion
    fn search_sequential(&self, needle: String) -> PyResult<usize> {
        let contents = fs::read_to_string(&self.path)?;

        let result = contents.lines().map(|line| count_line(line, &needle)).sum();

        Ok(result)
    }
}

fn matches(word: &str, needle: &str) -> bool {
    let mut needle = needle.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match needle.next() {
            None => {
                return !ch.is_alphabetic();
            }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return needle.next().is_none();
}

/// Count the occurrences of needle in line, case insensitive
#[pyfunction]
fn count_line(line: &str, needle: &str) -> usize {
    let mut total = 0;
    for word in line.split(' ') {
        if matches(word, needle) {
            total += 1;
        }
    }
    total
}






/// This module is a python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(torres))?;
    m.add_wrapped(wrap_pyfunction!(axpy_py))?;
    m.add_wrapped(wrap_pyfunction!(cross_py))?;

    m.add_wrapped(wrap_pyfunction!(count_line))?;
    m.add_class::<WordCounter>()?;

    Ok(())
}
