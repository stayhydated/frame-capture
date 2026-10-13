# frame-capture

[![Codecov: frame-capture][codecov-badge]][codecov]
[![crates.io: frame-capture][crate-badge]][crate]

Target-neutral route and capture-session primitives for Rust applications.
Use this crate directly when building a custom renderer facade, launch tool, or
shared protocol layer. Bevy applications normally use
[`frame-capture-bevy`][bevy]; applications with an existing screenshot pipeline
normally use [`frame-capture-routes`][routes].

Requires Rust 1.99 or newer.

## Overview

The runtime surface provides typed route and scenario IDs,
`CaptureEnv`, capture sessions, pixel sizes, frame gates, launch environment
data, and validated PNG output paths.

`FRAME_CAPTURE_PATH` selects capture mode. A custom facade is responsible for
rendering the selected route at the requested size and frame and saving the PNG
to the requested path.

## Features

The default feature set enables `macros`. `json-schema` is opt-in.

| Feature | Default | Purpose |
| --- | --- | --- |
| `default` | Yes | Enables `macros`. |
| `json-schema` | No | Enables Schemars schemas for capture metadata and validated protocol values. |
| `macros` | Yes | Reexports `CaptureRoute` and `CaptureScenario` from `frame-capture-macros`. |

## Example

A launch tool can validate capture inputs before passing them to its child
process. This example constructs environment data; the application performs
the rendering and PNG write.

```rust
use frame_capture::CaptureLaunchEnv;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let launch = CaptureLaunchEnv::builder()
        .route_id("settings")?
        .output_path("captures/settings.png")?
        .frame(2)?
        .build();

    let vars = launch.env_map_lossy();
    assert_eq!(vars["FRAME_CAPTURE_ROUTE"], "settings");
    assert_eq!(vars["FRAME_CAPTURE_PATH"], "captures/settings.png");
    Ok(())
}
```

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture.svg?label=frame-capture
[crate]: https://crates.io/crates/frame-capture
[bevy]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-bevy/README.md
[routes]: https://github.com/stayhydated/frame-capture/blob/master/crates/frame-capture-routes/README.md
