use std::sync::atomic::{AtomicUsize, Ordering};

use frame_capture_routes::{
    CaptureEnv, CaptureRoute as _, CaptureRouteId, CaptureRoutesEnvExt as _, PixelSize,
    registered_route, registered_route_for_key, registered_routes,
};

static INSTALLS: AtomicUsize = AtomicUsize::new(0);

#[derive(frame_capture_routes::CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "960x540")]
enum TypedRoute {
    #[capture_route(id = "typed/dashboard", title = "Typed Dashboard")]
    Dashboard,
    #[capture_route(id = "typed/review", title = "Typed Review")]
    Review,
}

#[frame_capture_routes::capture_route(
    id = "registry/root",
    key = RegistryRootRoute,
    title = "Registry Root",
    size = "640x480"
)]
fn install_root() {
    INSTALLS.fetch_add(1, Ordering::Relaxed);
}

#[frame_capture_routes::capture_route(
    id = "registry/review",
    title = "Registry Review",
    width = 320,
    height = 240
)]
fn install_review() {}

#[frame_capture_routes::capture_route]
fn registry_conventional() {}

#[test]
fn facade_reexports_typed_route_derive() {
    let session = CaptureEnv::with_prefix("FRAME_CAPTURE_ROUTES_TYPED_TEST")
        .read_session::<TypedRoute>()
        .unwrap();

    assert_eq!(*session.route(), TypedRoute::Dashboard);
    assert_eq!(
        TypedRoute::ROUTES,
        &[TypedRoute::Dashboard, TypedRoute::Review]
    );
    assert_eq!(TypedRoute::Review.id(), "typed/review");
    assert_eq!(TypedRoute::Review.spec().title(), "Typed Review");
    assert_eq!(
        TypedRoute::Review.spec().default_size(),
        PixelSize::new(960, 540)
    );
}

#[test]
fn registers_route_specs() {
    let routes = registered_routes();
    assert!(
        routes
            .iter()
            .any(|route| route.spec().id() == "registry/root")
    );
    assert!(
        routes
            .iter()
            .any(|route| route.spec().id() == "registry/review")
    );

    let route = registered_route(&route_id("registry/review")).unwrap();
    assert_eq!(route.spec().title(), "Registry Review");
    assert_eq!(route.spec().default_size(), PixelSize::new(320, 240));

    let route = registered_route(&route_id("registry_conventional")).unwrap();
    assert_eq!(route.spec().title(), "RegistryConventional");
    assert_eq!(route.spec().default_size(), PixelSize::new(1920, 1080));
}

#[test]
fn registered_route_installs_page() {
    let route = registered_route(&route_id("registry/root")).unwrap();
    route.install();

    assert_eq!(INSTALLS.load(Ordering::Relaxed), 1);
}

#[test]
fn env_reads_registered_session() {
    let session = CaptureEnv::with_prefix("FRAME_CAPTURE_REGISTRY_TEST")
        .read_registered_session(&route_id("registry/root"))
        .unwrap();

    assert_eq!(session.spec().id(), "registry/root");
    assert!(!session.is_capture());
}

#[test]
fn typed_route_key_reads_registered_session() {
    let route = registered_route_for_key::<RegistryRootRoute>().unwrap();
    assert_eq!(route.spec().id(), "registry/root");

    let route = registered_route_for_key::<RegistryConventionalRoute>().unwrap();
    assert_eq!(route.spec().id(), "registry_conventional");

    let session = CaptureEnv::with_prefix("FRAME_CAPTURE_REGISTRY_TEST")
        .read_registered_session_for::<RegistryRootRoute>()
        .unwrap();

    assert_eq!(session.spec().id(), "registry/root");
}

#[test]
fn validates_registered_route_ids_are_unique() {
    frame_capture_routes::validate_registered_routes().unwrap();
}

fn route_id(value: &str) -> CaptureRouteId {
    CaptureRouteId::new(value).unwrap()
}
