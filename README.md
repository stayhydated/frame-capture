# frame-capture

[![Build Status](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/github/stayhydated/frame-capture/graph/badge.svg?token=34CV04UOU1)](https://codecov.io/github/stayhydated/frame-capture)
[![Docs](https://docs.rs/frame-capture/badge.svg)](https://docs.rs/frame-capture/)
[![Crates.io](https://img.shields.io/crates/v/frame-capture.svg)](https://crates.io/crates/frame-capture)

Typed route selection and deterministic PNG capture for Rust UI applications.
`frame-capture` uses the same route catalog for normal launches, capture runs,
and tool-facing discovery.

## Choose an integration

| Application | Crate |
| --- | --- |
| Bevy with the provided offscreen capture runtime | [`frame-capture-bevy`](crates/frame-capture-bevy) |
| egui, GPUI, raw wgpu, or another host-owned renderer | [`frame-capture-routes`](crates/frame-capture-routes) |
| A custom renderer facade or shared protocol layer | [`frame-capture`](crates/frame-capture) |
| Bevy route registration with a host-owned capture runtime | [`frame-capture-routes-bevy`](crates/frame-capture-routes-bevy) |
| Read-only route discovery over MCP | [`frame-capture-mcp`](crates/frame-capture-mcp) |

## Capture a route

Set a route and PNG output path when launching the application:

```sh
FRAME_CAPTURE_ROUTE=dashboard \
FRAME_CAPTURE_PATH=captures/dashboard.png \
cargo run -p my-app
```

`FRAME_CAPTURE_PATH` enables capture mode. Without it, the selected route runs
normally. The Bevy facade saves the PNG itself; route-only integrations pass the
validated route, size, frame, and path to their existing screenshot pipeline.

## Documentation

- [User guide](https://stayhydated.github.io/frame-capture/book/)
- [API documentation](https://docs.rs/frame-capture/)
- [Project site](https://stayhydated.github.io/frame-capture/)
