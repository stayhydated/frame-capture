use frame_capture::{CaptureScenario};

#[derive(frame_capture_macros::CaptureScenario, Clone, Copy, Eq, PartialEq)]
enum Scenario {
    #[capture_scenario(id = "led-on")]
    LedOn,
    LedOff,
}

fn main() {
    assert_eq!(Scenario::from_id("led-on").unwrap().id(), "led-on");
    assert_eq!(Scenario::VARIANTS, ["led-on", "led-off"]);
}
