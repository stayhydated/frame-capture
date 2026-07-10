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

#[derive(Clone)]
struct VariantSpec {
    ident: Ident,
    id: LitStr,
    title: LitStr,
    width: u32,
    height: u32,
    toml_path: Option<String>,
}

struct RouteData {
    name: Ident,
    generics: Generics,
    root: TokenStream2,
    default: Ident,
    variants: Vec<VariantSpec>,
}

#[derive(Default)]
struct RouteDefaults {
    id_prefix: Option<String>,
    size: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Clone, Copy)]
enum CaptureIdKind {
    Scenario,
}

struct CaptureIdData {
    name: Ident,
    generics: Generics,
    kind: CaptureIdKind,
    root: TokenStream2,
    variants: Vec<CaptureIdVariantSpec>,
}

struct CaptureIdVariantSpec {
    ident: Ident,
    id: LitStr,
    title: LitStr,
    description: Option<LitStr>,
}

#[derive(Default)]
struct CaptureIdArgs {
    id: Option<String>,
    title: Option<String>,
    description: Option<String>,
}

struct CaptureIdDeriveArgs {
    root: TokenStream2,
}

struct RouteSpecInput {
    id_prefix: Option<String>,
    id: Option<String>,
    title: Option<String>,
    size: RouteSizeInput,
}

enum RouteSizeInput {
    TomlDefault,
    SizeLiteral {
        value: String,
        width: Option<u32>,
        height: Option<u32>,
    },
    Dimensions {
        width: Option<u32>,
        height: Option<u32>,
    },
}

#[derive(Clone)]
struct SharedSize {
    width: u32,
    height: u32,
    path: String,
}

#[derive(Default, FromMeta)]
struct RouteArgs {
    #[darling(default)]
    key: Option<Ident>,
    #[darling(default)]
    id: Option<String>,
    #[darling(default)]
    title: Option<String>,
    #[darling(default)]
    size: Option<String>,
    #[darling(default)]
    width: Option<u32>,
    #[darling(default)]
    height: Option<u32>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(capture_route), supports(enum_unit))]
struct RouteInput {
    ident: Ident,
    generics: Generics,
    data: Data<RouteVariant, ()>,
    #[darling(default, rename = "crate")]
    root: Option<SynPath>,
    #[darling(default)]
    default: Option<Ident>,
    #[darling(default)]
    id_prefix: Option<String>,
    #[darling(default)]
    size: Option<String>,
    #[darling(default)]
    width: Option<u32>,
    #[darling(default)]
    height: Option<u32>,
}

#[derive(FromVariant)]
#[darling(attributes(capture_route), supports(unit))]
struct RouteVariant {
    ident: Ident,
    #[darling(default)]
    id: Option<String>,
    #[darling(default)]
    title: Option<String>,
    #[darling(default)]
    size: Option<String>,
    #[darling(default)]
    width: Option<u32>,
    #[darling(default)]
    height: Option<u32>,
}

fn expand_registered_capture_route(
    attr: TokenStream2,
    item: ItemFn,
    facade: TokenStream2,
) -> Result<TokenStream2> {
    let args = NestedMeta::parse_meta_list(attr).map_err(Error::from)?;
    let mut shared_size = None;
    let args = RouteArgs::from_list(&args)?;
    let key = args.key.clone().unwrap_or_else(|| {
        let key = format!("{}Route", item.sig.ident.to_string().to_upper_camel_case());
        format_ident!("{key}", span = item.sig.ident.span())
    });
    let route = args.into_spec(&item.sig.ident, &mut shared_size)?;
    let install = &item.sig.ident;
    let spec = route_spec_tokens_with_root(&route, facade.clone());
    let dependencies = toml_dependency_tokens([route.toml_path]);
    let visibility = &item.vis;
    let id = &route.id;
    let key = quote! {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        #visibility struct #key;

        impl #facade::RegisteredRouteKey for #key {
            const ID: &'static str = #id;
        }
    };

    Ok(quote! {
        #item

        #key

        #dependencies

        #facade::__private::inventory::submit! {
            #facade::RegisteredRoute::new(#spec, #install)
        }
    })
}

