
# RDP Rust

[![Build Status](https://travis-ci.org/user01/rdp-rust.svg?branch=master)](https://travis-ci.org/user01/rdp-rust)

Binary version of MIT licensed RDP [implementation](https://github.com/fhirschmann/rdp) by [fhirschmann](https://github.com/fhirschmann).

Wheels are built for Linux Python 3.5, 3.6, and 3.7. OS X wheels are built for 3.6 and 3.7.

Benchmarks show approximately 60x speed increase.

Rust implementation of the Ramer-Douglas-Peucker algorithm (Ramer 1972; Douglas and Peucker 1973) for 2D data.

## Installation

```bash
pip install rdp-rust
```

## Usage

The inputs must all be floating points values.

```python
>>> import numpy as np
>>> from rdp_rust import reduce_points

>>> arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
>>> reduce_points(small_arr, 0.2)
array([[5., 0.],
       [3., 0.],
       [3., 2.]])
```

```python
>>> import numpy as np
>>> from rdp_rust import mask_points

>>> arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
>>> mask_points(small_arr, 0.2)
array([ True, False,  True, False,  True])
```

## Example

![Demo of RDP](https://github.com/user01/rdp-rust/raw/master/rdp-demo.gif "Demo of RDP")
