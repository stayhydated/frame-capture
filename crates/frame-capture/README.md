# frame-capture

Target-neutral route and capture-session primitives for Rust application
screenshots.

This crate intentionally stops at the shared protocol: route specs, capture
dimensions, environment parsing, output paths, scenarios, and frame gating.
Renderer facades such as Bevy or an app-owned egui/GPUI integration own the
graphics backend, texture, and screenshot mechanics.

## Use

```toml
[dependencies]
frame-capture = "0.1"
```

Define routes with the default `macros` feature:

```rust
use frame_capture::{CaptureEnv, CaptureRoute as _};

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
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
        assert_eq!(capture.size(), route.spec().default_size());
        assert_eq!(capture.frame().get(), 12);
    }

    Ok(())
}
```

Route IDs default to `snake_case`, `id_prefix` prepends a shared route path
segment such as `desktop/`, titles default to the variant name, and sizes can
come from route attributes, enum attributes, or `frame-capture.toml`:

```toml
[default_size]
width = 1920
height = 1080
```

## Capture Environment

`CaptureEnv::frame_capture()` reads these variables:

| Variable | Purpose |
| --- | --- |
| `FRAME_CAPTURE_ROUTE` | Route id to render. Missing means the route enum default. |
| `FRAME_CAPTURE_PATH` | PNG output file path. Presence enables capture mode. |
| `FRAME_CAPTURE_FRAME` | Frame to wait for before capture. Defaults to `12`. |
| `FRAME_CAPTURE_WIDTH` | Override capture width. Must be paired with height. |
| `FRAME_CAPTURE_HEIGHT` | Override capture height. Must be paired with width. |
| `FRAME_CAPTURE_SCENARIO` | Optional app-defined state scenario id. |

`CaptureEnv::with_prefix("APP")` creates `APP_CAPTURE_ROUTE` and
`APP_CAPTURE_*`.
Use `CaptureEnv::try_with_prefix("APP")` when the prefix is user-supplied and
must be validated before constructing the environment.
`CaptureEnv::builder()` can set every variable name explicitly.

Use `CaptureLaunchEnv` when an external tool needs to return environment data
for a host-owned launch command:

```rust
use frame_capture::CaptureLaunchEnv;

let launch_env = CaptureLaunchEnv::builder()
    .route_id("desktop/dashboard")?
    .output_path("captures/dashboard/current.png")?
    .frame(12)?
    .size(1920, 1080)?
    .build();

let env = launch_env.env_map_lossy();
assert_eq!(env["FRAME_CAPTURE_ROUTE"], "desktop/dashboard");
assert_eq!(env["FRAME_CAPTURE_PATH"], "captures/dashboard/current.png");
assert_eq!(env["FRAME_CAPTURE_HEIGHT"], "1080");

# Ok::<(), Box<dyn std::error::Error>>(())
```

The helper validates route ids, PNG output paths, nonzero frames, and
all-or-nothing width/height overrides before producing `FRAME_CAPTURE_*`
variables. It does not launch a process.

## Scenarios

Scenarios are typed ids for app-owned state presets.

```rust
#[derive(frame_capture::CaptureScenario, Clone, Copy, Debug, Eq, PartialEq)]
enum UiScenario {
    #[capture_scenario(id = "empty", title = "Empty State")]
    Empty,
    Loaded,
}

fn read_state() -> Result<(), Box<dyn std::error::Error>> {
    let env = frame_capture::CaptureEnv::frame_capture();
    let session = env.read_session_with_scenario::<UiRoute, UiScenario>()?;
    let _scenario = session.scenario();

    Ok(())
}
```

Use `read_session::<Route>()` for route-only apps. Use
`read_session_with_scenario` or `read_session_with_inputs` when scenario
environment variables should be accepted and carried with the selected route
and capture config.

## Output Paths

Use `capture_output_path_for_name` or `capture_output_path_for_stem` when a
runner needs route-local output names:

```rust
use frame_capture::{CaptureOutputPath, CaptureOutputStem};

let output = CaptureOutputStem::current();
let path = CaptureOutputPath::for_stem("captures", UiRoute::Dashboard, &output)
    .expect("valid capture output path");
assert_eq!(path.as_path(), std::path::Path::new("captures/dashboard/current.png"));
```

## Implementation Boundary

This crate owns the target-neutral capture protocol. Renderer facades and host
applications own windows, textures, render passes, image encoders, event loops,
and screenshot saving.
