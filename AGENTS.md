# Working in frame-capture

Use `just --list` for the local command index. Choose the owning surface below
before changing a capture contract.

## Where to work

| Surface | Ownership |
| --- | --- |
| `crates/frame-capture` | Public, target-neutral routes, IDs, sessions, environment parsing, sizes, frame gates, and PNG paths |
| `crates/frame-capture-bevy` | Public Bevy facade, offscreen runtime, readiness, window setup, and state/resource mapping |
| `crates/frame-capture-routes` | Public host-rendered facade and generic registered-route support |
| `crates/frame-capture-routes-bevy` | Public `fn(&mut App)` registration, also reexported by the Bevy facade's `registry` feature |
| `crates/frame-capture-macros` | Derive and attribute parsing, generated route keys, facade paths, and diagnostics |
| `crates/frame-capture-toml` | Parser for shared capture-size defaults consumed by the macros |
| `crates/frame-capture-mcp` | Read-only route catalog tools; capture execution belongs to the client and host application |
| `examples/bevy`, `examples/gpui` | Application integrations and sources for their checked-in PNG captures |
| `book/src`, `skills/use-frame-capture`, `skills/use-frame-capture-bevy` | User-facing integration guidance; begin with `book/src/SUMMARY.md` |
| `web/src`, `xtask` | Public catalog and its book, `llms.txt`, and Pages build pipeline |

`examples/gpui` has its own manifest and is excluded from the root Cargo
workspace. Validate it with `--manifest-path examples/gpui/Cargo.toml`.

## What changes together

- When route/scenario syntax, environment inputs, sizing, frame gates, output
  paths, or facade APIs change, update the affected rustdocs, README examples,
  book pages, public skills, and application examples. Keep implementation
  rationale beside code and tests.
- Keep macro expansion and facade reexports aligned. Diagnostic expectations
  live in `crates/frame-capture/tests/ui`; update `.stderr` files only for
  intentional diagnostic changes. Registered-route tests live in the owning
  facade's `tests` directory and cover keys, installer signatures, and duplicate
  IDs.
- Keep `frame-capture.toml` examples aligned with macro size precedence and
  parent-directory discovery.
- For Bevy runtime changes, preserve live plugin behavior and capture-mode
  handling of route, scenario, size, frame, path, readiness, and warmup. The
  capture target is assigned in `PostStartup` to cameras created by `Startup`.
- When MCP catalog behavior changes, update enum and registered-route tools,
  their tests, and `book/src/mcp.md` together.
- When an example's expected visuals change, regenerate its PNG captures using
  the matching `justfile` recipe. Keep route IDs, titles, sizes, and documented
  commands aligned; regenerate captures only for intended visual changes.
- Edit `book/src` and `web/src` as sources. Generate `web/public/book`,
  `web/public/llms*`, and `web/dist` through `cargo xtask`.

## Validate the edited surface

- For Rust changes, use the narrowest package-specific `cargo check` or
  `cargo test` that covers the change. `just check`, `just clippy`, and
  `just test` cover the workspace with all features and targets.
- For Markdown, run `rumdl check` on the changed files. `just fmt` also formats
  Rust and TOML, so use it when those surfaces need formatting.
- For rustdocs, use `just test-docs` (builds documentation and opens it).
  CI uses `cargo doc --workspace --all-features --no-deps --locked`.
- For book changes, run `cargo xtask build book` and
  `cargo xtask build llms-txt`. Use `MDBOOK_BUILD__CREATE_MISSING=false` during
  validation so a missing summary target is reported without creating a page.
- For catalog changes, run `cargo xtask build web`; `just web-build` runs the
  complete publication build.

Report which checks ran successfully, which failed, and which were skipped.
