# Bevy frame-capture patterns

## Contents

- [Configure a typed route](#configure-a-typed-route)
- [Map routes and scenarios](#map-routes-and-scenarios)
- [Wait for readiness](#wait-for-readiness)
- [Install a registered route](#install-a-registered-route)

## Configure a typed route

```rust,ignore
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
```

Do not add `DefaultPlugins` a second time. Capture cameras and scene content
must exist by the end of `Startup`.

## Map routes and scenarios

Choose one representation that matches the app:

| Goal | Helper |
| --- | --- |
| Route enum is a Bevy `State` | `session.add_route_state(&mut app)` |
| Scenario enum is a Bevy `State` | `session.add_scenario_state(&mut app, fallback)` |
| Target-neutral enums are resources | `session.add_selected_resources(&mut app)` |
| App owns another state type | `add_mapped_route_state` or `add_mapped_scenario_state` |
| App owns another resource type | `insert_mapped_route_resource` or `insert_mapped_scenario_resource` |

Read scenarios with
`read_bevy_session_with_scenario::<Route, Scenario>()?` before using scenario
helpers.

## Wait for readiness

```rust,ignore
use frame_capture_bevy::{CaptureReady, CaptureWarmupPlugin};

app.insert_resource(CaptureReady::pending())
    .add_systems(Update, mark_loaded);

fn mark_loaded(mut ready: ResMut<CaptureReady>) {
    ready.mark_ready();
}

app.add_plugins(CaptureWarmupPlugin::frames(30));
```

Use app-owned readiness for asynchronous work and warmup frames for a fixed
frame-count delay.

## Install a registered route

Enable `frame-capture-bevy`'s `registry` feature, then configure the lower-level
runtime before installing the selected route:

```rust,ignore
use bevy::{
    prelude::*,
    window::{PresentMode, Window, WindowResolution},
};
use frame_capture_bevy::{
    BevyCaptureAppExt as _, BevyCaptureRegistryEnvExt as _, CaptureEnv,
    capture_window_plugin,
};

#[frame_capture_bevy::capture_route(
    id = "dashboard",
    title = "Dashboard",
    size = "1280x720"
)]
fn install_dashboard(app: &mut App) {
    app.add_plugins(DashboardPlugin);
}

frame_capture_bevy::validate_registered_routes()?;
let session = CaptureEnv::frame_capture()
    .read_registered_session_for::<InstallDashboardRoute>()?;
let size = session
    .capture()
    .map(|capture| capture.size())
    .unwrap_or_else(|| session.spec().default_size());
let window = Window {
    title: session.spec().title().to_owned(),
    resolution: WindowResolution::new(size.width(), size.height())
        .with_scale_factor_override(1.0),
    present_mode: PresentMode::AutoNoVsync,
    ..default()
};
let capture = session.capture().cloned();

let mut app = App::new();
app.add_capture_plugins(
    DefaultPlugins.set(capture_window_plugin(capture.as_ref(), window)),
    capture,
);
session.install(&mut app);
app.run();
```

Registered sessions parse route and capture variables. Read and apply a typed
scenario separately when the registered application supports one.
