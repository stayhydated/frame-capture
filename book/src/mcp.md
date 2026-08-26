# Expose route catalogs over MCP

`frame-capture-mcp` lets MCP clients discover valid route IDs, titles, and
default sizes before constructing a capture command. The server is read-only
and does not launch the application or save screenshots.

## Serve an enum catalog

Add the catalog and async runtime dependencies:

```toml
[dependencies]
frame-capture = "0.1"
frame-capture-mcp = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Then serve a route enum over stdio:

```rust,ignore
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

The enum server exposes `list_capture_routes` and `get_capture_route`. Each
route result includes `id`, `title`, and `default_size` with numeric `width` and
`height` plus a size label.

## Serve registered routes

Use `serve_registered_capture_routes_stdio()` for inventory-backed routes. That
server exposes `list_registered_capture_routes` and
`get_registered_capture_route`, and reports duplicate registration IDs as
errors.

Both MCP servers remain available across requests until EOF, cancellation, or
another explicit host shutdown signal. Serving one request and then
terminating is an application policy rather than tool-dispatch behavior.

To verify either server, configure its binary as a stdio MCP server, list the
available tools, and call the list tool. A successful response contains every
route in the typed catalog. An unknown ID passed to a get tool returns the
expected route choices.

After discovery, the client can build checked environment data with
`CaptureLaunchEnv`, launch the host application through its own process
workflow, and inspect the resulting PNG. Keep process launch and file mutation
outside the MCP catalog server.
