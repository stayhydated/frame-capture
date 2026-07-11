# frame-capture

[![Build Status](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/github/stayhydated/frame-capture/graph/badge.svg?token=34CV04UOU1)](https://codecov.io/github/stayhydated/frame-capture)
[![Docs](https://docs.rs/frame-capture/badge.svg)](https://docs.rs/frame-capture/)
[![Crates.io](https://img.shields.io/crates/v/frame-capture.svg)](https://crates.io/crates/frame-capture)

Deterministic screenshot capture workflows for Rust applications.

`frame-capture` gives an app a typed route catalog, a small environment-variable
protocol, and capture session metadata. The app chooses the route to render in
normal live mode or capture mode, while the renderer-specific integration owns
the actual window and screenshot plumbing. The shared capture protocol is
renderer-agnostic: host integrations can use their framework's native backend,
including GPUI's Metal renderer on macOS.

## Pick an Integration

Use `frame-capture-bevy` for Bevy apps that want the shared offscreen screenshot
runtime.

Use `frame-capture-routes` for any app that already owns its window and
screenshot pipeline, including egui, GPUI, and raw wgpu applications. This
facade supplies route and capture metadata without selecting a graphics backend.

Use `frame-capture` directly when you are building a custom renderer
integration and only need the shared route, environment, size, output-path,
scenario, and frame-gating types.

```toml
[dependencies]
frame-capture-bevy = "0.1"
# or
frame-capture-routes = "0.1"
# or, for custom integrations
frame-capture = "0.1"
```

## Documentation Map

Root and crate READMEs are public, example-first usage guides. Crate rustdocs
document API contracts and implementation boundaries. Repo-local contributor
workflow guidance lives in `AGENTS.md`, and public application-developer skill
guidance lives under `skills/`.

## Define Routes

Routes are normal Rust enums. Each route has an id, a title, and a default pixel
size.

```rust
use frame_capture::CaptureRoute as _;

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, id_prefix = "desktop", size = "1280x720")]
enum UiRoute {
    #[capture_route(title = "Dashboard")]
    Dashboard,
    #[capture_route(title = "Review")]
    Review,
}

assert_eq!(UiRoute::Dashboard.id(), "desktop/dashboard");
assert_eq!(UiRoute::Review.spec().default_size().width(), 1280);
```

Use `id_prefix` when one enum is reused by a specific app surface, such as
`desktop/dashboard` and `mobile/dashboard`.

If the size is omitted from the enum or route, the macro reads
`frame-capture.toml` from the package:

```toml
[default_size]
width = 1920
height = 1080
```

## Bevy Apps

`frame-capture-bevy` configures Bevy for normal interactive runs or capture runs
from the same binary. In capture mode it disables the primary window, renders to
an offscreen image, saves a PNG through Bevy's screenshot API, and exits.

```rust
use bevy::{
    prelude::*,
    window::{PresentMode, Window},
};
use frame_capture_bevy::{
    BevyCaptureEnvExt as _, CaptureEnv, CaptureRoute as _, RoutePlugin,
};

#[derive(frame_capture_bevy::CaptureRouteBevy, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
    #[capture_route(id = "review", title = "Review")]
    Review,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_bevy_session::<UiRoute>()?;
    let route = session.route();

    let window = Window {
        title: route.spec().title().to_owned(),
        resolution: session.window_resolution(),
        present_mode: PresentMode::AutoNoVsync,
        ..default()
    };

    let mut app = App::new();
    session.add_capture_plugins(
        &mut app,
        DefaultPlugins.set(session.capture_window_plugin(window)),
    );
    app.add_plugins(RoutePlugin::new(route, |route, app: &mut App| match route {
        UiRoute::Dashboard => app.add_plugins(DashboardPlugin),
        UiRoute::Review => app.add_plugins(ReviewPlugin),
    }));
    app.run();

    Ok(())
}

struct DashboardPlugin;
struct ReviewPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, _app: &mut App) {}
}

impl Plugin for ReviewPlugin {
    fn build(&self, _app: &mut App) {}
}
```

Use the session state helper when the selected capture route should also be the
Bevy `State` that drives `OnEnter`, `in_state`, or page-specific schedules:

```rust
use bevy::prelude::*;
#[derive(frame_capture_bevy::CaptureRouteBevy, Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    Dashboard,
    Review,
}

session.add_route_state(&mut app);
```

If the capture route is defined in a target-neutral core crate, keep it free of
Bevy derives and insert it as a wrapper resource instead:

```rust
use bevy::prelude::*;
use frame_capture_bevy::SelectedCaptureRoute;

session.add_selected_resources(&mut app);

fn setup(route: Res<SelectedCaptureRoute<UiRoute>>) {
    match route.route() {
        UiRoute::Dashboard => {}
        UiRoute::Review => {}
    }
}
```

For app-owned Bevy states or resources, map the selected capture input through
the session instead of making the capture enum carry Bevy-specific derives:

```rust
session.add_mapped_route_state(&mut app, UiPageState::from);
session.insert_mapped_scenario_resource(&mut app, UiRuntimeState::from_capture_scenario);
```

Use `window_resolution_with_live_size` when live mode has an app-specific size
override; capture mode still uses the requested capture size.

`read_bevy_session` rejects unsupported scenario variables by default.
Use `read_bevy_session_with_scenario` or `read_bevy_session_with_inputs` when
scenario ids should seed app-owned resources,
plugins, and Bevy states. Use `CaptureReady::pending()` when the screenshot
must wait for async data or asset loading.

## Route-Only Apps

`frame-capture-routes` is for render stacks where the host app owns the window,
renderer, frame loop, and screenshot mechanics. It gives the app the selected
route, output path, frame, and size without selecting a graphics API or
prescribing how the app renders and saves pixels.

```rust
use frame_capture_routes::{CaptureEnv, CaptureRoute as _};

#[derive(frame_capture_routes::CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
    #[capture_route(id = "review", title = "Review")]
    Review,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_session::<UiRoute>()?;
    let route = *session.route();

    if let Some(capture) = session.capture() {
        render_one_frame(route, capture.size());
        save_pixels(capture.path())?;
    } else {
        run_live_app(route);
    }

    Ok(())
}

fn render_one_frame(_route: UiRoute, _size: frame_capture_routes::PixelSize) {}

fn save_pixels(_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

fn run_live_app(_route: UiRoute) {}
```

When routes are distributed across modules, register installer functions instead
of keeping a single route enum:

```rust
use frame_capture_routes::{CaptureEnv, CaptureRoutesEnvExt as _};

#[frame_capture_routes::capture_route(id = "dashboard", title = "Dashboard", size = "1280x720")]
fn install_dashboard() {
    // Select app-owned state for this route.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_registered_session_for::<InstallDashboardRoute>()?;
    session.install();
    Ok(())
}
```

Route-only apps that support app-defined scenario ids should use
`read_session_with_scenario` or
`read_session_with_inputs` so those ids are parsed and carried with the same
selected route and capture config.

## Expose a Route Catalog Over MCP

Use `frame-capture-mcp` when tools need to inspect the app's route catalog. The
MCP server is read-only: it lists route metadata and returns route details, but
does not launch captures or save screenshots.

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
frame-capture = "0.1"
frame-capture-mcp = "0.1"
```

```rust
use frame_capture_mcp::serve_capture_routes_stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    serve_capture_routes_stdio::<UiRoute>().await
}
```

For registered-route catalogs, use
`serve_registered_capture_routes_stdio()`. Capture execution remains
environment-driven and owned by the host app.

## Running a Capture

The default capture environment uses `FRAME_CAPTURE_*` variables:

```sh
FRAME_CAPTURE_ROUTE=dashboard \
FRAME_CAPTURE_PATH=captures/dashboard/current.png \
cargo run -p my-app
```

`FRAME_CAPTURE_PATH` must point to a `.png` file.

Optional variables:

```sh
FRAME_CAPTURE_FRAME=12
FRAME_CAPTURE_WIDTH=1920
FRAME_CAPTURE_HEIGHT=1080
FRAME_CAPTURE_SCENARIO=empty-state
```

External tools that prepare a host-owned launch command can build the same
environment variables from validated capture types:

```rust
use frame_capture::CaptureLaunchEnv;

let launch_env = CaptureLaunchEnv::builder()
    .route_id("dashboard")?
    .output_path("captures/dashboard/current.png")?
    .frame(12)?
    .size(1920, 1080)?
    .build();

let env = launch_env.env_map_lossy();
assert_eq!(env["FRAME_CAPTURE_ROUTE"], "dashboard");
assert_eq!(env["FRAME_CAPTURE_PATH"], "captures/dashboard/current.png");
assert_eq!(env["FRAME_CAPTURE_WIDTH"], "1920");

# Ok::<(), Box<dyn std::error::Error>>(())
```

`CaptureLaunchEnv` only returns launch environment data. The host application,
script, or MCP tool still owns process spawning and screenshot storage.

Use `CaptureEnv::with_prefix("MY_APP")` for app-local variable names such as
`MY_APP_CAPTURE_ROUTE` and `MY_APP_CAPTURE_PATH`, or
`CaptureEnv::try_with_prefix` when you need to handle invalid prefixes without
panicking.

## Examples

Run the Bevy example live:

```sh
cargo run -p frame-capture-example-bevy
```

Capture the checked-in example routes and save PNGs under each example's
`captures/` directory:

```sh
just example-captures
```

Capture the Bevy dashboard route directly:

```sh
FRAME_CAPTURE_ROUTE=bevy/dashboard \
FRAME_CAPTURE_PATH=examples/bevy/captures/bevy/dashboard/current.png \
cargo run -p frame-capture-example-bevy
```

Capture the GPUI route-only dashboard directly:

```sh
FRAME_CAPTURE_ROUTE=gpui/dashboard \
FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/dashboard/current.png \
cargo run --manifest-path examples/gpui/Cargo.toml
```

Run the GPUI route-only example live:

```sh
cargo run --manifest-path examples/gpui/Cargo.toml
```

The GPUI example has its own manifest outside the default workspace, and
`just example-gpui-captures` invokes that manifest directly. Its capture path
uses GPUI's `test-support` `HeadlessAppContext` and
`gpui_platform::current_headless_renderer()`, leaving GPUI to select the native
platform renderer. That is Metal on macOS; the fork's
`linux-headless-renderer` branch supplies the Linux headless implementation used
by this repository.
