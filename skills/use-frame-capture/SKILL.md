---
name: use-frame-capture
description: Integrate, review, or refactor frame-capture routes, sessions, scenarios, launch inputs, and MCP catalogs in Rust applications with a host-owned renderer. Use for frame-capture and frame-capture-routes integrations; use the Bevy skill when frame-capture-bevy owns screenshot output.
---

# Use frame-capture

## Choose the facade

- Use `frame-capture-routes` for an application that already renders and saves
  screenshots.
- Use `frame-capture` for a custom facade or target-neutral protocol layer.
- Keep renderer-specific window, texture, readback, and image-saving code in the
  host application or facade.

## Integrate with the host

1. Inspect the host's existing startup, route, render-loop, and screenshot
   paths.
2. Define stable route IDs, user-facing titles, and positive default sizes.
3. Read one session during startup with
   `CaptureEnv::frame_capture().read_session::<Route>()?`.
4. Use a typed input session when `FRAME_CAPTURE_SCENARIO` should seed app
   state. Apply the scenario before starting either live or capture rendering.
5. Start the normal application at the selected route when `session.capture()`
   is `None`.
6. In capture mode, render the route at `capture.size()`,
   honor the one-based `capture.frame()` gate, create the output directory, and
   save a PNG to `capture.path()`.

For distributed installers, validate the registry and read a registered session
instead of an enum session. The generated key supplies the default route; the
environment can select another registered route.

For launch tooling, use `CaptureLaunchEnv::builder()` to validate variables.
Keep process construction and spawning in the calling tool.

## Session and tool contracts

- `FRAME_CAPTURE_PATH` alone switches the session into capture mode.
- Width and height overrides are an all-or-nothing pair of positive values.
- Route IDs may be relative paths; scenario IDs are single state identifiers.
- Plain `read_session` leaves scenarios unread. Use a typed input session or
  `read_scenario` when the host supports them.
- The selected route applies in both live and capture modes.
- MCP catalog servers expose metadata only. They do not launch applications or
  write screenshots.
- Treat MCP catalog servers as long-lived. EOF, cancellation, or another
  application-owned signal ends the host explicitly.

## Patterns

Read [generic patterns](references/generic-patterns.md) for concrete enum,
scenario, registered-route, launch-environment, and MCP snippets. Prefer the
current public API, user guide, and repository examples over memory when a
signature matters.
