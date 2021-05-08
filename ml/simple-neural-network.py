import numpy as np
import matplotlib.pyplot as plt
import itertools

marker = itertools.cycle((",", "+", ".", "o", "*"))
a = 2
b = 0.3
c = 0.4
d = -1.0


def plot_areas(datas, func, xlim, ylim, N=500):
    x, y = np.meshgrid(np.linspace(*xlim, N), np.linspace(*ylim, N))
    input_area = np.stack((x.flatten(), y.flatten()))
    out_func = func(input_area)
    output = np.argmax(out_func, axis=0).reshape(N, N)
    plt.contourf(x, y, output, cmap=plt.cm.bone)
    for data in datas:
        plt.scatter(data[:, 0], data[:, 1], marker=next(marker))


if __name__ == "__main__":

    def func(xy_pairs):
        weight = np.array([[a, b], [c, d]])
        bias = np.array([2, 2]).reshape((2, 1))
        return weight @ xy_pairs + bias

    datas = np.array(
        [
            [[0, 0], [0, 1]],
            [[1, 0], [1, 1]],
        ]
    )

    xlim = ylim = (-2, 2)
    plot_areas(datas, func, xlim, ylim)

    plt.savefig("test.pdf")
