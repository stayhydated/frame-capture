#[derive(frame_capture_macros::CaptureScenario, Clone, Copy, Eq, PartialEq)]
enum Scenario {
    #[capture_scenario(id = "led")]
    Led,
    #[capture_scenario(id = "led")]
    Indicator,
}

fn main() {}
