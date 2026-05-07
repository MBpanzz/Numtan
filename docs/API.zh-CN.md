# NumTan API 参考

本文档列出 NumTan `0.9.0` 导出的 Python 公共函数。

## 求导 API

- `tangent(function, x, method="central") -> float`
- `gradient(function, point) -> list[float]`

## 求根 API

- `newton(function, x0, tol=1e-10, max_iter=50) -> dict`
- `halley(function, x0, tol=1e-10, max_iter=50) -> dict`
- `householder(function, x0, order=3, tol=1e-10, max_iter=50) -> dict`

求根结果字典包含：

- `root`：最终根估计值
- `converged`：是否收敛
- `iterations`：迭代次数
- `history`：每一步迭代数据

## 线性代数 API

- `dot(left, right) -> float`
- `norm(vector) -> float`
- `add(left, right) -> list[float]`
- `sub(left, right) -> list[float]`
- `scale(vector, factor) -> list[float]`
- `mat_vec(matrix, vector) -> list[float]`
- `solve(matrix, rhs) -> list[float]`

## 优化 API

- `gradient_descent(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `tangent_minimize(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `stationary_newton(function, start, tol=1e-8, max_iter=100) -> dict`
- `gauss_newton(function, start, tol=1e-8, max_iter=100) -> dict`

优化结果字典包含：

- `point`：最终点
- `value`：最终目标函数值
- `converged`：是否收敛
- `iterations`：迭代次数
- `history`：每一步迭代数据

## ODE API

- `euler(function, y0, t0, t1, step) -> list[dict]`
- `midpoint(function, y0, t0, t1, step) -> list[dict]`
- `rk4(function, y0, t0, t1, step) -> list[dict]`
- `adaptive_rk4(function, y0, t0, t1, initial_step, tol=1e-8) -> list[dict]`

ODE 轨迹字典包含 `t` 和 `y`。

## 积分 API

- `tanh_sinh(function, a, b, tol=1e-10, max_levels=12) -> dict`
- `tan_sinh(function, a=0.0, tol=1e-10, max_levels=12) -> dict`
- `quad_inf(function, tol=1e-10, max_levels=12) -> dict`

积分结果字典包含 `value`、`error` 和 `levels`。

## 三角扩展 API

- `tanpi(x) -> float`
- `tanint(x) -> float`
- `atanint(x) -> float`
- `complex_tan(re, im) -> tuple[float, float]`
- `tan_deg(x) -> float`
- `tan_grad(x) -> float`

## 可视化 API

- `tangent_lines(function, x_start, x_end, centers) -> list[dict]`
- `newton_animation_data(function, x0, tol=1e-10, max_iter=50) -> list[dict]`
- `ode_direction_field(function, x_range, y_range, x_count, y_count) -> list[dict]`

线段字典包含 `x_start`、`y_start`、`x_end`、`y_end`、`center` 和 `slope`。

方向场字典包含 `x`、`y`、`dx` 和 `dy`。

## 统计 API

- `mean(values) -> float`
- `variance(values, sample=False) -> float`
- `summary(values) -> dict`
- `covariance(left, right, sample=False) -> float`
- `correlation(left, right) -> float`
- `linear_regression(x, y) -> dict`
- `polynomial_regression(x, y, degree) -> list[float]`

## 插值 API

- `linspace(start, end, count) -> list[float]`
- `sample_grid(function, start, end, count) -> list[dict]`
- `linear_interpolate(x, y, query) -> float`
- `lagrange_interpolate(x, y, query) -> float`
- `finite_difference(values, spacing) -> list[float]`

## 多项式 API

- `polyval(coefficients, x) -> float`
- `polyder(coefficients) -> list[float]`
- `polyint(coefficients, constant=0.0) -> list[float]`
- `polyadd(left, right) -> list[float]`
- `polymul(left, right) -> list[float]`
- `polyroot(coefficients, guess, tol=1e-10, max_iter=50) -> float`

## 信号处理 API

- `moving_average(values, window) -> list[float]`
- `exponential_smooth(values, alpha) -> list[float]`
- `convolve(signal, kernel) -> list[float]`
- `normalize(values) -> list[float]`
- `find_peaks(values, threshold) -> list[dict]`

峰值字典包含 `index` 和 `value`。
