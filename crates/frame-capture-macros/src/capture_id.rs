use super::*;

impl CaptureIdData {
    pub(super) fn from_derive_input(
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

    pub(super) fn expand(&self) -> TokenStream2 {
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

pub(super) fn validate_route_id_literal(value: &str, span: &Ident) -> Result<()> {
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
