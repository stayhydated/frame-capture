# frame-capture-routes

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture-routes.svg)](https://crates.io/crates/frame-capture-routes)

Route-only capture facade for Rust applications that already own their window,
renderer, frame loop, and screenshot pipeline. It fits egui, GPUI, raw wgpu,
and other host-rendered applications.

Define routes with `CaptureRouteRoutes`, then read one session during startup:

```rust,ignore
let session = frame_capture_routes::CaptureEnv::frame_capture()
    .read_session::<UiRoute>()?;
```

Use the selected route in live and capture modes. In capture mode, honor
`capture.size()` and `capture.frame()`, create the output directory, and save a
PNG to `capture.path()`. Use typed input sessions for scenarios and registered
routes when installers are distributed across modules.
