# frame-capture-toml

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

- [Configuration guide](https://stayhydated.github.io/frame-capture/book/configuration.html)
- [API documentation](https://docs.rs/frame-capture-toml/)
