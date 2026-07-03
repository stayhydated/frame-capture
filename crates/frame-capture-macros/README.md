# frame-capture-macros

Proc macros for `frame-capture` route, scenario, and registered-route
declarations.

Most applications use these macros through `frame-capture`,
`frame-capture-bevy`, or `frame-capture-routes`. Use this crate directly only when
building a facade or when a proc-macro dependency is needed without the runtime
crate reexports.

## Route Enums

```rust
#[derive(frame_capture_macros::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(crate = frame_capture, default = Dashboard, id_prefix = "desktop", size = "1280x720")]
enum UiRoute {
    #[capture_route(title = "Dashboard")]
    Dashboard,
    Review,
}
```

Route ids default to `snake_case`. Add enum-level `id_prefix = "desktop"` to
prepend a shared route path to every variant id, including explicit ids. Titles
default to the variant name, and sizes can be declared with
`size = "WIDTHxHEIGHT"`, `width = ...` plus `height = ...`, or a package-local
`frame-capture.toml`.

## Scenario Enums

```rust
#[derive(frame_capture_macros::CaptureScenario, Clone, Copy, Debug, Eq, PartialEq)]
enum UiScenario {
    #[capture_scenario(
        id = "empty",
        title = "Empty State",
        description = "No records"
    )]
    Empty,
    Loaded,
}
```

Scenario ids use the same validation as route state ids.

## Registered Route Attributes

The crate provides facade-specific registered route attributes:

- `routes_capture_route` expands for `frame-capture-routes`.
- `routes_bevy_capture_route` expands for `frame-capture-routes-bevy`.
- `bevy_capture_route` expands for `frame-capture-bevy`.

Facade crates reexport these as `capture_route`, so application code usually
uses the facade path instead of importing from this crate.

## Implementation Boundary

This crate owns proc-macro parsing and expansion only. Runtime protocol types
belong to `frame-capture` and facade-specific installer signatures belong to the
matching facade crates.
