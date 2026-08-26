---
name: use-frame-capture
description: Integrate, review, or refactor frame-capture in Rust applications that own their renderer or screenshot pipeline. Use when Codex needs to work with frame-capture, frame-capture-routes, egui, GPUI, raw wgpu, custom renderer facades, typed route or scenario enums, registered routes, FRAME_CAPTURE_* session parsing, frame gates, PNG output paths, CaptureLaunchEnv, or read-only frame-capture-mcp route catalogs. Use the Bevy-specific skill when frame-capture-bevy owns the offscreen runtime.
---

# Use frame-capture

## Choose the facade

- Use `frame-capture-routes` for an application that already renders and saves
  screenshots.
- Use `frame-capture` for a custom facade or target-neutral protocol layer.
- Keep renderer-specific window, texture, readback, and image-saving code in the
  host application or facade.

## Follow the integration workflow

1. Inspect the host's existing startup, route, render-loop, and screenshot
   paths.
2. Define stable route IDs, user-facing titles, and positive default sizes.
3. Read one session during startup with
   `CaptureEnv::frame_capture().read_session::<Route>()?`.
4. Use a typed input session when `FRAME_CAPTURE_SCENARIO` should seed app
   state.
5. Start the normal application at the selected route when `session.capture()`
   is `None`.
6. In capture mode, apply the scenario, render the route at `capture.size()`,
   honor the one-based `capture.frame()` gate, create the output directory, and
   save a PNG to `capture.path()`.
7. Call `validate_registered_routes()` when route installers use inventory,
   then read the selected session through its generated route key.
8. Use `CaptureLaunchEnv::builder()` when another tool needs validated launch
   variables. Keep process construction and spawning in that tool.

## Preserve these contracts

- `FRAME_CAPTURE_PATH` alone switches the session into capture mode.
- Width and height overrides are an all-or-nothing pair of positive values.
- Route IDs may be relative paths; scenario IDs are single state identifiers.
- The selected route applies in both live and capture modes.
- MCP catalog servers expose metadata only. They do not launch applications or
  write screenshots.
- Treat MCP catalog servers as long-lived. EOF, cancellation, or another
  application-owned signal ends the host explicitly.

## Load detailed patterns

Read [generic patterns](references/generic-patterns.md) for concrete enum,
scenario, registered-route, launch-environment, and MCP snippets. Prefer the
current public API, user guide, and repository examples over memory when a
signature matters.
