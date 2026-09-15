# frame-capture-bevy

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture-bevy.svg)](https://crates.io/crates/frame-capture-bevy)

Bevy facade for running the same route interactively or through an offscreen
PNG capture runtime.

A Bevy integration follows four steps:

1. Define routes with `CaptureRouteBevy`.
2. Read a `BevyCaptureSession` before constructing `App`.
3. Derive the window resolution from the session and add `DefaultPlugins`
   through `session.add_capture_plugins`.
4. Install the selected route with `RoutePlugin`, a session state/resource
   helper, or a registered route installer.

Create the output directory before launching the application:

```sh
mkdir -p captures
FRAME_CAPTURE_ROUTE=dashboard \
FRAME_CAPTURE_PATH=captures/dashboard.png \
cargo run -p my-bevy-app
```

Use `CaptureReady` when capture must wait for asynchronous preparation. Enable
the `registry` feature when routes are registered as `fn(&mut App)` installers.
