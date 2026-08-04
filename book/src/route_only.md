# Capture route-only applications

Route-only hosts such as GPUI, egui, and raw wgpu use
`frame-capture-routes`. The application resolves validated capture inputs and
passes them to its existing renderer.

## Run the host renderer

```rust,ignore
let session = frame_capture_routes::CaptureEnv::frame_capture()
    .read_session::<UiRoute>()?;

match session.capture() {
    Some(capture) => {
        renderer.wait_until(capture.frame());
        let pixels = renderer.render(*session.route(), capture.size());
        create_parent_directory(capture.path())?;
        renderer.save_png(&pixels, capture.path())?;
    }
    None => renderer.run_live(*session.route()),
}
```

The protocol does not replace renderer-specific screenshot code. It provides
the stable selection contract around that code. The host must apply the
selected route and scenario, render at `capture.size()`, honor
`capture.frame()`, create the output directory, and save a PNG to
`capture.path()`.

## Build launch environment data

Tools that prepare a host command can use `CaptureLaunchEnv::builder()` to
validate and produce `FRAME_CAPTURE_*` entries:

```rust,ignore
let launch_env = frame_capture_routes::CaptureLaunchEnv::builder()
    .route_id("dashboard")?
    .output_path("target/dashboard.png")?
    .frame(12)?
    .size(1280, 720)?
    .build();

let env = launch_env.env_map_lossy();
assert_eq!(env["FRAME_CAPTURE_ROUTE"], "dashboard");
assert_eq!(env["FRAME_CAPTURE_PATH"], "target/dashboard.png");
```

The fallible setters reject invalid route IDs, non-PNG paths, zero frames, and
zero dimensions. `build()` returns environment data only; the process manager
still owns command construction, working directory, environment installation,
and spawning.

The repository's GPUI example demonstrates the complete host-owned path:

```sh
FRAME_CAPTURE_ROUTE=gpui/dashboard \
FRAME_CAPTURE_PATH=target/gpui-dashboard.png \
cargo run --manifest-path examples/gpui/Cargo.toml
```

A successful route-only integration exits without an error and leaves a
nonempty PNG at the requested path.