impl CaptureIdData {
    fn from_derive_input(
        input: &DeriveInput,
        kind: CaptureIdKind,
        default_root: TokenStream2,
    ) -> Result<Self> {
        let SynData::Enum(data) = &input.data else {
            return Err(
                Error::custom(format!("{} can only be derived for enums", kind.name()))
                    .with_span(&input.ident),
            );
        };
        let derive_args = CaptureIdDeriveArgs::from_attrs(&input.attrs, kind, default_root)?;

        let mut accepted_ids = BTreeSet::new();
        let mut variants = Vec::with_capacity(data.variants.len());
        for variant in &data.variants {
            if !matches!(variant.fields, syn::Fields::Unit) {
                return Err(Error::custom(format!(
                    "{} can only be derived for enums with unit variants",
                    kind.name()
                ))
                .with_span(&variant.ident));
            }

            let args = CaptureIdArgs::from_attrs(&variant.attrs, kind)?;
            let id = args
                .id
                .unwrap_or_else(|| variant.ident.to_string().to_kebab_case());
            validate_capture_id_literal(&id, &variant.ident)?;
            if !accepted_ids.insert(id.clone()) {
                return Err(
                    Error::custom(format!("duplicate capture id `{id}`")).with_span(&variant.ident)
                );
            }

            variants.push(CaptureIdVariantSpec {
                ident: variant.ident.clone(),
                id: LitStr::new(&id, variant.ident.span()),
                title: LitStr::new(
                    &args
                        .title
                        .unwrap_or_else(|| variant.ident.to_string().to_upper_camel_case()),
                    variant.ident.span(),
                ),
                description: args
                    .description
                    .map(|description| LitStr::new(&description, variant.ident.span())),
            });
        }

        if variants.is_empty() {
            return Err(Error::custom(format!(
                "{} requires at least one enum variant",
                kind.name()
            ))
            .with_span(&input.ident));
        }

        Ok(Self {
            name: input.ident.clone(),
            generics: input.generics.clone(),
            kind,
            root: derive_args.root,
            variants,
        })
    }

    fn expand(&self) -> TokenStream2 {
        let name = &self.name;
        let root = &self.root;
        let trait_path = self.kind.trait_path(&self.root);
        let error_path = self.kind.error_path(&self.root);
        let variants_const = self.kind.variants_const();
        let item_variants_const = self.kind.item_variants_const();
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();
        let primary_ids = self.variants.iter().map(|variant| &variant.id);
        let item_specs = self
            .variants
            .iter()
            .map(|variant| capture_item_spec_tokens(variant, &self.root));
        let item_variants = self.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let spec = capture_item_spec_tokens(variant, &self.root);
            quote! { #root::CaptureItemVariant { value: Self::#ident, spec: #spec } }
        });
        let typed_variants = self.variants.iter().map(|variant| &variant.ident);
        let expected_ids = self.variants.iter().map(|variant| &variant.id);
        let match_ids = self.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let id = &variant.id;
            quote! { #id => Ok(Self::#ident), }
        });
        let match_specs = self.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let id = &variant.id;
            quote! { Self::#ident => #id }
        });
        let match_item_specs = self.variants.iter().enumerate().map(|(index, variant)| {
            let ident = &variant.ident;
            let index = syn::Index::from(index);
            quote! { Self::#ident => Self::SPECS[#index] }
        });

        quote! {
            impl #impl_generics #trait_path for #name #ty_generics #where_clause {
                const #variants_const: &'static [Self] = &[#(Self::#typed_variants),*];
                const VARIANTS: &'static [&'static str] = &[#(#primary_ids),*];
                const SPECS: &'static [#root::CaptureItemSpec] = &[#(#item_specs),*];
                const #item_variants_const: &'static [#root::CaptureItemVariant<Self>] = &[#(#item_variants),*];

                fn id(self) -> &'static str {
                    match self {
                        #(#match_specs),*
                    }
                }

                fn spec(self) -> #root::CaptureItemSpec {
                    match self {
                        #(#match_item_specs),*
                    }
                }

                fn from_id(value: &str) -> Result<Self, #error_path> {
                    match value {
                        #(#match_ids)*
                        _ => Err(#error_path::new(value, [#(#expected_ids),*])),
                    }
                }
            }
        }
    }
}

