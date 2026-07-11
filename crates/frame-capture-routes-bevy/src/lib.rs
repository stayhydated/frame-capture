//! Bevy `App` route-registration facade for `frame-capture`.
//!
//! This crate owns the Bevy-specific route registry surface: registered routes
//! install themselves through `fn(&mut App)`. Use it for host-owned Bevy capture
//! pipelines, or through the `registry` feature of `frame-capture-bevy` when the
//! app needs the shared Bevy screenshot runtime.

pub use bevy_app::App;
pub use frame_capture::{
    CaptureConfig, CaptureEnv, CaptureEnvBuilder, CaptureEnvVar, CaptureFrame, CaptureFrameGate,
    CaptureItemSpec, CaptureItemVariant, CaptureLaunchEnv, CaptureLaunchEnvBuilder,
    CaptureLaunchEnvError, CaptureLaunchEnvVar, CaptureOutputName, CaptureOutputPath,
    CaptureOutputRoot, CaptureOutputStem, CapturePixelSizeInfo, CaptureRoute, CaptureRouteCatalog,
    CaptureRouteId, CaptureRouteIdRef, CaptureRouteInfo, CaptureRouteSource, CaptureRouteVariant,
    CaptureScenario, CaptureScenarioId, CaptureScenarioIdRef, CaptureSession, CaptureStateId,
    CaptureStateIdRef, NoCaptureScenario, ParseCaptureEnvVarError, ParseCaptureFrameError,
    ParseCaptureRouteIdError, ParseCaptureStateIdError, ParseRouteError, ParseScenarioError,
    PixelSize, RouteSpec, capture_output_path_for_name, capture_output_path_for_stem,
    capture_route_catalog, capture_route_infos,
};
#[cfg(feature = "macros")]
pub use frame_capture_macros::routes_bevy_capture_route as capture_route;
pub use frame_capture_routes::{
    CaptureRouteRegistration, RegisteredRouteError, RegisteredRouteKey,
};

#[doc(hidden)]
pub mod __private {
    pub use inventory;
}

pub mod prelude {
    pub use crate::{
        App, BevyCaptureRegistryEnvExt, CaptureConfig, CaptureEnv, CaptureEnvBuilder,
        CaptureEnvVar, CaptureFrame, CaptureFrameGate, CaptureItemSpec, CaptureItemVariant,
        CaptureLaunchEnv, CaptureLaunchEnvBuilder, CaptureLaunchEnvError, CaptureLaunchEnvVar,
        CaptureOutputName, CaptureOutputPath, CaptureOutputRoot, CaptureOutputStem,
        CapturePixelSizeInfo, CaptureRoute, CaptureRouteCatalog, CaptureRouteId, CaptureRouteIdRef,
        CaptureRouteInfo, CaptureRouteSource, CaptureRouteVariant, CaptureScenario,
        CaptureScenarioId, CaptureScenarioIdRef, CaptureSession, CaptureStateId, CaptureStateIdRef,
        NoCaptureScenario, ParseCaptureEnvVarError, ParseCaptureFrameError, ParseRouteError,
        PixelSize, RegisteredCaptureSession, RegisteredRoute, RegisteredRouteKey, RouteSpec,
        capture_output_path_for_name, capture_output_path_for_stem, capture_route_catalog,
        capture_route_infos, registered_route, registered_route_for_key, registered_routes,
        validate_registered_routes,
    };

    #[cfg(feature = "macros")]
    pub use crate::capture_route;
}

#[derive(Clone, Copy)]
pub struct RegisteredRoute {
    spec: RouteSpec,
    install: fn(&mut App),
}

inventory::collect!(RegisteredRoute);

#[derive(Clone)]
pub struct RegisteredCaptureSession {
    inner: frame_capture_routes::RegisteredCaptureSessionFor<RegisteredRoute>,
}

pub trait BevyCaptureRegistryEnvExt {
    /// Reads a Bevy registered route session from capture env vars.
    ///
    /// # Errors
    ///
    /// Returns [`RegisteredRouteError`] when route env vars are invalid, the
    /// selected route is missing, duplicate route ids exist, or capture settings
    /// are invalid.
    fn read_registered_session(
        &self,
        default_route: &CaptureRouteId,
    ) -> Result<RegisteredCaptureSession, RegisteredRouteError>;

    /// Reads only the Bevy registered capture config when capture mode is requested.
    ///
    /// # Errors
    ///
    /// Returns [`RegisteredRouteError`] when route lookup or capture settings
    /// are invalid.
    fn read_registered_capture(
        &self,
        default_route: &CaptureRouteId,
    ) -> Result<Option<CaptureConfig>, RegisteredRouteError>;

