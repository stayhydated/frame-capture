# Choose an integration

Choose the crate that matches the host application's renderer ownership. Most
applications need one facade, not every crate in the workspace.

| Application | Dependency | Screenshot owner |
| --- | --- | --- |
| Bevy with the provided offscreen runtime | `frame-capture-bevy` | `frame-capture-bevy` |
| GPUI, egui, raw wgpu, or another existing renderer | `frame-capture-routes` | Host application |
| A custom renderer facade or shared protocol crate | `frame-capture` | Custom facade or host |
| Bevy route registration with another capture runtime | `frame-capture-routes-bevy` | Host application |

## Bevy with offscreen capture

Use `frame-capture-bevy` for normal interactive runs and environment-driven
offscreen captures from the same binary:

```toml
[dependencies]
bevy = "0.19"
frame-capture-bevy = "0.1"
```

Enable its `registry` feature only when route installers are distributed as
registered `fn(&mut App)` functions. The feature reexports the Bevy registry
API, so a standard offscreen-capture application does not also need a direct
`frame-capture-routes-bevy` dependency.

## Applications with an existing renderer

Use `frame-capture-routes` when the application already owns its window, frame
loop, pixel readback, and PNG save operation:

```toml
[dependencies]
frame-capture-routes = "0.1"
```

The facade selects and validates the route, scenario, size, frame, and output
path. It does not select a graphics backend or save the image.

## Custom renderer facades

Use `frame-capture` directly for a renderer integration or shared protocol
layer:

```toml
[dependencies]
frame-capture = "0.1"
```

Use [route-only applications](route_only.md) for the host-owned capture
workflow. Use [Bevy applications](bevy.md) when `frame-capture-bevy` should own
the runtime.
