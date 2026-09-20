# frame-capture-macros

[![Codecov: frame-capture-macros][codecov-badge]][codecov]
[![crates.io: frame-capture-macros][crate-badge]][crate]

Proc macros for typed capture routes, scenarios, and registered route
installers. Application crates normally use the reexports from their selected
facade.

## Overview

| Facade | Route derive | Scenario derive |
| --- | --- | --- |
| `frame-capture` | `CaptureRoute` | `CaptureScenario` |
| `frame-capture-routes` | `CaptureRouteRoutes` | `CaptureScenarioRoutes` |
| `frame-capture-bevy` | `CaptureRouteBevy` | `CaptureScenarioBevy` |

`frame-capture-routes` reexports `capture_route` for registered `fn()`
installers. `frame-capture-bevy` exposes the Bevy `fn(&mut App)` form through
its `registry` feature; `frame-capture-routes-bevy` exposes it for host-owned
capture runtimes.

Use `frame-capture-macros` directly when building a facade that needs to select
the generated runtime path.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-macros
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-macros.svg?label=frame-capture-macros
[crate]: https://crates.io/crates/frame-capture-macros
