use super::{super::*, support::*};

#[test]
fn capture_env_reads_capture_defaults_overrides_and_errors() {
    let env = env("FRAME_CAPTURE_ENV_CONFIG_TEST");
    clear(&env);
    let default_size = PixelSize::new(640, 480);
    assert!(!env.is_capture_requested());
    assert_eq!(env.read_capture(default_size).unwrap(), None);

    unsafe { std::env::set_var(env.path_var(), "capture.png") };
    assert!(env.is_capture_requested());
    let capture = env.read_capture(default_size).unwrap().unwrap();
    assert_eq!(capture.frame(), DEFAULT_CAPTURE_FRAME);
    assert_eq!(capture.size(), default_size);

    unsafe { std::env::set_var(env.frame_var(), "bad") };
    assert!(matches!(
        env.read_capture(default_size),
        Err(CaptureEnvError::InvalidInteger { .. })
    ));
    unsafe { std::env::set_var(env.frame_var(), "0") };
    assert!(matches!(
        env.read_capture(default_size),
        Err(CaptureEnvError::ZeroDimension { .. })
    ));
    unsafe { std::env::remove_var(env.frame_var()) };

    unsafe { std::env::set_var(env.width_var(), "320") };
    assert!(matches!(
        env.read_capture(default_size),
        Err(CaptureEnvError::PartialSize { .. })
    ));
    unsafe { std::env::set_var(env.height_var(), "bad") };
    assert!(matches!(
        env.read_capture(default_size),
        Err(CaptureEnvError::InvalidInteger { .. })
    ));
    unsafe { std::env::set_var(env.height_var(), "0") };
    assert!(matches!(
        env.read_capture(default_size),
        Err(CaptureEnvError::ZeroDimension { .. })
    ));
    unsafe { std::env::set_var(env.height_var(), "240") };
    assert_eq!(
        env.read_capture(default_size).unwrap().unwrap().size(),
        PixelSize::new(320, 240)
    );

    unsafe { std::env::set_var(env.path_var(), "capture.jpg") };
    assert!(matches!(
        env.read_capture(default_size),
        Err(CaptureEnvError::InvalidOutputPath { .. })
    ));
    clear(&env);
}

#[test]
fn capture_env_session_alias_carries_typed_inputs() {
    let env = env("FRAME_CAPTURE_ENV_SESSION_ALIAS_TEST");
    clear(&env);
    unsafe {
        std::env::set_var(env.route_var(), "review");
        std::env::set_var(env.scenario_var(), "loaded");
    }
    let session = env.read_session_with_scenario::<Route, Scenario>().unwrap();
    assert_eq!(session.route(), Route::Review);
    assert_eq!(session.scenario(), Some(Scenario::Loaded));
    assert!(!session.is_capture());
    clear(&env);
}