impl CaptureIdKind {
    fn name(self) -> &'static str {
        match self {
            Self::Scenario => "CaptureScenario",
        }
    }

    fn trait_path(self, root: &TokenStream2) -> TokenStream2 {
        match self {
            Self::Scenario => quote!(#root::CaptureScenario),
        }
    }

    fn variants_const(self) -> Ident {
        match self {
            Self::Scenario => format_ident!("SCENARIOS"),
        }
    }

    fn item_variants_const(self) -> Ident {
        match self {
            Self::Scenario => format_ident!("SCENARIO_SPECS"),
        }
    }

    fn error_path(self, root: &TokenStream2) -> TokenStream2 {
        match self {
            Self::Scenario => quote!(#root::ParseScenarioError),
        }
    }

    fn attr_name(self) -> &'static str {
        match self {
            Self::Scenario => "capture_scenario",
        }
    }
}

impl CaptureIdDeriveArgs {
    fn from_attrs(
        attrs: &[Attribute],
        kind: CaptureIdKind,
        default_root: TokenStream2,
    ) -> Result<Self> {
        let mut root = None;
        for attr in attrs
            .iter()
            .filter(|attr| attr.path().is_ident(kind.attr_name()))
        {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("crate") {
                    let path = meta.value()?.parse::<SynPath>()?;
                    if root.replace(quote!(::#path)).is_some() {
                        return Err(meta.error("duplicate `crate`"));
                    }
                    return Ok(());
                }

                Err(meta.error("expected `crate`"))
            })
            .map_err(Error::from)?;
        }

        Ok(Self {
            root: root.unwrap_or(default_root),
        })
    }
}

impl CaptureIdArgs {
    fn from_attrs(attrs: &[Attribute], kind: CaptureIdKind) -> Result<Self> {
        let mut args = Self::default();
        for attr in attrs
            .iter()
            .filter(|attr| attr.path().is_ident(kind.attr_name()))
        {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("id") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    if args.id.replace(value).is_some() {
                        return Err(meta.error("duplicate `id`"));
                    }
                    return Ok(());
                }

                if meta.path.is_ident("title") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    if args.title.replace(value).is_some() {
                        return Err(meta.error("duplicate `title`"));
                    }
                    return Ok(());
                }

                if meta.path.is_ident("description") {
                    let value = meta.value()?.parse::<LitStr>()?.value();
                    if args.description.replace(value).is_some() {
                        return Err(meta.error("duplicate `description`"));
                    }
                    return Ok(());
                }

                Err(meta.error("expected `id`, `title`, or `description`"))
            })
            .map_err(Error::from)?;
        }

        Ok(args)
    }
}

fn validate_capture_id_literal(value: &str, span: &Ident) -> Result<()> {
    if value.is_empty() {
        return Err(Error::custom("capture id must not be empty").with_span(span));
    }
    if value == "." || value == ".." || value.contains('/') || value.contains('\\') {
        return Err(Error::custom("capture id must be an id, not a path").with_span(span));
    }

    Ok(())
}

fn validate_route_id_literal(value: &str, span: &Ident) -> Result<()> {
    if value.is_empty() {
        return Err(Error::custom("route id must not be empty").with_span(span));
    }
    if value.contains('\\')
        || value.starts_with('/')
        || value.ends_with('/')
        || value
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..")
    {
        return Err(Error::custom("route id must be a relative route id").with_span(span));
    }

    Ok(())
}

impl RouteData {
    fn from_derive_input(input: &DeriveInput, default_root: TokenStream2) -> Result<Self> {
        Self::from_route_input(RouteInput::from_derive_input(input)?, default_root)
    }

