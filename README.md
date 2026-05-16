# NumTan

**Tangent-centered numerical tools for Python, powered by Rust.**

NumTan is a compact numerical toolkit for teaching, experiments, and small scientific workflows. It brings together derivatives, root finding, ODE solvers, quadrature, statistics, interpolation, polynomial helpers, signal utilities, and plot-ready visualization data under one lightweight Python API.

The project is built in Rust and exposed to Python with PyO3, so the core algorithms stay fast, dependency-light, and easy to package as wheels.

Current stable version: `1.0.0`.

## AI Assistance Disclosure

This project was developed with AI assistance. Human maintainers remain responsible for reviewing, testing, and accepting code, documentation, release decisions, and published artifacts.

## Why NumTan

- **Small surface, broad coverage**: common numerical routines without pulling in a large scientific stack.
- **Teaching-friendly results**: iterative methods expose `history` data for plotting and inspection.
- **Rust core, Python ergonomics**: call plain Python functions while the numerical kernels live in Rust.
- **Plot-ready by design**: visualization helpers return regular dictionaries and lists, not framework-specific objects.
- **Stable v1 API**: the public Python API is frozen for the `1.x` series.

## Installation

Install the stable release from PyPI:

```bash
python -m pip install numtan
```

Upgrade an existing installation:

```bash
python -m pip install --upgrade numtan
```

Install a local wheel:

```bash
python -m pip install target/wheels/numtan-1.0.0-cp313-cp313-win_amd64.whl
```

Build and install from this repository for development:

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

Expected values are approximately `sqrt(2)`, `12.0`, `1/3`, and `e`.

## Feature Map

| Area | Functions |
| --- | --- |
| Derivatives | `tangent`, `gradient` |
| Root finding | `newton`, `halley`, `householder` |
| Linear algebra | `dot`, `norm`, `add`, `sub`, `scale`, `mat_vec`, `solve` |
| Optimization | `gradient_descent`, `tangent_minimize`, `stationary_newton`, `gauss_newton` |
| ODE solvers | `euler`, `midpoint`, `rk4`, `adaptive_rk4` |
| Integration | `tanh_sinh`, `tan_sinh`, `quad_inf` |
| Trigonometric extras | `tanpi`, `tanint`, `atanint`, `complex_tan`, `tan_deg`, `tan_grad` |
| Visualization data | `tangent_lines`, `newton_animation_data`, `ode_direction_field` |
| Statistics | `mean`, `variance`, `summary`, `covariance`, `correlation`, `linear_regression`, `polynomial_regression` |
| Interpolation | `linspace`, `sample_grid`, `linear_interpolate`, `lagrange_interpolate`, `finite_difference` |
| Polynomials | `polyval`, `polyder`, `polyint`, `polyadd`, `polymul`, `polyroot` |
| Signals | `moving_average`, `exponential_smooth`, `convolve`, `normalize`, `find_peaks` |

## Examples

### Roots With Iteration History

```python
result = nt.newton(lambda x: x**3 - 2.0 * x - 5.0, 2.0)

print(result["root"])
print(result["converged"])
print(result["history"][0])
```

### Optimization

```python
minimum = nt.gradient_descent(
    lambda v: (v[0] - 2.0) ** 2 + (v[1] + 1.0) ** 2,
    [0.0, 0.0],
)

print(minimum["point"])
```

### Data for Plotting

```python
lines = nt.tangent_lines(lambda x: x * x, 0.0, 2.0, [0.5, 1.0, 1.5])
field = nt.ode_direction_field(lambda t, y: y, (0.0, 1.0), (0.0, 2.0), 8, 8)
```

The returned values are plain Python lists and dictionaries, ready for Matplotlib, Plotly, or a notebook widget.

## Documentation

- English documentation index: `docs/INDEX.md`
- Chinese documentation index: `docs/INDEX.zh-CN.md`
- User guide: `docs/USAGE.md`
- API reference: `docs/API.md`
- Chinese README: `README.zh-CN.md`
- Release guide: `docs/RELEASE.md`
- Runnable demo: `examples/python_demo.py`

## Testing

Run Rust tests:

```bash
cargo test
```

After installing the Python extension, run the Python checks:

```bash
python tests/python_smoke.py
python tests/api_surface.py
```

## Design Notes

- Core algorithms live in `src/core` and do not depend on Python.
- Python bindings live in `src/api` and convert Rust results into Python `dict`, `list`, `tuple`, and `float` values.
- Callback exceptions raised in Python are propagated back to Python.
- Undefined numerical inputs, such as correlation with a constant vector, raise `ValueError`.

## Status

NumTan `1.0.0` is the first stable API release. It is ready for normal Python installation, local teaching material, numerical demos, and small dependency-conscious tools.
