# Bevy frame-capture Patterns

## App Setup

Use `add_capture_plugins` instead of adding `DefaultPlugins` separately. In capture mode it disables the primary window path, adds the schedule runner, redirects cameras to an offscreen image, saves the PNG, and exits successfully.

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
    app.add_plugins(RoutePlugin::new(session.route(), |route, app: &mut App| {
        match route {
            UiRoute::Dashboard => {
                app.add_plugins(DashboardPlugin);
            }
            UiRoute::Review => {
                app.add_plugins(ReviewPlugin);
            }
        }
    }));
    app.run();

    Ok(())
}

struct DashboardPlugin;
struct ReviewPlugin;
impl Plugin for DashboardPlugin { fn build(&self, _app: &mut App) {} }
impl Plugin for ReviewPlugin { fn build(&self, _app: &mut App) {} }
```

## Readiness

`CaptureReady` defaults to ready. Insert `CaptureReady::pending()` only when capture should wait for async data, asset loading, or app-owned stabilization.

```rust
use frame_capture_bevy::CaptureReady;

app.insert_resource(CaptureReady::pending())
    .add_systems(Update, mark_capture_ready);

fn mark_capture_ready(mut ready: ResMut<CaptureReady>) {
    ready.mark_ready();
}
```

For a fixed frame delay, use `CaptureWarmupPlugin`:

```rust
use frame_capture_bevy::CaptureWarmupPlugin;

app.add_plugins(CaptureWarmupPlugin::frames(30));
```

## Bevy States

Use the session route-state helper when route selection should drive normal Bevy
state schedules and run conditions.

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

Use the session selected-resource helper when the capture route enum lives in a
target-neutral core crate and should not derive Bevy traits.

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

Use mapped state helpers when target-neutral capture inputs should drive
app-owned Bevy states.

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

Use `SelectedStatePlugin` after mapping capture inputs to an app-specific state,
or use `session.add_scenario_state` when the scenario enum itself is a Bevy
state with an explicit fallback for live runs.

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

Use `session.add_selected_resources` when systems need the optional typed
scenario values as Bevy resources without making those enums derive
Bevy traits.

Use mapped resource helpers when capture inputs should seed app-owned resources.

```rust
#[derive(Resource)]
struct UiRuntimeState {
    loaded: bool,
}

session.insert_mapped_scenario_resource(&mut app, |scenario| UiRuntimeState {
    loaded: matches!(scenario, Some(UiScenario::Loaded)),
});
```

## Scenarios

Scenarios are app-owned state presets. Read them from the env and use `ScenarioPlugin` to apply normal Bevy resources, plugins, or state before capture readiness is marked.

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

## Registry Feature

Use the `registry` feature when route installers should be registered as `fn(&mut App)`. The registry replaces `RoutePlugin` route dispatch; still use the capture plugin/window setup from the app setup pattern.

```toml
frame-capture-bevy = { version = "0.1", features = ["registry"] }
```

```rust
use frame_capture_bevy::{BevyCaptureRegistryEnvExt as _, CaptureEnv};

#[frame_capture_bevy::capture_route(id = "dashboard", title = "Dashboard", size = "1280x720")]
fn install_dashboard(app: &mut bevy::prelude::App) {
    app.add_plugins(DashboardPlugin);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    frame_capture_bevy::validate_registered_routes()?;
    let session = CaptureEnv::frame_capture().read_registered_session_for::<InstallDashboardRoute>()?;
    let mut app = bevy::prelude::App::new();
    session.install(&mut app);
    Ok(())
}
```

## Implementation Notes

- Preserve live mode: normal Bevy plugins and primary window behavior should still work without capture variables.
- In capture mode, use `session.capture_window_plugin(window)` so there is no primary window.
- Spawn cameras before capture target setup runs; cameras are redirected to the offscreen image.
- Save paths should use an image extension Bevy/image can infer, normally `.png`.
