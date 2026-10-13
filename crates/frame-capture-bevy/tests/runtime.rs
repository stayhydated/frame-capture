use bevy::{
    app::PluginGroupBuilder, camera::RenderTarget, prelude::*, render::view::screenshot::Screenshot,
};
use frame_capture_bevy::{
    BevyCaptureAppExt as _, BevyCaptureConfig, CaptureConfig, CaptureFrame, CaptureReady,
    CaptureWarmupPlugin, PixelSize, prelude::CaptureSystems,
};

struct CaptureAssetsPlugins;

impl PluginGroup for CaptureAssetsPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(MinimalPlugins)
            .add(|app: &mut App| {
                app.init_resource::<Assets<Image>>();
            })
    }
}

fn capture_app(frame: u32) -> App {
    let capture = CaptureConfig::try_new(
        "capture.png",
        CaptureFrame::new(frame),
        PixelSize::new(32, 20),
    )
    .unwrap();
    let mut app = App::new();
    app.add_capture_plugins(CaptureAssetsPlugins, Some(capture))
        .add_systems(Startup, |mut commands: Commands| {
            commands.spawn(Camera::default());
        });
    app
}

fn screenshot_count(app: &mut App) -> usize {
    let world = app.world_mut();
    world.query::<&Screenshot>().iter(world).count()
}

#[test]
fn capture_plugins_accept_a_group_without_winit_and_redirect_startup_cameras() {
    let mut app = capture_app(1);
    assert_eq!(
        app.world().resource::<BevyCaptureConfig>().size(),
        PixelSize::new(32, 20)
    );

    app.update();

    let world = app.world_mut();
    let target = world
        .query_filtered::<&RenderTarget, With<Camera>>()
        .single(world)
        .unwrap();
    let RenderTarget::Image(target) = target else {
        panic!("Startup camera must render to the capture image");
    };
    let image = world
        .resource::<Assets<Image>>()
        .get(&target.handle)
        .unwrap();
    assert_eq!(image.width(), 32);
    assert_eq!(image.height(), 20);
    assert_eq!(screenshot_count(&mut app), 1);
}

#[test]
fn readiness_before_capture_systems_requests_on_the_same_update_and_latches() {
    fn mark_ready(mut ready: ResMut<CaptureReady>, mut updates: Local<u32>) {
        *updates += 1;
        if *updates == 3 {
            ready.mark_ready();
        }
    }

    let mut app = capture_app(2);
    app.insert_resource(CaptureReady::pending())
        .add_systems(Update, mark_ready.before(CaptureSystems));

    app.update();
    assert_eq!(screenshot_count(&mut app), 0);
    app.update();
    assert_eq!(screenshot_count(&mut app), 0);
    app.update();
    assert_eq!(screenshot_count(&mut app), 1);
    app.update();
    assert_eq!(screenshot_count(&mut app), 1);
}

#[test]
fn warmup_finishes_before_capture_systems_on_the_requested_update() {
    let mut app = capture_app(1);
    app.add_plugins(CaptureWarmupPlugin::frames(2));

    app.update();
    assert_eq!(screenshot_count(&mut app), 0);
    app.update();
    assert_eq!(screenshot_count(&mut app), 1);
}
