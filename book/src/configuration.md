# Configure sizes and output paths

Set route sizes at compile time and override them per capture when needed.
Use stable route IDs to derive predictable PNG locations.

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

## Parse defaults in a custom tool

Use `frame-capture-toml` directly when a tool needs to read the same default-size
format as the route macros. `CaptureToml::parse` accepts document text; the
caller chooses and reads the file. The parser does not search parent directories
or read `FRAME_CAPTURE_TOML`.

With `frame-capture-toml` available to the tool, parse a complete document:

```rust
use frame_capture_toml::CaptureToml;

fn main() -> Result<(), frame_capture_toml::CaptureTomlError> {
    let defaults = CaptureToml::parse(
        "[default_size]\nwidth = 1280\nheight = 720\n",
    )?;
    assert_eq!(defaults.default_size.width(), 1280);
    assert_eq!(defaults.default_size.height(), 720);
    Ok(())
}
```

The result contains the validated pixel dimensions. If parsing fails, check that
`[default_size]` supplies both `width` and `height` as positive integers that fit
in `u32`. Invalid TOML and values outside that integer range produce
`CaptureTomlError::Toml`; missing or zero dimensions produce the corresponding
dimension error. A missing `[default_size]` table produces `MissingDefaultSize`.

## Build deterministic output paths

Use `CaptureOutputPath` for deterministic route-local PNG locations. This
example uses the `UiRoute` catalog from [typed routes](routes.md):

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
