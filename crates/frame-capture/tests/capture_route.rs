#![cfg(feature = "macros")]

use std::str::FromStr as _;

use frame_capture::{CaptureRoute, CaptureScenario as _, PixelSize};

#[derive(CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Root)]
enum DemoPage {
    #[capture_route(id = "root", title = "Root", size = "1920x1080")]
    Root,
    #[capture_route(
        id = "settings/tool",
        title = "Tool Settings",
        width = 800,
        height = 480
    )]
    ToolSettings,
}

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = RootPage)]
enum ConventionalPage {
    RootPage,
    ToolSettings,
}

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Root, size = "1920x1080")]
enum SharedSizePage {
    #[capture_route(id = "root", title = "Root")]
    Root,
    #[capture_route(id = "settings/tool", title = "Tool Settings")]
    ToolSettings,
}

#[derive(frame_capture::CaptureRoute, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard, id_prefix = "desktop", size = "1440x900")]
enum PrefixedPage {
    Dashboard,
    #[capture_route(id = "settings/tool", title = "Tool Settings")]
    ToolSettings,
}

#[derive(frame_capture::CaptureScenario, Clone, Copy, Debug, Eq, PartialEq)]
enum OverlayPanel {
    #[capture_scenario(
        id = "target",
        title = "Target Note",
        description = "Target note overlay"
    )]
    TargetNote,
    Scale,
    #[capture_scenario(id = "state")]
    State,
}

#[test]
fn derive_generates_route_metadata() {
    assert_eq!(DemoPage::DEFAULT, DemoPage::Root);
    assert_eq!(DemoPage::ROUTES, &[DemoPage::Root, DemoPage::ToolSettings]);
    assert_eq!(DemoPage::VARIANTS.len(), 2);
    assert_eq!(DemoPage::ROUTE_SPECS[1].route, DemoPage::ToolSettings);
    assert_eq!(DemoPage::ROUTE_SPECS[1].spec, DemoPage::ToolSettings.spec());
    assert_eq!(DemoPage::ToolSettings.id(), "settings/tool");
    assert_eq!(DemoPage::ToolSettings.spec().title(), "Tool Settings");
    assert_eq!(
        DemoPage::ToolSettings.spec().default_size(),
        PixelSize::new(800, 480)
    );
}

#[test]
fn derive_generates_display_and_from_str() {
    assert_eq!(DemoPage::Root.to_string(), "root");
    assert_eq!(
        DemoPage::from_str("settings/tool"),
        Ok(DemoPage::ToolSettings)
    );

    let error = DemoPage::from_str("missing").unwrap_err();
    assert_eq!(error.value(), "missing");
    assert_eq!(error.expected(), &["root", "settings/tool"]);
}

#[test]
fn derive_defaults_metadata_from_conventions_and_toml() {
    assert_eq!(ConventionalPage::RootPage.id(), "root_page");
    assert_eq!(ConventionalPage::RootPage.spec().title(), "RootPage");
    assert_eq!(
        ConventionalPage::ToolSettings.spec().default_size(),
        PixelSize::new(1920, 1080)
    );
}

#[test]
fn derive_supports_enum_level_default_size() {
    assert_eq!(SharedSizePage::DEFAULT, SharedSizePage::Root);
    assert_eq!(
        SharedSizePage::ToolSettings.spec().default_size(),
        PixelSize::new(1920, 1080)
    );
    assert_eq!(SharedSizePage::ToolSettings.id(), "settings/tool");
}

#[test]
fn derive_supports_enum_level_id_prefix() {
    assert_eq!(PrefixedPage::Dashboard.id(), "desktop/dashboard");
    assert_eq!(PrefixedPage::ToolSettings.id(), "desktop/settings/tool");
    assert_eq!(
        PrefixedPage::from_str("desktop/settings/tool"),
        Ok(PrefixedPage::ToolSettings)
    );

    let error = PrefixedPage::from_str("settings/tool").unwrap_err();
    assert_eq!(
        error.expected(),
        &["desktop/dashboard", "desktop/settings/tool"]
    );
}

#[test]
fn derives_typed_capture_scenario_metadata() {
    assert_eq!(OverlayPanel::TargetNote.id(), "target");
    assert_eq!(OverlayPanel::TargetNote.id_ref().as_str(), "target");
    assert_eq!(OverlayPanel::Scale.id(), "scale");
    assert_eq!(
        OverlayPanel::SCENARIOS,
        &[
            OverlayPanel::TargetNote,
            OverlayPanel::Scale,
            OverlayPanel::State
        ]
    );
    assert_eq!(OverlayPanel::VARIANTS, &["target", "scale", "state"]);
    assert_eq!(OverlayPanel::SPECS[0].id(), "target");
    assert_eq!(
        OverlayPanel::SCENARIO_SPECS[0].value,
        OverlayPanel::TargetNote
    );
    assert_eq!(OverlayPanel::SCENARIO_SPECS[0].spec, OverlayPanel::SPECS[0]);
    assert_eq!(OverlayPanel::SPECS[0].title(), "Target Note");
    assert_eq!(
        OverlayPanel::SPECS[0].description(),
        Some("Target note overlay")
    );
    assert_eq!(OverlayPanel::TargetNote.spec(), OverlayPanel::SPECS[0]);
    assert_eq!(
        OverlayPanel::from_id("target"),
        Ok(OverlayPanel::TargetNote)
    );
    assert_eq!(OverlayPanel::from_id("state"), Ok(OverlayPanel::State));

    let error = OverlayPanel::from_id("missing").unwrap_err();
    assert_eq!(error.expected(), &["target", "scale", "state"]);
}
