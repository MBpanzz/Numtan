# NumTan API 参考

本文档覆盖 NumTan `1.0.0` 导出的 Python 公共 API。

NumTan 在开发过程中使用了 AI 辅助。最终 API、文档和发布版本由人类维护者审查并确认。

从 PyPI 安装：

```bash
python -m pip install numtan
```

推荐导入方式：

```python
import numtan as nt
```

## 版本

- `__version__ -> str`

## 常见返回结构

求根器返回字典，包含：

- `root`：最终根估计
- `converged`：是否收敛
- `iterations`：迭代次数
- `history`：逐步迭代数据

优化器返回字典，包含：

- `point`：最终点
- `value`：最终目标函数值
- `converged`：是否收敛
- `iterations`：迭代次数
- `history`：逐步迭代数据

ODE 求解器返回由字典组成的列表，每个字典包含 `t` 和 `y`。

积分函数返回字典，包含 `value`、`error` 和 `levels`。

## 求导

- `tangent(function, x, method="central") -> float`
- `gradient(function, point) -> list[float]`

## 求根

- `newton(function, x0, tol=1e-10, max_iter=50) -> dict`
- `halley(function, x0, tol=1e-10, max_iter=50) -> dict`
- `householder(function, x0, order=3, tol=1e-10, max_iter=50) -> dict`

## 线性代数

- `dot(left, right) -> float`
- `norm(vector) -> float`
- `add(left, right) -> list[float]`
- `sub(left, right) -> list[float]`
- `scale(vector, factor) -> list[float]`
- `mat_vec(matrix, vector) -> list[float]`
- `solve(matrix, rhs) -> list[float]`

向量使用 Python 列表。矩阵使用行主序二维列表。

## 优化

- `gradient_descent(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `tangent_minimize(function, start, step_size=0.01, tol=1e-8, max_iter=1000) -> dict`
- `stationary_newton(function, start, tol=1e-8, max_iter=100) -> dict`
- `gauss_newton(function, start, tol=1e-8, max_iter=100) -> dict`

## ODE 求解

- `euler(function, y0, t0, t1, step) -> list[dict]`
- `midpoint(function, y0, t0, t1, step) -> list[dict]`
- `rk4(function, y0, t0, t1, step) -> list[dict]`
- `adaptive_rk4(function, y0, t0, t1, initial_step, tol=1e-8) -> list[dict]`

ODE 回调函数接收 `(t, y)`。

## 积分

- `tanh_sinh(function, a, b, tol=1e-10, max_levels=12) -> dict`
- `tan_sinh(function, a=0.0, tol=1e-10, max_levels=12) -> dict`
- `quad_inf(function, tol=1e-10, max_levels=12) -> dict`

## 三角扩展

- `tanpi(x) -> float`
- `tanint(x) -> float`
- `atanint(x) -> float`
- `complex_tan(re, im) -> tuple[float, float]`
- `tan_deg(x) -> float`
- `tan_grad(x) -> float`

## 可视化数据

- `tangent_lines(function, x_start, x_end, centers) -> list[dict]`
- `newton_animation_data(function, x0, tol=1e-10, max_iter=50) -> list[dict]`
- `ode_direction_field(function, x_range, y_range, x_count, y_count) -> list[dict]`

切线段字典包含 `x_start`、`y_start`、`x_end`、`y_end`、`center` 和 `slope`。

方向场字典包含 `x`、`y`、`dx` 和 `dy`。

## 统计

- `mean(values) -> float`
- `variance(values, sample=False) -> float`
- `summary(values) -> dict`
- `covariance(left, right, sample=False) -> float`
- `correlation(left, right) -> float`
- `linear_regression(x, y) -> dict`
- `polynomial_regression(x, y, degree) -> list[float]`

`summary` 返回 `count`、`mean`、`variance`、`std_dev`、`min` 和 `max`。

`linear_regression` 返回 `slope`、`intercept` 和 `r2`。

## 插值与网格

- `linspace(start, end, count) -> list[float]`
- `sample_grid(function, start, end, count) -> list[dict]`
- `linear_interpolate(x, y, query) -> float`
- `lagrange_interpolate(x, y, query) -> float`
- `finite_difference(values, spacing) -> list[float]`

## 多项式

- `polyval(coefficients, x) -> float`
- `polyder(coefficients) -> list[float]`
- `polyint(coefficients, constant=0.0) -> list[float]`
- `polyadd(left, right) -> list[float]`
- `polymul(left, right) -> list[float]`
- `polyroot(coefficients, guess, tol=1e-10, max_iter=50) -> float`

多项式系数按升幂排列。

## 信号处理

- `moving_average(values, window) -> list[float]`
- `exponential_smooth(values, alpha) -> list[float]`
- `convolve(signal, kernel) -> list[float]`
- `normalize(values) -> list[float]`
- `find_peaks(values, threshold) -> list[dict]`

峰值字典包含 `index` 和 `value`。

## 错误语义

- 参数形状错误、空输入、奇异线性系统和未定义统计操作会抛出 `ValueError`。
- 用户提供的 Python 回调中抛出的异常会原样传播，不会变成 Rust panic。
