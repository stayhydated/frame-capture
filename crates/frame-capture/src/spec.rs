use serde::Serialize;

use crate::{
    CapturePixelSizeInfo, CaptureRoute, CaptureRouteIdRef, CaptureStateIdRef,
    ParseCaptureRouteIdError, ParseCaptureStateIdError, PixelSize,
};

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct RouteSpec {
    id: CaptureRouteIdRef,
    title: &'static str,
    default_size: PixelSize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize)]
pub struct CaptureItemSpec {
    id: CaptureStateIdRef,
    title: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<&'static str>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct CaptureRouteCatalog {
    routes: Vec<CaptureRouteInfo>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct CaptureRouteInfo {
    id: String,
    title: String,
    default_size: CapturePixelSizeInfo,
}

impl RouteSpec {
    /// Creates route metadata from a static id, title, and default size.
    ///
    /// # Panics
    ///
    /// Panics when `id` is empty, absolute, contains an empty component, uses
    /// `.` or `..`, or contains a backslash. Use [`Self::try_new`] when ids are
    /// recoverable input.
    pub const fn new(id: &'static str, title: &'static str, default_size: PixelSize) -> Self {
        Self {
            id: CaptureRouteIdRef::new(id),
            title,
            default_size,
        }
    }

    /// Creates route metadata after validating the route id.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureRouteIdError`] when `id` is not a relative route
    /// id.
    pub fn try_new(
        id: &'static str,
        title: &'static str,
        default_size: PixelSize,
    ) -> Result<Self, ParseCaptureRouteIdError> {
        Ok(Self {
            id: CaptureRouteIdRef::try_new(id)?,
            title,
            default_size,
        })
    }

    pub const fn id_ref(self) -> CaptureRouteIdRef {
        self.id
    }

    pub const fn id(self) -> &'static str {
        self.id.as_str()
    }

    pub const fn title(self) -> &'static str {
        self.title
    }

    pub const fn default_size(self) -> PixelSize {
        self.default_size
    }
}

impl CaptureItemSpec {
    /// Creates scenario or state metadata from a static id and title.
    ///
    /// # Panics
    ///
    /// Panics when `id` is empty, `.`, `..`, or contains a path separator. Use
    /// [`Self::try_new`] when ids are recoverable input.
    pub const fn new(id: &'static str, title: &'static str) -> Self {
        Self {
            id: CaptureStateIdRef::new(id),
            title,
            description: None,
        }
    }

    /// Creates scenario or state metadata after validating the id.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureStateIdError`] when `id` is empty or path-like.
    pub fn try_new(
        id: &'static str,
        title: &'static str,
    ) -> Result<Self, ParseCaptureStateIdError> {
        Ok(Self {
            id: CaptureStateIdRef::try_new(id)?,
            title,
            description: None,
        })
    }

    /// Creates scenario or state metadata with a description.
    ///
    /// # Panics
    ///
    /// Panics when `id` is empty, `.`, `..`, or contains a path separator.
    pub const fn with_description(
        id: &'static str,
        title: &'static str,
        description: &'static str,
    ) -> Self {
        Self {
            id: CaptureStateIdRef::new(id),
            title,
            description: Some(description),
        }
    }

    pub const fn id_ref(self) -> CaptureStateIdRef {
        self.id
    }

    pub const fn id(self) -> &'static str {
        self.id.as_str()
    }

    pub const fn title(self) -> &'static str {
        self.title
    }

    pub const fn description(self) -> Option<&'static str> {
        self.description
    }
}

impl CaptureRouteCatalog {
    pub fn new(routes: Vec<CaptureRouteInfo>) -> Self {
        Self { routes }
    }

    pub fn from_specs(routes: impl IntoIterator<Item = RouteSpec>) -> Self {
        Self::new(routes.into_iter().map(CaptureRouteInfo::from).collect())
    }

    pub fn routes(&self) -> &[CaptureRouteInfo] {
        &self.routes
    }

    pub fn into_routes(self) -> Vec<CaptureRouteInfo> {
        self.routes
    }
}

impl CaptureRouteInfo {
    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn default_size(&self) -> &CapturePixelSizeInfo {
        &self.default_size
    }
}

impl FromIterator<RouteSpec> for CaptureRouteCatalog {
    fn from_iter<T: IntoIterator<Item = RouteSpec>>(iter: T) -> Self {
        Self::from_specs(iter)
    }
}

impl From<RouteSpec> for CaptureRouteInfo {
    fn from(spec: RouteSpec) -> Self {
        Self {
            id: spec.id().to_owned(),
            title: spec.title().to_owned(),
            default_size: CapturePixelSizeInfo::from(spec.default_size()),
        }
    }
}

pub fn capture_route_catalog<R: CaptureRoute>() -> CaptureRouteCatalog {
    CaptureRouteCatalog::from_specs(R::ROUTE_SPECS.iter().map(|variant| variant.spec))
}

pub fn capture_route_infos<R: CaptureRoute>() -> Vec<CaptureRouteInfo> {
    capture_route_catalog::<R>().into_routes()
}
