# NumTan

NumTan 是一个以"切线"为核心概念的轻量级数值计算工具库。核心算法使用 Rust 实现，并通过 PyO3 导出为 Python 模块。它适合教学、实验、小型数值任务和需要可解释迭代过程的场景。

当前版本：`0.9.0`。

英文文档：`README.md`

## 功能概览

- v0.1：数值求导与标量方程求根
- v0.2：小型向量、矩阵与线性系统工具
- v0.3：常微分方程显式步进与自适应 RK4
- v0.4：tanh-sinh 积分、无穷区间积分与正切相关特殊函数
- v0.5：用于绘图和教学展示的可视化数据导出
- v0.6：描述统计、相关性和回归工具
- v0.7：插值、采样网格和有限差分
- v0.8：多项式计算工具
- v0.9：信号平滑、卷积、归一化和峰值检测

## 安装

从本地 wheel 安装：

```bash
python -m pip install target/wheels/numtan-0.9.0-cp313-cp313-win_amd64.whl
```

开发模式构建并安装：

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
import numtan as nt

root = nt.newton(lambda x: x * x - 2.0, 1.0)["root"]
slope = nt.tangent(lambda x: x**3, 2.0)
area = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)["value"]

print(root, slope, area)
```

输出大约为：

```text
1.4142135623730951 12.0 0.3333333333333333
```

## 常用示例

### 求导与求根

```python
nt.tangent(lambda x: x**3, 2.0)
nt.gradient(lambda v: v[0] ** 2 + v[1] ** 2, [1.0, 2.0])
nt.newton(lambda x: x * x - 2.0, 1.0)
nt.halley(lambda x: x**3 - 8.0, 1.0)
nt.householder(lambda x: x**3 - 8.0, 1.0, 3)
```

### 线性代数

```python
nt.dot([1.0, 2.0], [3.0, 4.0])
nt.norm([3.0, 4.0])
nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0])
```

### 优化

```python
nt.gradient_descent(lambda v: (v[0] - 2.0) ** 2, [0.0])
nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)
nt.gauss_newton(lambda v: [v[0] - 2.0], [0.0])
```

### ODE 与积分

```python
nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)
nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)
nt.quad_inf(lambda x: __import__("math").exp(-x * x))
```

### 统计、插值、多项式和信号处理

```python
nt.summary([1.0, 2.0, 3.0])
nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25)
nt.polyval([1.0, 0.0, 1.0], 2.0)
nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5)
```

## 中文文档

- 文档总索引：`docs/INDEX.zh-CN.md`
- 英文文档总索引：`docs/INDEX.md`
- 英文 README：`README.md`
- 中文使用指南：`docs/USAGE.zh-CN.md`
- 中文 API 参考：`docs/API.zh-CN.md`
- 中文发布指南：`docs/RELEASE.zh-CN.md`
- 可运行 Python 示例：`examples/python_demo.py`

## 测试

运行 Rust 测试：

```bash
cargo test
```

安装扩展后运行 Python 测试：

```bash
python tests/python_smoke.py
python tests/api_surface.py
```

## 设计说明

- 核心算法位于 `src/core`，不依赖 Python。
- Python 绑定位于 `src/api`，负责把 Rust 结果转换为 Python 的 `dict`、`list`、`tuple` 和 `float`。
- 迭代算法尽量返回 `history`，方便教学、绘图和调试。
- Python 回调中的异常会被正常传回 Python，不会在 Rust 侧 panic。

## 项目状态

NumTan `0.9.0` 适合作为预览版或候选发布版本。若要发布稳定版 `1.0.0`，建议继续补充更多数值边界测试、多平台 wheel CI 和完整自动生成 API 文档。
