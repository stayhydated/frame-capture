# frame-capture-macros

Proc macros for typed capture routes, scenarios, and registered route
installers. Application crates normally use the reexports from their selected
facade:

| Facade | Route derive | Scenario derive |
| --- | --- | --- |
| `frame-capture` | `CaptureRoute` | `CaptureScenario` |
| `frame-capture-routes` | `CaptureRouteRoutes` | `CaptureScenarioRoutes` |
| `frame-capture-bevy` | `CaptureRouteBevy` | `CaptureScenarioBevy` |

Each facade also reexports its matching `capture_route` attribute for registered
installers. Use `frame-capture-macros` directly only when building a facade that
needs to select the generated runtime path.

- [Route guide](https://stayhydated.github.io/frame-capture/book/routes.html)
- [API documentation](https://docs.rs/frame-capture-macros/)
