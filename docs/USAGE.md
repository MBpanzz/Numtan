# NumTan Usage Guide

This guide shows the main Python workflows supported by NumTan `0.9.2`.

## Import

```python
import math
import numtan as nt
```

## Derivatives

Use `tangent` for a scalar derivative estimate:

```python
slope = nt.tangent(lambda x: x**3, 2.0)
print(slope)
```

Use `gradient` for a multivariate scalar function:

```python
grad = nt.gradient(lambda v: v[0] ** 2 + 3.0 * v[1] ** 2, [2.0, 1.0])
print(grad)
```

## Root Finding

Root solvers return dictionaries with `root`, `converged`, `iterations`, and `history`.

```python
result = nt.newton(lambda x: x * x - 2.0, 1.0)
print(result["root"])
print(result["history"][0])
```

Halley and Householder are available for faster local convergence when the function behaves smoothly:

```python
print(nt.halley(lambda x: x**3 - 8.0, 1.0)["root"])
print(nt.householder(lambda x: x**3 - 8.0, 1.0, 3)["root"])
```

## Linear Algebra

Vectors are passed as Python lists. Matrices are row-major lists of lists.

```python
print(nt.dot([1.0, 2.0], [3.0, 4.0]))
print(nt.norm([3.0, 4.0]))
print(nt.mat_vec([[1.0, 2.0], [3.0, 4.0]], [1.0, 1.0]))
print(nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0]))
```

## Optimization

Optimization functions return dictionaries with `point`, `value`, `converged`, `iterations`, and `history`.

```python
result = nt.gradient_descent(
    lambda v: (v[0] - 2.0) ** 2 + (v[1] + 1.0) ** 2,
    [0.0, 0.0],
    step_size=0.01,
)
print(result["point"])
```

Scalar minimization and stationary-point search:

```python
print(nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)["point"])
print(nt.stationary_newton(lambda x: (x - 4.0) ** 2, 0.0)["point"])
```

Gauss-Newton solves nonlinear least-squares residual systems:

```python
print(nt.gauss_newton(lambda v: [v[0] - 2.0, v[1] + 3.0], [0.0, 0.0])["point"])
```

## ODE Solvers

ODE solvers expect `f(t, y)` and return a list of dictionaries with `t` and `y`.

```python
path = nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)
print(path[-1])
```

Available steppers:

```python
nt.euler(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.midpoint(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)
```

## Integration

Quadrature functions return dictionaries with `value`, `error`, and `levels`.

```python
finite = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)
gaussian = nt.quad_inf(lambda x: math.exp(-x * x))
print(finite["value"], gaussian["value"])
```

Use `tan_sinh` for semi-infinite intervals from `a` to infinity:

```python
print(nt.tan_sinh(lambda x: math.exp(-x), 0.0)["value"])
```

## Tangent and Trigonometric Extras

```python
print(nt.tanpi(0.25))
print(nt.tan_deg(45.0))
print(nt.tan_grad(50.0))
print(nt.tanint(0.5))
print(nt.atanint(1.0))
print(nt.complex_tan(0.5, 0.25))
```

## Visualization Data

NumTan returns plain Python data for plotting with any library.

```python
lines = nt.tangent_lines(lambda x: x * x, 0.0, 2.0, [0.5, 1.0, 1.5])
animation = nt.newton_animation_data(lambda x: x * x - 2.0, 1.0)
field = nt.ode_direction_field(lambda t, y: y, (0.0, 1.0), (0.0, 2.0), 8, 8)
```

## Statistics and Regression

```python
print(nt.summary([1.0, 2.0, 3.0, 4.0]))
print(nt.correlation([1.0, 2.0, 3.0], [2.0, 4.0, 6.0]))
print(nt.linear_regression([1.0, 2.0, 3.0], [3.0, 5.0, 7.0]))
print(nt.polynomial_regression([0.0, 1.0, 2.0], [1.0, 3.0, 7.0], 2))
```

## Interpolation and Grids

```python
print(nt.linspace(0.0, 1.0, 5))
print(nt.sample_grid(lambda x: x * x, 0.0, 1.0, 5))
print(nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25))
print(nt.lagrange_interpolate([0.0, 1.0, 2.0], [1.0, 2.0, 5.0], 1.5))
print(nt.finite_difference([0.0, 1.0, 4.0], 1.0))
```

## Polynomial Tools

Coefficients are ordered by ascending degree. For example, `[1.0, 0.0, 1.0]` represents `1 + x^2`.

```python
print(nt.polyval([1.0, 0.0, 1.0], 2.0))
print(nt.polyder([1.0, 0.0, 1.0]))
print(nt.polyint([0.0, 2.0], constant=1.0))
print(nt.polyadd([1.0, 2.0], [3.0]))
print(nt.polymul([1.0, 1.0], [1.0, -1.0]))
print(nt.polyroot([-4.0, 0.0, 1.0], 3.0))
```

## Signal Helpers

```python
print(nt.moving_average([1.0, 3.0, 5.0], 2))
print(nt.exponential_smooth([0.0, 10.0], 0.5))
print(nt.convolve([1.0, 2.0], [1.0, 1.0]))
print(nt.normalize([2.0, 4.0, 6.0]))
print(nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5))
```

## Error Handling

Invalid arguments raise `ValueError`. Exceptions raised inside Python callbacks are propagated unchanged.

```python
try:
    nt.tangent(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 1.0)
except RuntimeError as exc:
    print(exc)
```
