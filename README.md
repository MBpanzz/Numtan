# NumTan

NumTan is a tangent-centered numerical toolkit implemented in Rust and exported as a Python module with PyO3. It is designed as a compact teaching-friendly numerical library: derivatives, root finding, ODE stepping, quadrature, regression, interpolation, polynomial helpers, signal utilities, and plot-ready visualization data all share one lightweight API.

Current version: `0.9.1`.

## Features

- v0.1: numerical derivatives and scalar root solvers
- v0.2: small vector and matrix helpers for multivariate workflows
- v0.3: explicit ODE steppers and adaptive RK4
- v0.4: tanh-sinh style quadrature and tangent trigonometric extras
- v0.5: visualization data exporters
- v0.6: descriptive statistics and regression helpers
- v0.7: interpolation and finite-difference grids
- v0.8: polynomial tools
- v0.9: signal smoothing, convolution, and peak detection

## Installation

Install from a local wheel:

```bash
python -m pip install target/wheels/numtan-0.9.1-cp313-cp313-win_amd64.whl
```

Build and install in editable development mode:

```bash
python -m pip install maturin
maturin develop --release
```

Build a release wheel:

```bash
maturin build --release --compatibility pypi --auditwheel=repair
```

## Quick Start

```python
import numtan as nt

root = nt.newton(lambda x: x * x - 2.0, 1.0)["root"]
slope = nt.tangent(lambda x: x**3, 2.0)
area = nt.tanh_sinh(lambda x: x * x, 0.0, 1.0)["value"]

print(root, slope, area)
```

Expected output is approximately:

```text
1.4142135623730951 12.0 0.3333333333333333
```

## API Overview

### Derivatives and Roots

```python
nt.tangent(lambda x: x**3, 2.0)
nt.gradient(lambda v: v[0] ** 2 + v[1] ** 2, [1.0, 2.0])
nt.newton(lambda x: x * x - 2.0, 1.0)
nt.halley(lambda x: x**3 - 8.0, 1.0)
nt.householder(lambda x: x**3 - 8.0, 1.0, 3)
```

### Linear Algebra

```python
nt.dot([1.0, 2.0], [3.0, 4.0])
nt.norm([3.0, 4.0])
nt.solve([[2.0, 1.0], [1.0, 3.0]], [1.0, 2.0])
```

### Optimization

```python
nt.gradient_descent(lambda v: (v[0] - 2.0) ** 2, [0.0])
nt.tangent_minimize(lambda x: (x - 3.0) ** 2, 0.0)
nt.gauss_newton(lambda v: [v[0] - 2.0], [0.0])
```

### ODE and Integration

```python
nt.rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.05)
nt.adaptive_rk4(lambda t, y: y, 1.0, 0.0, 1.0, 0.2)
nt.quad_inf(lambda x: __import__("math").exp(-x * x))
```

### Statistics, Interpolation, Polynomials, and Signals

```python
nt.summary([1.0, 2.0, 3.0])
nt.linear_interpolate([0.0, 1.0], [0.0, 2.0], 0.25)
nt.polyval([1.0, 0.0, 1.0], 2.0)
nt.find_peaks([0.0, 2.0, 1.0, 3.0, 0.0], 1.5)
```

## Documentation

- Documentation index: `docs/INDEX.md`
- Chinese documentation index: `docs/INDEX.zh-CN.md`
- Chinese README: `README.zh-CN.md`
- User guide: `docs/USAGE.md`
- API reference: `docs/API.md`
- Release checklist: `docs/RELEASE.md`
- Runnable Python demo: `examples/python_demo.py`

## Testing

Run Rust tests:

```bash
cargo test
```

Run Python smoke tests after installing the extension:

```bash
python tests/python_smoke.py
python tests/api_surface.py
```

## Design Notes

- Core algorithms live in `src/core` and contain no Python-specific code.
- Python bindings live in `src/api` and convert Rust results into Python dictionaries, lists, tuples, and floats.
- Iterative algorithms return history data when useful for teaching, plotting, and debugging.
- Python callback exceptions are propagated back to Python instead of panicking in Rust.

## Project Status

NumTan `0.9.1` is suitable as a preview or release-candidate package. Before a stable `1.0.0`, the recommended next steps are broader numerical edge-case testing, multi-platform CI wheel publishing, and full generated API documentation.
