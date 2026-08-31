use std::num::NonZeroU32;

use super::{super::*, support::*};

#[test]
fn capture_frames_cover_nonzero_parse_display_and_serde_contracts() {
    let nonzero = NonZeroU32::new(3).unwrap();
    let frame = CaptureFrame::from_nonzero(nonzero);
    assert_eq!(frame.get(), 3);
    assert_eq!(frame.into_nonzero(), nonzero);
    assert_eq!(CaptureFrame::from(nonzero), frame);
    assert_eq!(NonZeroU32::from(frame), nonzero);
    assert_eq!(frame.to_string(), "3");
    assert_eq!("3".parse::<CaptureFrame>().unwrap(), frame);
    assert!(matches!(
        "three".parse::<CaptureFrame>(),
        Err(ParseCaptureFrameError::Invalid { .. })
    ));
    assert_eq!(
        "0".parse::<CaptureFrame>(),
        Err(ParseCaptureFrameError::Zero)
    );
    assert_eq!(serde_json::to_string(&frame).unwrap(), "3");
    assert!(serde_json::from_str::<CaptureFrame>("0").is_err());
    assert!(std::panic::catch_unwind(|| CaptureFrame::new(0)).is_err());
}

#[test]
fn capture_env_builder_sets_every_protocol_name() {
    let env = CaptureEnv::builder()
        .route_var("APP_ROUTE")
        .unwrap()
        .path_var("APP_PATH")
        .unwrap()
        .frame_var("APP_FRAME")
        .unwrap()
        .width_var("APP_WIDTH")
        .unwrap()
        .height_var("APP_HEIGHT")
        .unwrap()
        .scenario_var("APP_SCENARIO")
        .unwrap()
        .build();
    assert_eq!(
        [
            env.route_var(),
            env.path_var(),
            env.frame_var(),
            env.width_var(),
            env.height_var(),
            env.scenario_var()
        ],
        [
            "APP_ROUTE",
            "APP_PATH",
            "APP_FRAME",
            "APP_WIDTH",
            "APP_HEIGHT",
            "APP_SCENARIO"
        ]
    );
    assert_eq!(
        CaptureEnv::with_prefix("APP2").route_var(),
        "APP2_CAPTURE_ROUTE"
    );
    assert_eq!(CaptureEnv::default(), CaptureEnv::frame_capture());
    assert!(std::panic::catch_unwind(|| CaptureEnv::with_prefix("BAD=APP")).is_err());
}

#[test]
fn capture_env_reads_route_and_scenario_values_and_errors() {
    let env = env("FRAME_CAPTURE_ENV_IDS_TEST");
    clear(&env);
    assert_eq!(Route::Root.spec().id(), "root");
    assert_eq!(Scenario::Loaded.id(), "loaded");
    assert_eq!(env.read_route::<Route>().unwrap(), Route::Root);
    assert_eq!(env.read_scenario::<Scenario>().unwrap(), None);
    assert_eq!(env.read_scenario_id().unwrap(), None);

    unsafe { std::env::set_var(env.route_var(), "review") };
    assert_eq!(env.read_route::<Route>().unwrap(), Route::Review);
    let default = CaptureRouteId::new("root").unwrap();
    assert_eq!(
        env.read_route_id_or(&default).unwrap().1,
        CaptureRouteSource::Env
    );

    unsafe { std::env::set_var(env.route_var(), "missing") };
    assert!(matches!(
        env.read_route::<Route>(),
        Err(CaptureEnvError::InvalidRoute { .. })
    ));
    unsafe { std::env::set_var(env.route_var(), "../missing") };
    assert!(matches!(
        env.read_route_id_or(&default),
        Err(CaptureEnvError::InvalidRouteId { .. })
    ));

    unsafe { std::env::set_var(env.scenario_var(), "loaded") };
    assert_eq!(
        env.read_scenario::<Scenario>().unwrap(),
        Some(Scenario::Loaded)
    );
    assert_eq!(env.read_scenario_id().unwrap().unwrap().as_str(), "loaded");
    unsafe { std::env::set_var(env.scenario_var(), "missing") };
    assert!(matches!(
        env.read_scenario::<Scenario>(),
        Err(CaptureEnvError::InvalidScenario { .. })
    ));
    unsafe { std::env::set_var(env.scenario_var(), "states/loaded") };
    assert!(matches!(
        env.read_scenario_id(),
        Err(CaptureEnvError::InvalidStateId { .. })
    ));
    clear(&env);
}

#[cfg(unix)]
#[test]
fn capture_env_rejects_non_unicode_string_values() {
    use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};

    let env = env("FRAME_CAPTURE_ENV_UNICODE_TEST");
    clear(&env);
    unsafe { std::env::set_var(env.route_var(), OsString::from_vec(vec![0xff])) };
    assert!(matches!(
        env.read_route::<Route>(),
        Err(CaptureEnvError::NotUnicode { .. })
    ));
    clear(&env);
}
