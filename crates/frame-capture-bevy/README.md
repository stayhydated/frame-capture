# frame-capture-bevy

Bevy facade for `frame-capture` route selection and offscreen screenshot capture.

Use this crate when a Bevy app should run normally without capture variables and
save a deterministic screenshot when capture variables are present.

## Use

```toml
[dependencies]
bevy = "0.19"
frame-capture-bevy = "0.1"
```

```rust
use bevy::{
    prelude::*,
    window::{PresentMode, Window},
};
use frame_capture_bevy::{
    BevyCaptureEnvExt as _, CaptureEnv, CaptureRoute as _, RoutePlugin,
};

#[derive(frame_capture_bevy::CaptureRouteBevy, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
    #[capture_route(id = "review", title = "Review")]
    Review,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture().read_bevy_session::<UiRoute>()?;
    let route = session.route();

    let window = Window {
        title: route.spec().title().to_owned(),
        resolution: session.window_resolution(),
        present_mode: PresentMode::AutoNoVsync,
        ..default()
    };

    let mut app = App::new();
    session.add_capture_plugins(
        &mut app,
        DefaultPlugins.set(session.capture_window_plugin(window)),
    );
    app.add_plugins(RoutePlugin::new(route, |route, app: &mut App| match route {
        UiRoute::Dashboard => app.add_plugins(DashboardPlugin),
        UiRoute::Review => app.add_plugins(ReviewPlugin),
    }));
    app.run();

    Ok(())
}

struct DashboardPlugin;
struct ReviewPlugin;

impl Plugin for DashboardPlugin {
    fn build(&self, _app: &mut App) {}
}

impl Plugin for ReviewPlugin {
    fn build(&self, _app: &mut App) {}
}
```

## Capture Mode

`add_capture_plugins` adds the normal Bevy plugins in live mode. In capture mode
it disables `WinitPlugin`, configures Bevy without a primary window, adds a
schedule runner, redirects cameras to an offscreen image, saves the screenshot,
and exits with `AppExit::Success`.

`read_bevy_session` reads the route and capture config together and rejects
scenario variables by default. Use `read_bevy_session_with_scenario` or
`read_bevy_session_with_inputs` when scenario ids should seed app state or
resources.

Run a capture:

```sh
FRAME_CAPTURE_ROUTE=dashboard \
FRAME_CAPTURE_PATH=captures/dashboard.png \
cargo run -p my-bevy-app
```

Optional variables include `FRAME_CAPTURE_FRAME`, `FRAME_CAPTURE_WIDTH`,
`FRAME_CAPTURE_HEIGHT`, and `FRAME_CAPTURE_SCENARIO`.

## Bevy States

When a capture route is also the app's page or mode state, derive Bevy
`States` on the route enum and install it from the Bevy capture session:

```rust
use bevy::prelude::*;

#[derive(frame_capture_bevy::CaptureRouteBevy, Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
#[capture_route(default = Dashboard, size = "1280x720")]
enum UiRoute {
    Dashboard,
    Review,
}

session.add_route_state(&mut app);
app.add_systems(OnEnter(UiRoute::Dashboard), spawn_dashboard);
```

When the route enum lives in a target-neutral crate and should not depend on
Bevy, insert the selected route as a wrapper resource instead:

```rust
use bevy::prelude::*;
use frame_capture_bevy::SelectedCaptureRoute;

session.add_selected_resources(&mut app);
app.add_systems(Startup, setup_selected_route);

fn setup_selected_route(route: Res<SelectedCaptureRoute<UiRoute>>) {
    match route.route() {
        UiRoute::Dashboard => {}
        UiRoute::Review => {}
    }
}
```

If a target-neutral route or scenario should become an app-specific Bevy state,
map it through the session:

```rust
#[derive(States, Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum UiPageState {
    Dashboard,
    Review,
}

session.add_mapped_route_state(&mut app, |route| match route {
    UiRoute::Dashboard => UiPageState::Dashboard,
    UiRoute::Review => UiPageState::Review,
});
```

