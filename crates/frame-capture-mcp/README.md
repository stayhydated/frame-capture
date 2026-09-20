# frame-capture-mcp

[![Codecov: frame-capture-mcp][codecov-badge]][codecov]
[![crates.io: frame-capture-mcp][crate-badge]][crate]

Read-only MCP servers for discovering capture route IDs, titles, and default
pixel sizes before a client launches an application.

## Overview

Use `serve_capture_routes_stdio::<Route>()` for an enum catalog or
`serve_registered_capture_routes_stdio()` for registered routes. The servers
list routes and return route details; the client remains responsible for
building launch environment data, starting the application, and handling the
resulting PNG.

The servers remain available across requests until EOF, cancellation, or
another explicit host shutdown signal. A host that wants one request terminates
explicitly after receiving its result.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-mcp
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-mcp.svg?label=frame-capture-mcp
[crate]: https://crates.io/crates/frame-capture-mcp
