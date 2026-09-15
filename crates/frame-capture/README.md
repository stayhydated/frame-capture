# frame-capture

[![CI](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/stayhydated/frame-capture/actions/workflows/ci.yml)
[![Codecov](https://codecov.io/gh/stayhydated/frame-capture/branch/master/graph/badge.svg)](https://codecov.io/gh/stayhydated/frame-capture)
[![Book](https://img.shields.io/badge/book-online-blue)](https://stayhydated.github.io/frame-capture/book/)
[![crates.io](https://img.shields.io/crates/v/frame-capture.svg)](https://crates.io/crates/frame-capture)

Target-neutral route and capture-session primitives for Rust applications.
Use this crate directly when building a custom renderer facade or shared
protocol layer. Bevy applications normally use `frame-capture-bevy`; applications
with an existing screenshot pipeline normally use `frame-capture-routes`.

The default `macros` feature provides the `CaptureRoute` and `CaptureScenario`
derives. The runtime surface provides typed route and scenario IDs,
`CaptureEnv`, capture sessions, pixel sizes, frame gates, launch environment
data, and validated PNG output paths.

`FRAME_CAPTURE_PATH` selects capture mode. A custom facade is responsible for
rendering the selected route at the requested size and frame and saving the PNG
to the requested path.
