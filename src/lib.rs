#![feature(test)]

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
// use rayon::prelude::*;
// use std::fs;
// use std::path::PathBuf;
#[macro_use] extern crate ndarray;
extern crate test;

// use ndarray::{ArrayD, ArrayViewD};
// use ndarray::{ArrayD, ArrayViewD, ArrayViewMutD};
#[allow(unused_imports)]
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
#[allow(unused_imports)]
use pyo3::{IntoPy, PyObject, ToPyObject};

use ndarray::arr2;
use ndarray::Array;

mod rdp;


#[pyfunction]
fn cross_py(
    py: Python,
    points: &PyArrayDyn<f64>,
    epsilon: f64,
) -> PyResult<Py<PyArrayDyn<f64>>> {

    // let the_points = arr2(&[[1., 1.], [2.0, 2.0], [3.0, 3.0], [5.0, 3.0], [5.0, 5.0]]);
    let the_points = points.as_array().to_owned();
    // println!("The points {:?}", the_points);
    let indices = rdp::iter(&the_points, epsilon);
    let final_points = rdp::mask(&the_points, &indices);

    // println!("{:?}", array);
    Ok(
        final_points.into_dyn().into_pyarray(py).to_owned()
    )
}



/// This module is a python module implemented in Rust.
#[pymodule]
fn rdp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(cross_py))?;

    Ok(())
}
