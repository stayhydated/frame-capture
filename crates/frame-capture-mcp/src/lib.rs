//! MCP server helpers for exposing `frame-capture` route metadata.
//!
//! Applications instantiate a server for either an enum-backed route catalog or
//! the registered-route inventory. Servers are read-only: they list routes and
//! return route details, but do not launch captures, save screenshots, or mutate
//! application files.

use std::{error::Error, marker::PhantomData};

use frame_capture::{
    CaptureCatalogValidationError, CaptureRoute, CaptureRouteId, ParseRouteError,
    validate_capture_routes,
};
pub use frame_capture::{
    CapturePixelSizeInfo as CapturePixelSize, CaptureRouteCatalog as CaptureRoutes,
    CaptureRouteInfo,
};
use rmcp::{
    ErrorData, RoleServer, ServerHandler, ServiceExt as _,
    handler::server::{
        router::tool::ToolRouter, tool::ToolCallContext, wrapper::Json, wrapper::Parameters,
    },
    model::{
        CacheScope, CallToolRequestParams, CallToolResponse, Implementation, ListToolsResult,
        MetaObject, PaginatedRequestParams, ProtocolVersion, ServerCapabilities, ServerInfo,
    },
    schemars,
    service::RequestContext,
    tool, tool_handler, tool_router,
};
use serde::Deserialize;
use thiserror::Error;

#[derive(Clone, Debug, Deserialize, schemars::JsonSchema)]
pub struct CaptureRouteRequest {
    /// Capture route id.
    id: CaptureRouteId,
}

#[derive(Clone, Debug)]
pub struct CaptureRoutesServer<R: CaptureRoute> {
    tool_router: ToolRouter<Self>,
    route: PhantomData<R>,
}

#[derive(Clone, Debug)]
pub struct RegisteredCaptureRoutesServer {
    tool_router: ToolRouter<Self>,
}

#[derive(Debug, Error)]
pub enum CaptureMcpError {
    #[error(transparent)]
    InvalidCatalog(#[from] CaptureCatalogValidationError),
    #[error(transparent)]
    UnknownRoute(#[from] ParseRouteError),
    #[error(transparent)]
    RegisteredRoute(#[from] frame_capture_routes::RegisteredRouteError),
}

impl<R> Default for CaptureRoutesServer<R>
where
    R: CaptureRoute + Send + Sync,
{
    fn default() -> Self {
        Self::new()
    }
}

impl Default for RegisteredCaptureRoutesServer {
    fn default() -> Self {
        Self::new()
    }
}

#[tool_router]
impl<R> CaptureRoutesServer<R>
where
    R: CaptureRoute + Send + Sync,
{
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
            route: PhantomData,
        }
    }

    #[tool(description = "List capture routes and their default pixel sizes.")]
    fn list_capture_routes(&self) -> Result<Json<CaptureRoutes>, String> {
        capture_routes::<R>()
            .map(CaptureRoutes::new)
            .map(Json)
            .map_err(|error| error.to_string())
    }

    #[tool(description = "Get one capture route by id.")]
    fn get_capture_route(
        &self,
        Parameters(request): Parameters<CaptureRouteRequest>,
    ) -> Result<Json<CaptureRouteInfo>, String> {
        capture_route::<R>(&request.id)
            .map(Json)
            .map_err(|error| error.to_string())
    }
}

#[tool_router]
impl RegisteredCaptureRoutesServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "List registered capture routes and their default pixel sizes.")]
    fn list_registered_capture_routes(&self) -> Result<Json<CaptureRoutes>, String> {
        registered_capture_routes()
            .map(CaptureRoutes::new)
            .map(Json)
            .map_err(|error| error.to_string())
    }

    #[tool(description = "Get one registered capture route by id.")]
    fn get_registered_capture_route(
        &self,
        Parameters(request): Parameters<CaptureRouteRequest>,
    ) -> Result<Json<CaptureRouteInfo>, String> {
        registered_capture_route(&request.id)
            .map(Json)
            .map_err(|error| error.to_string())
    }
}

#[tool_handler]
impl<R> ServerHandler for CaptureRoutesServer<R>
where
    R: CaptureRoute + Send + Sync,
{
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let context = ToolCallContext::new(self, request, context);
        let mut response = self.tool_router.call(context).await?;
        if let CallToolResponse::Complete(result) = &mut response {
            result.meta = Some(server_result_meta("frame-capture"));
        }
        Ok(response)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(tool_list_result(
            &self.tool_router,
            context.protocol_version().as_ref(),
            "frame-capture",
        ))
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2026_07_28)
            .with_server_info(Implementation::new(
                "frame-capture",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions("Expose frame-capture route metadata.")
    }
}

