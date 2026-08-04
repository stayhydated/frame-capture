# Introduction

`frame-capture` lets a Rust application render a named UI route normally or
save that same route as a deterministic PNG. It provides typed route and
scenario IDs plus an environment-variable protocol for the output path, frame,
and pixel size.

This guide is for application developers who already have a Rust UI and know
how their framework starts and renders it. Choose the integration that matches
who owns the screenshot pipeline:

- `frame-capture-bevy` provides Bevy's offscreen screenshot runtime.
- `frame-capture-routes` provides the protocol for GPUI, egui, raw wgpu, and
  other applications that save their own screenshots.
- `frame-capture` provides target-neutral types for custom facades.
- `frame-capture-mcp` exposes route metadata to tools without launching the
  application.

`FRAME_CAPTURE_PATH` is the mode switch. When it is absent, the session starts
the application live at the selected route. When it is present,
`session.capture()` contains the requested PNG path, frame, and size. The Bevy
facade renders and saves that capture; a route-only host must pass the same
values to its own renderer and screenshot pipeline.

A working integration exits successfully in capture mode and leaves a nonempty
PNG at the requested location.

Start with [choose an integration](integrations.md), then define the route
catalog and read one capture session during application startup.
