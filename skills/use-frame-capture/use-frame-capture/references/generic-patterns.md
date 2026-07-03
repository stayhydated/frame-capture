# Generic frame-capture Patterns

## Route-Only Facade

Use `frame-capture-routes` when the host app owns rendering and saving pixels.

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
fn save_pixels(_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
fn run_live_app(_route: UiRoute) {}
```

`FRAME_CAPTURE_PATH` enables capture mode and must point to a `.png` file. `FRAME_CAPTURE_WIDTH` and `FRAME_CAPTURE_HEIGHT` override route size only when both are present. `FRAME_CAPTURE_FRAME` defaults to `12`.

## Target-Neutral Crate

Use `frame-capture` directly for custom facades and shared protocol layers.

```rust
use frame_capture::{CaptureEnv, CaptureRoute as _};

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
    #[capture_route(id = "review", title = "Review")]
    Review,
}

let session = CaptureEnv::frame_capture().read_session::<UiRoute>()?;
let route = *session.route();
```

## Registered Routes

Use registered routes when route installers live beside feature modules.

```rust
use frame_capture_routes::{CaptureEnv, CaptureRoutesEnvExt as _};

#[frame_capture_routes::capture_route(id = "dashboard", title = "Dashboard", size = "1280x720")]
fn install_dashboard() {
    // Select app-owned state, router path, or content for this route.
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    frame_capture_routes::validate_registered_routes()?;
    let session = CaptureEnv::frame_capture().read_registered_session_for::<InstallDashboardRoute>()?;
    session.install();

    if let Some(capture) = session.capture() {
        render_registered(session.spec(), capture.size());
        save(capture.path())?;
    }

    Ok(())
}

fn render_registered(_route: frame_capture_routes::RouteSpec, _size: frame_capture_routes::PixelSize) {}
fn save(_path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> { Ok(()) }
```

The macro derives the key name from the function, such as `InstallDashboardRoute`. Use `key = MyRouteKey` only when the generated name would be unclear or unstable.

## Scenarios

Scenarios are typed state ids, not route paths. The host app decides how they affect rendering.

```rust
#[derive(frame_capture_routes::CaptureScenarioRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_scenario]
enum UiScenario {
    #[capture_scenario(id = "empty", title = "Empty State")]
    Empty,
    Loaded,
}

let env = frame_capture_routes::CaptureEnv::frame_capture();
let session = env.read_session_with_scenario::<UiRoute, UiScenario>()?;
let scenario = session.scenario();
```

## Output Paths

Use `CaptureOutputPath` when deriving deterministic route-local PNG paths.

```rust
use frame_capture_routes::{CaptureOutputPath, CaptureOutputStem};

let path = CaptureOutputPath::for_stem(
    "captures",
    UiRoute::Dashboard,
    &CaptureOutputStem::current(),
)?;
```

## MCP Route Catalogs

Use `frame-capture-mcp` when tools need to inspect the route catalog. The MCP
server is read-only and must not launch captures, save screenshots, mutate
files, or inspect application state beyond route metadata.

```rust
use frame_capture_mcp::serve_capture_routes_stdio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    serve_capture_routes_stdio::<UiRoute>().await
}
```

Use `serve_registered_capture_routes_stdio()` for registered-route catalogs.
Capture execution remains environment-driven and host-owned.

## Implementation Notes

- Keep route ids stable; relative path-like ids are allowed.
- Use package-local `frame-capture.toml` `default_size` with explicit
  `width` and `height` when a route catalog should share one macro fallback
  size.
- Keep renderer-specific screenshot code in the host app or facade; `frame-capture` and `frame-capture-routes` provide route metadata and capture session data.
- Keep route-only examples concrete by saving a deterministic PNG when `FRAME_CAPTURE_PATH` is set. Prefer the UI framework's native screenshot or test renderer when it exists; use a small host-owned preview renderer only as a portability fallback.
