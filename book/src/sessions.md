# Read capture sessions

Read one environment-backed session during startup, then use its route in both
live and capture mode:

```rust,ignore
use frame_capture_routes::{CaptureEnv, CaptureRoute as _};

let session = CaptureEnv::frame_capture().read_session::<UiRoute>()?;
let route = *session.route();

if let Some(capture) = session.capture() {
    render(route, capture.size());
    wait_for_frame(capture.frame());
    save_png(capture.path())?;
} else {
    run_live(route);
}
```

`session.capture()` is `None` in live mode. In capture mode it contains the
validated PNG path, frame gate, and pixel size.

## Capture environment

`CaptureEnv::frame_capture()` reads these variables:

| Variable | Required | Default | Meaning |
| --- | --- | --- | --- |
| `FRAME_CAPTURE_ROUTE` | No | Catalog default | Route ID to render |
| `FRAME_CAPTURE_PATH` | For capture mode | None | PNG output file and capture-mode switch |
| `FRAME_CAPTURE_FRAME` | No | `12` | Positive, one-based frame gate |
| `FRAME_CAPTURE_WIDTH` | No | Route width | Capture width; pair with height |
| `FRAME_CAPTURE_HEIGHT` | No | Route height | Capture height; pair with width |
| `FRAME_CAPTURE_SCENARIO` | No | None | App-defined scenario ID |

Frame and size values only affect a capture when `FRAME_CAPTURE_PATH` is
present. The width and height overrides must be supplied together and both must
be greater than zero. The path must end in `.png`.

Use `CaptureEnv::with_prefix("MY_APP")` for names such as
`MY_APP_CAPTURE_ROUTE` and `MY_APP_CAPTURE_PATH`. Use
`CaptureEnv::try_with_prefix` when the prefix comes from input and should return
an error instead of panicking.

## Read scenarios

Scenarios are typed application state IDs:

```rust,ignore
#[derive(
    frame_capture_routes::CaptureScenarioRoutes,
    Clone,
    Copy,
    Debug,
    Eq,
    PartialEq,
)]
#[capture_scenario]
enum UiScenario {
    Empty,
    Loaded,
    Alert,
}

let session = CaptureEnv::frame_capture()
    .read_session_with_scenario::<UiRoute, UiScenario>()?;
```

Apply the selected scenario before rendering. A scenario is state input, not a
route path, so it cannot contain path separators. Route-only `read_session`
does not consume `FRAME_CAPTURE_SCENARIO`; use
`read_session_with_scenario` or `read_session_with_inputs` when the application
supports scenarios. Bevy's plain `read_bevy_session` rejects a supplied
scenario to prevent silently ignoring it.

## Diagnose invalid sessions

| Symptom | Cause | Action |
| --- | --- | --- |
| Unknown capture route or scenario | The environment ID is not in the typed catalog | Use one of the expected IDs printed in the error |
| “set both … width and … height” | Only one size override is set | Set both variables or unset both |
| Output-path error | The path is empty or does not name a `.png` file | Provide a PNG filename such as `target/dashboard.png` |
| No capture is written | `FRAME_CAPTURE_PATH` is absent | Set the path variable to enable capture mode |
