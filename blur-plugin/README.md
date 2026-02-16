# blur-plugin

Blur effect plugin for `image-processor`.

Build
```bash
cargo build -p blur-plugin
```

Parameters
- Format: `radius=<usize>;iterations=<usize>`
- Example: `radius=5;iterations=5`
- `radius` or `iterations` of `0` results in no-op.

Interface
- Exports `process_image(width, height, rgba_ptr, params_ptr)` as a C ABI symbol.
- Operates on an in-place RGBA8 buffer (`width * height * 4` bytes).
