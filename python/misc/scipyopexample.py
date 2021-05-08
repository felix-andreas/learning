from scipy.optimize import minimize
import numpy as np

def func(x):
    return (x - 3) ** 2



result = minimize(func, [0])
print(result)
print(type(result))

