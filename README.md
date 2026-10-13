# frame-capture

[![CI][ci-badge]][ci]
[![Codecov][codecov-badge]][codecov]
[![Book][book-badge]][book]

`frame-capture` gives Rust UI application authors typed route selection and
deterministic PNG capture, with one catalog shared by interactive launches,
capture runs, and tool-facing discovery.

Requires Rust 1.99 or newer for all workspace packages. The Bevy integration
targets Bevy 0.19; host-rendered integrations choose their own rendering backend.

Use [`frame-capture-bevy`][bevy-readme] for the supplied screenshot runtime or
[`frame-capture-routes`][routes-readme] for an existing screenshot pipeline.
Custom facades and launch tools can use [`frame-capture`][core-readme] directly.

## Crates

| Crate | Purpose |
| --- | --- |
| [`frame-capture`][core-readme] | Target-neutral routes, sessions, launch inputs, sizes, frame gates, and PNG paths for custom facades and tools. |
| [`frame-capture-bevy`][bevy-readme] | Bevy plugins and an offscreen PNG capture runtime for applications. |
| [`frame-capture-macros`][macros-readme] | Route, scenario, and registered-route proc macros, normally reached through facade reexports. |
| [`frame-capture-mcp`][mcp-readme] | Read-only MCP route catalog servers for launch tools. |
| [`frame-capture-routes`][routes-readme] | Route sessions and registration for host-owned renderers. |
| [`frame-capture-routes-bevy`][routes-bevy-readme] | Bevy `fn(&mut App)` route registration for host-owned capture runtimes. |
| [`frame-capture-toml`][toml-readme] | Shared default-size parsing for macros and custom tools. |

## Example

From the repository root, capture the [maintained Bevy example][bevy-example]
dashboard with a working graphics backend:

```sh
FRAME_CAPTURE_ROUTE=bevy/dashboard \
FRAME_CAPTURE_PATH=target/dashboard.png \
cargo run -p frame-capture-example-bevy
```

`FRAME_CAPTURE_PATH` enables capture mode. Without it, the selected route runs
normally.

A successful run prints `Screenshot saved to target/dashboard.png`, writes the
PNG, and exits.

[ci-badge]: https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master
[ci]: https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml
[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[book-badge]: https://img.shields.io/badge/Book-mdBook-blue
[book]: https://stayhydated.github.io/frame-capture/book/
[bevy-example]: examples/bevy/src/app.rs
[core-readme]: crates/frame-capture/README.md
[bevy-readme]: crates/frame-capture-bevy/README.md
[macros-readme]: crates/frame-capture-macros/README.md
[mcp-readme]: crates/frame-capture-mcp/README.md
[routes-readme]: crates/frame-capture-routes/README.md
[routes-bevy-readme]: crates/frame-capture-routes-bevy/README.md
[toml-readme]: crates/frame-capture-toml/README.md
