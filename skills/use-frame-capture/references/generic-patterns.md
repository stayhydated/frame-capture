# Generic frame-capture patterns

## Read a route-only session

Use `frame-capture-routes` when the host owns rendering and PNG output:

```rust,ignore
use frame_capture_routes::{CaptureEnv, CaptureRoute as _};

#[derive(frame_capture_routes::CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
    #[capture_route(id = "review", title = "Review")]
    Review,
}

let session = CaptureEnv::frame_capture().read_session::<UiRoute>()?;
let route = *session.route();

match session.capture() {
    Some(capture) => {
        wait_until(capture.frame());
        let pixels = render(route, capture.size());
        create_parent_directory(capture.path())?;
        save_png(&pixels, capture.path())?;
    }
    None => run_live(route),
}
```

`FRAME_CAPTURE_PATH` enables capture mode. Size overrides require both
`FRAME_CAPTURE_WIDTH` and `FRAME_CAPTURE_HEIGHT`; the default frame is `12`.

## Apply a typed scenario

Read a typed input session when the host supports scenario presets:

```rust,ignore
let session = CaptureEnv::frame_capture()
    .read_session_with_scenario::<UiRoute, UiScenario>()?;
apply_scenario(session.scenario());
```

Apply the scenario before rendering. Scenario IDs cannot contain path
separators.

## Install a registered route

```rust,ignore
use frame_capture_routes::{CaptureEnv, CaptureRoutesEnvExt as _};

#[frame_capture_routes::capture_route(
    id = "dashboard",
    title = "Dashboard",
    size = "1280x720"
)]
fn install_dashboard() {
    // Select host-owned content.
}

frame_capture_routes::validate_registered_routes()?;
let session = CaptureEnv::frame_capture()
    .read_registered_session_for::<InstallDashboardRoute>()?;
session.install();
```

The macro derives `InstallDashboardRoute` from `install_dashboard`. Set
`key = MyRouteKey` when an explicit generated key is clearer.

## Build launch environment data

```rust,ignore
let launch = frame_capture_routes::CaptureLaunchEnv::builder()
    .route_id("dashboard")?
    .output_path("captures/dashboard.png")?
    .frame(12)?
    .size(1280, 720)?
    .build();

command.envs(launch.env_map_lossy());
```

The builder validates protocol values but does not create or launch a process.

## Expose route metadata

Use `frame_capture_mcp::serve_capture_routes_stdio::<UiRoute>()` for an enum
catalog or `serve_registered_capture_routes_stdio()` for inventory-backed
routes. Keep capture execution in the environment-driven host application.
Retain the server across requests; request completion does not imply shutdown.
