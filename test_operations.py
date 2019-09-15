import numpy as np
import rdp_rust
from rdp import rdp


def test_two():
    arr = np.array([[1., 1.], [5.0, 5.0]])
    res_rs = rdp_rust.cross_py(arr, 0.1)
    res_py = rdp(arr, 0.1)
    assert res_rs.shape == res_py.shape
    np.testing.assert_almost_equal(res_rs, res_py)

def test_basic():
    arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
    res_rs = rdp_rust.cross_py(arr, 0.1)
    res_py = rdp(arr, 0.1)
    assert res_rs.shape == res_py.shape
    np.testing.assert_almost_equal(res_rs, res_py)
