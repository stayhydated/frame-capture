# frame-capture-macros

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture-macros.svg)](https://crates.io/crates/frame-capture-macros)

Proc macros for typed capture routes, scenarios, and registered route
installers. Application crates normally use the reexports from their selected
facade:

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
