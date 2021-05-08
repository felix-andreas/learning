import numpy as np
import matplotlib.pyplot as plt

with open("results.csv") as file:
    number, math, quake = np.loadtxt(file, unpack=True)


plt.plot(number, math, label="math")
plt.plot(number, quake + 0.01, label="quake + 0.01")
plt.legend()
plt.tight_layout()
plt.savefig("comparison.svg")
