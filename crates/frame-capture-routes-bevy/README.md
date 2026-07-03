# frame-capture-routes-bevy

Bevy `App` route-registration facade for `frame-capture`.

This crate registers capture routes as `fn(&mut App)` installers. It does not
include the Bevy screenshot runtime; use it when route registration should be
Bevy-specific but capture rendering is handled elsewhere, or through the
`registry` feature of `frame-capture-bevy`.

## Use

```toml
[dependencies]
frame-capture-routes-bevy = "0.1"
```

```rust
use bevy_app::App;
use frame_capture_routes_bevy::{BevyCaptureRegistryEnvExt as _, CaptureEnv};

#[frame_capture_routes_bevy::capture_route(
    id = "dashboard",
    title = "Dashboard",
    size = "1280x720"
)]
fn install_dashboard(app: &mut App) {
    app.add_systems(bevy_app::Startup, setup_dashboard);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_registered_session_for::<InstallDashboardRoute>()?;

    let mut app = App::new();
    session.install(&mut app);

    Ok(())
}

fn setup_dashboard() {}
```

`read_registered_session_for::<InstallDashboardRoute>()` uses the generated
route key as the default route. If `FRAME_CAPTURE_ROUTE` is set, the requested
route is resolved against the registered Bevy route inventory instead.

## Runtime Helpers

`RegisteredCaptureSession` exposes:

- `route()` for the selected registered route.
- `spec()` for the route id, title, and default size.
- `capture()` or `into_capture()` for optional capture configuration.
- `install(&mut App)` to run the route installer.
- `is_capture()` to distinguish capture mode from live mode.

Use `registered_routes()` to list routes and `validate_registered_routes()` to
detect duplicate ids.

## Features

The default `macros` feature enables the `capture_route` attribute. Disable
default features when route registrations are created manually with
`RegisteredRoute::new`.

## Implementation Boundary

This crate owns Bevy `App` route registration only. Window setup, offscreen
images, schedule runners, screenshot saving, and capture exit behavior belong
to `frame-capture-bevy` or the host application.
