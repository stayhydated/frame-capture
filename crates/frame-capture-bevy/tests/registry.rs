#![cfg(feature = "registry")]

use bevy::prelude::*;
use frame_capture_bevy::{
    BevyCaptureRegistryEnvExt as _, CaptureEnv, CaptureRouteId, PixelSize, registered_route,
    registered_route_for_key, registered_routes,
};

#[derive(Resource)]
struct RootInstalled;

#[frame_capture_bevy::capture_route(
    id = "registry/root",
    key = RegistryRootRoute,
    title = "Registry Root",
    size = "640x480"
)]
fn install_root(app: &mut App) {
    app.insert_resource(RootInstalled);
}

#[frame_capture_bevy::capture_route(
    id = "registry/review",
    title = "Registry Review",
    width = 320,
    height = 240
)]
fn install_review(_: &mut App) {}

#[frame_capture_bevy::capture_route]
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
    route.install(&mut app);

    assert!(app.world().contains_resource::<RootInstalled>());
}

#[test]
fn typed_route_key_uses_bevy_facade_root() {
    assert_eq!(
        <RegistryRootRoute as frame_capture_bevy::RegisteredRouteKey>::ID,
        "registry/root"
    );
}

#[test]
fn bevy_facade_reexports_typed_registry_helpers() {
    let route = registered_route_for_key::<RegistryRootRoute>().unwrap();
    assert_eq!(route.spec().id(), "registry/root");

    let route = registered_route_for_key::<RegistryConventionalRoute>().unwrap();
    assert_eq!(route.spec().id(), "registry_conventional");

    let session = CaptureEnv::with_prefix("FRAME_CAPTURE_BEVY_REGISTRY_TEST")
        .read_registered_session_for::<RegistryRootRoute>()
        .unwrap();

    assert_eq!(session.spec().id(), "registry/root");
}

#[test]
fn bevy_facade_reexports_registry_validation() {
    frame_capture_bevy::validate_registered_routes().unwrap();
}

fn route_id(value: &str) -> CaptureRouteId {
    CaptureRouteId::new(value).unwrap()
}
