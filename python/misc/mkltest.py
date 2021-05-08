from numba import jit
import random
from elements.utils.profiler import profile


def monte_carlo_pi(nsamples):
    acc = 0
    for i in range(nsamples):
        x = random.random()
        y = random.random()
        if (x ** 2 + y ** 2) < 1.0:
            acc += 1
    return 4.0 * acc / nsamples


@jit(nopython=True)
def monte_carlo_pi_jit(nsamples):
    acc = 0
    for i in range(nsamples):
        x = random.random()
        y = random.random()
        if (x ** 2 + y ** 2) < 1.0:
            acc += 1
    return 4.0 * acc / nsamples

num = 10
monte_carlo_pi = profile(monte_carlo_pi, num=num, print_pstats=False)
monte_carlo_pi_jit = profile(monte_carlo_pi_jit, num=num, print_pstats=False)
nsamples = 10000000
monte_carlo_pi(nsamples)
monte_carlo_pi_jit(nsamples)
