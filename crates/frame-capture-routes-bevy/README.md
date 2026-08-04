# frame-capture-routes-bevy

Bevy `App` route registration for host-owned capture runtimes. Each registered
route installs itself through `fn(&mut App)` and carries a stable ID, title, and
default pixel size.

Most applications that want the provided Bevy screenshot runtime should enable
the `registry` feature on `frame-capture-bevy`. Depend on this crate directly
when another runtime owns window setup, rendering, and screenshot output.

```toml
[dependencies]
frame-capture-routes-bevy = "0.1"
```

Declare installers with `#[frame_capture_routes_bevy::capture_route(...)]`,
validate the registry during startup, read the selected registered session, and
call `session.install(&mut app)`.

- [Integration guide](https://stayhydated.github.io/frame-capture/book/integrations.html)
- [API documentation](https://docs.rs/frame-capture-routes-bevy/)
