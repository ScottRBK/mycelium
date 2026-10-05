# Running without uv

Use an installed `mycelium-map` 0.4.2+, or install it into an activated Python 3.12+
virtual environment:

```bash
python -m pip install --upgrade --only-binary=mycelium-map 'mycelium-map>=0.4.2'
mycelium-map export --help
```

Replace the workflow's uvx prefix with `mycelium-map`. Prebuilt wheels include the Rust engine;
if no wheel matches, report the Python/platform limitation instead of starting a source build.