    fn from_route_input(input: RouteInput, default_root: TokenStream2) -> Result<Self> {
        let name = input.ident;
        let generics = input.generics;
        let root = input
            .root
            .map(|path| quote!(::#path))
            .unwrap_or(default_root);
        let default = input.default;
        let route_variants = input.data.take_enum().ok_or_else(|| {
            Error::custom("CaptureRoute can only be derived for enums").with_span(&name)
        })?;
        let mut shared_size = None;
        let defaults = RouteDefaults {
            id_prefix: input.id_prefix,
            size: input.size,
            width: input.width,
            height: input.height,
        };
        let mut variants = Vec::with_capacity(route_variants.len());
        for variant in route_variants {
            variants.push(variant.into_spec(&defaults, &mut shared_size)?);
        }

        if variants.is_empty() {
            return Err(
                Error::custom("CaptureRoute requires at least one route variant").with_span(&name),
            );
        }

        let default = default.unwrap_or_else(|| variants[0].ident.clone());
        if !variants.iter().any(|variant| variant.ident == default) {
            return Err(
                Error::custom("default route must name one of the enum variants")
                    .with_span(&default),
            );
        }

        Ok(Self {
            name,
            generics,
            root,
            default,
            variants,
        })
    }

    fn expand_direct(&self) -> TokenStream2 {
        let root = &self.root;
        let match_ids = self.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let id = &variant.id;
            quote! { #id => Ok(Self::#ident), }
        });
        let ids = self.variants.iter().map(|variant| &variant.id);
        let capture_route_impl = self.expand_capture_route(quote! {
            match value {
                #(#match_ids)*
                _ => Err(#root::ParseRouteError::new(value, [#(#ids),*])),
            }
        });
        let dependencies = self.toml_dependency_tokens();
        let name = &self.name;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        quote! {
            #dependencies

            #capture_route_impl

            impl #impl_generics ::std::fmt::Display for #name #ty_generics #where_clause {
                fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                    formatter.write_str(#root::CaptureRoute::id(*self))
                }
            }

            impl #impl_generics ::std::str::FromStr for #name #ty_generics #where_clause {
                type Err = #root::ParseRouteError;

                fn from_str(value: &str) -> Result<Self, Self::Err> {
                    <Self as #root::CaptureRoute>::from_id(value)
                }
            }
        }
    }

    fn expand_capture_route(&self, from_id: TokenStream2) -> TokenStream2 {
        let name = &self.name;
        let root = &self.root;
        let default = &self.default;
        let (impl_generics, ty_generics, where_clause) = self.generics.split_for_impl();

        let typed_routes = self.variants.iter().map(|variant| &variant.ident);
        let route_specs = self
            .variants
            .iter()
            .map(|variant| route_spec_tokens(variant, root.clone()));
        let route_variants = self.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let spec = route_spec_tokens(variant, root.clone());
            quote! { #root::CaptureRouteVariant { route: Self::#ident, spec: #spec } }
        });
        let match_specs = self.variants.iter().map(|variant| {
            let ident = &variant.ident;
            let spec = route_spec_tokens(variant, root.clone());
            quote! { Self::#ident => #spec }
        });

        quote! {
            impl #impl_generics #root::CaptureRoute for #name #ty_generics #where_clause {
                const DEFAULT: Self = Self::#default;
                const ROUTES: &'static [Self] = &[#(Self::#typed_routes),*];
                const VARIANTS: &'static [#root::RouteSpec] = &[
                    #(#route_specs),*
                ];
                const ROUTE_SPECS: &'static [#root::CaptureRouteVariant<Self>] = &[
                    #(#route_variants),*
                ];

                fn spec(self) -> #root::RouteSpec {
                    match self {
                        #(#match_specs),*
                    }
                }

                fn from_id(value: &str) -> Result<Self, #root::ParseRouteError> {
                    #from_id
                }
            }
        }
    }

    fn toml_dependency_tokens(&self) -> TokenStream2 {
        toml_dependency_tokens(
            self.variants
                .iter()
                .map(|variant| variant.toml_path.clone()),
        )
    }
}

impl RouteArgs {
    fn into_spec(self, ident: &Ident, shared_size: &mut Option<SharedSize>) -> Result<VariantSpec> {
        RouteSpecInput {
            id_prefix: None,
            id: self.id,
            title: self.title,
            size: RouteSizeInput::from_parts(self.size, self.width, self.height),
        }
        .into_spec(ident, shared_size)
    }
}

impl RouteVariant {
    fn into_spec(
        self,
        defaults: &RouteDefaults,
        shared_size: &mut Option<SharedSize>,
    ) -> Result<VariantSpec> {
        RouteSpecInput {
            id_prefix: defaults.id_prefix.clone(),
            id: self.id,
            title: self.title,
            size: RouteSizeInput::from_parts(
                self.size.or_else(|| defaults.size.clone()),
                self.width.or(defaults.width),
                self.height.or(defaults.height),
            ),
        }
        .into_spec(&self.ident, shared_size)
    }
}

