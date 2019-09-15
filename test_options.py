import numpy as np
import pytest
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



small_arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
small_expected = np.array([5.0, 0, 3, 0, 3, 2.0]).reshape(3, 2)
small_epsilon = 0.1

@pytest.mark.benchmark(
    group="small-points",
    warmup=False,
)
def test_rust_version(benchmark):
    actual = benchmark(rdp_rust.cross_py, small_arr, small_epsilon)
    assert actual.shape == small_expected.shape
    np.testing.assert_almost_equal(actual, small_expected)

@pytest.mark.benchmark(
    group="small-points",
    warmup=False,
)
def test_py_version(benchmark):
    actual = benchmark(rdp, small_arr, small_epsilon)
    assert actual.shape == small_expected.shape
    np.testing.assert_almost_equal(actual, small_expected)

medium_arr = np.array([44, 95, 26, 91, 22, 90, 21, 90,
    19, 89, 17, 89, 15, 87, 15, 86, 16, 85,
    20, 83, 26, 81, 28, 80, 30, 79, 32, 74,
    32, 72, 33, 71, 34, 70, 38, 68, 43, 66,
    49, 64, 52, 63, 52, 62, 53, 59, 54, 57,
    56, 56, 57, 56, 58, 56, 59, 56, 60, 56,
    61, 55, 61, 55, 63, 55, 64, 55, 65, 54,
    67, 54, 68, 54, 76, 53, 82, 52, 84, 52,
    87, 51, 91, 51, 93, 51, 95, 51, 98, 50,
    105, 50, 113, 49, 120, 48, 127, 48, 131, 47,
    134, 47, 137, 47, 139, 47, 140, 47, 142, 47,
    145, 46, 148, 46, 152, 46, 154, 46, 155, 46,
    159, 46, 160, 46, 165, 46, 168, 46, 169, 45,
    171, 45, 173, 45, 176, 45, 182, 45, 190, 44,
    204, 43, 204, 43, 207, 43, 215, 40, 215, 38,
    215, 37, 200, 37, 195, 41.0]).reshape(-1, 2)
medium_expected = np.array([[44.0, 95.0], [22.0, 90.0], [21.0, 90.0], [19.0, 89.0], [17.0, 89.0], [15.0, 87.0], [15.0, 86.0], [16.0, 85.0], [20.0, 83.0], [26.0, 81.0], [30.0, 79.0], [32.0, 74.0], [32.0, 72.0], [34.0, 70.0], [38.0, 68.0], [43.0, 66.0], [52.0, 63.0], [52.0, 62.0], [53.0, 59.0], [54.0, 57.0], [56.0, 56.0], [60.0, 56.0], [61.0, 55.0], [64.0, 55.0], [65.0, 54.0], [68.0, 54.0], [76.0, 53.0], [82.0, 52.0], [84.0, 52.0], [87.0, 51.0], [95.0, 51.0], [98.0, 50.0], [105.0, 50.0], [120.0, 48.0], [127.0, 48.0], [131.0, 47.0], [142.0, 47.0], [145.0, 46.0], [168.0, 46.0], [169.0, 45.0], [182.0, 45.0], [190.0, 44.0], [204.0, 43.0], [207.0, 43.0], [215.0, 40.0], [215.0, 37.0], [200.0, 37.0], [195.0, 41.0]])
medium_epsilon = 0.1

def test_medium_line():
    for epsilon in np.arange(0.1, 3.0, 0.1):
        result_py = rdp(medium_arr, epsilon)
        result_rust = rdp_rust.cross_py(medium_arr, epsilon)

        # np.testing.assert_almost_equal(result_py, result_rust)
        assert result_py.shape == result_rust.shape, f"epsilon: {epsilon}"

@pytest.mark.benchmark(
    group="medium-points",
    warmup=False,
)
def test_rust_version(benchmark):
    actual = benchmark(rdp_rust.cross_py, medium_arr, medium_epsilon)
    assert actual.shape == medium_expected.shape
    np.testing.assert_almost_equal(actual, medium_expected)

@pytest.mark.benchmark(
    group="medium-points",
    warmup=False,
)
def test_py_version(benchmark):
    actual = benchmark(rdp, medium_arr, medium_epsilon)
    assert actual.shape == medium_expected.shape
    np.testing.assert_almost_equal(actual, medium_expected)
