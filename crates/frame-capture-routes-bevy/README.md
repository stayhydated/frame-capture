# frame-capture-routes-bevy

[![Codecov: frame-capture-routes-bevy][codecov-badge]][codecov]
[![crates.io: frame-capture-routes-bevy][crate-badge]][crate]

Bevy `App` route registration for host-owned capture runtimes. Each registered
route installs itself through `fn(&mut App)` and carries a stable ID, title, and
default pixel size.

Requires Rust 1.99 or newer and Bevy 0.19.

## Overview

Most applications that want the provided Bevy screenshot runtime should enable
the `registry` feature on [`frame-capture-bevy`][bevy]. Depend on this crate directly
when another runtime owns window setup, rendering, and screenshot output.

Declare installers with `#[frame_capture_routes_bevy::capture_route(...)]`,
validate the registry during startup, read the selected registered session, and
call `session.install(&mut app)`.

## Features

The default feature set enables `macros`.

| Feature | Default | Purpose |
| --- | --- | --- |
| `default` | Yes | Enables `macros`. |
| `macros` | Yes | Reexports the Bevy `capture_route` attribute and enables `frame-capture/macros` for route/scenario derives. |

## Example

With the default `macros` feature, register an installer and resolve its typed
key before the host configures and runs the Bevy app:

```rust
use frame_capture_routes_bevy::{
    App, BevyCaptureRegistryEnvExt as _, CaptureEnv, validate_registered_routes,
};

#[frame_capture_routes_bevy::capture_route(
    id = "dashboard", key = DashboardRoute, title = "Dashboard", size = "960x540"
)]
fn install_dashboard(app: &mut App) {
    app.world_mut().spawn_empty();
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    validate_registered_routes()?;
    let session = CaptureEnv::frame_capture()
        .read_registered_session_for::<DashboardRoute>()?;
    let mut app = App::new();
    session.install(&mut app);
    assert_eq!(session.spec().id(), "dashboard");
    Ok(())
}
```

The installer is called for the selected route. The host still owns plugins,
rendering, and saving `session.capture()`; the [registry tests][tests] cover
registration and installation independently of a graphics backend.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-routes-bevy
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-routes-bevy.svg?label=frame-capture-routes-bevy
[crate]: https://crates.io/crates/frame-capture-routes-bevy
[bevy]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-bevy/README.md
[tests]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-routes-bevy/tests/registry.rs
