import pytest

import numpy as np
import rdp_rust
from rdp import rdp


def test_two():
    arr = np.array([[1.0, 1.0], [5.0, 5.0]])
    res_rs = rdp_rust.reduce_points(arr, 0.1)
    res_py = rdp(arr, 0.1)
    assert res_rs.shape == res_py.shape
    np.testing.assert_almost_equal(res_rs, res_py)


def test_basic():
    arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
    res_rs = rdp_rust.reduce_points(arr, 0.1)
    res_py = rdp(arr, 0.1)
    assert res_rs.shape == res_py.shape
    np.testing.assert_almost_equal(res_rs, res_py)

def test_basic_mask():
    arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
    res_rs = rdp_rust.mask_points(arr, 0.1)
    res_py = rdp(arr, 0.1, return_mask=True)
    assert res_rs.shape == res_py.shape
    np.testing.assert_almost_equal(res_rs, res_py)


small_arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
small_expected = np.array([5.0, 0, 3, 0, 3, 2.0]).reshape(3, 2)
small_epsilon = 0.1


def test_small_line():
    for epsilon in np.arange(0.1, 30.0, 0.1):
        result_py = rdp(small_arr, epsilon)
        result_rust = rdp_rust.reduce_points(small_arr, epsilon)
        assert result_py.shape == result_rust.shape, f"epsilon: {epsilon}"
        np.testing.assert_almost_equal(result_py, result_rust)


@pytest.mark.benchmark(group="small-points", warmup=True)
def test_rust_version(benchmark):
    actual = benchmark(rdp_rust.reduce_points, small_arr, small_epsilon)
    assert actual.shape == small_expected.shape
    np.testing.assert_almost_equal(actual, small_expected)


@pytest.mark.benchmark(group="small-points", warmup=True)
def test_py_version(benchmark):
    actual = benchmark(rdp, small_arr, small_epsilon)
    assert actual.shape == small_expected.shape
    np.testing.assert_almost_equal(actual, small_expected)


def test_medium_line():
    for epsilon in np.linspace(0.9, 0.005, 50):
        result_py = rdp(medium_arr, epsilon)
        result_rust = rdp_rust.reduce_points(medium_arr, epsilon)
        assert result_py.shape == result_rust.shape, f"epsilon: {epsilon}"
        np.testing.assert_almost_equal(result_py, result_rust)


@pytest.mark.benchmark(group="medium-points", warmup=True)
def test_rust_version(benchmark):
    actual = benchmark(rdp_rust.reduce_points, medium_arr, medium_epsilon)
    assert actual.shape == medium_expected.shape
    np.testing.assert_almost_equal(actual, medium_expected)


@pytest.mark.benchmark(group="medium-points", warmup=True)
def test_py_version(benchmark):
    actual = benchmark(rdp, medium_arr, medium_epsilon)
    assert actual.shape == medium_expected.shape
    np.testing.assert_almost_equal(actual, medium_expected)


medium_arr = np.array(
    [
        18.71120367913943,
        35.67559590537299,
        19.87575446038943,
        37.21327118250812,
        20.33718024163943,
        37.19577035281653,
        21.23805914788943,
        37.0556176917564,
        21.41384039788943,
        36.6512354470846,
        21.67751227288943,
        36.580689928979275,
        22.00710211663943,
        36.633605119022356,
        22.05104742913943,
        36.26243735259619,
        22.31471930413943,
        36.52773844590173,
        22.64430914788943,
        36.085067452949716,
        23.03981696038943,
        36.28015225239681,
        23.23757086663943,
        35.8716942147415,
        23.76491461663943,
        36.067308390223566,
        23.80885992913943,
        35.67559590537299,
        24.27028571038943,
        35.76479144718892,
        24.53395758538943,
        35.35366637248113,
        25.23708258538943,
        35.960669831102834,
        23.89675055413943,
        37.31819090754369,
        22.13893805413943,
        37.87528917447911,
        20.97438727288943,
        38.41099084386128,
        18.90895758538943,
        37.87528917447911,
        17.94216071038943,
        37.03808036667257,
        17.21706305413943,
        35.44322069725778,
        17.61257086663943,
        33.88911578982415,
        19.37038336663943,
        34.18045430429265,
        20.09548102288943,
        35.47901456666836,
        21.04030524163943,
        36.13832057287443,
        21.61159430413943,
        35.746960327835474,
        21.61159430413943,
        35.31781680026504,
        22.13893805413943,
        35.639889738821545,
        22.13893805413943,
        35.21017271507273,
        22.75417242913943,
        35.44322069725778,
        22.75417242913943,
        35.01245420877086,
        23.39137946038943,
        35.28195132861475,
        23.43532477288943,
        34.814256575733275,
        24.00661383538943,
        35.10238572512127,
        24.07253180413943,
        34.705947263853155,
    ]
).reshape(-1, 2)
medium_expected = np.array(
    [
        [18.71120368, 35.67559591],
        [19.87575446, 37.21327118],
        [21.23805915, 37.05561769],
        [21.4138404, 36.65123545],
        [22.00710212, 36.63360512],
        [22.05104743, 36.26243735],
        [22.3147193, 36.52773845],
        [22.64430915, 36.08506745],
        [23.03981696, 36.28015225],
        [23.23757087, 35.87169421],
        [23.76491462, 36.06730839],
        [23.80885993, 35.67559591],
        [24.27028571, 35.76479145],
        [24.53395759, 35.35366637],
        [25.23708259, 35.96066983],
        [23.89675055, 37.31819091],
        [20.97438727, 38.41099084],
        [18.90895759, 37.87528917],
        [17.94216071, 37.03808037],
        [17.21706305, 35.4432207],
        [17.61257087, 33.88911579],
        [19.37038337, 34.1804543],
        [20.09548102, 35.47901457],
        [21.04030524, 36.13832057],
        [21.6115943, 35.74696033],
        [21.6115943, 35.3178168],
        [22.13893805, 35.63988974],
        [22.13893805, 35.21017272],
        [22.75417243, 35.4432207],
        [22.75417243, 35.01245421],
        [23.39137946, 35.28195133],
        [23.43532477, 34.81425658],
        [24.00661384, 35.10238573],
        [24.0725318, 34.70594726],
    ]
)
medium_epsilon = 0.1
