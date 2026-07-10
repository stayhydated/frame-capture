#[derive(frame_capture_macros::CaptureRoute)]
#[capture_route(default = Root, size = "10x10")]
struct RouteStruct;

#[derive(frame_capture_macros::CaptureRoute)]
#[capture_route(default = Root, size = "10x10")]
enum RoutePayload {
    Root(u32),
}

#[derive(frame_capture_macros::CaptureRoute)]
#[capture_route(size = "10x10")]
enum EmptyRoute {}

#[derive(frame_capture_macros::CaptureRoute)]
#[capture_route(default = Missing, size = "10x10")]
enum MissingDefaultRoute {
    Root,
}

#[derive(frame_capture_macros::CaptureRoute)]
#[capture_route(default = Root, size = "10x10")]
enum EmptyRouteId {
    #[capture_route(id = "")]
    Root,
}

#[derive(frame_capture_macros::CaptureRoute)]
#[capture_route(default = Root, id_prefix = "../invalid", size = "10x10")]
enum InvalidRoutePrefix {
    Root,
}

#[derive(frame_capture_macros::CaptureRoute)]
#[capture_route(default = Root, width = 0, height = 10)]
enum ZeroRouteWidth {
    Root,
}

fn main() {}
