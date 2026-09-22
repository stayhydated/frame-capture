# frame-capture-routes

[![Codecov: frame-capture-routes][codecov-badge]][codecov]
[![crates.io: frame-capture-routes][crate-badge]][crate]

Route-only capture facade for Rust applications that already own their window,
renderer, frame loop, and screenshot pipeline. It fits egui, GPUI, raw wgpu,
and other host-rendered applications.

## Overview

Define routes with `CaptureRouteRoutes`, then read one session during startup
with `CaptureEnv::frame_capture().read_session::<Route>()?`.

Use the selected route in live and capture modes. In capture mode, honor
`capture.size()` and `capture.frame()`, create the output directory, and save a
PNG to `capture.path()`. Use typed input sessions for scenarios and registered
routes when installers are distributed across modules.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-routes
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-routes.svg?label=frame-capture-routes
[crate]: https://crates.io/crates/frame-capture-routes
