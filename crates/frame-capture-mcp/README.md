# frame-capture-mcp

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture-mcp.svg)](https://crates.io/crates/frame-capture-mcp)

Read-only MCP servers for discovering capture route IDs, titles, and default
pixel sizes before a client launches an application.

Use `serve_capture_routes_stdio::<Route>()` for an enum catalog or
`serve_registered_capture_routes_stdio()` for registered routes. The servers
list routes and return route details; the client remains responsible for
building launch environment data, starting the application, and handling the
resulting PNG.

The servers remain available across requests until EOF, cancellation, or
another explicit host shutdown signal. A host that wants one request terminates
explicitly after receiving its result.
