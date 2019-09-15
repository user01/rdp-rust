import numpy as np
import rdp_rust
from rdp import rdp

arr = np.array([[1., 1.], [1.2, 1.05], [2.0, 2.0], [3.0, 3.0], [5.0, 3.0], [5.0, 5.0]])
# arr = np.array([[1., 1.], [5.0, 5.0]])
arr = np.array([5.0, 0, 4, 0, 3, 0, 3, 1, 3, 2]).reshape(5, 2)
res = rdp_rust.cross_py(arr, 0.1)
print('RUST')
print(res)

res = rdp(arr, 0.1)
print('Python')
print(res)
