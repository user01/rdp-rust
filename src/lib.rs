#![feature(test)]

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
#[macro_use] extern crate ndarray;
extern crate test;

use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
#[allow(unused_imports)]
use pyo3::{IntoPy, PyObject, ToPyObject};


mod rdp;


#[pyfunction]
fn reduce_points(
    py: Python,
    points: &PyArrayDyn<f64>,
    epsilon: f64,
) -> PyResult<Py<PyArrayDyn<f64>>> {
    let the_points = points.as_array().to_owned();
    let indices = rdp::iter(&the_points, epsilon);
    let final_points = rdp::mask(&the_points, &indices);
    Ok(
        final_points.into_dyn().into_pyarray(py).to_owned()
    )
}



/// This module is a python module implemented in Rust.
#[pymodule]
fn rdp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(reduce_points))?;

    Ok(())
}