    /// Reads a Bevy registered route session using a typed route key.
    ///
    /// # Errors
    ///
    /// Returns [`RegisteredRouteError`] when the route key is invalid, route env
    /// vars are invalid, the selected route is missing, duplicate route ids
    /// exist, or capture settings are invalid.
    fn read_registered_session_for<K: RegisteredRouteKey>(
        &self,
    ) -> Result<RegisteredCaptureSession, RegisteredRouteError>;

    /// Reads only the Bevy registered capture config for a typed route key.
    ///
    /// # Errors
    ///
    /// Returns [`RegisteredRouteError`] when the route key, route lookup, or
    /// capture settings are invalid.
    fn read_registered_capture_for<K: RegisteredRouteKey>(
        &self,
    ) -> Result<Option<CaptureConfig>, RegisteredRouteError>;
}

impl RegisteredRoute {
    pub const fn new(spec: RouteSpec, install: fn(&mut App)) -> Self {
        Self { spec, install }
    }

    pub fn spec(&self) -> RouteSpec {
        self.spec
    }

    pub fn install(&self, app: &mut App) {
        (self.install)(app);
    }
}

impl CaptureRouteRegistration for RegisteredRoute {
    fn spec(&self) -> RouteSpec {
        self.spec
    }
}

impl RegisteredCaptureSession {
    pub fn route(&self) -> &'static RegisteredRoute {
        self.inner.route()
    }

    pub fn capture(&self) -> Option<&CaptureConfig> {
        self.inner.capture()
    }

    pub fn into_capture(self) -> Option<CaptureConfig> {
        self.inner.into_capture()
    }

    pub fn install(&self, app: &mut App) {
        self.route().install(app);
    }

    pub fn spec(&self) -> RouteSpec {
        self.inner.spec()
    }

    pub fn is_capture(&self) -> bool {
        self.inner.is_capture()
    }
}

impl BevyCaptureRegistryEnvExt for CaptureEnv {
    fn read_registered_session(
        &self,
        default_route: &CaptureRouteId,
    ) -> Result<RegisteredCaptureSession, RegisteredRouteError> {
        let inner = frame_capture_routes::read_registered_session_for::<RegisteredRoute>(
            self,
            default_route,
        )?;

        Ok(RegisteredCaptureSession { inner })
    }

    fn read_registered_capture(
        &self,
        default_route: &CaptureRouteId,
    ) -> Result<Option<CaptureConfig>, RegisteredRouteError> {
        frame_capture_routes::read_registered_capture_for::<RegisteredRoute>(self, default_route)
    }

    fn read_registered_session_for<K: RegisteredRouteKey>(
        &self,
    ) -> Result<RegisteredCaptureSession, RegisteredRouteError> {
        self.read_registered_session(&frame_capture_routes::registered_route_key_id::<K>()?)
    }

    fn read_registered_capture_for<K: RegisteredRouteKey>(
        &self,
    ) -> Result<Option<CaptureConfig>, RegisteredRouteError> {
        frame_capture_routes::read_registered_capture_for::<RegisteredRoute>(
            self,
            &frame_capture_routes::registered_route_key_id::<K>()?,
        )
    }
}

pub fn registered_routes() -> Vec<&'static RegisteredRoute> {
    frame_capture_routes::registered_routes_for::<RegisteredRoute>()
}

/// Validates the Bevy registered-route inventory.
///
/// # Errors
///
/// Returns [`RegisteredRouteError::DuplicateRoute`] when a route id appears
/// more than once.
pub fn validate_registered_routes() -> Result<(), RegisteredRouteError> {
    frame_capture_routes::validate_registered_routes_for::<RegisteredRoute>()
}

/// Looks up a Bevy registered route by id.
///
/// # Errors
///
/// Returns [`RegisteredRouteError::MissingRoute`] when the id is absent or
/// [`RegisteredRouteError::DuplicateRoute`] when the id is registered more than
/// once.
pub fn registered_route(
    id: &CaptureRouteId,
) -> Result<&'static RegisteredRoute, RegisteredRouteError> {
    frame_capture_routes::registered_route_for::<RegisteredRoute>(id)
}

/// Looks up a Bevy registered route by typed route key.
///
/// # Errors
///
/// Returns [`RegisteredRouteError`] when the key id is invalid, missing, or
/// registered more than once.
pub fn registered_route_for_key<K>() -> Result<&'static RegisteredRoute, RegisteredRouteError>
where
    K: RegisteredRouteKey,
{
    registered_route(&frame_capture_routes::registered_route_key_id::<K>()?)
}
