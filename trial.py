import numpy as np
import rdp_rust

arr = np.array([1.0,2.0])
res = rdp_rust.cross_py(arr, arr)
print(res.dtype)
print(res)
print(res.flags)
