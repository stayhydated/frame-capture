# frame-capture-routes

Route-only capture facade for Rust applications that already own their window,
renderer, frame loop, and screenshot pipeline. It fits egui, GPUI, raw wgpu,
and other host-rendered applications.

## Add the dependency

```toml
[dependencies]
frame-capture-routes = "0.1"
```

Define routes with `CaptureRouteRoutes`, then read one session during startup:

```rust,ignore
let session = frame_capture_routes::CaptureEnv::frame_capture()
    .read_session::<UiRoute>()?;
```

Use the selected route in live and capture modes. In capture mode, honor
`capture.size()` and `capture.frame()`, create the output directory, and save a
PNG to `capture.path()`. Use typed input sessions for scenarios and registered
routes when installers are distributed across modules.

- [Route-only guide](https://stayhydated.github.io/frame-capture/book/route_only.html)
- [API documentation](https://docs.rs/frame-capture-routes/)