#[tool_handler]
impl ServerHandler for RegisteredCaptureRoutesServer {
    async fn call_tool(
        &self,
        request: CallToolRequestParams,
        context: RequestContext<RoleServer>,
    ) -> Result<CallToolResponse, ErrorData> {
        let context = ToolCallContext::new(self, request, context);
        let mut response = self.tool_router.call(context).await?;
        if let CallToolResponse::Complete(result) = &mut response {
            result.meta = Some(server_result_meta("frame-capture-registered-routes"));
        }
        Ok(response)
    }

    async fn list_tools(
        &self,
        _request: Option<PaginatedRequestParams>,
        context: RequestContext<RoleServer>,
    ) -> Result<ListToolsResult, ErrorData> {
        Ok(tool_list_result(
            &self.tool_router,
            context.protocol_version().as_ref(),
            "frame-capture-registered-routes",
        ))
    }

    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_protocol_version(ProtocolVersion::V_2026_07_28)
            .with_server_info(Implementation::new(
                "frame-capture-registered-routes",
                env!("CARGO_PKG_VERSION"),
            ))
            .with_instructions("Expose registered frame-capture route metadata.")
    }
}

fn tool_list_result<S>(
    tool_router: &ToolRouter<S>,
    protocol_version: Option<&ProtocolVersion>,
    server_name: &str,
) -> ListToolsResult
where
    S: Send + Sync + 'static,
{
    let mut result = ListToolsResult::with_all_items(tool_router.list_all());
    if protocol_version.is_some_and(|version| version >= &ProtocolVersion::V_2026_07_28) {
        result.ttl_ms = Some(0);
        result.cache_scope = Some(CacheScope::Private);
    }
    result.meta = Some(server_result_meta(server_name));
    result
}

fn server_result_meta(server_name: &str) -> MetaObject {
    let mut meta = MetaObject::default();
    meta.0.insert(
        "io.modelcontextprotocol/serverInfo".to_string(),
        rmcp::serde_json::json!({
            "name": server_name,
            "version": env!("CARGO_PKG_VERSION"),
        }),
    );
    meta
}

/// Lists route metadata for an enum-backed route catalog.
///
/// # Errors
///
/// Returns [`CaptureMcpError`] when the route catalog fails validation.
pub fn capture_routes<R: CaptureRoute>() -> Result<Vec<CaptureRouteInfo>, CaptureMcpError> {
    validate_capture_routes::<R>()?;
    Ok(frame_capture::capture_route_infos::<R>())
}

/// Looks up one enum-backed route by id.
///
/// # Errors
///
/// Returns [`CaptureMcpError`] when the route catalog fails validation or `id`
/// is not one of the catalog route ids.
pub fn capture_route<R: CaptureRoute>(
    id: &CaptureRouteId,
) -> Result<CaptureRouteInfo, CaptureMcpError> {
    validate_capture_routes::<R>()?;
    Ok(CaptureRouteInfo::from(R::from_id(id.as_str())?.spec()))
}

/// Lists route metadata for the registered-route inventory.
///
/// # Errors
///
/// Returns [`CaptureMcpError`] when registered route ids are duplicated.
pub fn registered_capture_routes() -> Result<Vec<CaptureRouteInfo>, CaptureMcpError> {
    frame_capture_routes::validate_registered_routes()?;
    Ok(frame_capture_routes::registered_routes()
        .into_iter()
        .map(|route| CaptureRouteInfo::from(route.spec()))
        .collect())
}

/// Looks up one registered route by id.
///
/// # Errors
///
/// Returns [`CaptureMcpError`] when registered route ids are duplicated or `id`
/// is not registered.
pub fn registered_capture_route(id: &CaptureRouteId) -> Result<CaptureRouteInfo, CaptureMcpError> {
    frame_capture_routes::validate_registered_routes()?;
    Ok(CaptureRouteInfo::from(
        frame_capture_routes::registered_route(id)?.spec(),
    ))
}

/// Serves enum-backed route metadata over stdio.
///
/// # Errors
///
/// Returns an error when stdio transport setup, request handling, or service
/// shutdown fails.
pub async fn serve_capture_routes_stdio<R>() -> Result<(), Box<dyn Error + Send + Sync>>
where
    R: CaptureRoute + Send + Sync,
{
    let service = CaptureRoutesServer::<R>::new()
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;

    Ok(())
}

