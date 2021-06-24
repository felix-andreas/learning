import numpy as np

n = 1_000
x = np.linspace(0, 1, n * n).reshape(n, n)

print(x.dtype)
print((x @ x).sum())