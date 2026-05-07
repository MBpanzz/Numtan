# NumTan API Reference

This reference lists the public Python functions exported by NumTan `0.9.2`.

## Derivative API

- `tangent(function, x, method="central") -> float`
- `gradient(function, point) -> list[float]`

## Root API

- `newton(function, x0, tol=1e-10, max_iter=50) -> dict`
- `halley(function, x0, tol=1e-10, max_iter=50) -> dict`
- `householder(function, x0, order=3, tol=1e-10, max_iter=50) -> dict`

Root-result dictionaries contain:

- `root`: final root estimate
- `converged`: convergence flag
- `iterations`: iteration count
- `history`: per-step iteration data

## Linear Algebra API

- `dot(left, right) -> float`
- `norm(vector) -> float`
- `add(left, right) -> list[float]`
- `sub(left, right) -> list[float]`
- `scale(vector, factor) -> list[float]`
- `mat_vec(matrix, vector) -> list[float]`
- `solve(matrix, rhs) -> list[float]`

## Optimization API

- `gradient_descent(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `tangent_minimize(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `stationary_newton(function, start, tol=1e-8, max_iter=100) -> dict`
- `gauss_newton(function, start, tol=1e-8, max_iter=100) -> dict`

Optimization-result dictionaries contain:

- `point`: final point
- `value`: final objective value
- `converged`: convergence flag
- `iterations`: iteration count
- `history`: per-step iteration data

## ODE API

- `euler(function, y0, t0, t1, step) -> list[dict]`
- `midpoint(function, y0, t0, t1, step) -> list[dict]`
- `rk4(function, y0, t0, t1, step) -> list[dict]`
- `adaptive_rk4(function, y0, t0, t1, initial_step, tol=1e-8) -> list[dict]`

ODE path dictionaries contain `t` and `y`.

## Integration API

- `tanh_sinh(function, a, b, tol=1e-10, max_levels=12) -> dict`
- `tan_sinh(function, a=0.0, tol=1e-10, max_levels=12) -> dict`
- `quad_inf(function, tol=1e-10, max_levels=12) -> dict`

Quadrature-result dictionaries contain `value`, `error`, and `levels`.

## Trigonometric Extras API

- `tanpi(x) -> float`
- `tanint(x) -> float`
- `atanint(x) -> float`
- `complex_tan(re, im) -> tuple[float, float]`
- `tan_deg(x) -> float`
- `tan_grad(x) -> float`

## Visualization API

- `tangent_lines(function, x_start, x_end, centers) -> list[dict]`
- `newton_animation_data(function, x0, tol=1e-10, max_iter=50) -> list[dict]`
- `ode_direction_field(function, x_range, y_range, x_count, y_count) -> list[dict]`

Line dictionaries contain `x_start`, `y_start`, `x_end`, `y_end`, `center`, and `slope`.

Direction-field dictionaries contain `x`, `y`, `dx`, and `dy`.

## Statistics API

- `mean(values) -> float`
- `variance(values, sample=False) -> float`
- `summary(values) -> dict`
- `covariance(left, right, sample=False) -> float`
- `correlation(left, right) -> float`
- `linear_regression(x, y) -> dict`
- `polynomial_regression(x, y, degree) -> list[float]`

## Interpolation API

- `linspace(start, end, count) -> list[float]`
- `sample_grid(function, start, end, count) -> list[dict]`
- `linear_interpolate(x, y, query) -> float`
- `lagrange_interpolate(x, y, query) -> float`
- `finite_difference(values, spacing) -> list[float]`

## Polynomial API

- `polyval(coefficients, x) -> float`
- `polyder(coefficients) -> list[float]`
- `polyint(coefficients, constant=0.0) -> list[float]`
- `polyadd(left, right) -> list[float]`
- `polymul(left, right) -> list[float]`
- `polyroot(coefficients, guess, tol=1e-10, max_iter=50) -> float`

## Signal API

- `moving_average(values, window) -> list[float]`
- `exponential_smooth(values, alpha) -> list[float]`
- `convolve(signal, kernel) -> list[float]`
- `normalize(values) -> list[float]`
- `find_peaks(values, threshold) -> list[dict]`

Peak dictionaries contain `index` and `value`.
