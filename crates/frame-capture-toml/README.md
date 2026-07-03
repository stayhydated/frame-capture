# frame-capture-toml

TOML parsing helpers for shared `frame-capture` defaults.

This crate is used by the route macros to read package-local default capture
sizes. It is intentionally small and only parses explicit `default_size` entries
with `width` and `height` values.

## Accepted TOML

```toml
[default_size]
width = 1920
height = 1080
```

## Use

```rust
use frame_capture_toml::{CaptureToml, PixelSize};

let parsed = CaptureToml::parse(
    r#"
[default_size]
width = 1280
height = 720
"#
)
    .expect("valid TOML");
assert_eq!(parsed.default_size, PixelSize::new(1280, 720));
```

`DEFAULT_FILE_NAME` is `frame-capture.toml`.

## Implementation Boundary

This crate is not a general workspace manifest parser. It accepts only explicit
default-size configuration and keeps size literal parsing aligned with route
macro attributes.
