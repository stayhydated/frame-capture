use std::cell::Cell;

use frame_capture_routes::{
    CaptureEnv, CaptureRoute as _, CaptureRouteId, CaptureRouteRegistration,
    CaptureRoutesEnvExt as _, CaptureScenario as _, PixelSize, RegisteredRoute,
    RegisteredRouteError, RegisteredRouteKey, RouteSpec, read_registered_session_for,
    registered_route, registered_route_for, registered_route_for_key, registered_routes,
    registered_routes_for, validate_registered_routes_for,
};

std::thread_local! {
    static INSTALLS: Cell<usize> = const { Cell::new(0) };
}

#[derive(frame_capture_routes::CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, size = "960x540")]
enum TypedRoute {
    #[capture_route(id = "typed/dashboard", title = "Typed Dashboard")]
    Dashboard,
    #[capture_route(id = "typed/review", title = "Typed Review")]
    Review,
}

#[derive(frame_capture_routes::CaptureScenarioRoutes, Clone, Copy, Debug, Eq, PartialEq)]
enum TypedScenario {
    Loaded,
}

#[frame_capture_routes::capture_route(
    id = "registry/root",
    key = RegistryRootRoute,
    title = "Registry Root",
    size = "640x480"
)]
fn install_root() {
    INSTALLS.with(|installs| installs.set(installs.get() + 1));
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

struct DuplicateRegistration(RouteSpec);

impl CaptureRouteRegistration for DuplicateRegistration {
    fn spec(&self) -> RouteSpec {
        self.0
    }
}

frame_capture_routes::__private::inventory::collect!(DuplicateRegistration);
frame_capture_routes::__private::inventory::submit! {
    DuplicateRegistration(RouteSpec::new("duplicate", "Duplicate A", PixelSize::new(10, 10)))
}
frame_capture_routes::__private::inventory::submit! {
    DuplicateRegistration(RouteSpec::new("duplicate", "Duplicate B", PixelSize::new(20, 20)))
}

struct EmptyRegistration;

impl CaptureRouteRegistration for EmptyRegistration {
    fn spec(&self) -> RouteSpec {
        unreachable!("the empty inventory has no route values")
    }
}

frame_capture_routes::__private::inventory::collect!(EmptyRegistration);

struct InvalidRouteKey;

impl RegisteredRouteKey for InvalidRouteKey {
    const ID: &'static str = "../invalid";
}

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
    assert_eq!(TypedScenario::Loaded.id(), "loaded");
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
    let before = INSTALLS.with(Cell::get);
    route.install();

    assert_eq!(INSTALLS.with(Cell::get), before + 1);
}

#[test]
fn env_reads_registered_session() {
    let session = CaptureEnv::with_prefix("FRAME_CAPTURE_REGISTRY_TEST")
        .read_registered_session(&route_id("registry/root"))
        .unwrap();

    assert_eq!(session.spec().id(), "registry/root");
    assert!(!session.is_capture());
    session.install();
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

#[test]
fn manually_constructed_route_exposes_spec_and_installer() {
    let route = RegisteredRoute::new(
        RouteSpec::new("manual", "Manual", PixelSize::new(40, 30)),
        || {},
    );
    assert_eq!(route.spec().id(), "manual");
    route.install();
}

#[test]
fn registered_capture_session_exposes_capture_config() {
    let env = CaptureEnv::with_prefix("FRAME_CAPTURE_REGISTRY_CAPTURE_TEST");
    unsafe {
        std::env::set_var(env.path_var(), "capture.png");
        std::env::set_var(env.frame_var(), "3");
        std::env::set_var(env.width_var(), "320");
        std::env::set_var(env.height_var(), "200");
    }

    let session = env
        .read_registered_session(&route_id("registry/root"))
        .unwrap();
    assert!(session.is_capture());
    assert_eq!(session.spec().id(), "registry/root");
    assert_eq!(session.route().spec().title(), "Registry Root");
    assert_eq!(session.capture().unwrap().frame().get(), 3);
    assert_eq!(session.capture().unwrap().size(), PixelSize::new(320, 200));
    assert_eq!(
        env.read_registered_capture(&route_id("registry/root"))
            .unwrap()
            .unwrap()
            .size(),
        PixelSize::new(320, 200)
    );
    assert!(
        env.read_registered_capture_for::<RegistryRootRoute>()
            .unwrap()
            .is_some()
    );
    assert_eq!(session.into_capture().unwrap().frame().get(), 3);

    for var in [
        env.path_var(),
        env.frame_var(),
        env.width_var(),
        env.height_var(),
    ] {
        unsafe { std::env::remove_var(var) };
    }
    assert_eq!(
        env.read_registered_capture(&route_id("registry/root"))
            .unwrap(),
        None
    );
}

#[test]
fn generic_registry_helpers_report_missing_duplicate_and_invalid_routes() {
    let missing = route_id("missing");
    assert!(matches!(
        registered_route(&missing),
        Err(RegisteredRouteError::MissingRoute { .. })
    ));

    assert_eq!(registered_routes_for::<DuplicateRegistration>().len(), 2);
    assert!(matches!(
        validate_registered_routes_for::<DuplicateRegistration>(),
        Err(RegisteredRouteError::DuplicateRoute { .. })
    ));
    assert!(matches!(
        registered_route_for::<DuplicateRegistration>(&route_id("duplicate")),
        Err(RegisteredRouteError::DuplicateRoute { .. })
    ));
    assert!(matches!(
        frame_capture_routes::registered_route_key_id::<InvalidRouteKey>(),
        Err(RegisteredRouteError::InvalidRouteId { .. })
    ));

    let env = CaptureEnv::with_prefix("FRAME_CAPTURE_EMPTY_REGISTRY_TEST");
    assert!(matches!(
        read_registered_session_for::<EmptyRegistration>(&env, &missing),
        Err(RegisteredRouteError::MissingRoute { .. })
    ));
}

#[test]
fn registered_env_translates_selected_route_errors() {
    let env = CaptureEnv::with_prefix("FRAME_CAPTURE_REGISTRY_ERROR_TEST");
    unsafe { std::env::set_var(env.route_var(), "registry/missing") };
    assert!(matches!(
        env.read_registered_session(&route_id("registry/root")),
        Err(RegisteredRouteError::InvalidRoute { .. })
    ));

    unsafe { std::env::set_var(env.route_var(), "../invalid") };
    assert!(matches!(
        env.read_registered_session(&route_id("registry/root")),
        Err(RegisteredRouteError::InvalidRouteId { .. })
    ));
    unsafe { std::env::remove_var(env.route_var()) };

    #[cfg(unix)]
    {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};

        unsafe { std::env::set_var(env.route_var(), OsString::from_vec(vec![0xff])) };
        assert!(matches!(
            env.read_registered_session(&route_id("registry/root")),
            Err(RegisteredRouteError::Env(_))
        ));
        unsafe { std::env::remove_var(env.route_var()) };
    }
}

fn route_id(value: &str) -> CaptureRouteId {
    CaptureRouteId::new(value).unwrap()
}
