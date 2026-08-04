# frame-capture-bevy

Bevy facade for running the same route interactively or through an offscreen
PNG capture runtime.

## Add the dependencies

```toml
[dependencies]
bevy = "0.19"
frame-capture-bevy = "0.1"
```

A Bevy integration follows four steps:

1. Define routes with `CaptureRouteBevy`.
2. Read a `BevyCaptureSession` before constructing `App`.
3. Derive the window resolution from the session and add `DefaultPlugins`
   through `session.add_capture_plugins`.
4. Install the selected route with `RoutePlugin`, a session state/resource
   helper, or a registered route installer.

```sh
FRAME_CAPTURE_ROUTE=dashboard \
FRAME_CAPTURE_PATH=captures/dashboard.png \
cargo run -p my-bevy-app
```

Use `CaptureReady` when capture must wait for asynchronous preparation. Enable
the `registry` feature when routes are registered as `fn(&mut App)` installers.

- [Bevy guide](https://stayhydated.github.io/frame-capture/book/bevy.html)
- [API documentation](https://docs.rs/frame-capture-bevy/)
