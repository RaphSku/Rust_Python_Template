# Rust_Python_Template
Template for using Rust with Python

## Setup
Create a virtual python environment and then run
```
pip install maturin
```

```bash
maturin new --bindings pyo3 --mixed example_project
```

Switch to the `python/example_project` directory and run
```bash
poetry init
```
