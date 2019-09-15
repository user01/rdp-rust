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


// #[allow(dead_code)]
// fn cross(_x: ArrayViewD<f64>, _y: ArrayViewD<f64>) -> i32 {
//     // x[0]
//     32
// }

#[pyfunction]
fn cross_py(
    py: Python,
    _x: &PyArrayDyn<f64>,
    _y: &PyArrayDyn<f64>,
) -> Py<PyArrayDyn<f64>> {
    // let x = x.as_array();
    // (x[0]).to_object(py)

    let the_points = arr2(&[[1., 1.], [2.0, 2.0], [3.0, 3.0], [5.0, 3.0], [5.0, 5.0]]);
    // println!("The points {:?}", the_points);
    let indices = rdp::iter(&the_points, 0.1);
    let final_points = rdp::mask(&the_points, &indices);
    // let _msk = Array::from_vec(indices);
    // let _g = Array::from_vec(final_points);
    // // g.
    // // g.into_dyn().into_pyarray(py).to_owned()
    let array = arr2(&[[1.0, 2.0],[3.0, 4.0]]);
    println!("{:?}", array);
    // println!("{:?}", _g);
    // let array = arr2(&final_points);
    // g.view().into_dyn().into_pyarray(py).to_owned()
    // array.into_dyn().into_pyarray(py).to_owned()
    final_points.into_dyn().into_pyarray(py).to_owned()
    // _g.into_dyn().into_pyarray(py).to_owned()
    // array
    // g.to_owned()
}

// fn axpy(a: f64, x: ArrayViewD<f64>, y: ArrayViewD<f64>) -> ArrayD<f64> {
//     a * &x + &y
// }

// #[pyfunction]
// fn axpy_py(
//     py: Python,
//     a: f64,
//     x: &PyArrayDyn<f64>,
//     y: &PyArrayDyn<f64>,
// ) -> Py<PyArrayDyn<f64>> {
//     let x = x.as_array();
//     let y = y.as_array();
//     let res = axpy(a, x, y);
//     res.into_pyarray(py).to_owned()
// }


// #[pyfunction]
// /// Formats the sum of two numbers as string
// fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
//     Ok((a + b).to_string())
// }



/// This module is a python module implemented in Rust.
#[pymodule]
fn rdp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    // m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    // m.add_wrapped(wrap_pyfunction!(axpy_py))?;
    m.add_wrapped(wrap_pyfunction!(cross_py))?;

    Ok(())
}
