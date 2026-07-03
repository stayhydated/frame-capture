use std::time::Duration;

use bevy::{
    app::{App, AppExit, Plugin, PluginGroup, ScheduleRunnerPlugin},
    camera::RenderTarget,
    prelude::*,
    render::render_resource::TextureFormat,
    render::view::screenshot::{Screenshot, ScreenshotCaptured},
    winit::WinitPlugin,
};
use frame_capture::{CaptureConfig, CaptureFrameGate};

use crate::{BevyCaptureConfig, DEFAULT_CAPTURE_FPS, save::save_capture_to_disk};

pub struct CapturePlugin;

#[derive(Default, Resource)]
struct CaptureState {
    gate: CaptureFrameGate,
}

#[derive(Clone, Resource)]
struct CaptureTarget {
    image: Handle<Image>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Resource)]
pub enum CaptureReady {
    Pending,
    Ready,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaptureWarmupPlugin {
    frames: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Resource)]
struct CaptureWarmup {
    frames_remaining: u32,
}

pub trait BevyCaptureAppExt {
    fn add_capture_runtime(&mut self, capture: Option<CaptureConfig>) -> &mut Self;

    fn add_capture_plugins<P: PluginGroup>(
        &mut self,
        plugins: P,
        capture: Option<CaptureConfig>,
    ) -> &mut Self;
}

impl CaptureReady {
    pub const fn ready() -> Self {
        Self::Ready
    }

    pub const fn pending() -> Self {
        Self::Pending
    }

    pub const fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }

    pub fn mark_ready(&mut self) {
        *self = Self::Ready;
    }

    pub fn mark_pending(&mut self) {
        *self = Self::Pending;
    }
}

impl Default for CaptureReady {
    fn default() -> Self {
        Self::ready()
    }
}

impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CaptureState>()
            .init_resource::<CaptureReady>()
            .add_systems(PostStartup, setup_capture_target)
            .add_systems(Update, trigger_capture);
    }
}

impl CaptureWarmupPlugin {
    pub const fn new(frames: u32) -> Self {
        Self { frames }
    }

    pub const fn frames(frames: u32) -> Self {
        Self::new(frames)
    }

    pub const fn frame_count(self) -> u32 {
        self.frames
    }
}

impl Plugin for CaptureWarmupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CaptureReady::pending())
            .insert_resource(CaptureWarmup {
                frames_remaining: self.frames,
            })
            .add_systems(Update, mark_capture_warmup_ready.before(trigger_capture));
    }
}

impl BevyCaptureAppExt for App {
    fn add_capture_runtime(&mut self, capture: Option<CaptureConfig>) -> &mut Self {
        if let Some(capture) = capture {
            self.insert_resource(BevyCaptureConfig::new(capture))
                .add_plugins(CapturePlugin)
                .add_plugins(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
                    1.0 / DEFAULT_CAPTURE_FPS,
                )));
        }

        self
    }

    fn add_capture_plugins<P: PluginGroup>(
        &mut self,
        plugins: P,
        capture: Option<CaptureConfig>,
    ) -> &mut Self {
        if capture.is_some() {
            self.add_plugins(plugins.build().disable::<WinitPlugin>());
        } else {
            self.add_plugins(plugins);
        }

        self.add_capture_runtime(capture)
    }
}

fn setup_capture_target(
    mut commands: Commands,
    config: Res<BevyCaptureConfig>,
    mut images: ResMut<Assets<Image>>,
    mut render_targets: Query<&mut RenderTarget, With<Camera>>,
) {
    let mut image = Image::new_target_texture(
        config.size().width(),
        config.size().height(),
        TextureFormat::Rgba8UnormSrgb,
        None,
    );
    image.texture_descriptor.label = Some("frame_capture_bevy_target");

    let image = images.add(image);
    for mut render_target in &mut render_targets {
        *render_target = RenderTarget::Image(image.clone().into());
    }

    commands.insert_resource(CaptureTarget { image });
}

fn trigger_capture(
    mut commands: Commands,
    config: Res<BevyCaptureConfig>,
    ready: Res<CaptureReady>,
    target: Option<Res<CaptureTarget>>,
    mut state: ResMut<CaptureState>,
) {
    if state.gate.requested() {
        return;
    }

    state.gate.advance();
    if !ready.is_ready() || !state.gate.ready(config.frame()) {
        return;
    }

    let Some(target) = target else {
        return;
    };
    let path = config.path().to_path_buf();

    state.gate.mark_requested();
    commands
        .spawn(Screenshot::image(target.image.clone()))
        .observe(
            move |captured: On<ScreenshotCaptured>, mut app_exit: MessageWriter<AppExit>| {
                match save_capture_to_disk(path.clone(), captured) {
                    Ok(()) => app_exit.write(AppExit::Success),
                    Err(error) => {
                        eprintln!("Cannot save screenshot: {error}");
                        app_exit.write(AppExit::error())
                    },
                };
            },
        );
}

fn mark_capture_warmup_ready(mut warmup: ResMut<CaptureWarmup>, mut ready: ResMut<CaptureReady>) {
    if warmup.frames_remaining > 0 {
        warmup.frames_remaining -= 1;
    }
    if warmup.frames_remaining == 0 {
        ready.mark_ready();
    }
}