impl RouteSizeInput {
    fn from_parts(size: Option<String>, width: Option<u32>, height: Option<u32>) -> Self {
        match size {
            Some(value) => Self::SizeLiteral {
                value,
                width,
                height,
            },
            None if width.is_none() && height.is_none() => Self::TomlDefault,
            None => Self::Dimensions { width, height },
        }
    }

    fn resolve(self, shared_size: &mut Option<SharedSize>, ident: &Ident) -> Result<RouteSize> {
        match self {
            Self::TomlDefault => RouteSize::from_shared(load_shared_size(shared_size, ident)?),
            Self::SizeLiteral {
                value,
                width,
                height,
            } => {
                let parsed_size = parse_size_lit(&value, ident)?;
                RouteSize::from_parts(
                    width.or(Some(parsed_size.0)),
                    height.or(Some(parsed_size.1)),
                    None,
                    ident,
                )
            },
            Self::Dimensions { width, height } => {
                let shared_size = (width.is_none() || height.is_none())
                    .then(|| load_shared_size(shared_size, ident))
                    .transpose()?;
                RouteSize::from_parts(width, height, shared_size.as_ref(), ident)
            },
        }
    }
}

struct RouteSize {
    width: u32,
    height: u32,
    toml_path: Option<String>,
}

impl RouteSize {
    fn from_shared(shared_size: SharedSize) -> Result<Self> {
        Ok(Self {
            width: shared_size.width,
            height: shared_size.height,
            toml_path: Some(shared_size.path),
        })
    }

    fn from_parts(
        width: Option<u32>,
        height: Option<u32>,
        shared_size: Option<&SharedSize>,
        ident: &Ident,
    ) -> Result<Self> {
        let width = width
            .or_else(|| shared_size.map(|size| size.width))
            .ok_or_else(|| {
                Error::custom("route variant requires `size = \"WIDTHxHEIGHT\"` or `width = ...`")
                    .with_span(ident)
            })?;
        let height = height
            .or_else(|| shared_size.map(|size| size.height))
            .ok_or_else(|| {
                Error::custom("route variant requires `size = \"WIDTHxHEIGHT\"` or `height = ...`")
                    .with_span(ident)
            })?;

        Ok(Self {
            width: parse_u32_lit(width, ident)?,
            height: parse_u32_lit(height, ident)?,
            toml_path: shared_size.map(|size| size.path.clone()),
        })
    }
}

impl RouteSpecInput {
    fn into_spec(self, ident: &Ident, shared_size: &mut Option<SharedSize>) -> Result<VariantSpec> {
        let id = route_id_with_prefix(
            self.id.unwrap_or_else(|| ident.to_string().to_snake_case()),
            self.id_prefix,
            ident,
        )?;
        validate_route_id_literal(&id, ident)?;
        let title = self
            .title
            .unwrap_or_else(|| ident.to_string().to_upper_camel_case());
        let size = self.size.resolve(shared_size, ident)?;

        Ok(VariantSpec {
            ident: ident.clone(),
            id: LitStr::new(&id, ident.span()),
            title: LitStr::new(&title, ident.span()),
            width: size.width,
            height: size.height,
            toml_path: size.toml_path,
        })
    }
}

fn route_id_with_prefix(id: String, id_prefix: Option<String>, span: &Ident) -> Result<String> {
    let Some(id_prefix) = id_prefix else {
        return Ok(id);
    };

    validate_route_id_literal(&id_prefix, span)?;
    Ok(format!("{id_prefix}/{id}"))
}

fn load_shared_size(shared_size: &mut Option<SharedSize>, span: &Ident) -> Result<SharedSize> {
    if let Some(size) = shared_size {
        return Ok(size.clone());
    }

    let path = find_toml_path().ok_or_else(|| {
        Error::custom(format!(
            "`size` was omitted, but no `{}` file was found from CARGO_MANIFEST_DIR upward",
            frame_capture_toml::DEFAULT_FILE_NAME
        ))
        .with_span(span)
    })?;
    let source = fs::read_to_string(&path)
        .map_err(|error| Error::custom(format!("{}: {error}", path.display())).with_span(span))?;
    let config = frame_capture_toml::CaptureToml::parse(&source)
        .map_err(|error| Error::custom(format!("{}: {error}", path.display())).with_span(span))?;
    let size = SharedSize {
        width: config.default_size.width(),
        height: config.default_size.height(),
        path: path.display().to_string(),
    };
    *shared_size = Some(size.clone());

    Ok(size)
}

