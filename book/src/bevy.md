# Capture Bevy applications

`frame-capture-bevy` runs the same Bevy binary interactively or with an
offscreen screenshot runtime. The application continues to own route-specific
scene construction and state.

## Configure the application

Read the session before constructing `App`, derive the window resolution from
it, and add Bevy plugins through the session:

```rust,ignore
use bevy::{
    prelude::*,
    window::{PresentMode, Window},
};
use frame_capture_bevy::{
    BevyCaptureEnvExt as _, CaptureEnv, CaptureRoute as _, RoutePlugin,
};

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
app.add_plugins(RoutePlugin::new(route, |route, app: &mut App| {
    match route {
        UiRoute::Dashboard => app.add_plugins(DashboardPlugin),
        UiRoute::Review => app.add_plugins(ReviewPlugin),
    }
}));
app.run();
```

Do not add `DefaultPlugins` separately. In live mode,
`add_capture_plugins` adds the supplied plugins normally. In capture mode it
disables the primary-window path, adds the schedule runner, redirects cameras
created during `Startup` to an offscreen image in `PostStartup`, saves the
requested PNG, and exits.

Use `read_bevy_session_with_scenario` when the application maps
`FRAME_CAPTURE_SCENARIO` into plugins, states, or resources.

## Map selected inputs into Bevy

Choose the helper that matches the application's existing state ownership:

| Goal | Session helper |
| --- | --- |
| Use the route enum as a Bevy `State` | `add_route_state` |
| Use the scenario enum as a Bevy `State` with a live-mode fallback | `add_scenario_state` |
| Read target-neutral route and scenario enums as resources | `add_selected_resources` |
| Convert a route or scenario into an app-owned state | `add_mapped_route_state` or `add_mapped_scenario_state` |
| Convert a route or scenario into an app-owned resource | `insert_mapped_route_resource` or `insert_mapped_scenario_resource` |

For example, a route enum that also derives Bevy `States` can drive `OnEnter`
schedules directly:

```rust,ignore
session.add_route_state(&mut app);
app.add_systems(OnEnter(UiRoute::Dashboard), spawn_dashboard);
```

Keep a route enum target-neutral when it belongs to a shared crate. In that
case, use `add_selected_resources` so systems can read
`SelectedCaptureRoute<UiRoute>`, or map it into an application-specific state.

Use `window_resolution_with_live_size` when live mode needs a window size that
differs from the route's default. Capture mode still uses the requested capture
size, and both resolution helpers set the scale-factor override to `1.0`.

## Control readiness

`CaptureReady` defaults to ready. Insert `CaptureReady::pending()` only when
the capture must wait for asynchronous data or assets, then mark it ready from
a system:

```rust,ignore
use frame_capture_bevy::CaptureReady;

app.insert_resource(CaptureReady::pending());

fn mark_loaded(mut ready: ResMut<CaptureReady>) {
    ready.mark_ready();
}
```

For a fixed delay, add `CaptureWarmupPlugin::frames(n)`. Readiness and the frame
gate are both required before the screenshot request runs.

## Use registered routes

Enable the `registry` feature when route installers live beside Bevy feature
modules. A registered session exposes the selected route spec and optional
capture config instead of the typed session's window helpers, so configure the
window and capture runtime before installing the route:

```toml
[dependencies]
frame-capture-bevy = { version = "0.1", features = ["registry"] }
```

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
let env = CaptureEnv::frame_capture();
let session = env.read_registered_session_for::<InstallDashboardRoute>()?;

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

The registered-route session resolves route and capture variables. If the app
supports `FRAME_CAPTURE_SCENARIO`, retain `env`, call
`env.read_scenario::<Scenario>()?`, and apply the result before `app.run()`.

## Run and verify the example

From the repository root, run the example interactively:

```sh
cargo run -p frame-capture-example-bevy
```

Then capture the dashboard with the alert scenario:

```sh
FRAME_CAPTURE_ROUTE=bevy/dashboard \
FRAME_CAPTURE_PATH=target/dashboard.png \
FRAME_CAPTURE_SCENARIO=alert \
cargo run -p frame-capture-example-bevy
```

Success prints `Screenshot saved to target/dashboard.png`, exits successfully,
and leaves a nonempty PNG. The output path's parent directory must already
exist.

If a capture never exits, check for `CaptureReady::pending()` that is never
marked ready. If saving fails, create the output directory and confirm the path
ends in `.png`. If the image has no app content, spawn the capture camera and
scene no later than Bevy's `Startup` schedule so the runtime can redirect the
camera in `PostStartup`.
