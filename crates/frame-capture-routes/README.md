# frame-capture-routes

Route-only capture facade for `frame-capture` applications.

Use this crate for egui, GPUI, raw wgpu, or any app where route selection and
capture metadata are shared, but rendering and screenshot saving stay owned by
the host application.

## Use

```toml
[dependencies]
frame-capture-routes = "0.1"
```

```rust
use frame_capture_routes::{CaptureEnv, CaptureRoute as _};

#[derive(frame_capture_routes::CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, id_prefix = "desktop", size = "1280x720")]
enum UiRoute {
    #[capture_route(title = "Dashboard")]
    Dashboard,
    #[capture_route(title = "Review")]
    Review,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_session::<UiRoute>()?;
    let route = *session.route();

    if let Some(capture) = session.capture() {
        render(route, capture.size());
        save(capture.path())?;
    } else {
        run_live(route);
    }

    Ok(())
}

fn render(_route: UiRoute, _size: frame_capture_routes::PixelSize) {}

fn save(_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn run_live(_route: UiRoute) {}
```

`CaptureEnv::frame_capture()` uses `FRAME_CAPTURE_ROUTE`, PNG-only `FRAME_CAPTURE_PATH`,
`FRAME_CAPTURE_FRAME`, `FRAME_CAPTURE_WIDTH`, `FRAME_CAPTURE_HEIGHT`,
`FRAME_CAPTURE_SCENARIO`.
Use `read_session_with_scenario` or `read_session_with_inputs` when the
route-only app supports scenario ids.

Tools that prepare a launch command can use the re-exported
`CaptureLaunchEnv` builder to produce the same variables without hand-building
raw strings:

```rust
use frame_capture_routes::CaptureLaunchEnv;

let launch_env = CaptureLaunchEnv::builder()
    .route_id("desktop/dashboard")?
    .output_path("captures/dashboard/current.png")?
    .size(1280, 720)?
    .build();

let env = launch_env.env_map_lossy();
assert_eq!(env["FRAME_CAPTURE_ROUTE"], "desktop/dashboard");
assert_eq!(env["FRAME_CAPTURE_PATH"], "captures/dashboard/current.png");

# Ok::<(), Box<dyn std::error::Error>>(())
```

`CaptureLaunchEnv` returns environment data only; the host application or tool
still owns process launch and screenshot persistence.

## Registered Routes

Use registered routes when route installers are distributed across modules or
when a function is a better route boundary than a single enum variant.

```rust
use frame_capture_routes::{CaptureEnv, CaptureRoutesEnvExt as _};

#[frame_capture_routes::capture_route(id = "dashboard", title = "Dashboard", size = "1280x720")]
fn install_dashboard() {
    // Select app-owned state or content for this route.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_registered_session_for::<InstallDashboardRoute>()?;
    session.install();

    if let Some(capture) = session.capture() {
        render_registered(session.spec(), capture.size());
        save(capture.path())?;
    }

    Ok(())
}

fn render_registered(
    _route: frame_capture_routes::RouteSpec,
    _size: frame_capture_routes::PixelSize,
) {
}

fn save(_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
```

The macro creates a typed route key from the function name. For example,
`install_dashboard` creates `InstallDashboardRoute`. Pass `key = MyRouteKey` to
choose the generated key name explicitly.

Use `validate_registered_routes()` in startup or tests to catch duplicate route
ids.

## Features

The default `macros` feature enables route/scenario derives (for this crate:
`CaptureRouteRoutes` and `CaptureScenarioRoutes`) and the
`capture_route` registration attribute. Disable default features when consuming
only the runtime registry and reexports.

## Implementation Boundary

This crate does not render frames, open windows, or save screenshots. It owns
route selection, registered route metadata, optional capture config, and the
host-owned installer boundary.