fn find_toml_path() -> Option<PathBuf> {
    let explicit = env::var_os("FRAME_CAPTURE_TOML").map(PathBuf::from);
    if let Some(path) = explicit.filter(|path| path.is_file()) {
        return Some(path.canonicalize().unwrap_or(path));
    }

    let manifest_dir = env::var_os("CARGO_MANIFEST_DIR")?;
    let mut current = PathBuf::from(manifest_dir);
    loop {
        let candidate = current.join(frame_capture_toml::DEFAULT_FILE_NAME);
        if candidate.is_file() {
            return Some(candidate.canonicalize().unwrap_or(candidate));
        }

        if !current.pop() {
            return None;
        }
    }
}

fn toml_dependency_tokens(paths: impl IntoIterator<Item = Option<String>>) -> TokenStream2 {
    let paths = paths
        .into_iter()
        .flatten()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .map(|path| LitStr::new(&path, proc_macro2::Span::call_site()));

    quote! {
        #(const _: &str = include_str!(#paths);)*
    }
}

fn parse_size_lit(value: &str, span: &Ident) -> Result<(u32, u32)> {
    let size = frame_capture_toml::parse_size(value).map_err(|error| {
        let message = match error {
            frame_capture_toml::ParseSizeError::MissingSeparator => "size must use WIDTHxHEIGHT",
            frame_capture_toml::ParseSizeError::InvalidWidth => {
                "size width must be a positive integer"
            },
            frame_capture_toml::ParseSizeError::InvalidHeight => {
                "size height must be a positive integer"
            },
            frame_capture_toml::ParseSizeError::ZeroWidth
            | frame_capture_toml::ParseSizeError::ZeroHeight => {
                "size dimensions must be greater than zero"
            },
        };
        Error::custom(message).with_span(span)
    })?;

    Ok((size.width(), size.height()))
}

fn parse_u32_lit(value: u32, span: &Ident) -> Result<u32> {
    if value == 0 {
        return Err(Error::custom("route dimensions must be greater than zero").with_span(span));
    }

    Ok(value)
}

fn capture_item_spec_tokens(variant: &CaptureIdVariantSpec, root: &TokenStream2) -> TokenStream2 {
    let id = &variant.id;
    let title = &variant.title;
    if let Some(description) = &variant.description {
        quote! {
            #root::CaptureItemSpec::with_description(#id, #title, #description)
        }
    } else {
        quote! {
            #root::CaptureItemSpec::new(#id, #title)
        }
    }
}

fn route_spec_tokens(variant: &VariantSpec, root: TokenStream2) -> TokenStream2 {
    route_spec_tokens_with_root(variant, root)
}

