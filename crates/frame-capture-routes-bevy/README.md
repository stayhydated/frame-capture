# frame-capture-routes-bevy

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture-routes-bevy.svg)](https://crates.io/crates/frame-capture-routes-bevy)

Bevy `App` route registration for host-owned capture runtimes. Each registered
route installs itself through `fn(&mut App)` and carries a stable ID, title, and
default pixel size.

Most applications that want the provided Bevy screenshot runtime should enable
the `registry` feature on `frame-capture-bevy`. Depend on this crate directly
when another runtime owns window setup, rendering, and screenshot output.

Declare installers with `#[frame_capture_routes_bevy::capture_route(...)]`,
validate the registry during startup, read the selected registered session, and
call `session.install(&mut app)`.
