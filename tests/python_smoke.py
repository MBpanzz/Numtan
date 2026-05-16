import math

import numtan as nt


def near(left, right, eps=1e-6):
    assert abs(left - right) <= eps, (left, right)


assert nt.__version__ == "1.0.0"
near(nt.tangent(lambda x: x**3, 2.0), 12.0, 1e-5)
gradient = nt.gradient(lambda v: v[0] ** 2 + 3 * v[1] ** 2, [2.0, 1.0])
near(gradient[0], 4.0, 1e-5)
near(gradient[1], 6.0, 1e-5)

near(nt.newton(lambda x: x * x - 2.0, 1.0)["root"], math.sqrt(2.0), 1e-8)
near(nt.halley(lambda x: x**3 - 8.0, 1.0)["root"], 2.0, 1e-8)
near(nt.householder(lambda x: x**3 - 8.0, 1.0, 3)["root"], 2.0, 1e-8)

near(nt.dot([1.0, 2.0], [3.0, 4.0]), 11.0)
near(nt.norm([3.0, 4.0]), 5.0)
assert nt.add([1.0, 2.0], [3.0, 4.0]) == [4.0, 6.0]
assert nt.sub([4.0, 6.0], [1.0, 2.0]) == [3.0, 4.0]
assert nt.scale([1.0, 2.0], 3.0) == [3.0, 6.0]
assert nt.mat_vec([[1.0, 2.0], [3.0, 4.0]], [1.0, 1.0]) == [3.0, 7.0]
solution = nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0])
near(solution[0], 0.2)
near(solution[1], 0.6)

minimum = nt.gradient_descent(lambda v: (v[0] - 2.0) ** 2 + (v[1] + 1.0) ** 2, [0.0, 0.0])
near(minimum["point"][0], 2.0, 1e-5)
near(minimum["point"][1], -1.0, 1e-5)
near(nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)["point"][0], 3.0, 1e-5)
near(nt.stationary_newton(lambda x: (x - 4.0) ** 2, 0.0)["point"][0], 4.0, 1e-4)
near(nt.gauss_newton(lambda v: [v[0] - 2.0, v[1] + 3.0], [0.0, 0.0])["point"][0], 2.0)

near(nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)[-1]["y"], math.e, 1e-6)
assert len(nt.euler(lambda t, y: y, 1.0, 0.0, 0.2, 0.1)) == 3
assert len(nt.midpoint(lambda t, y: y, 1.0, 0.0, 0.2, 0.1)) == 3
near(nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)[-1]["y"], math.e, 1e-6)

near(nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)["value"], 1.0 / 3.0, 1e-8)
near(nt.quad_inf(lambda x: math.exp(-x * x))["value"], math.sqrt(math.pi), 1e-6)

near(nt.tanpi(0.25), 1.0)
near(nt.tan_deg(45.0), 1.0)
near(nt.tan_grad(50.0), 1.0)
assert math.isfinite(nt.tanint(0.5))
assert math.isfinite(nt.atanint(1.0))
assert all(math.isfinite(value) for value in nt.complex_tan(0.5, 0.25))

assert len(nt.tangent_lines(lambda x: x * x, 0.0, 2.0, [1.0])) == 1
assert len(nt.newton_animation_data(lambda x: x * x - 2.0, 1.0)) > 0
assert len(nt.ode_direction_field(lambda x, y: y, (0.0, 1.0), (0.0, 1.0), 2, 2)) == 4

near(nt.mean([1.0, 2.0, 3.0]), 2.0)
near(nt.variance([1.0, 2.0, 3.0]), 2.0 / 3.0)
near(nt.summary([1.0, 2.0, 3.0])["mean"], 2.0)
near(nt.correlation([1.0, 2.0, 3.0], [2.0, 4.0, 6.0]), 1.0)
near(nt.linear_regression([1.0, 2.0, 3.0], [3.0, 5.0, 7.0])["slope"], 2.0)
near(nt.polynomial_regression([0.0, 1.0, 2.0], [1.0, 3.0, 7.0], 2)[2], 1.0)
for caller in [
    lambda: nt.correlation([1.0, 1.0], [2.0, 3.0]),
    lambda: nt.linear_regression([1.0, 1.0], [2.0, 3.0]),
]:
    try:
        caller()
    except ValueError:
        pass
    else:
        raise AssertionError("undefined statistical operation did not fail")

assert nt.linspace(0.0, 1.0, 3) == [0.0, 0.5, 1.0]
assert len(nt.sample_grid(lambda x: x * x, 0.0, 1.0, 3)) == 3
near(nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25), 0.5)
near(nt.lagrange_interpolate([0.0, 1.0, 2.0], [1.0, 2.0, 5.0], 1.5), 3.25)
near(nt.finite_difference([0.0, 1.0, 4.0], 1.0)[1], 2.0)

near(nt.polyval([1.0, 0.0, 1.0], 2.0), 5.0)
assert nt.polyder([1.0, 0.0, 1.0]) == [0.0, 2.0]
assert nt.polyadd([1.0, 2.0], [3.0]) == [4.0, 2.0]
assert nt.polymul([1.0, 1.0], [1.0, -1.0]) == [1.0, 0.0, -1.0]
near(nt.polyroot([-4.0, 0.0, 1.0], 3.0), 2.0, 1e-8)

assert nt.moving_average([1.0, 3.0, 5.0], 2) == [1.0, 2.0, 4.0]
assert nt.exponential_smooth([0.0, 10.0], 0.5) == [0.0, 5.0]
assert nt.convolve([1.0, 2.0], [1.0, 1.0]) == [1.0, 3.0, 2.0]
assert nt.normalize([2.0, 4.0, 6.0]) == [0.0, 0.5, 1.0]
assert len(nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5)) == 2

for caller in [
    lambda: nt.tangent(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 1.0),
    lambda: nt.gradient(lambda v: (_ for _ in ()).throw(RuntimeError("boom")), [1.0]),
    lambda: nt.newton(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 1.0),
    lambda: nt.rk4(lambda t, y: (_ for _ in ()).throw(RuntimeError("boom")), 1.0, 0.0, 1.0, 0.1),
    lambda: nt.tanh_sinh(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 0.0, 1.0),
    lambda: nt.gradient_descent(lambda v: (_ for _ in ()).throw(RuntimeError("boom")), [1.0]),
    lambda: nt.sample_grid(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 0.0, 1.0, 2),
    lambda: nt.tangent_lines(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 0.0, 1.0, [0.5]),
]:
    try:
        caller()
    except RuntimeError as exc:
        assert "boom" in str(exc)
    else:
        raise AssertionError("callback exception was not propagated")

print("python smoke ok")
