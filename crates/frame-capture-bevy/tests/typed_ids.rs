use frame_capture_bevy::{CaptureRoute as _, CaptureScenario as _};

#[derive(frame_capture_bevy::CaptureRouteBevy, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Root, size = "640x480")]
enum BevyPage {
    Root,
}

#[derive(frame_capture_bevy::CaptureScenarioBevy, Clone, Copy, Debug, Eq, PartialEq)]
enum BevyScenario {
    #[capture_scenario(id = "target")]
    Target,
    Scale,
}

#[test]
fn bevy_facade_reexports_typed_capture_id_derives() {
    assert_eq!(BevyPage::Root.id(), "root");
    assert_eq!(BevyScenario::from_id("target"), Ok(BevyScenario::Target));
    assert_eq!(BevyScenario::Scale.id(), "scale");
}
