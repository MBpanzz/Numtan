# NumTan 使用指南

本文档介绍 NumTan `1.0.0` 的主要工作流。安装方式如下：

```bash
python -m pip install numtan
```

说明：NumTan 在开发过程中使用了 AI 辅助。最终代码、文档和发布版本由人类维护者审查并确认。

导入模块：

```python
import math
import numtan as nt
```

## 使用模型

NumTan 的接口保持统一：

- 输入使用普通 Python 浮点数和列表。
- 数值函数使用普通 Python callable。
- 迭代算法返回包含收敛信息和 `history` 的字典。
- 绘图辅助函数返回普通列表和字典。
- 非法参数抛出 `ValueError`，Python 回调内部异常会原样向外传播。

## 数值求导

估计一元函数导数：

```python
slope = nt.tangent(lambda x: x**3, 2.0)
print(slope)  # 约为 12.0
```

估计多元标量函数的梯度：

```python
grad = nt.gradient(lambda v: v[0] ** 2 + 3.0 * v[1] ** 2, [2.0, 1.0])
print(grad)  # 约为 [4.0, 6.0]
```

## 方程求根

求根器返回 `root`、`converged`、`iterations` 和 `history`。

```python
result = nt.newton(lambda x: x * x - 2.0, 1.0)

print(result["root"])
print(result["converged"])
print(result["history"][0])
```

函数足够光滑时，可以使用 Halley 或 Householder 获得更快的局部收敛：

```python
print(nt.halley(lambda x: x**3 - 8.0, 1.0)["root"])
print(nt.householder(lambda x: x**3 - 8.0, 1.0, 3)["root"])
```

## 线性代数

向量使用 Python 列表，矩阵使用行主序二维列表。

```python
print(nt.dot([1.0, 2.0], [3.0, 4.0]))
print(nt.norm([3.0, 4.0]))
print(nt.add([1.0, 2.0], [3.0, 4.0]))
print(nt.mat_vec([[1.0, 2.0], [3.0, 4.0]], [1.0, 1.0]))
print(nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0]))
```

## 优化

优化结果包含 `point`、`value`、`converged`、`iterations` 和 `history`。

```python
minimum = nt.gradient_descent(
    lambda v: (v[0] - 2.0) ** 2 + (v[1] + 1.0) ** 2,
    [0.0, 0.0],
)
print(minimum["point"])
```

标量最小化和驻点搜索：

```python
print(nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)["point"])
print(nt.stationary_newton(lambda x: (x - 4.0) ** 2, 0.0)["point"])
```

Gauss-Newton 使用残差向量：

```python
result = nt.gauss_newton(lambda v: [v[0] - 2.0, v[1] + 3.0], [0.0, 0.0])
print(result["point"])
```

## ODE 求解

ODE 求解器接收 `f(t, y)`，返回由 `{"t": ..., "y": ...}` 字典组成的轨迹。

```python
path = nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)
print(path[-1]["y"])
```

可用步进器：

```python
nt.euler(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.midpoint(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.1)
nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)
```

## 数值积分

积分结果包含 `value`、`error` 和 `levels`。

```python
finite = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)
gaussian = nt.quad_inf(lambda x: math.exp(-x * x))

print(finite["value"])
print(gaussian["value"])
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

NumTan 不绑定绘图库。它返回普通数据结构，方便交给 Matplotlib、Plotly 或 notebook 组件。

```python
lines = nt.tangent_lines(lambda x: x * x, 0.0, 2.0, [0.5, 1.0, 1.5])
animation = nt.newton_animation_data(lambda x: x * x - 2.0, 1.0)
field = nt.ode_direction_field(lambda t, y: y, (0.0, 1.0), (0.0, 2.0), 8, 8)
```

## 统计与回归

```python
print(nt.mean([1.0, 2.0, 3.0]))
print(nt.variance([1.0, 2.0, 3.0]))
print(nt.summary([1.0, 2.0, 3.0, 4.0]))
print(nt.correlation([1.0, 2.0, 3.0], [2.0, 4.0, 6.0]))
print(nt.linear_regression([1.0, 2.0, 3.0], [3.0, 5.0, 7.0]))
print(nt.polynomial_regression([0.0, 1.0, 2.0], [1.0, 3.0, 7.0], 2))
```

对常量向量计算相关系数、用常量 `x` 做线性回归都是未定义操作，会抛出 `ValueError`。

## 插值与采样

```python
print(nt.linspace(0.0, 1.0, 5))
print(nt.sample_grid(lambda x: x * x, 0.0, 1.0, 5))
print(nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25))
print(nt.lagrange_interpolate([0.0, 1.0, 2.0], [1.0, 2.0, 5.0], 1.5))
print(nt.finite_difference([0.0, 1.0, 4.0], 1.0))
```

## 多项式

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

非法数值输入会抛出 `ValueError`：

```python
try:
    nt.correlation([1.0, 1.0], [2.0, 3.0])
except ValueError as exc:
    print(exc)
```

Python 回调中抛出的异常会原样传播：

```python
try:
    nt.tangent(lambda x: (_ for _ in ()).throw(RuntimeError("boom")), 1.0)
except RuntimeError as exc:
    print(exc)
```
