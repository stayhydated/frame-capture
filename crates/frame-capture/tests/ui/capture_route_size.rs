#[derive(frame_capture_macros::CaptureRoute, Clone, Copy, Eq, PartialEq)]
#[capture_route(crate = frame_capture, default = MissingSeparator)]
enum MissingSeparatorRoute {
    #[capture_route(size = "640")]
    MissingSeparator,
}

#[derive(frame_capture_macros::CaptureRoute, Clone, Copy, Eq, PartialEq)]
#[capture_route(crate = frame_capture, default = InvalidWidth)]
enum InvalidWidthRoute {
    #[capture_route(size = "widex480")]
    InvalidWidth,
}

#[derive(frame_capture_macros::CaptureRoute, Clone, Copy, Eq, PartialEq)]
#[capture_route(crate = frame_capture, default = InvalidHeight)]
enum InvalidHeightRoute {
    #[capture_route(size = "640xhigh")]
    InvalidHeight,
}

#[derive(frame_capture_macros::CaptureRoute, Clone, Copy, Eq, PartialEq)]
#[capture_route(crate = frame_capture, default = ZeroWidth)]
enum ZeroWidthRoute {
    #[capture_route(size = "0x480")]
    ZeroWidth,
}

fn main() {}