Use `SelectedStatePlugin` when you have already mapped capture input to an app
state value. Use `session.add_scenario_state` when a scenario enum should become
a Bevy state with an explicit live/default fallback:

```rust
use bevy::prelude::*;
use frame_capture_bevy::{BevyCaptureEnvExt as _, CaptureEnv};

#[derive(
    frame_capture_bevy::CaptureScenarioBevy, Clone, Copy, Debug, Eq, Hash, PartialEq, States,
)]
#[capture_scenario]
enum UiScenario {
    Empty,
    Loaded,
}

let session = CaptureEnv::frame_capture().read_bevy_session_with_scenario::<UiRoute, UiScenario>()?;
session.add_scenario_state(&mut app, UiScenario::Empty);
```

`session.add_selected_resources` provides the same wrapper resource pattern for
optional scenario ids. Use `insert_mapped_route_resource` or
`insert_mapped_scenario_resource` when capture input should seed an app-owned
resource:

```rust
#[derive(Resource)]
struct UiRuntimeState {
    loaded: bool,
}

let session = CaptureEnv::frame_capture().read_bevy_session_with_scenario::<UiRoute, UiScenario>()?;
session.insert_mapped_scenario_resource(&mut app, |scenario| UiRuntimeState {
    loaded: matches!(scenario, Some(UiScenario::Loaded)),
});
```

When live mode has an app-specific window-size override, pass it through
`window_resolution_with_live_size`. Capture mode still uses the requested
capture size:

```rust
let live_size = PixelSize::new(1920, 1080);
let resolution = session.window_resolution_with_live_size(live_size);
```

## State and Readiness

Scenarios are app-owned state presets. Read them from the same environment and
install them through `ScenarioPlugin`:

```rust
use bevy::prelude::*;
use frame_capture_bevy::{BevyCaptureEnvExt as _, CaptureEnv, ScenarioPlugin};

#[derive(Default, Resource)]
struct UiState {
    loaded: bool,
}

impl UiState {
    fn loaded() -> Self {
        Self { loaded: true }
    }
}

#[derive(frame_capture_bevy::CaptureScenarioBevy, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_scenario]
enum UiScenario {
    #[capture_scenario(id = "empty", title = "Empty State")]
    Empty,
    Loaded,
}

let session = CaptureEnv::frame_capture().read_bevy_session_with_scenario::<UiRoute, UiScenario>()?;

app.add_plugins(ScenarioPlugin::new(session.scenario(), |scenario, app: &mut App| {
    match scenario {
        Some(UiScenario::Loaded) => app.insert_resource(UiState::loaded()),
        Some(UiScenario::Empty) | None => app.init_resource::<UiState>(),
    };
}));
```

Insert `CaptureReady::pending()` when the app should wait for async state or
asset loading before saving. Mark it ready from a system:

```rust
use frame_capture_bevy::CaptureReady;

fn mark_loaded(mut ready: ResMut<CaptureReady>) {
    ready.mark_ready();
}
```

For a fixed frame delay, use `CaptureWarmupPlugin`:

```rust
use frame_capture_bevy::CaptureWarmupPlugin;

app.add_plugins(CaptureWarmupPlugin::frames(30));
```

## Registry Feature

Enable `registry` when routes should be registered as `fn(&mut App)` installers:

```toml
frame-capture-bevy = { version = "0.1", features = ["registry"] }
```

```rust
use frame_capture_bevy::{BevyCaptureRegistryEnvExt as _, CaptureEnv};

#[frame_capture_bevy::capture_route(id = "dashboard", title = "Dashboard", size = "1280x720")]
fn install_dashboard(app: &mut bevy::prelude::App) {
    app.add_plugins(DashboardPlugin);
}

let session = CaptureEnv::frame_capture().read_registered_session_for::<InstallDashboardRoute>()?;
session.install(&mut app);
```

## Implementation Boundary

This crate owns the Bevy live/capture runtime and helpers that map selected
capture inputs into Bevy plugins, states, or resources. Application code owns
route-specific scene setup, assets, app state, and readiness decisions.
