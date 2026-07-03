---
name: use-frame-capture
description: Use when Codex needs to add, review, or update generic frame-capture integrations in Rust applications that own their renderer or screenshot pipeline, including direct frame-capture usage, frame-capture-routes, egui, GPUI, raw wgpu, custom renderer facades, route enums, registered routes, capture environment parsing, scenarios, frame gates, output paths, and route catalog metadata.
---

# Use frame-capture

## Scope Boundary

Treat this skill as a hosted public-usage guide for `frame-capture` consumers.
Use it for route-only or target-neutral capture work in Rust applications that
own their renderer or screenshot pipeline: direct `frame-capture` usage,
`frame-capture-routes`, egui, GPUI, raw wgpu, custom renderer facades, route
enums, registered routes, capture environment parsing, scenarios, frame
gates, output paths, and route catalog metadata.

Do not use this skill as a contributor guide for `frame-capture` repository
internals. For workspace maintenance, release, CI, architecture, or
renderer-specific screenshot runtime work, read the repository source,
`AGENTS.md`, and the relevant crate documentation directly. Use
`use-frame-capture-bevy` instead when the application wants the Bevy offscreen
screenshot runtime.

## Core Workflow

Start from the application surface that owns rendering and screenshots.
`frame-capture` provides typed route metadata and capture request data; the host
app or facade owns renderer-specific window, texture, frame-loop, and screenshot
mechanics.

1. Choose the surface: use `frame-capture-routes` for egui, GPUI, raw wgpu, and
   apps that already own screenshot capture; use `frame-capture` directly for a
   custom renderer integration or shared protocol layer.
2. Follow the host app's existing route structure, startup path, and route id
   conventions.
3. Define a typed route catalog with explicit route ids, titles, and sizes.
   Prefer enum-level `size = "WIDTHxHEIGHT"` unless a route needs a specific
   override. Use enum-level `id_prefix = "desktop"` or similar when one route
   enum is reused by a specific app surface.
4. Read a route-only session with `CaptureEnv::frame_capture().read_session::<Route>()?`,
   or use `read_session_with_scenario` or `read_session_with_inputs` when the
   app supports typed scenario ids.
   The selected route is always available; `session.capture()` is `Some` only
   when the capture path env var is present.
5. In live mode, start the normal app at the selected route. In capture mode,
   render the route at `capture.size()`, wait until `capture.frame()` if the
   host has a frame loop, and save pixels to `capture.path()`.
6. Map `session.scenario()` to app-owned state presets when using typed input
   sessions.
7. For registered routes, call `validate_registered_routes()` during startup
   and use `read_registered_session_for::<GeneratedRouteKey>()?` to resolve the
   default route.
8. For tool-facing route discovery, expose a read-only `frame-capture-mcp`
   server. Keep capture execution environment-driven and owned by the host app.

## Reference Selection

Load only the reference needed for the task:

- `references/generic-patterns.md`: route enum, capture loop, registered route, scenario, and output path patterns for concrete generic integrations.

Prefer current public docs or source examples over memory when details matter.

## Implementation Rules

Use `frame-capture-routes` registered routes when route installers are
distributed across modules.

Route ids may be relative route paths. Scenario ids are state ids, not
paths.

Keep renderer-specific screenshot mechanics in the host app or facade. Generic
integrations should resolve route, size, frame, output path, and scenario
through the capture protocol, then pass those values into the app's existing
rendering pipeline.

For capture runs:

- Use the selected route in both live and capture mode.
- Use `capture.size()` for capture-mode pixel dimensions.
- Use `capture.frame()` only as a frame gate; readiness and state preparation remain app-owned.
- Save PNG output to `capture.path()`.
- Apply scenarios before rendering.

For MCP route catalog servers:

- Keep MCP helpers read-only.
- Do not launch captures, save screenshots, mutate files, or inspect app state beyond the supplied route catalog.
- Keep tool names and serialized route metadata aligned with `frame-capture-mcp` tests and docs.
