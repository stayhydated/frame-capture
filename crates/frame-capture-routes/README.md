# frame-capture-routes

[![Codecov: frame-capture-routes][codecov-badge]][codecov]
[![crates.io: frame-capture-routes][crate-badge]][crate]

Route-only capture facade for Rust applications that already own their window,
renderer, frame loop, and screenshot pipeline. It fits egui, GPUI, raw wgpu,
and other host-rendered applications.

Depend on this facade directly for host-owned screenshot pipelines. Use
[`frame-capture-bevy`][bevy] when the supplied Bevy runtime should save the PNG.

Requires Rust 1.99 or newer. The host supplies rendering and PNG output.

## Overview

Define routes with `CaptureRouteRoutes`, then read one session during startup
with `CaptureEnv::frame_capture().read_session::<Route>()?`.

Use the selected route in live and capture modes. In capture mode, honor
`capture.size()` and `capture.frame()`, create the output directory, and save a
PNG to `capture.path()`. Use typed input sessions for scenarios and registered
routes when installers are distributed across modules.

## Features

The default feature set enables `macros`.

| Feature | Default | Purpose |
| --- | --- | --- |
| `default` | Yes | Enables `macros`. |
| `macros` | Yes | Reexports `CaptureRouteRoutes`, `CaptureScenarioRoutes`, and `capture_route` for `fn()` installers. |

## Example

With the default `macros` feature, define a route catalog and read the session
once before the host starts its renderer:

```rust
use frame_capture_routes::{CaptureEnv, CaptureRoute as _, CaptureRouteRoutes};

#[derive(CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "960x540")]
enum Route {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_session::<Route>()?;
    assert_eq!(session.route().id(), "dashboard");
    if let Some(capture) = session.capture() {
        println!("Render {} to {}", capture.size(), capture.path().display());
    }
    Ok(())
}
```

The session selects `dashboard` in both modes. This example reports the capture
request; the host must honor its size and frame and save the PNG. The
[facade tests][tests] cover typed and registered sessions.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-routes
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-routes.svg?label=frame-capture-routes
[crate]: https://crates.io/crates/frame-capture-routes
[bevy]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-bevy/README.md
[tests]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-routes/tests/routes.rs
