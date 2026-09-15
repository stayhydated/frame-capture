# frame-capture

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture.svg)](https://crates.io/crates/frame-capture)

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

From the repository root, capture the Bevy example dashboard:

```sh
FRAME_CAPTURE_ROUTE=bevy/dashboard \
FRAME_CAPTURE_PATH=target/dashboard.png \
cargo run -p frame-capture-example-bevy
```

`FRAME_CAPTURE_PATH` enables capture mode. Without it, the selected route runs
normally. The Bevy facade saves the PNG itself; route-only integrations pass the
validated route, size, frame, and path to their existing screenshot pipeline.

A successful run prints `Screenshot saved to target/dashboard.png`, writes the
PNG, and exits.
