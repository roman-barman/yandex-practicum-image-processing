# image-processor

CLI tool that loads a dynamic plugin and applies it to an image.

Build
```bash
cargo build -p image-processor
```

Usage
```bash
cargo run -p image-processor -- \
  --input images/input/photo.png \
  --output images/output/photo.png \
  --plugin libblur_plugin \
  --params plugin-parameters/blur.txt
```

Arguments
- `--input`: Input image path (png only).
- `--output`: Output image path (png only).
- `--plugin`: Plugin name or filename (extension optional).
- `--params`: Path to a text file with plugin parameters.
- `--plugin-path`: Directory to load plugins from (defaults to `target/debug`).

Notes
- The CLI appends `.so` on Linux and `.dll` on Windows when the extension is omitted.
