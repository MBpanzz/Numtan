# NumTan 使用指南

本文档介绍 NumTan `0.9.0` 的主要 Python 使用方式。

## 导入

```python
import math
import numtan as nt
```

## 数值求导

使用 `tangent` 估计一元函数在某一点的导数：

```python
slope = nt.tangent(lambda x: x**3, 2.0)
print(slope)
```

使用 `gradient` 估计多元标量函数的梯度：

```python
grad = nt.gradient(lambda v: v[0] ** 2 + 3.0 * v[1] ** 2, [2.0, 1.0])
print(grad)
```

## 方程求根

求根函数返回字典，包含 `root`、`converged`、`iterations` 和 `history`。

```python
result = nt.newton(lambda x: x * x - 2.0, 1.0)
print(result["root"])
print(result["history"][0])
```

Halley 和 Householder 适合光滑函数的局部快速收敛：

```python
print(nt.halley(lambda x: x**3 - 8.0, 1.0)["root"])
print(nt.householder(lambda x: x**3 - 8.0, 1.0, 3)["root"])
```

## 线性代数

向量使用 Python 列表传入，矩阵使用行主序二维列表。

```python
print(nt.dot([1.0, 2.0], [3.0, 4.0]))
print(nt.norm([3.0, 4.0]))
print(nt.mat_vec([[1.0, 2.0], [3.0, 4.0]], [1.0, 1.0]))
print(nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0]))
```

## 优化

优化函数返回字典，包含 `point`、`value`、`converged`、`iterations` 和 `history`。

```python
result = nt.gradient_descent(
    lambda v: (v[0] - 2.0) ** 2 + (v[1] + 1.0) ** 2,
    [0.0, 0.0],
    step_size=0.01,
)
print(result["point"])
```

标量最小化和驻点搜索：

```python
print(nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)["point"])
print(nt.stationary_newton(lambda x: (x - 4.0) ** 2, 0.0)["point"])
```

Gauss-Newton 用于非线性最小二乘残差系统：

```python
print(nt.gauss_newton(lambda v: [v[0] - 2.0, v[1] + 3.0], [0.0, 0.0])["point"])
```

## 常微分方程

ODE 求解器接收 `f(t, y)`，返回由 `t` 和 `y` 字典组成的列表。

```python
path = nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)
print(path[-1])
```

可用步进器：

```python
nt.euler(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.midpoint(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)
```

## 数值积分

积分函数返回字典，包含 `value`、`error` 和 `levels`。

```python
finite = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)
gaussian = nt.quad_inf(lambda x: math.exp(-x * x))
print(finite["value"], gaussian["value"])
```

`tan_sinh` 用于从 `a` 到正无穷的半无穷区间：

```python
print(nt.tan_sinh(lambda x: math.exp(-x), 0.0)["value"])
```

## 正切与三角扩展

```python
print(nt.tanpi(0.25))
print(nt.tan_deg(45.0))
print(nt.tan_grad(50.0))
print(nt.tanint(0.5))
print(nt.atanint(1.0))
print(nt.complex_tan(0.5, 0.25))
```

## 可视化数据

NumTan 返回普通 Python 数据，可交给任意绘图库使用。

```python
lines = nt.tangent_lines(lambda x: x * x, 0.0, 2.0, [0.5, 1.0, 1.5])
animation = nt.newton_animation_data(lambda x: x * x - 2.0, 1.0)
field = nt.ode_direction_field(lambda t, y: y, (0.0, 1.0), (0.0, 2.0), 8, 8)
```

## 统计与回归

```python
print(nt.summary([1.0, 2.0, 3.0, 4.0]))
print(nt.correlation([1.0, 2.0, 3.0], [2.0, 4.0, 6.0]))
print(nt.linear_regression([1.0, 2.0, 3.0], [3.0, 5.0, 7.0]))
print(nt.polynomial_regression([0.0, 1.0, 2.0], [1.0, 3.0, 7.0], 2))
```

## 插值与网格

```python
print(nt.linspace(0.0, 1.0, 5))
print(nt.sample_grid(lambda x: x * x, 0.0, 1.0, 5))
print(nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25))
print(nt.lagrange_interpolate([0.0, 1.0, 2.0], [1.0, 2.0, 5.0], 1.5))
print(nt.finite_difference([0.0, 1.0, 4.0], 1.0))
```

## 多项式工具

多项式系数按升幂排列。例如 `[1.0, 0.0, 1.0]` 表示 `1 + x^2`。

```python
print(nt.polyval([1.0, 0.0, 1.0], 2.0))
print(nt.polyder([1.0, 0.0, 1.0]))
print(nt.polyint([0.0, 2.0], constant=1.0))
print(nt.polyadd([1.0, 2.0], [3.0]))
print(nt.polymul([1.0, 1.0], [1.0, -1.0]))
print(nt.polyroot([-4.0, 0.0, 1.0], 3.0))
```

## 信号处理

```python
print(nt.moving_average([1.0, 3.0, 5.0], 2))
print(nt.exponential_smooth([0.0, 10.0], 0.5))
print(nt.convolve([1.0, 2.0], [1.0, 1.0]))
print(nt.normalize([2.0, 4.0, 6.0]))
print(nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5))
```

## 异常处理

参数非法时会抛出 `ValueError`。Python 回调内部抛出的异常会原样传回 Python。

```python
try:
    nt.tangent(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 1.0)
except RuntimeError as exc:
    print(exc)
```
