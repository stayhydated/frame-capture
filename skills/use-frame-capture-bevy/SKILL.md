---
name: use-frame-capture-bevy
description: Integrate, review, or refactor frame-capture-bevy in Bevy applications. Use when Codex needs to wire live and offscreen capture modes, CaptureRouteBevy or CaptureScenarioBevy, BevyCaptureSession, RoutePlugin, capture_window_plugin, CaptureReady, CaptureWarmupPlugin, Bevy state or resource mapping, registered fn(&mut App) routes, FRAME_CAPTURE_* inputs, frame gates, scenarios, or deterministic PNG output. Use the generic frame-capture skill when the host owns a non-Bevy screenshot pipeline.
---

# Use frame-capture Bevy

## Follow the integration workflow

1. Inspect the existing Bevy plugin, route, state, camera, and startup setup.
2. Define stable routes with `CaptureRouteBevy` and positive default sizes.
3. Read the session before constructing `App`. Use
   `read_bevy_session_with_scenario` when scenario IDs seed app state; plain
   `read_bevy_session` rejects a supplied scenario.
4. Build the live window with `session.window_resolution()` or
   `window_resolution_with_live_size`.
5. Add `DefaultPlugins` only through
   `session.add_capture_plugins(&mut app, DefaultPlugins.set(session.capture_window_plugin(window)))`.
6. Install route-specific behavior with `RoutePlugin`, a session state/resource
   helper, or a registered route installer.
7. Spawn capture cameras and scene content by `Startup`, before the offscreen
   target is assigned in `PostStartup`.
8. Use `CaptureReady::pending()` for asynchronous preparation and mark it ready
   from a system. Use `CaptureWarmupPlugin::frames(n)` for a fixed delay.
9. Run the app normally. Capture mode saves the requested PNG and exits through
   `AppExit`.

## Map selected inputs deliberately

- Use `add_route_state` when the route enum itself drives Bevy state schedules.
- Use `add_scenario_state` when the scenario enum is a Bevy state and live mode
  has an explicit fallback.
- Use `add_selected_resources` for target-neutral route and scenario enums.
- Use the mapped state helpers for app-owned Bevy states.
- Use the mapped resource helpers for app-owned resources.

## Use registered routes

Enable the `registry` feature only for inventory-backed `fn(&mut App)` route
installers. Validate the registry, resolve the session, configure the capture
window and runtime from `session.capture()`, then call
`session.install(&mut app)` before `app.run()`.

## Preserve these contracts

- Live mode keeps the supplied Bevy plugin group's normal behavior.
- Capture mode disables `WinitPlugin`, removes the primary window, runs the
  schedule runner, redirects cameras to an offscreen image, and saves the PNG.
- Route, scenario, size, frame, output path, readiness, and warmup inputs must
  all affect the capture as documented.
- The output directory must exist before Bevy saves the image.
- MCP route catalogs remain read-only; the Bevy application owns capture
  execution.

## Load detailed patterns

Read [Bevy patterns](references/bevy-patterns.md) for typed setup, state and
resource mapping, readiness, and registered-route wiring. Prefer the current
public API, user guide, and repository Bevy example over memory when a signature
matters.
