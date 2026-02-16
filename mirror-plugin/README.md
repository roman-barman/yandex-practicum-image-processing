# mirror-plugin

Mirror effect plugin for `image-processor`.

Build
```bash
cargo build -p mirror-plugin
```

Parameters
- `horizontal` to mirror left/right.
- `vertical` to mirror top/bottom.

Interface
- Exports `process_image(width, height, rgba_ptr, params_ptr)` as a C ABI symbol.
- Operates on an in-place RGBA8 buffer (`width * height * 4` bytes).
