# frame-capture-mcp

MCP server helpers for exposing `frame-capture` route metadata.

This crate serves route metadata over stdio for tools that need to inspect an
application's capture catalog. It is read-only: it lists routes and returns
route details, but it does not launch captures or save screenshots.

## Use With a Route Enum

```toml
[dependencies]
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
frame-capture = "0.1"
frame-capture-mcp = "0.1"
```

```rust
use frame_capture_mcp::serve_capture_routes_stdio;

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
    #[capture_route(id = "review", title = "Review")]
    Review,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    serve_capture_routes_stdio::<UiRoute>().await
}
```

## Use With Registered Routes

Apps that register routes with `frame-capture-routes` can serve the registry
directly:

```rust
use frame_capture_mcp::serve_registered_capture_routes_stdio;

#[frame_capture_routes::capture_route(id = "dashboard", title = "Dashboard", size = "1280x720")]
fn install_dashboard() {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    serve_registered_capture_routes_stdio().await
}
```

## Tools

The enum server exposes:

- `list_capture_routes`
- `get_capture_route`

The registered-route server exposes:

- `list_registered_capture_routes`
- `get_registered_capture_route`

Each route includes `id`, `title`, and `default_size` with `width`, `height`,
and a string label.

## Implementation Boundary

This crate is read-only. It may expose route catalog metadata through MCP tools,
but it must not launch captures, save screenshots, mutate files, or inspect
application state beyond the supplied route catalog.
