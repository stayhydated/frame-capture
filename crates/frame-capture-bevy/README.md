# frame-capture-bevy

[![Codecov: frame-capture-bevy][codecov-badge]][codecov]
[![crates.io: frame-capture-bevy][crate-badge]][crate]

Bevy facade for running the same route interactively or through an offscreen
PNG capture runtime.

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

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-bevy
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-bevy.svg?label=frame-capture-bevy
[crate]: https://crates.io/crates/frame-capture-bevy
