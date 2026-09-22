# frame-capture-toml

[![Codecov: frame-capture-toml][codecov-badge]][codecov]
[![crates.io: frame-capture-toml][crate-badge]][crate]

Parser for the shared `frame-capture.toml` default-size format used by the route
macros.

## Example

```toml
[default_size]
width = 1920
height = 1080
```

Both dimensions are required and must be positive integers. Application code
normally configures this file and lets its selected facade's route macro read
it. Depend on `frame-capture-toml` directly only when parsing the same format in
a custom tool or facade.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-toml
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-toml.svg?label=frame-capture-toml
[crate]: https://crates.io/crates/frame-capture-toml
