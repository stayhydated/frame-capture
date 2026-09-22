# frame-capture-routes-bevy

[![Codecov: frame-capture-routes-bevy][codecov-badge]][codecov]
[![crates.io: frame-capture-routes-bevy][crate-badge]][crate]

Bevy `App` route registration for host-owned capture runtimes. Each registered
route installs itself through `fn(&mut App)` and carries a stable ID, title, and
default pixel size.

## Overview

Most applications that want the provided Bevy screenshot runtime should enable
the `registry` feature on `frame-capture-bevy`. Depend on this crate directly
when another runtime owns window setup, rendering, and screenshot output.

Declare installers with `#[frame_capture_routes_bevy::capture_route(...)]`,
validate the registry during startup, read the selected registered session, and
call `session.install(&mut app)`.

[codecov-badge]: https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg?component=frame-capture-routes-bevy
[codecov]: https://codecov.io/gh/stayhydated/frame-capture
[crate-badge]: https://img.shields.io/crates/v/frame-capture-routes-bevy.svg?label=frame-capture-routes-bevy
[crate]: https://crates.io/crates/frame-capture-routes-bevy
