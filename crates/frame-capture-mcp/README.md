# frame-capture-mcp

[![Codecov: frame-capture-mcp][codecov-badge]][codecov]
[![crates.io: frame-capture-mcp][crate-badge]][crate]

Read-only MCP servers for discovering capture route IDs, titles, and default
pixel sizes before a client launches an application.

Depend on `frame-capture-mcp` directly in a tool-facing host, alongside the
facade that owns the application's route catalog.

Requires Rust 1.99 or newer. Stdio server helpers are asynchronous and need a
host runtime compatible with `rmcp`; catalog queries can run synchronously.

## Overview

Use `serve_capture_routes_stdio::<Route>()` for an enum catalog or
`serve_registered_capture_routes_stdio()` for registered routes. The servers
list routes and return route details; the client remains responsible for
building launch environment data, starting the application, and handling the
resulting PNG.

## Example

An enum catalog can be queried without opening an MCP transport. The route
declaration uses the default `macros` feature of `frame-capture`:

```rust
#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "960x540")]
enum Route {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
}

fn main() -> Result<(), frame_capture_mcp::CaptureMcpError> {
    let routes = frame_capture_mcp::capture_routes::<Route>()?;
    assert_eq!(routes[0].id(), "dashboard");
    assert_eq!(routes[0].default_size().width(), 960);
    Ok(())
}
```

This returns validated metadata without launching an app or writing a PNG.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-mcp
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-mcp.svg?label=frame-capture-mcp
[crate]: https://crates.io/crates/frame-capture-mcp
