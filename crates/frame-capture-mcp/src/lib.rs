//! MCP server helpers for exposing `frame-capture` route metadata.
//!
//! The server is app-instantiated, because capture routes are normally
//! compile-time enums owned by the consuming application.
//! Servers are read-only: they list routes and return route details, but do not
//! launch captures, save screenshots, or mutate application files.

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
    ServerHandler, ServiceExt as _,
    handler::server::{router::tool::ToolRouter, wrapper::Json, wrapper::Parameters},
    model::{ServerCapabilities, ServerInfo},
    schemars, tool, tool_handler, tool_router,
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
    #[allow(dead_code)]
    tool_router: ToolRouter<Self>,
    route: PhantomData<R>,
}

#[derive(Clone, Debug)]
pub struct RegisteredCaptureRoutesServer {
    #[allow(dead_code)]
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
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Expose frame-capture route metadata.")
    }
}

#[tool_handler]
impl ServerHandler for RegisteredCaptureRoutesServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo::new(ServerCapabilities::builder().enable_tools().build())
            .with_instructions("Expose registered frame-capture route metadata.")
    }
}

pub fn capture_routes<R: CaptureRoute>() -> Result<Vec<CaptureRouteInfo>, CaptureMcpError> {
    validate_capture_routes::<R>()?;
    Ok(frame_capture::capture_route_infos::<R>())
}

pub fn capture_route<R: CaptureRoute>(
    id: &CaptureRouteId,
) -> Result<CaptureRouteInfo, CaptureMcpError> {
    validate_capture_routes::<R>()?;
    Ok(CaptureRouteInfo::from(R::from_id(id.as_str())?.spec()))
}

pub fn registered_capture_routes() -> Result<Vec<CaptureRouteInfo>, CaptureMcpError> {
    frame_capture_routes::validate_registered_routes()?;
    Ok(frame_capture_routes::registered_routes()
        .into_iter()
        .map(|route| CaptureRouteInfo::from(route.spec()))
        .collect())
}

pub fn registered_capture_route(id: &CaptureRouteId) -> Result<CaptureRouteInfo, CaptureMcpError> {
    frame_capture_routes::validate_registered_routes()?;
    Ok(CaptureRouteInfo::from(
        frame_capture_routes::registered_route(id)?.spec(),
    ))
}

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

pub async fn serve_registered_capture_routes_stdio() -> Result<(), Box<dyn Error + Send + Sync>> {
    let service = RegisteredCaptureRoutesServer::new()
        .serve(rmcp::transport::stdio())
        .await?;
    service.waiting().await?;

    Ok(())
}
