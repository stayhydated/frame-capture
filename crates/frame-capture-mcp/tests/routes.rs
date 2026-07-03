use frame_capture_mcp::{
    CaptureMcpError, CaptureRoutesServer, RegisteredCaptureRoutesServer, capture_route,
    capture_routes, registered_capture_route, registered_capture_routes,
};

use frame_capture::{
    CaptureRoute, CaptureRouteId, CaptureRouteVariant, ParseRouteError, PixelSize, RouteSpec,
};

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "960x540")]
enum TestRoute {
    #[capture_route(id = "dashboard", title = "Dashboard")]
    Dashboard,
    #[capture_route(id = "detail", title = "Detail", size = "640x480")]
    Detail,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum InvalidRoute {
    Root,
    Detail,
}

impl CaptureRoute for InvalidRoute {
    const DEFAULT: Self = Self::Root;
    const ROUTES: &'static [Self] = &[Self::Detail];
    const VARIANTS: &'static [RouteSpec] = &[
        RouteSpec::new("root", "Root", PixelSize::new(960, 540)),
        RouteSpec::new("detail", "Detail", PixelSize::new(640, 480)),
    ];
    const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = &[
        CaptureRouteVariant {
            route: Self::Root,
            spec: Self::VARIANTS[0],
        },
        CaptureRouteVariant {
            route: Self::Detail,
            spec: Self::VARIANTS[1],
        },
    ];

    fn spec(self) -> RouteSpec {
        match self {
            Self::Root => Self::VARIANTS[0],
            Self::Detail => Self::VARIANTS[1],
        }
    }

    fn from_id(value: &str) -> Result<Self, ParseRouteError> {
        match value {
            "root" => Ok(Self::Root),
            "detail" => Ok(Self::Detail),
            _ => Err(ParseRouteError::new(value, ["root", "detail"])),
        }
    }
}

#[frame_capture_routes::capture_route(id = "registry/dashboard", title = "Registry Dashboard")]
fn install_dashboard() {}

#[test]
fn route_catalog_uses_capture_route_specs() {
    let routes = capture_routes::<TestRoute>().unwrap();

    assert_eq!(routes.len(), 2);
    assert_eq!(routes[0].id(), "dashboard");
    assert_eq!(routes[0].title(), "Dashboard");
    assert_eq!(routes[0].default_size().label(), "960x540");
    assert_eq!(routes[1].id(), "detail");
    assert_eq!(routes[1].default_size().width(), 640);
    assert_eq!(routes[1].default_size().height(), 480);
}

#[test]
fn route_lookup_validates_ids_with_capture_route_parser() {
    let id = CaptureRouteId::new("detail").unwrap();
    let route = capture_route::<TestRoute>(&id).unwrap();
    assert_eq!(route.title(), "Detail");

    let id = CaptureRouteId::new("missing").unwrap();
    let error = capture_route::<TestRoute>(&id).unwrap_err();
    assert!(matches!(error, CaptureMcpError::UnknownRoute(_)));
    assert!(
        error
            .to_string()
            .contains("unknown capture route `missing`")
    );
}

#[test]
fn invalid_route_catalog_fails_before_listing() {
    let error = capture_routes::<InvalidRoute>().unwrap_err();
    assert!(matches!(error, CaptureMcpError::InvalidCatalog(_)));
}

#[test]
fn registered_route_catalog_uses_inventory_specs() {
    let routes = registered_capture_routes().unwrap();

    assert!(
        routes
            .iter()
            .any(|route| route.id() == "registry/dashboard")
    );

    let id = CaptureRouteId::new("registry/dashboard").unwrap();
    let route = registered_capture_route(&id).unwrap();
    assert_eq!(route.title(), "Registry Dashboard");
}

#[test]
fn mcp_server_can_be_constructed_for_route_enum() {
    let _server = CaptureRoutesServer::<TestRoute>::new();
}

#[test]
fn registered_mcp_server_can_be_constructed() {
    let _server = RegisteredCaptureRoutesServer::new();
}
