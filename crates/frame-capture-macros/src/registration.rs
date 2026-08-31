use super::*;

pub(super) fn expand_registered_capture_route(
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
