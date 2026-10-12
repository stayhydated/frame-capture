# frame-capture-macros

[![Codecov: frame-capture-macros][codecov-badge]][codecov]
[![crates.io: frame-capture-macros][crate-badge]][crate]

Proc macros for typed capture routes, scenarios, and registered route
installers. Application crates normally use the reexports from their selected
facade.

Requires Rust 1.99 or newer. Generated implementations require the runtime
types exported by the selected facade.

## Overview

| Facade | Route derive | Scenario derive |
| --- | --- | --- |
| [`frame-capture`][core] (`macros`) | `CaptureRoute` | `CaptureScenario` |
| [`frame-capture-routes`][routes] (`macros`) | `CaptureRouteRoutes` | `CaptureScenarioRoutes` |
| [`frame-capture-bevy`][bevy] (`macros`) | `CaptureRouteBevy` | `CaptureScenarioBevy` |

`frame-capture-routes` reexports `capture_route` for registered `fn()`
installers. `frame-capture-bevy` exposes the Bevy `fn(&mut App)` form through
its `registry` feature; `frame-capture-routes-bevy` exposes it for host-owned
capture runtimes.

Use `frame-capture-macros` directly when building a facade that needs to select
the generated runtime path.

## Example

A facade author can invoke the macro directly while keeping `frame-capture`
available as its generated runtime path:

```rust
use frame_capture::CaptureRoute as _;

#[derive(frame_capture_macros::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "960x540")]
enum Route {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
}

fn main() {
    assert_eq!(Route::Dashboard.id(), "dashboard");
    assert_eq!(Route::Dashboard.spec().default_size().width(), 960);
}
```

The derive generates typed IDs and metadata, as exercised by the
[route derive tests][tests]. Application code normally selects the facade
reexport in the table above.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-macros
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-macros.svg?label=frame-capture-macros
[crate]: https://crates.io/crates/frame-capture-macros
[core]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture/README.md
[routes]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-routes/README.md
[bevy]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-bevy/README.md
[tests]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture/tests/capture_route.rs
