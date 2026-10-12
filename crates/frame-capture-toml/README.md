# frame-capture-toml

[![Codecov: frame-capture-toml][codecov-badge]][codecov]
[![crates.io: frame-capture-toml][crate-badge]][crate]

Parser for the shared `frame-capture.toml` default-size format used by the route
macros.

Application code normally configures this file and lets its selected facade's
route macro read it. Depend on `frame-capture-toml` directly when parsing the
same format in a custom tool or facade.

Requires Rust 1.99 or newer.

## Configuration

Place `frame-capture.toml` in the application package directory or a parent
directory. Route macros discover it from `CARGO_MANIFEST_DIR` and use its size
when a route declaration has no explicit dimensions:

```toml
[default_size]
width = 1920
height = 1080
```

Both dimensions are required and must be positive integers. This configuration
sets the shared default size to 1920 × 1080.

## Example

A custom tool can parse the same text through the public API:

```rust
use frame_capture_toml::CaptureToml;

fn main() -> Result<(), frame_capture_toml::CaptureTomlError> {
    let defaults = CaptureToml::parse(
        "[default_size]\nwidth = 1920\nheight = 1080\n",
    )?;
    assert_eq!(defaults.default_size.width(), 1920);
    assert_eq!(defaults.default_size.height(), 1080);
    Ok(())
}
```

The [parser and its tests][parser] own the format and validation rules.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-toml
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-toml.svg?label=frame-capture-toml
[crate]: https://crates.io/crates/frame-capture-toml
[parser]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-toml/src/lib.rs
