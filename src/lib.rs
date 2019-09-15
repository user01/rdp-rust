#![feature(test)]

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use rayon::prelude::*;
// use std::fs;
// use std::path::PathBuf;
#[macro_use] extern crate ndarray;
extern crate test;

use ndarray::{ArrayD, ArrayViewD};
// use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
#[allow(unused_imports)]
use pyo3::{IntoPy, PyObject, ToPyObject};

mod rdp;


#[allow(dead_code)]
fn cross(_x: ArrayViewD<f64>, _y: ArrayViewD<f64>) -> i32 {
    // x[0]
    32
}

#[pyfunction]
fn cross_py(
    py: Python,
    x: &PyArrayDyn<f64>,
    _y: &PyArrayDyn<f64>,
) -> PyObject {
    let x = x.as_array();
    (x[0]).to_object(py)
}

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



/// This module is a python module implemented in Rust.
#[pymodule]
fn rdp_rust(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    m.add_wrapped(wrap_pyfunction!(axpy_py))?;
    m.add_wrapped(wrap_pyfunction!(cross_py))?;

    Ok(())
}
