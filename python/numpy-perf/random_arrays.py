# see http://www.bitsofbits.com/2014/09/21/numpy-micro-optimization-and-numexpr/
#%% compute results
import subprocess
from time import process_time as time

import numpy as np

rng = np.random.default_rng()


def make_x_y(n):
    return (rng.random([n, n]), rng.random([n, n]))


def test_timing(func, n_bytes, repeats=10):
    n_elements = int(np.sqrt(n_bytes // (2 * 8)))
    x, y = make_x_y(n_elements)  # two arrays of doubles
    t = (1 << 63) - 1
    for _ in range(repeats):
        t0 = time()
        func(x, y)
        t1 = time()
        t = min(t, t1 - t0)
    print(f"{n_bytes:.2e} ns per element: {1e9 * t / n_elements}")
    return 1e9 * t / n_elements


func = np.dot
n_bytes = [1 << x for x in range(14, 34)]
results = [test_timing(func, n) for n in n_bytes]

#%% plot result

import os
import matplotlib.pyplot as plt

MEMORY_SIZE = os.sysconf("SC_PAGE_SIZE") * os.sysconf("SC_PHYS_PAGES")
L3_CACHE = int(
    subprocess.run(["getconf", "LEVEL3_CACHE_SIZE"], capture_output=True).stdout
)


plt.semilogx(n_bytes, results, "--o")
plt.axvline(L3_CACHE)
plt.axvline(MEMORY_SIZE)
whatdoesthisreturn = plt.savefig(f"results-{func.__name__}.svg")

# %%
