# NumTan 发布指南

本文档描述 NumTan 的本地发布流程。

## 发布前检查

运行格式化和 Rust 测试：

```bash
cargo fmt
cargo test
```

本地安装扩展并运行 Python 测试：

```bash
maturin develop --release
python tests/python_smoke.py
python tests/api_surface.py
```

## 构建 Wheel

构建 PyPI 兼容 wheel：

```bash
maturin build --release --compatibility pypi --auditwheel=repair
```

生成的 wheel 位于：

```text
target/wheels/
```

## 检查元数据

```bash
python -m twine check target/wheels/*.whl
```

Windows PowerShell：

```powershell
python -m twine check target\wheels\*.whl
```

## 从 Wheel 测试安装

```bash
python -m pip install --force-reinstall target/wheels/<wheel-file>.whl
python -c "import numtan as nt; print(nt.tanpi(0.25))"
```

## 上传

建议先上传到 TestPyPI：

```bash
python -m twine upload --repository testpypi target/wheels/*
```

确认 TestPyPI 安装无误后，再上传到正式 PyPI：

```bash
python -m twine upload target/wheels/*
```

## 发布说明检查清单

- 确认 `Cargo.toml` 与 `pyproject.toml` 中的版本号一致。
- 确认 `README.md` 和 `README.zh-CN.md` 描述当前 API 能力。
- 确认 `docs/API.md` 和 `docs/API.zh-CN.md` 列出全部公开 Python 函数。
- 确认从 wheel 安装后，`tests/python_smoke.py` 和 `tests/api_surface.py` 通过。
- 面向正式稳定版本发布前，确认 Windows、macOS 和 Linux 的 CI 构建全部通过。
