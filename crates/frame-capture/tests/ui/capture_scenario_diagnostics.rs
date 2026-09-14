#[derive(frame_capture_macros::CaptureScenario)]
struct ScenarioStruct;

#[derive(frame_capture_macros::CaptureScenario)]
enum ScenarioPayload {
    Value(u32),
}

#[derive(frame_capture_macros::CaptureScenario)]
enum EmptyScenario {}

#[derive(frame_capture_macros::CaptureScenario)]
#[capture_scenario(crate = frame_capture, crate = frame_capture)]
enum DuplicateCrate {
    Value,
}

#[derive(frame_capture_macros::CaptureScenario)]
#[capture_scenario(unknown = frame_capture)]
enum UnknownContainerArgument {
    Value,
}

#[derive(frame_capture_macros::CaptureScenario)]
enum DuplicateVariantArguments {
    #[capture_scenario(id = "a", id = "b")]
    DuplicateId,
}

#[derive(frame_capture_macros::CaptureScenario)]
enum DuplicateTitleArgument {
    #[capture_scenario(title = "A", title = "B")]
    DuplicateTitle,
}

#[derive(frame_capture_macros::CaptureScenario)]
enum DuplicateDescriptionArgument {
    #[capture_scenario(description = "A", description = "B")]
    DuplicateDescription,
}

#[derive(frame_capture_macros::CaptureScenario)]
enum UnknownVariantArgument {
    #[capture_scenario(unknown = "value")]
    UnknownArgument,
}

#[derive(frame_capture_macros::CaptureScenario)]
enum EmptyScenarioId {
    #[capture_scenario(id = "")]
    Value,
}

fn main() {}
