---
name: use-frame-capture-bevy
description: Use when Codex needs to add, review, or update Bevy integrations for frame-capture, including frame-capture-bevy live and capture mode setup, offscreen screenshot runtime wiring, RoutePlugin route selection, capture_window_plugin, CaptureReady, Bevy registered-route APIs, capture env parsing, frame gates, and scenarios.
---

# Use frame-capture Bevy

## Scope Boundary

Treat this skill as a hosted public-usage guide for `frame-capture-bevy`
consumers. Use it for Bevy apps that should run normally without capture
variables and save deterministic screenshots when capture variables are present:
live and capture mode setup, offscreen screenshot runtime wiring, `RoutePlugin`
route selection, `capture_window_plugin`, `CaptureReady`, Bevy registered-route
APIs, capture environment parsing, frame gates, and scenarios.

Do not use this skill as a contributor guide for `frame-capture` repository
internals. For workspace maintenance, release, CI, architecture, or
target-neutral route-only integrations, read the repository source,
`AGENTS.md`, and the relevant crate documentation directly. Use
`use-frame-capture` instead when the application owns its renderer or screenshot
pipeline outside the Bevy capture runtime.

## Core Workflow

Start from the application's existing Bevy setup and route structure. Preserve
live behavior when capture variables are absent; `frame-capture-bevy` owns the
Bevy offscreen screenshot runtime, and app code owns route-specific scene setup
and state.

1. Define routes with `frame_capture_bevy::CaptureRouteBevy` and
   `#[capture_route(default = ..., size = "...")]`.
2. Read the session before building the app:
   `let session = CaptureEnv::frame_capture().read_bevy_session::<Route>()?;`.
   Use `read_bevy_session_with_scenario` or `read_bevy_session_with_inputs`
   when the app supports typed scenario ids. Plain `read_bevy_session` rejects
   scenario variables.
3. Compute the Bevy window resolution from `session.window_resolution()` unless
   the live app has its own explicit size override. Use a scale factor override
   of `1.0` for deterministic pixels.
4. Add Bevy plugins through
   `session.add_capture_plugins(&mut app, DefaultPlugins.set(session.capture_window_plugin(window)))`.
   This preserves normal Bevy plugin behavior in live mode and configures the
   offscreen runtime in capture mode.
5. Install route-specific systems or plugins through
   `RoutePlugin::new(session.route(), |route, app| { ... })`.
6. When the selected capture route should drive Bevy schedules, derive Bevy
   `States` on the route enum and call `session.add_route_state(&mut app)`. If
   the route enum lives in a target-neutral core crate, keep it free of Bevy
   traits and call `session.add_selected_resources(&mut app)` so systems can
   read `SelectedCaptureRoute<Route>`, or use
   `session.add_mapped_route_state(&mut app, ...)` when the selected route maps
   to an app-owned Bevy state.
7. Apply capture scenarios through
   `ScenarioPlugin::new(session.scenario(), |scenario, app| { ... })` when
   scenario ids should seed app-owned plugins or larger setup. Use
   `session.add_scenario_state(&mut app, default_state)` when the scenario enum
   itself is a Bevy `State`, `session.add_mapped_scenario_state(&mut app, ...)`
   when it maps to an app state,
   `session.insert_mapped_scenario_resource(&mut app, ...)` when it seeds an app
   resource, or `session.add_selected_resources(&mut app)` when systems need the
   optional typed scenario without a Bevy dependency in the scenario crate.
8. Insert `CaptureReady::pending()` only when capture must wait for async state
   or assets, then call `mark_ready()` from a system. Use
   `CaptureWarmupPlugin::frames(n)` for a fixed frame delay.
9. Enable the `registry` feature only when route installers should be registered
  as `fn(&mut App)` inventory entries.

## Reference Selection

Load only the reference needed for the task:

- `references/bevy-patterns.md`: copyable app setup, readiness, scenario, and registry patterns for concrete Bevy integrations.

Prefer current public docs or source examples over memory when details matter.

## Implementation Rules

Live mode should keep normal Bevy plugin behavior.

Capture mode should disable the primary window path, use the schedule runner,
render to an offscreen image, save a PNG, and exit successfully.

Respect the requested route, size, frame, output path, scenario, and
`CaptureReady`.

When tool-facing route discovery is needed, expose route metadata through a
read-only `frame-capture-mcp` server and keep capture execution owned by the
Bevy app's environment-driven startup path.

Keep capture-specific state explicit:

- Use `CaptureReady::pending()` only when a capture must wait for asynchronous preparation.
- Call `mark_ready()` from a system once assets, state, or route setup are ready.
- Use `CaptureWarmupPlugin::frames(n)` for fixed frame-count stabilization.
- Use `ScenarioPlugin` to apply scenario state before capture readiness is marked.
- Use `session.add_route_state`, `session.add_scenario_state`, or
  `SelectedStatePlugin` when capture inputs should become Bevy `State` resources
  for `OnEnter` schedules and `in_state` run conditions.
- Use `session.add_mapped_route_state` or `session.add_mapped_scenario_state`
  when target-neutral capture ids map to app-owned Bevy states.
- Use `RouteResourcePlugin` or `ScenarioResourcePlugin` when target-neutral
  capture enums should be readable from Bevy systems without deriving Bevy traits;
  `session.add_selected_resources` installs selected input resources from a Bevy
  capture session.
- Use `session.insert_mapped_route_resource` or
  `session.insert_mapped_scenario_resource` when capture inputs should seed
  app-owned resources.

Keep route wiring explicit:

- Use `RoutePlugin` for route-specific systems and plugins.
- Keep enum routes and registered-route installers aligned when both are present.
- Keep the `registry` feature gate accurate when reexporting or consuming Bevy registered-route APIs.

Update the Bevy example and checked-in captures only for intentional visual or
runtime behavior changes.
