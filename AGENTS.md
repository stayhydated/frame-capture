# AGENTS.md

This is the working guide for contributors and coding agents in the
`frame-capture` workspace. Use it to decide which crate owns a change, what
docs, examples, captures, tests, and public skills must stay synchronized, and
which narrow validation command fits the edited surface.

Start here:

- Bevy screenshot runtime: `crates/frame-capture-bevy`.
- egui, GPUI, raw wgpu, or host-owned screenshot routes:
  `crates/frame-capture-routes`.
- Target-neutral protocol and custom renderer integrations:
  `crates/frame-capture`.
- Route catalog tooling: `crates/frame-capture-mcp`.
- User guide and catalog: `book/src/SUMMARY.md` and `web/src/lib.rs`.
- Local command index: `justfile`; run `just --list`.

## Quick Decision Flow

1. Find the surface in the workspace map before editing.
2. Route public API contracts to crate README files, `book/src`, rustdocs,
   examples, and public skills. Keep implementation rationale near code, Rust
   tests, trybuild fixtures, or short comments.
3. When derive syntax, environment variables, route metadata, capture sizing,
   frame gates, output paths, scenarios, registered routes, Bevy runtime
   behavior, or MCP tools change, update the owning crate, docs, examples,
   tests, captures, affected public skill guidance, and this guide when it
   names the changed workflow.
4. Validate with the smallest evidenced command that covers the edited crate,
   example, capture, docs surface, MCP tool, or workflow.

## Audience Labels

- **User-facing**: normal entry points for application developers, public
  examples, and public skills.
- **Public integration**: public crates for facades, extensions, tooling, or
  deeper customization.
- **Validation**: tests, trybuild fixtures, example routes, and checked-in
  captures that encode behavior.
- **Internal**: workspace maintenance surfaces and generated captures that
  support examples.

## Workspace Map

### User-Facing Entry Points

- `crates/frame-capture-bevy`
  Audience: **User-facing**
  Role: Bevy facade for live mode and deterministic capture mode. Owns the
  offscreen screenshot runtime, `CaptureReady`, capture-window plugin setup,
  route plugins, and the optional registered-route feature.

- `crates/frame-capture-routes`
  Audience: **User-facing**
  Role: route-only facade for egui, GPUI, raw wgpu, and applications that own
  rendering and screenshot output. Re-exports shared capture primitives and
  supports registered function routes.

- `crates/frame-capture`
  Audience: **User-facing**
  Role: target-neutral protocol and custom renderer entry point. Defines route
  specs, capture sessions, environment parsing, typed ids, pixel sizes, output
  paths, scenarios, frame gates, and route macro re-exports.

### Public Integration

- `crates/frame-capture-macros`
  Audience: **Public integration**
  Role: proc macros for route, scenario, and registered-route declarations.
  Most applications should use re-exports from a facade crate.

- `crates/frame-capture-routes-bevy`
  Audience: **Public integration**
  Role: Bevy `App` registered-route facade without screenshot runtime. Used by
  host-owned Bevy capture pipelines and by `frame-capture-bevy` with its
  `registry` feature.

- `crates/frame-capture-mcp`
  Audience: **Public integration**
  Role: read-only MCP helpers for exposing route catalog metadata over stdio.
  It lists routes and returns details; it must not launch captures or save
  screenshots.

- `crates/frame-capture-toml`
  Audience: **Public integration**
  Role: parser for package-local capture defaults read by route macros. Most
  users configure `frame-capture.toml` instead of depending on this crate.

### Examples, Skills, and Tooling

- `examples/bevy`
  Audience: **User-facing**
  Role: Bevy screenshot runtime example, source of checked-in captures under
  `examples/bevy/captures/`.

- `examples/gpui`
  Audience: **User-facing**
  Role: route-only GPUI enum example. It is excluded from the root workspace
  and has its own manifest.

- `skills/use-frame-capture` and `skills/use-frame-capture-bevy`
  Audience: **User-facing**
  Role: public application-developer guidance for target-neutral/route-only and
  Bevy capture workflows. Keep maintainer-only details in this guide, rustdocs,
  tests, or fixtures.

- `frame-capture.toml`
  Audience: **Internal**
  Role: workspace-level default capture size used by route macros when no enum
  or route-specific size is supplied.

### Documentation and Publishing

- `book/src`
  Audience: **User-facing**
  Role: mdBook source for integrations, routes, sessions, Bevy, route-only,
  MCP, and environment configuration.

- `web`
  Audience: **User-facing**
  Role: Dioxus catalog for the book, API docs, and source.

- `xtask`
  Audience: **Internal**
  Role: reproducible book, `llms.txt`, and Pages-site builds.

## Synchronization Rules

- When public workflows, derive or attribute syntax, environment variables,
  route metadata, capture sizing, frame gates, output paths, scenario behavior,
  registered-route APIs, Bevy runtime behavior, or MCP schemas change, update
  the root README, matching `book/src` chapters, affected crate README files,
  examples, rustdocs, tests, captures, public skills, and this guide when they
  name the changed behavior.
- Keep `frame-capture.toml` guidance aligned with route macro default-size
  behavior.
- Keep `justfile` capture recipes aligned with direct environment-variable
  capture commands in the root README and examples.
- Update trybuild `.stderr` files only when macro diagnostics intentionally
  change.
- Regenerate checked-in PNG captures only when expected visual output changes.
- Build `web/public/book`, `web/public/llms*`, and `web/dist` through
  `cargo xtask`; edit `book/src` and `web/src` as their sources.

## Validation and Editing Rules

- Use `just --list` to inspect available repository recipes.
- Use `just fmt`, `just check`, `just clippy`, `just test`, or `just ci` when a
  change spans the scope of those recipes.
- Use `just test-docs` for rustdoc changes and `cargo package --workspace --list`
  when matching the CI package job.
- Use `cargo xtask build book` and `cargo xtask build llms-txt` for book
  changes and `cargo xtask build web` for the catalog. `just web-build` runs
  the complete publication pipeline.
- For focused Rust work, prefer the smallest package-specific `cargo check` or
  `cargo test` command that covers the affected crate.
- For route macro or typed-id changes, keep `CaptureRoute`,
  `CaptureScenario`, `routes`, and `capture_routes` behavior aligned with
  trybuild tests and facade re-exports.
- For registered routes, preserve duplicate route-id validation and keep route
  key names, explicit `key = ...` behavior, and installer signatures documented
  and tested.
- For Bevy capture runtime changes, keep live mode as normal Bevy plugin
  behavior and capture mode as offscreen schedule-runner output that respects
  route, size, frame, output path, scenario, and `CaptureReady`.
- For MCP route catalog changes, keep helpers read-only and keep enum route and
  registered-route tools aligned.
- For public examples, keep route ids, titles, sizes, README snippets, and
  checked-in captures aligned.
- If validation cannot be run, state exactly what was skipped and what remains
  unvalidated.
