# frame-capture-bevy

[![Codecov: frame-capture-bevy][codecov-badge]][codecov]
[![crates.io: frame-capture-bevy][crate-badge]][crate]

`frame-capture-bevy` is the application-facing Bevy facade for running the same
route interactively or through an offscreen PNG capture runtime. Depend on it
directly when the facade should own screenshot output; use
[`frame-capture-routes-bevy`][routes-bevy] when the host owns that runtime.

Requires Rust 1.99 or newer and Bevy 0.19. Capture runs need a working graphics
backend and an existing output directory.

## Overview

A Bevy integration follows four steps:

1. Define routes with `CaptureRouteBevy`.
2. Read a `BevyCaptureSession` before constructing `App`.
3. Derive the window resolution from the session and add `DefaultPlugins`
   through `session.add_capture_plugins`.
4. Install the selected route with `RoutePlugin`, a session state/resource
   helper, or a registered route installer.

Use `CaptureReady` when capture must wait for asynchronous preparation. Enable
the `registry` feature when routes are registered as `fn(&mut App)` installers.

## Features

The default feature set enables `macros`. `registry` also enables `macros`.

| Feature | Default | Purpose |
| --- | --- | --- |
| `default` | Yes | Enables `macros`. |
| `macros` | Yes | Reexports `CaptureRouteBevy` and `CaptureScenarioBevy`. |
| `registry` | No | Enables `macros`, the `capture_route` attribute, and `frame-capture-routes-bevy` registry/session APIs. |

## Example

This excerpt from the [complete Bevy application][example] requires the default
`macros` feature. Its host supplies `BevyExampleRoute`, `BevyExampleScenario`,
and a `Window` configured with `session.window_resolution()`.

```rust
use bevy::prelude::*;
use frame_capture_bevy::{BevyCaptureEnvExt as _, CaptureEnv};
use frame_capture_example_bevy::{BevyExampleRoute, BevyExampleScenario};

let session = CaptureEnv::frame_capture()
    .read_bevy_session_with_scenario::<BevyExampleRoute, BevyExampleScenario>()?;
// The host constructs `window` using this session's resolution.
let mut app = App::new();
session.add_capture_plugins(
    &mut app,
    DefaultPlugins.set(session.capture_window_plugin(window)),
);
session.add_route_state(&mut app);
```

The complete application adds a camera and scene in `Startup`, then runs the
app. With `FRAME_CAPTURE_PATH`, it writes the selected route to the requested
PNG and exits; without it, the same route remains interactive.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-bevy
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-bevy.svg?label=frame-capture-bevy
[crate]: https://crates.io/crates/frame-capture-bevy
[routes-bevy]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-routes-bevy/README.md
[example]: https://github.com/stayhydated/frame-capture/blob/master/examples/bevy/src/app.rs
