# frame-capture-mcp

Read-only MCP servers for discovering capture route IDs, titles, and default
pixel sizes before a client launches an application.

```toml
[dependencies]
frame-capture = "0.1"
frame-capture-mcp = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

Use `serve_capture_routes_stdio::<Route>()` for an enum catalog or
`serve_registered_capture_routes_stdio()` for registered routes. The servers
list routes and return route details; the client remains responsible for
building launch environment data, starting the application, and handling the
resulting PNG.

The servers remain available across requests until EOF, cancellation, or
another explicit host shutdown signal. A host that wants one request terminates
explicitly after receiving its result.

- [MCP guide](https://stayhydated.github.io/frame-capture/book/mcp.html)
- [API documentation](https://docs.rs/frame-capture-mcp/)
