use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
#[macro_use]
extern crate ndarray;

use ndarray::Array;
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::{pymodule, Py, PyModule, PyResult, Python};
#[allow(unused_imports)]
use pyo3::{IntoPy, PyObject, ToPyObject};

mod rdp;

/// reduce_points(points, epsilon, /)
/// --
///
/// Perform RDP point reduction on the float64 numpy array of points
/// points must be a numpy array of n by d, where n is the number
/// of points and d is the dimension.
/// Epsilon (float) is the threshold where points are removed as
/// irrelevant, as defined by euclidean distance in the d
/// dimensional space
/// Makes use of the Ramer-Douglas-Peucker algorithm
/// https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
#[pyfunction]
fn reduce_points(
    py: Python,
    points: &PyArrayDyn<f64>,
    epsilon: f64,
) -> PyResult<Py<PyArrayDyn<f64>>> {
    let the_points = unsafe { points.as_array().to_owned() };
    let dims = &the_points.shape().len();
    if *dims != 2 {
        Err(PyValueError::new_err(
            "Incorrect shape. Must be numpy floating of n points by d dimension",
        ))
    } else if epsilon < 0.0 {
        Err(PyValueError::new_err(
            "Epsilon must be a float greater than 0",
        ))
    } else {
        let indices = rdp::iter(&the_points, epsilon);
        let final_points = rdp::mask(&the_points, &indices);
        Ok(final_points.into_dyn().into_pyarray(py).to_owned())
    }
}

/// mask_points(points, epsilon, /)
/// --
///
/// Provide the mask for the RDP point reduction on the float64
/// numpy array of points
/// points must be a numpy array of n by d, where n is the number
/// of points and d is the dimension.
/// Epsilon (float) is the threshold where points are removed as
/// irrelevant, as defined by euclidean distance in the d
/// dimensional space
/// Makes use of the Ramer-Douglas-Peucker algorithm
/// https://en.wikipedia.org/wiki/Ramer%E2%80%93Douglas%E2%80%93Peucker_algorithm
#[pyfunction]
fn mask_points(
    py: Python,
    points: &PyArrayDyn<f64>,
    epsilon: f64,
) -> PyResult<Py<PyArrayDyn<bool>>> {
    let the_points = unsafe { points.as_array().to_owned() };
    let dims = &the_points.shape().len();
    if *dims != 2 {
        Err(PyValueError::new_err(
            "Incorrect shape. Must be numpy floating of n points by d dimension",
        ))
    } else if epsilon < 0.0 {
        Err(PyValueError::new_err(
            "Epsilon must be a float greater than 0",
        ))
    } else {
        let indices = rdp::iter(&the_points, epsilon);
        let result = Array::from_shape_vec(indices.len(), indices).expect("Unable to build");
        Ok(result.into_dyn().into_pyarray(py).to_owned())
    }
}

#[pymodule]
fn rdp_rust(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(reduce_points))?;
    m.add_wrapped(wrap_pyfunction!(mask_points))?;

    Ok(())
}
