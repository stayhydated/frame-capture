# Define typed routes

Define one typed catalog so live runs, capture commands, output paths, and tools
refer to the same visual surfaces. Each route has an ID, title, and default
pixel size.

## Define an enum catalog

```rust,ignore
use frame_capture_routes::CaptureRoute as _;

#[derive(
    frame_capture_routes::CaptureRouteRoutes,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
#[capture_route(default = Dashboard, size = "1280x720", id_prefix = "desktop")]
enum UiRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,

    #[capture_route(id = "review", title = "Review", size = "960x640")]
    Review,
}
```

This catalog exposes `desktop/dashboard` and `desktop/review`. The review route
uses `960x640`; the dashboard route inherits `1280x720`.

Use the derive exported by your facade:

| Facade | Route derive | Scenario derive |
| --- | --- | --- |
| `frame-capture` | `CaptureRoute` | `CaptureScenario` |
| `frame-capture-routes` | `CaptureRouteRoutes` | `CaptureScenarioRoutes` |
| `frame-capture-bevy` | `CaptureRouteBevy` | `CaptureScenarioBevy` |

Route IDs may contain relative path components such as `desktop/dashboard`,
but cannot be absolute or contain `.` or `..` components. `id_prefix` prepends
the prefix to generated and explicit variant IDs. Generated IDs use
`snake_case`, titles default to the variant name, and the first variant is the
default unless `default = ...` selects another one.

Put a shared size on the enum and override only routes with a different canvas.
If no size attribute supplies a dimension, the macro reads
[`frame-capture.toml`](configuration.md#configure-a-shared-default-size).

## Register route installers

Use registered routes when installers live beside feature modules instead of
in one enum:

```rust,ignore
use frame_capture_routes::{CaptureEnv, CaptureRoutesEnvExt as _};

#[frame_capture_routes::capture_route(
    id = "dashboard",
    title = "Dashboard",
    size = "1280x720"
)]
fn install_dashboard() {
    // Select application-owned content.
}

frame_capture_routes::validate_registered_routes()?;
let session = CaptureEnv::frame_capture()
    .read_registered_session_for::<InstallDashboardRoute>()?;
session.install();
```

The macro derives the key name from the function:
`install_dashboard` creates `InstallDashboardRoute`. Use `key = MyRouteKey` to
choose an explicit key. Call `validate_registered_routes()` during startup or
tests so duplicate route IDs fail before a capture begins.
