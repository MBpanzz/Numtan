# NumTan

**由 Rust 驱动、面向 Python 的切线主题数值计算工具库。**

NumTan 是一个轻量级数值计算库，适合教学、实验、小型科学计算工作流，以及需要展示迭代过程的场景。它把求导、求根、ODE 求解、积分、统计、插值、多项式、信号处理和可视化数据导出放在同一个简洁 API 下。

项目核心使用 Rust 编写，并通过 PyO3 暴露为 Python 模块，因此兼顾运行速度、打包体积和 Python 使用体验。

当前稳定版本：`1.0.0`。

## AI 参与说明

本项目在开发过程中使用了 AI 辅助。代码、文档、发布决策和最终发布产物仍由人类维护者负责审查、测试和确认。

## 为什么选择 NumTan

- **接口小，覆盖广**：常见数值计算任务不必引入庞大的科学计算栈。
- **适合教学展示**：迭代算法返回 `history`，方便观察、绘图和调试。
- **Rust 内核，Python 调用**：用户写普通 Python 函数，核心计算由 Rust 完成。
- **绘图友好**：可视化函数返回普通列表和字典，不绑定特定绘图库。
- **稳定 v1 API**：`1.x` 系列会保持当前公开 Python API 稳定。

## 安装

从 PyPI 安装稳定版：

```bash
python -m pip install numtan
```

升级已有安装：

```bash
python -m pip install --upgrade numtan
```

从本地 wheel 安装：

```bash
python -m pip install target/wheels/numtan-1.0.0-cp313-cp313-win_amd64.whl
```

从源码开发安装：

```bash
python -m pip install maturin
maturin develop --release
```

构建发布用 wheel：

```bash
maturin build --release --compatibility pypi --auditwheel=repair
```

## 快速开始

```python
import math
import numtan as nt

root = nt.newton(lambda x: x * x - 2.0, 1.0)["root"]
slope = nt.tangent(lambda x: x**3, 2.0)
area = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)["value"]
ode_last = nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)[-1]["y"]

print(root)
print(slope)
print(area)
print(ode_last, math.e)
```

结果大约对应 `sqrt(2)`、`12.0`、`1/3` 和 `e`。

## 功能地图

| 领域 | 函数 |
| --- | --- |
| 求导 | `tangent`, `gradient` |
| 求根 | `newton`, `halley`, `householder` |
| 线性代数 | `dot`, `norm`, `add`, `sub`, `scale`, `mat_vec`, `solve` |
| 优化 | `gradient_descent`, `tangent_minimize`, `stationary_newton`, `gauss_newton` |
| ODE 求解 | `euler`, `midpoint`, `rk4`, `adaptive_rk4` |
| 积分 | `tanh_sinh`, `tan_sinh`, `quad_inf` |
| 三角扩展 | `tanpi`, `tanint`, `atanint`, `complex_tan`, `tan_deg`, `tan_grad` |
| 可视化数据 | `tangent_lines`, `newton_animation_data`, `ode_direction_field` |
| 统计 | `mean`, `variance`, `summary`, `covariance`, `correlation`, `linear_regression`, `polynomial_regression` |
| 插值 | `linspace`, `sample_grid`, `linear_interpolate`, `lagrange_interpolate`, `finite_difference` |
| 多项式 | `polyval`, `polyder`, `polyint`, `polyadd`, `polymul`, `polyroot` |
| 信号处理 | `moving_average`, `exponential_smooth`, `convolve`, `normalize`, `find_peaks` |

## 示例

### 带迭代历史的求根

```python
result = nt.newton(lambda x: x**3 - 2.0 * x - 5.0, 2.0)

print(result["root"])
print(result["converged"])
print(result["history"][0])
```

### 优化

```python
minimum = nt.gradient_descent(
    lambda v: (v[0] - 2.0) ** 2 + (v[1] + 1.0) ** 2,
    [0.0, 0.0],
)

print(minimum["point"])
```

### 绘图数据

```python
lines = nt.tangent_lines(lambda x: x * x, 0.0, 2.0, [0.5, 1.0, 1.5])
field = nt.ode_direction_field(lambda t, y: y, (0.0, 1.0), (0.0, 2.0), 8, 8)
```

返回值都是普通 Python 列表和字典，可直接交给 Matplotlib、Plotly 或 notebook 组件。

## 文档

- 英文文档总索引：`docs/INDEX.md`
- 中文文档总索引：`docs/INDEX.zh-CN.md`
- 使用指南：`docs/USAGE.zh-CN.md`
- API 参考：`docs/API.zh-CN.md`
- 英文 README：`README.md`
- 发布指南：`docs/RELEASE.zh-CN.md`
- 可运行示例：`examples/python_demo.py`

## 测试

运行 Rust 测试：

```bash
cargo test
```

安装 Python 扩展后运行检查：

```bash
python tests/python_smoke.py
python tests/api_surface.py
```

## 设计说明

- 核心算法位于 `src/core`，不依赖 Python。
- Python 绑定位于 `src/api`，负责把 Rust 结果转换为 Python 的 `dict`、`list`、`tuple` 和 `float`。
- Python 回调中抛出的异常会原样传回 Python。
- 未定义的数值输入会抛出 `ValueError`，例如对常量向量计算相关系数。

## 项目状态

NumTan `1.0.0` 是首个稳定 API 版本，适合常规 Python 安装、教学材料、本地数值演示和小型低依赖工具。