fn route_spec_tokens_with_root(variant: &VariantSpec, root: TokenStream2) -> TokenStream2 {
    let id = &variant.id;
    let title = &variant.title;
    let width = variant.width;
    let height = variant.height;

    quote! {
        #root::RouteSpec::new(
            #id,
            #title,
            #root::PixelSize::new(#width, #height),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        env, panic,
        path::{Path, PathBuf},
        process,
        sync::{Mutex, OnceLock},
        time::{SystemTime, UNIX_EPOCH},
    };

    static ENV_LOCK: OnceLock<Mutex<()>> = OnceLock::new();

    fn env_lock() -> &'static Mutex<()> {
        ENV_LOCK.get_or_init(|| Mutex::new(()))
    }

    fn with_capture_env<R, F>(manifest_dir: &Path, explicit_toml: Option<&Path>, f: F) -> R
    where
        R: std::panic::UnwindSafe,
        F: FnOnce() -> R + std::panic::UnwindSafe,
    {
        let _lock = env_lock().lock().unwrap();
        let previous_manifest = env::var_os("CARGO_MANIFEST_DIR");
        let previous_toml = env::var_os("FRAME_CAPTURE_TOML");

        unsafe {
            env::set_var("CARGO_MANIFEST_DIR", manifest_dir);
        }
        match explicit_toml {
            Some(path) => unsafe {
                env::set_var("FRAME_CAPTURE_TOML", path);
            },
            None => unsafe {
                env::remove_var("FRAME_CAPTURE_TOML");
            },
        }

        let result = panic::catch_unwind(f);

        if let Some(value) = previous_manifest {
            unsafe {
                env::set_var("CARGO_MANIFEST_DIR", value);
            }
        } else {
            unsafe {
                env::remove_var("CARGO_MANIFEST_DIR");
            }
        }

        if let Some(value) = previous_toml {
            unsafe {
                env::set_var("FRAME_CAPTURE_TOML", value);
            }
        } else {
            unsafe {
                env::remove_var("FRAME_CAPTURE_TOML");
            }
        }

        result.unwrap_or_else(|payload| panic::resume_unwind(payload))
    }

    fn write_default_size_toml(path: &Path, width: u32, height: u32) {
        let source = format!("[default_size]\nwidth = {width}\nheight = {height}\n");
        fs::write(path, source).unwrap();
    }

    fn temp_root(test_name: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();
        let dir = env::temp_dir()
            .join("frame-capture-macros-tests")
            .join(format!("{test_name}-{}-{nanos}", process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn macro_resolves_explicit_capture_toml_var() {
        let root = temp_root("explicit_capture_toml");
        let manifest_dir = root.join("manifest");
        let root_toml = root.join("frame-capture.toml");
        let explicit_toml = root.join("custom.toml");

        fs::create_dir_all(&manifest_dir).unwrap();
        write_default_size_toml(&root_toml, 1920, 1080);
        write_default_size_toml(&explicit_toml, 1600, 900);

        with_capture_env(&manifest_dir, Some(&explicit_toml), || {
            let resolved = find_toml_path().unwrap();

            assert_eq!(resolved, explicit_toml.canonicalize().unwrap());
            assert!(resolved.exists());
        });

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn macro_prefers_manifest_local_toml_over_root() {
        let root = temp_root("manifest_local_toml");
        let workspace = root.join("workspace");
        let manifest_dir = workspace.join("examples").join("bevy");
        let workspace_toml = workspace.join(frame_capture_toml::DEFAULT_FILE_NAME);
        let local_toml = manifest_dir.join(frame_capture_toml::DEFAULT_FILE_NAME);

        fs::create_dir_all(&manifest_dir).unwrap();
        write_default_size_toml(&workspace_toml, 1920, 1080);
        write_default_size_toml(&local_toml, 1365, 769);

        with_capture_env(&manifest_dir, None, || {
            let resolved = find_toml_path().unwrap();
            let mut shared = None;
            let span = Ident::new("Route", proc_macro2::Span::call_site());
            let shared_size = load_shared_size(&mut shared, &span).unwrap();

            assert_eq!(resolved, local_toml.canonicalize().unwrap());
            assert_eq!(
                shared_size.path,
                local_toml.canonicalize().unwrap().to_string_lossy()
            );
            assert_eq!(shared_size.width, 1365);
            assert_eq!(shared_size.height, 769);
        });

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn macro_falls_back_to_root_toml_when_override_missing() {
        let root = temp_root("manifest_root_toml");
        let workspace = root.join("workspace");
        let manifest_dir = workspace.join("examples").join("bevy");
        let workspace_toml = workspace.join(frame_capture_toml::DEFAULT_FILE_NAME);

        fs::create_dir_all(&manifest_dir).unwrap();
        write_default_size_toml(&workspace_toml, 1400, 780);

        with_capture_env(&manifest_dir, None, || {
            let resolved = find_toml_path().unwrap();

            assert_eq!(resolved, workspace_toml.canonicalize().unwrap());
            assert_eq!(resolved.to_string_lossy(), workspace_toml.to_string_lossy());
        });

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn macro_reports_missing_toml_and_partial_dimensions() {
        let root = temp_root("missing_toml");
        let manifest_dir = root.join("manifest");
        fs::create_dir_all(&manifest_dir).unwrap();

        with_capture_env(&manifest_dir, None, || {
            let span = Ident::new("Route", proc_macro2::Span::call_site());
            assert_eq!(find_toml_path(), None);
            assert!(load_shared_size(&mut None, &span).is_err());
            assert!(RouteSize::from_parts(None, Some(10), None, &span).is_err());
            assert!(RouteSize::from_parts(Some(10), None, None, &span).is_err());
        });

        let _ = fs::remove_dir_all(root);
    }
}
