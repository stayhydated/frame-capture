use bevy::{
    prelude::default,
    window::{ExitCondition, Window, WindowPlugin},
};
use frame_capture::CaptureConfig;

pub fn capture_window_plugin(capture: Option<&CaptureConfig>, live_window: Window) -> WindowPlugin {
    WindowPlugin {
        primary_window: if capture.is_some() {
            None
        } else {
            Some(live_window)
        },
        exit_condition: if capture.is_some() {
            ExitCondition::DontExit
        } else {
            ExitCondition::OnAllClosed
        },
        ..default()
    }
}

pub fn is_capture_enabled(capture: Option<&CaptureConfig>) -> bool {
    capture.is_some()
}
