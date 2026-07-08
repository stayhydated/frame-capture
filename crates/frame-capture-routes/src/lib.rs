//! Route-only capture facade for `frame-capture`.
//!
//! Use this crate directly for render stacks such as egui and GPUI where route
//! registration does not need framework-specific types. Framework facades with
//! custom installer signatures, such as Bevy, can reuse the generic helpers.

use frame_capture::CaptureEnvError;
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
pub use frame_capture_macros::routes_capture_route as capture_route;
#[cfg(feature = "macros")]
pub use frame_capture_macros::{CaptureRouteRoutes, CaptureScenarioRoutes};
use thiserror::Error;

#[doc(hidden)]
pub mod __private {
    pub use inventory;
}

pub mod prelude {
    pub use crate::{
        CaptureConfig, CaptureEnv, CaptureEnvBuilder, CaptureEnvVar, CaptureFrame,
        CaptureFrameGate, CaptureItemSpec, CaptureItemVariant, CaptureLaunchEnv,
        CaptureLaunchEnvBuilder, CaptureLaunchEnvError, CaptureLaunchEnvVar, CaptureOutputName,
        CaptureOutputPath, CaptureOutputRoot, CaptureOutputStem, CapturePixelSizeInfo,
        CaptureRoute, CaptureRouteCatalog, CaptureRouteId, CaptureRouteIdRef, CaptureRouteInfo,
        CaptureRouteSource, CaptureRouteVariant, CaptureRoutesEnvExt, CaptureScenario,
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
    install: fn(),
}

inventory::collect!(RegisteredRoute);

#[derive(Clone)]
pub struct RegisteredCaptureSession {
    inner: RegisteredCaptureSessionFor<RegisteredRoute>,
}

pub trait CaptureRouteRegistration: Sync + 'static {
    fn spec(&self) -> RouteSpec;
}

pub trait RegisteredRouteKey {
    const ID: &'static str;
}

#[derive(Clone)]
pub struct RegisteredCaptureSessionFor<R: CaptureRouteRegistration> {
    route: &'static R,
    capture: Option<CaptureConfig>,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum RegisteredRouteError {
    #[error("{0}")]
    Env(#[from] CaptureEnvError),
    #[error("{var}: {source}")]
    InvalidRouteId {
        var: String,
        source: ParseCaptureRouteIdError,
    },
    #[error("{var}: {source}")]
    InvalidRoute {
        var: String,
        source: ParseRouteError,
    },
    #[error("registered capture route `{id}` was not found")]
    MissingRoute { id: String },
    #[error("registered capture route `{id}` was registered more than once")]
    DuplicateRoute { id: String },
}

pub trait CaptureRoutesEnvExt {
    /// Reads a registered route session from capture env vars.
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

    /// Reads only the registered capture config when capture mode is requested.
    ///
    /// # Errors
    ///
    /// Returns [`RegisteredRouteError`] when route lookup or capture settings
    /// are invalid.
    fn read_registered_capture(
        &self,
        default_route: &CaptureRouteId,
    ) -> Result<Option<CaptureConfig>, RegisteredRouteError>;

    /// Reads a registered route session using a typed route key as the default.
    ///
    /// # Errors
    ///
    /// Returns [`RegisteredRouteError`] when the route key is invalid, route env
    /// vars are invalid, the selected route is missing, duplicate route ids
    /// exist, or capture settings are invalid.
    fn read_registered_session_for<K: RegisteredRouteKey>(
        &self,
    ) -> Result<RegisteredCaptureSession, RegisteredRouteError>;

    /// Reads only the registered capture config for a typed route key.
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
    pub const fn new(spec: RouteSpec, install: fn()) -> Self {
        Self { spec, install }
    }

    pub fn spec(&self) -> RouteSpec {
        self.spec
    }

    pub fn install(&self) {
        (self.install)();
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

    pub fn install(&self) {
        self.route().install();
    }

    pub fn spec(&self) -> RouteSpec {
        self.inner.spec()
    }

    pub fn is_capture(&self) -> bool {
        self.inner.is_capture()
    }
}

impl<R: CaptureRouteRegistration> RegisteredCaptureSessionFor<R> {
    pub fn route(&self) -> &'static R {
        self.route
    }

    pub fn capture(&self) -> Option<&CaptureConfig> {
        self.capture.as_ref()
    }

    pub fn into_capture(self) -> Option<CaptureConfig> {
        self.capture
    }

    pub fn spec(&self) -> RouteSpec {
        self.route.spec()
    }

    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }
}

impl CaptureRoutesEnvExt for CaptureEnv {
    fn read_registered_session(
        &self,
        default_route: &CaptureRouteId,
    ) -> Result<RegisteredCaptureSession, RegisteredRouteError> {
        let inner = read_registered_session_for::<RegisteredRoute>(self, default_route)?;

        Ok(RegisteredCaptureSession { inner })
    }

    fn read_registered_capture(
        &self,
        default_route: &CaptureRouteId,
    ) -> Result<Option<CaptureConfig>, RegisteredRouteError> {
        read_registered_capture_for::<RegisteredRoute>(self, default_route)
    }

    fn read_registered_session_for<K: RegisteredRouteKey>(
        &self,
    ) -> Result<RegisteredCaptureSession, RegisteredRouteError> {
        self.read_registered_session(&registered_route_key_id::<K>()?)
    }

    fn read_registered_capture_for<K: RegisteredRouteKey>(
        &self,
    ) -> Result<Option<CaptureConfig>, RegisteredRouteError> {
        read_registered_capture_for::<RegisteredRoute>(self, &registered_route_key_id::<K>()?)
    }
}

pub fn registered_routes() -> Vec<&'static RegisteredRoute> {
    registered_routes_for::<RegisteredRoute>()
}

