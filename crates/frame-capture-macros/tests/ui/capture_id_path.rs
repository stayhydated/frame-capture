#[derive(frame_capture_macros::CaptureScenario, Clone, Copy, Eq, PartialEq)]
enum Scenario {
    #[capture_scenario(id = "states/led-on")]
    LedOn,
}

fn main() {}
