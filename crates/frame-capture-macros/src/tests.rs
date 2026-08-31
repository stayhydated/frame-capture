use super::{routes::RouteSize, *};
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
    let lock = env_lock()
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
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

    drop(lock);
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
    dir.canonicalize().unwrap()
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
