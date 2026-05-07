import math

import numtan as nt


def main():
    root = nt.newton(lambda x: x * x - 2.0, 1.0)["root"]
    slope = nt.tangent(lambda x: x**3, 2.0)
    area = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)["value"]
    ode_last = nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)[-1]
    regression = nt.linear_regression([1.0, 2.0, 3.0], [3.0, 5.0, 7.0])
    peaks = nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5)

    print("root", root)
    print("slope", slope)
    print("area", area)
    print("ode", ode_last)
    print("gaussian", nt.quad_inf(lambda x: math.exp(-x * x))["value"])
    print("regression", regression)
    print("peaks", peaks)


if __name__ == "__main__":
    main()
