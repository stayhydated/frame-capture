use super::*;

#[derive(Clone)]
pub(super) struct VariantSpec {
    pub(super) ident: Ident,
    pub(super) id: LitStr,
    pub(super) title: LitStr,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) toml_path: Option<String>,
}

pub(super) struct RouteData {
    pub(super) name: Ident,
    pub(super) generics: Generics,
    pub(super) root: TokenStream2,
    pub(super) default: Ident,
    pub(super) variants: Vec<VariantSpec>,
}

#[derive(Default)]
pub(super) struct RouteDefaults {
    pub(super) id_prefix: Option<String>,
    pub(super) size: Option<String>,
    pub(super) width: Option<u32>,
    pub(super) height: Option<u32>,
}

#[derive(Clone, Copy)]
pub(super) enum CaptureIdKind {
    Scenario,
}

pub(super) struct CaptureIdData {
    pub(super) name: Ident,
    pub(super) generics: Generics,
    pub(super) kind: CaptureIdKind,
    pub(super) root: TokenStream2,
    pub(super) variants: Vec<CaptureIdVariantSpec>,
}

pub(super) struct CaptureIdVariantSpec {
    pub(super) ident: Ident,
    pub(super) id: LitStr,
    pub(super) title: LitStr,
    pub(super) description: Option<LitStr>,
}

#[derive(Default)]
pub(super) struct CaptureIdArgs {
    pub(super) id: Option<String>,
    pub(super) title: Option<String>,
    pub(super) description: Option<String>,
}

pub(super) struct CaptureIdDeriveArgs {
    pub(super) root: TokenStream2,
}

pub(super) struct RouteSpecInput {
    pub(super) id_prefix: Option<String>,
    pub(super) id: Option<String>,
    pub(super) title: Option<String>,
    pub(super) size: RouteSizeInput,
}

pub(super) enum RouteSizeInput {
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
pub(super) struct SharedSize {
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) path: String,
}

#[derive(Default, FromMeta)]
pub(super) struct RouteArgs {
    #[darling(default)]
    pub(super) key: Option<Ident>,
    #[darling(default)]
    pub(super) id: Option<String>,
    #[darling(default)]
    pub(super) title: Option<String>,
    #[darling(default)]
    pub(super) size: Option<String>,
    #[darling(default)]
    pub(super) width: Option<u32>,
    #[darling(default)]
    pub(super) height: Option<u32>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(capture_route), supports(enum_unit))]
pub(super) struct RouteInput {
    pub(super) ident: Ident,
    pub(super) generics: Generics,
    pub(super) data: Data<RouteVariant, ()>,
    #[darling(default, rename = "crate")]
    pub(super) root: Option<SynPath>,
    #[darling(default)]
    pub(super) default: Option<Ident>,
    #[darling(default)]
    pub(super) id_prefix: Option<String>,
    #[darling(default)]
    pub(super) size: Option<String>,
    #[darling(default)]
    pub(super) width: Option<u32>,
    #[darling(default)]
    pub(super) height: Option<u32>,
}

#[derive(FromVariant)]
#[darling(attributes(capture_route), supports(unit))]
pub(super) struct RouteVariant {
    pub(super) ident: Ident,
    #[darling(default)]
    pub(super) id: Option<String>,
    #[darling(default)]
    pub(super) title: Option<String>,
    #[darling(default)]
    pub(super) size: Option<String>,
    #[darling(default)]
    pub(super) width: Option<u32>,
    #[darling(default)]
    pub(super) height: Option<u32>,
}
