# frame-capture-toml

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture-toml.svg)](https://crates.io/crates/frame-capture-toml)

Parser for the shared `frame-capture.toml` default-size format used by the route
macros.

```toml
[default_size]
width = 1920
height = 1080
```

Both dimensions are required and must be positive integers. Application code
normally configures this file and lets its selected facade's route macro read
it. Depend on `frame-capture-toml` directly only when parsing the same format in
a custom tool or facade.
