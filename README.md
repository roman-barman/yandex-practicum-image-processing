# Yandex Practicum Image Processing

Rust workspace for experimenting with image processing via dynamically loaded plugins.

Projects
- `image-processor`: CLI that loads a plugin and applies it to an input image.
- `blur-plugin`: Example blur plugin.
- `mirror-plugin`: Example mirror plugin.
- `plugin-parameters`: Sample parameter files for the plugins.

Requirements
- Rust toolchain (2024 edition)
- Supported image format: `png`

Quick start
```bash
cargo build -p blur-plugin
cargo build -p mirror-plugin

cargo run -p image-processor -- \
  --input images/input/photo.png \
  --output images/output/photo.png \
  --plugin libblur_plugin \
  --params plugin-parameters/blur.txt
```

Notes
- Plugins are loaded from `target/debug` by default; override with `--plugin-path`.
- Plugin name can be passed without extension; the CLI appends `.so` on Linux and `.dll` on Windows.
- Parameters are read from a text file and passed to the plugin as a UTF-8 string.
