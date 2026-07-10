use std::sync::atomic::{AtomicUsize, Ordering};

use frame_capture_routes_bevy::{
    App, BevyCaptureRegistryEnvExt as _, CaptureEnv, CaptureRouteId, PixelSize, RegisteredRoute,
    RouteSpec, registered_route, registered_route_for_key, registered_routes,
};

static INSTALLS: AtomicUsize = AtomicUsize::new(0);

#[frame_capture_routes_bevy::capture_route(
    id = "registry/root",
    key = RegistryRootRoute,
    title = "Registry Root",
    size = "640x480"
)]
fn install_root(_: &mut App) {
    INSTALLS.fetch_add(1, Ordering::Relaxed);
}

#[frame_capture_routes_bevy::capture_route(
    id = "registry/review",
    title = "Registry Review",
    width = 320,
    height = 240
)]
fn install_review(_: &mut App) {}

#[frame_capture_routes_bevy::capture_route]
fn registry_conventional(_: &mut App) {}

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
    let mut app = App::new();
    let before = INSTALLS.load(Ordering::Relaxed);
    route.install(&mut app);

    assert_eq!(INSTALLS.load(Ordering::Relaxed), before + 1);
}

#[test]
fn env_reads_registered_session() {
    let session = CaptureEnv::with_prefix("FRAME_CAPTURE_ROUTES_BEVY_TEST")
        .read_registered_session(&route_id("registry/root"))
        .unwrap();

    assert_eq!(session.spec().id(), "registry/root");
    assert!(!session.is_capture());
    session.install(&mut App::new());
}

#[test]
fn typed_route_key_reads_registered_session() {
    let route = registered_route_for_key::<RegistryRootRoute>().unwrap();
    assert_eq!(route.spec().id(), "registry/root");

    let route = registered_route_for_key::<RegistryConventionalRoute>().unwrap();
    assert_eq!(route.spec().id(), "registry_conventional");

    let session = CaptureEnv::with_prefix("FRAME_CAPTURE_ROUTES_BEVY_TEST")
        .read_registered_session_for::<RegistryRootRoute>()
        .unwrap();

    assert_eq!(session.spec().id(), "registry/root");
}

#[test]
fn validates_registered_route_ids_are_unique() {
    frame_capture_routes_bevy::validate_registered_routes().unwrap();
}

#[test]
fn manually_constructed_bevy_route_exposes_spec_and_installer() {
    let route = RegisteredRoute::new(
        RouteSpec::new("manual", "Manual", PixelSize::new(40, 30)),
        |_: &mut App| {},
    );
    assert_eq!(route.spec().id(), "manual");
    route.install(&mut App::new());
}

#[test]
fn bevy_registered_capture_session_exposes_capture_config() {
    let env = CaptureEnv::with_prefix("FRAME_CAPTURE_ROUTES_BEVY_CAPTURE_TEST");
    unsafe {
        std::env::set_var(env.path_var(), "capture.png");
        std::env::set_var(env.frame_var(), "2");
    }

    let session = env
        .read_registered_session(&route_id("registry/root"))
        .unwrap();
    assert!(session.is_capture());
    assert_eq!(session.route().spec().id(), "registry/root");
    assert_eq!(session.capture().unwrap().frame().get(), 2);
    assert!(
        env.read_registered_capture(&route_id("registry/root"))
            .unwrap()
            .is_some()
    );
    assert!(
        env.read_registered_capture_for::<RegistryRootRoute>()
            .unwrap()
            .is_some()
    );
    assert_eq!(session.into_capture().unwrap().frame().get(), 2);

    unsafe {
        std::env::remove_var(env.path_var());
        std::env::remove_var(env.frame_var());
    }
}

fn route_id(value: &str) -> CaptureRouteId {
    CaptureRouteId::new(value).unwrap()
}
