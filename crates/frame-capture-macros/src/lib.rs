//! Proc macros for `frame-capture` route, scenario, and registered-route
//! declarations.
//!
//! The runtime protocol types live in `frame-capture`. This crate parses derive
//! and attribute input, resolves route ids, titles, default sizes, generated
//! route keys, and facade paths, then emits implementations against the
//! appropriate runtime or facade crate.

use darling::{
    Error, FromDeriveInput, FromMeta, FromVariant, Result,
    ast::{Data, NestedMeta},
};
use fs_err as fs;
use heck::{ToKebabCase as _, ToSnakeCase as _, ToUpperCamelCase as _};
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use std::{collections::BTreeSet, env, path::PathBuf};
use syn::{
    Attribute, Data as SynData, DeriveInput, Generics, Ident, ItemFn, LitStr, Path as SynPath,
    parse_macro_input,
};

mod capture_id;
mod config;
mod emit;
mod model;
mod registration;
mod routes;

use capture_id::validate_route_id_literal;
use config::*;
use emit::*;
use model::*;
use registration::expand_registered_capture_route;

#[cfg(test)]
mod tests;

#[proc_macro_derive(CaptureRoute, attributes(capture_route))]
pub fn derive_capture_route(input: TokenStream) -> TokenStream {
    expand_capture_route_derive(input, None)
}

#[proc_macro_derive(CaptureRouteBevy, attributes(capture_route))]
pub fn derive_capture_route_bevy(input: TokenStream) -> TokenStream {
    expand_capture_route_derive(input, Some(quote!(::frame_capture_bevy)))
}

#[proc_macro_derive(CaptureRouteRoutes, attributes(capture_route))]
pub fn derive_capture_route_routes(input: TokenStream) -> TokenStream {
    expand_capture_route_derive(input, Some(quote!(::frame_capture_routes)))
}

fn expand_capture_route_derive(
    input: TokenStream,
    default_root: Option<TokenStream2>,
) -> TokenStream {
    let default_root = default_root.unwrap_or_else(|| quote!(::frame_capture));
    let input = parse_macro_input!(input as DeriveInput);
    RouteData::from_derive_input(&input, default_root)
        .map(|route| route.expand_direct())
        .unwrap_or_else(|error| error.write_errors())
        .into()
}

#[proc_macro_derive(CaptureScenario, attributes(capture_scenario))]
pub fn derive_capture_scenario(input: TokenStream) -> TokenStream {
    expand_capture_id_derive(input, CaptureIdKind::Scenario, None)
}

#[proc_macro_derive(CaptureScenarioBevy, attributes(capture_scenario))]
pub fn derive_capture_scenario_bevy(input: TokenStream) -> TokenStream {
    expand_capture_id_derive(
        input,
        CaptureIdKind::Scenario,
        Some(quote!(::frame_capture_bevy)),
    )
}

#[proc_macro_derive(CaptureScenarioRoutes, attributes(capture_scenario))]
pub fn derive_capture_scenario_routes(input: TokenStream) -> TokenStream {
    expand_capture_id_derive(
        input,
        CaptureIdKind::Scenario,
        Some(quote!(::frame_capture_routes)),
    )
}

#[proc_macro_attribute]
pub fn bevy_capture_route(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_registered_capture_route_attribute(attr, item, quote!(::frame_capture_bevy))
}

#[proc_macro_attribute]
pub fn routes_bevy_capture_route(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_registered_capture_route_attribute(attr, item, quote!(::frame_capture_routes_bevy))
}

#[proc_macro_attribute]
pub fn routes_capture_route(attr: TokenStream, item: TokenStream) -> TokenStream {
    expand_registered_capture_route_attribute(attr, item, quote!(::frame_capture_routes))
}

fn expand_capture_id_derive(
    input: TokenStream,
    kind: CaptureIdKind,
    default_root: Option<TokenStream2>,
) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let default_root = default_root.unwrap_or_else(|| quote!(::frame_capture));
    CaptureIdData::from_derive_input(&input, kind, default_root)
        .map(|id| id.expand())
        .unwrap_or_else(|error| error.write_errors())
        .into()
}

fn expand_registered_capture_route_attribute(
    attr: TokenStream,
    item: TokenStream,
    facade: TokenStream2,
) -> TokenStream {
    let attr = TokenStream2::from(attr);
    let item = parse_macro_input!(item as ItemFn);

    expand_registered_capture_route(attr, item, facade)
        .unwrap_or_else(|error| error.write_errors())
        .into()
}