/// Looks up a registered route by id.
///
/// # Errors
///
/// Returns [`RegisteredRouteError::MissingRoute`] when the id is absent or
/// [`RegisteredRouteError::DuplicateRoute`] when the id is registered more than
/// once.
pub fn registered_route(
    id: &CaptureRouteId,
) -> Result<&'static RegisteredRoute, RegisteredRouteError> {
    registered_route_for::<RegisteredRoute>(id)
}

/// Looks up a registered route by typed route key.
///
/// # Errors
///
/// Returns [`RegisteredRouteError`] when the key id is invalid, missing, or
/// registered more than once.
pub fn registered_route_for_key<K: RegisteredRouteKey>()
-> Result<&'static RegisteredRoute, RegisteredRouteError> {
    registered_route(&registered_route_key_id::<K>()?)
}

pub fn registered_routes_for<R>() -> Vec<&'static R>
where
    R: CaptureRouteRegistration + inventory::Collect,
{
    let mut routes = inventory::iter::<R>.into_iter().collect::<Vec<_>>();
    routes.sort_by_key(|route| route.spec().id());
    routes
}

/// Validates the global registered-route inventory.
///
/// # Errors
///
/// Returns [`RegisteredRouteError::DuplicateRoute`] when a route id appears
/// more than once.
pub fn validate_registered_routes() -> Result<(), RegisteredRouteError> {
    validate_registered_routes_for::<RegisteredRoute>()
}

/// Validates a typed registered-route inventory.
///
/// # Errors
///
/// Returns [`RegisteredRouteError::DuplicateRoute`] when a route id appears
/// more than once.
pub fn validate_registered_routes_for<R>() -> Result<(), RegisteredRouteError>
where
    R: CaptureRouteRegistration + inventory::Collect,
{
    let mut previous = None;
    for route in registered_routes_for::<R>() {
        let id = route.spec().id();
        if previous == Some(id) {
            return Err(RegisteredRouteError::DuplicateRoute { id: id.to_owned() });
        }
        previous = Some(id);
    }

    Ok(())
}

/// Looks up a typed registered route by id.
///
/// # Errors
///
/// Returns [`RegisteredRouteError::MissingRoute`] when the id is absent or
/// [`RegisteredRouteError::DuplicateRoute`] when the id is registered more than
/// once.
pub fn registered_route_for<R>(id: &CaptureRouteId) -> Result<&'static R, RegisteredRouteError>
where
    R: CaptureRouteRegistration + inventory::Collect,
{
    let mut routes = registered_routes_for::<R>()
        .into_iter()
        .filter(|route| route.spec().id() == id.as_str());
    let Some(route) = routes.next() else {
        return Err(RegisteredRouteError::MissingRoute {
            id: id.as_str().to_owned(),
        });
    };
    if routes.next().is_some() {
        return Err(RegisteredRouteError::DuplicateRoute {
            id: id.as_str().to_owned(),
        });
    }

    Ok(route)
}

/// Reads a typed registered route session from capture env vars.
///
/// # Errors
///
/// Returns [`RegisteredRouteError`] when route env vars are invalid, the
/// selected route is missing, duplicate route ids exist, or capture settings are
/// invalid.
pub fn read_registered_session_for<R>(
    env: &CaptureEnv,
    default_route: &CaptureRouteId,
) -> Result<RegisteredCaptureSessionFor<R>, RegisteredRouteError>
where
    R: CaptureRouteRegistration + inventory::Collect,
{
    let (route_id, source) = env
        .read_route_id_or(default_route)
        .map_err(|error| match error {
            CaptureEnvError::InvalidRouteId { var, source } => {
                RegisteredRouteError::InvalidRouteId { var, source }
            },
            error => RegisteredRouteError::Env(error),
        })?;
    let route = registered_route_for::<R>(&route_id).map_err(|error| match error {
        RegisteredRouteError::MissingRoute { .. } if source == CaptureRouteSource::Env => {
            RegisteredRouteError::InvalidRoute {
                var: env.route_var().to_owned(),
                source: ParseRouteError::new(
                    route_id.as_str(),
                    registered_routes_for::<R>()
                        .into_iter()
                        .map(|route| route.spec().id()),
                ),
            }
        },
        error => error,
    })?;
    let capture = env.read_capture(route.spec().default_size())?;

    Ok(RegisteredCaptureSessionFor { route, capture })
}

/// Reads only the typed registered capture config when capture mode is requested.
///
/// # Errors
///
/// Returns [`RegisteredRouteError`] when route lookup or capture settings are
/// invalid.
pub fn read_registered_capture_for<R>(
    env: &CaptureEnv,
    default_route: &CaptureRouteId,
) -> Result<Option<CaptureConfig>, RegisteredRouteError>
where
    R: CaptureRouteRegistration + inventory::Collect,
{
    if !env.is_capture_requested() {
        return Ok(None);
    }

    read_registered_session_for::<R>(env, default_route)
        .map(RegisteredCaptureSessionFor::into_capture)
}

/// Validates and returns a typed registered-route key id.
///
/// # Errors
///
/// Returns [`RegisteredRouteError::InvalidRouteId`] when `K::ID` is not a
/// valid relative route id.
pub fn registered_route_key_id<K: RegisteredRouteKey>()
-> Result<CaptureRouteId, RegisteredRouteError> {
    CaptureRouteId::new(K::ID).map_err(|source| RegisteredRouteError::InvalidRouteId {
        var: "RegisteredRouteKey::ID".to_owned(),
        source,
    })
}
