# Configure sizes and output paths

## Configure a shared default size

When a route or enum does not declare both dimensions, the route macro reads a
shared default from `frame-capture.toml`:

```toml
[default_size]
width = 1280
height = 720
```

The macro starts at `CARGO_MANIFEST_DIR` and uses the closest
`frame-capture.toml` found while searching parent directories. Set
`FRAME_CAPTURE_TOML` during compilation to select an explicit existing file.

Capture sizes use this precedence:

1. Paired `FRAME_CAPTURE_WIDTH` and `FRAME_CAPTURE_HEIGHT` values in capture
   mode.
2. A route-level `size`, `width`, or `height` attribute.
3. An enum-level size attribute.
4. The discovered `frame-capture.toml` default for any missing dimensions.

Every dimension must be greater than zero. The compile-time macro reports an
error if it cannot resolve both dimensions.

## Build deterministic output paths

Use `CaptureOutputPath` for deterministic route-local PNG locations:

```rust,ignore
use frame_capture_routes::{CaptureOutputPath, CaptureOutputStem};

let path = CaptureOutputPath::for_stem(
    "captures",
    UiRoute::Dashboard,
    &CaptureOutputStem::current(),
)?;

assert_eq!(
    path.as_path(),
    std::path::Path::new("captures/desktop/dashboard/current.png"),
);
```

`CaptureOutputPath` validates the filename and `.png` extension; it does not
create directories. Create `path.as_path().parent()` before asking the host
renderer to save. Relative paths are resolved from the launched application's
working directory.

Stable route IDs make output paths, CI artifacts, baselines, and MCP discovery
refer to the same surface. In this example, the `desktop/dashboard` route ID
becomes nested directories under `captures`.
