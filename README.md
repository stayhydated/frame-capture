# frame-capture

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]
[![crates.io: frame-capture][frame-capture-badge]][frame-capture-crate]
[![crates.io: frame-capture-bevy][frame-capture-bevy-badge]][frame-capture-bevy-crate]
[![crates.io: frame-capture-macros][frame-capture-macros-badge]][frame-capture-macros-crate]
[![crates.io: frame-capture-mcp][frame-capture-mcp-badge]][frame-capture-mcp-crate]
[![crates.io: frame-capture-routes][frame-capture-routes-badge]][frame-capture-routes-crate]
[![crates.io: frame-capture-routes-bevy][frame-capture-routes-bevy-badge]][frame-capture-routes-bevy-crate]
[![crates.io: frame-capture-toml][frame-capture-toml-badge]][frame-capture-toml-crate]

`frame-capture` gives Rust UI application authors typed route selection and
deterministic PNG capture, with one catalog shared by interactive launches,
capture runs, and tool-facing discovery.

## Crates

| Crate | Purpose | Source |
| --- | --- | --- |
| `frame-capture` | Target-neutral routes, sessions, environment parsing, sizes, frame gates, and PNG paths | [README](crates/frame-capture/README.md) |
| `frame-capture-bevy` | Bevy plugins and an offscreen PNG capture runtime | [README](crates/frame-capture-bevy/README.md) |
| `frame-capture-macros` | Route, scenario, and registered-route proc macros used by the facades | [README](crates/frame-capture-macros/README.md) |
| `frame-capture-mcp` | Read-only MCP route catalog servers | [README](crates/frame-capture-mcp/README.md) |
| `frame-capture-routes` | Route sessions and registration for host-owned renderers | [README](crates/frame-capture-routes/README.md) |
| `frame-capture-routes-bevy` | Bevy `fn(&mut App)` route registration for host-owned capture runtimes | [README](crates/frame-capture-routes-bevy/README.md) |
| `frame-capture-toml` | Shared default-size configuration parsing for macros and tools | [README](crates/frame-capture-toml/README.md) |

## Example

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

[ci-badge]: https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master
[ci]: https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml
[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[book-badge]: https://img.shields.io/badge/Book-mdBook-blue
[book]: https://stayhydated.github.io/frame-capture/book/
[frame-capture-badge]: https://img.shields.io/crates/v/frame-capture.svg?label=frame-capture
[frame-capture-crate]: https://crates.io/crates/frame-capture
[frame-capture-bevy-badge]: https://img.shields.io/crates/v/frame-capture-bevy.svg?label=frame-capture-bevy
[frame-capture-bevy-crate]: https://crates.io/crates/frame-capture-bevy
[frame-capture-macros-badge]: https://img.shields.io/crates/v/frame-capture-macros.svg?label=frame-capture-macros
[frame-capture-macros-crate]: https://crates.io/crates/frame-capture-macros
[frame-capture-mcp-badge]: https://img.shields.io/crates/v/frame-capture-mcp.svg?label=frame-capture-mcp
[frame-capture-mcp-crate]: https://crates.io/crates/frame-capture-mcp
[frame-capture-routes-badge]: https://img.shields.io/crates/v/frame-capture-routes.svg?label=frame-capture-routes
[frame-capture-routes-crate]: https://crates.io/crates/frame-capture-routes
[frame-capture-routes-bevy-badge]: https://img.shields.io/crates/v/frame-capture-routes-bevy.svg?label=frame-capture-routes-bevy
[frame-capture-routes-bevy-crate]: https://crates.io/crates/frame-capture-routes-bevy
[frame-capture-toml-badge]: https://img.shields.io/crates/v/frame-capture-toml.svg?label=frame-capture-toml
[frame-capture-toml-crate]: https://crates.io/crates/frame-capture-toml
