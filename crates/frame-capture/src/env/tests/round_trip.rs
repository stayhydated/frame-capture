use super::super::*;

#[test]
fn launch_env_constructors_and_vars_cover_route_only_requests() {
    let route = CaptureRouteId::new("review").unwrap();
    let launch = CaptureLaunchEnv::new(route.clone());
    assert_eq!(launch.env(), &CaptureEnv::frame_capture());
    assert_eq!(launch.route_id(), &route);
    assert_eq!(launch.output_path(), None);
    assert_eq!(launch.frame(), None);
    assert_eq!(launch.size(), None);
    assert_eq!(launch.scenario_id(), None);
    assert_eq!(launch.vars().len(), 1);
    assert_eq!(CaptureLaunchEnv::try_new("review").unwrap(), launch);
    assert!(CaptureLaunchEnv::try_new("../review").is_err());
    assert_eq!(CaptureLaunchEnv::optional_size(None, None), Ok(None));

    let var = CaptureLaunchEnvVar::new("APP_ROUTE", "review");
    assert_eq!(var.name(), "APP_ROUTE");
    assert_eq!(var.value(), OsStr::new("review"));
    assert_eq!(
        var.into_pair(),
        ("APP_ROUTE".to_owned(), OsString::from("review"))
    );
}

#[test]
fn frame_gate_advances_once_and_latches_requests() {
    let mut gate = CaptureFrameGate::default();
    assert_eq!(gate.frame(), 0);
    assert!(!gate.requested());
    assert!(!gate.ready(CaptureFrame::new(2)));
    gate.advance();
    gate.advance();
    assert!(gate.ready(CaptureFrame::new(2)));
    gate.mark_requested();
    gate.advance();
    assert_eq!(gate.frame(), 2);
    assert!(gate.requested());
    assert!(!gate.ready(CaptureFrame::new(2)));
}
