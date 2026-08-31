use super::*;

pub(super) fn capture_item_spec_tokens(
    variant: &CaptureIdVariantSpec,
    root: &TokenStream2,
) -> TokenStream2 {
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

pub(super) fn route_spec_tokens(variant: &VariantSpec, root: TokenStream2) -> TokenStream2 {
    route_spec_tokens_with_root(variant, root)
}

pub(super) fn route_spec_tokens_with_root(
    variant: &VariantSpec,
    root: TokenStream2,
) -> TokenStream2 {
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