/// Serves registered route metadata over stdio.
///
/// # Errors
///
/// Returns an error when stdio transport setup, request handling, or service
/// shutdown fails.
pub async fn serve_registered_capture_routes_stdio() -> Result<(), Box<dyn Error + Send + Sync>> {
    let service = RegisteredCaptureRoutesServer::new()
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use frame_capture::{CaptureRoute as _, PixelSize};

    use super::*;

    #[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
    #[capture_route(default = Root, size = "640x480")]
    enum Route {
        Root,
        Review,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum InvalidRoute {
        Root,
    }

    impl frame_capture::CaptureRoute for InvalidRoute {
        const DEFAULT: Self = Self::Root;
        const ROUTES: &'static [Self] = &[];
        const VARIANTS: &'static [frame_capture::RouteSpec] = &[frame_capture::RouteSpec::new(
            "root",
            "Root",
            PixelSize::new(10, 10),
        )];
        const ROUTE_SPECS: &'static [frame_capture::CaptureRouteVariant<Self>] =
            &[frame_capture::CaptureRouteVariant {
                route: Self::Root,
                spec: Self::VARIANTS[0],
            }];

        fn spec(self) -> frame_capture::RouteSpec {
            Self::VARIANTS[0]
        }

        fn from_id(value: &str) -> Result<Self, frame_capture::ParseRouteError> {
            (value == "root")
                .then_some(Self::Root)
                .ok_or_else(|| frame_capture::ParseRouteError::new(value, ["root"]))
        }
    }

    #[frame_capture_routes::capture_route(
        id = "registered/unit",
        title = "Registered Unit",
        size = "320x200"
    )]
    fn install_registered_unit() {}

    #[test]
    fn enum_server_tools_list_lookup_and_report_unknown_routes() {
        let server = CaptureRoutesServer::<Route>::default();
        let Json(routes) = server.list_capture_routes().unwrap();
        assert_eq!(routes.routes().len(), 2);

        let Json(route) = server
            .get_capture_route(Parameters(CaptureRouteRequest {
                id: CaptureRouteId::new("review").unwrap(),
            }))
            .unwrap();
        assert_eq!(route.id(), Route::Review.id());
        assert_eq!(route.default_size().width(), 640);

        let error = server
            .get_capture_route(Parameters(CaptureRouteRequest {
                id: CaptureRouteId::new("missing").unwrap(),
            }))
            .err()
            .unwrap();
        assert!(error.contains("unknown capture route `missing`"));

        let info = ServerHandler::get_info(&server);
        assert_eq!(
            info.instructions.as_deref(),
            Some("Expose frame-capture route metadata.")
        );
        assert_eq!(info.protocol_version, ProtocolVersion::V_2026_07_28);
        assert_eq!(info.server_info.name, "frame-capture");

        let error = CaptureRoutesServer::<InvalidRoute>::default()
            .list_capture_routes()
            .err()
            .unwrap();
        assert!(error.contains("capture route default `root` is missing from ROUTES"));
        assert_eq!(InvalidRoute::Root.spec().id(), "root");
        assert_eq!(InvalidRoute::from_id("root"), Ok(InvalidRoute::Root));
        assert!(InvalidRoute::from_id("missing").is_err());
    }

    #[test]
    fn registered_server_tools_list_lookup_and_report_unknown_routes() {
        let server = RegisteredCaptureRoutesServer::default();
        let Json(routes) = server.list_registered_capture_routes().unwrap();
        assert!(routes.routes().iter().any(|route| {
            route.id() == "registered/unit"
                && route.default_size().width() == PixelSize::new(320, 200).width()
        }));

        let Json(route) = server
            .get_registered_capture_route(Parameters(CaptureRouteRequest {
                id: CaptureRouteId::new("registered/unit").unwrap(),
            }))
            .unwrap();
        assert_eq!(route.title(), "Registered Unit");

        let error = server
            .get_registered_capture_route(Parameters(CaptureRouteRequest {
                id: CaptureRouteId::new("registered/missing").unwrap(),
            }))
            .err()
            .unwrap();
        assert!(error.contains("registered capture route `registered/missing` was not found"));

        let info = ServerHandler::get_info(&server);
        assert_eq!(
            info.instructions.as_deref(),
            Some("Expose registered frame-capture route metadata.")
        );
        assert_eq!(info.protocol_version, ProtocolVersion::V_2026_07_28);
        assert_eq!(info.server_info.name, "frame-capture-registered-routes");
    }
}
