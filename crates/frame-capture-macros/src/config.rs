use super::*;

pub(super) fn load_shared_size(
    shared_size: &mut Option<SharedSize>,
    span: &Ident,
) -> Result<SharedSize> {
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

pub(super) fn find_toml_path() -> Option<PathBuf> {
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

pub(super) fn toml_dependency_tokens(
    paths: impl IntoIterator<Item = Option<String>>,
) -> TokenStream2 {
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

pub(super) fn parse_size_lit(value: &str, span: &Ident) -> Result<(u32, u32)> {
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

pub(super) fn parse_u32_lit(value: u32, span: &Ident) -> Result<u32> {
    if value == 0 {
        return Err(Error::custom("route dimensions must be greater than zero").with_span(span));
    }

    Ok(value)
}
