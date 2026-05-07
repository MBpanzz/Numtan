# NumTan Release Guide

This guide describes the local release workflow for NumTan.

## Pre-release Checks

Run formatting and Rust tests:

```bash
cargo fmt
cargo test
```

Install the extension locally and run Python tests:

```bash
maturin develop --release
python tests/python_smoke.py
python tests/api_surface.py
```

## Build Wheel

Build a PyPI-compatible wheel:

```bash
maturin build --release --compatibility pypi --auditwheel=repair
```

Generated wheels are written to:

```text
target/wheels/
```

## Check Metadata

```bash
python -m twine check target/wheels/*.whl
```

On Windows PowerShell:

```powershell
python -m twine check target\wheels\*.whl
```

## Test Install From Wheel

```bash
python -m pip install --force-reinstall target/wheels/<wheel-file>.whl
python -c "import numtan as nt; print(nt.tanpi(0.25))"
```

## Upload

Upload to TestPyPI first:

```bash
python -m twine upload --repository testpypi target/wheels/*
```

Upload to PyPI after confirming TestPyPI installation:

```bash
python -m twine upload target/wheels/*
```

## Release Notes Checklist

- Confirm version in `Cargo.toml` and `pyproject.toml`.
- Confirm `README.md` describes the current API level.
- Confirm `docs/API.md` lists all public Python functions.
- Confirm `tests/python_smoke.py` and `tests/api_surface.py` pass after wheel installation.
- Confirm CI passes on Windows, macOS, and Linux before publishing a broad release.
