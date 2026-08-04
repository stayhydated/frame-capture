# frame-capture

Target-neutral route and capture-session primitives for Rust applications.
Use this crate directly when building a custom renderer facade or shared
protocol layer. Bevy applications normally use `frame-capture-bevy`; applications
with an existing screenshot pipeline normally use `frame-capture-routes`.

## Add the dependency

```toml
[dependencies]
frame-capture = "0.1"
```

The default `macros` feature provides the `CaptureRoute` and `CaptureScenario`
derives. The runtime surface provides typed route and scenario IDs,
`CaptureEnv`, capture sessions, pixel sizes, frame gates, launch environment
data, and validated PNG output paths.

`FRAME_CAPTURE_PATH` selects capture mode. A custom facade is responsible for
rendering the selected route at the requested size and frame and saving the PNG
to the requested path.

- [User guide](https://stayhydated.github.io/frame-capture/book/)
- [API documentation](https://docs.rs/frame-capture/)
