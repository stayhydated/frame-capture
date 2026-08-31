use super::*;

impl RouteData {
    pub(super) fn from_derive_input(
        input: &DeriveInput,
        default_root: TokenStream2,
    ) -> Result<Self> {
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

    pub(super) fn expand_direct(&self) -> TokenStream2 {
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
    pub(super) fn into_spec(
        self,
        ident: &Ident,
        shared_size: &mut Option<SharedSize>,
    ) -> Result<VariantSpec> {
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

pub(super) struct RouteSize {
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

    pub(super) fn from_parts(
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
