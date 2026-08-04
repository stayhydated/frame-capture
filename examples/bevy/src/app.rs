use bevy::{
    prelude::*,
    window::{PresentMode, Window},
};
use frame_capture_bevy::{BevyCaptureEnvExt as _, CaptureEnv, CaptureRoute as _};
use frame_capture_example_bevy::{BevyExampleRoute, BevyExampleScenario};

const DEMO_MARKER: &str = "frame-capture-bevy-demo";

#[derive(Clone, Copy, Resource)]
struct ActiveScenario(Option<BevyExampleScenario>);

pub(crate) fn run() -> Result<(), Box<dyn std::error::Error>> {
    let session = CaptureEnv::frame_capture()
        .read_bevy_session_with_scenario::<BevyExampleRoute, BevyExampleScenario>()?;
    let route = session.route();

    let window = Window {
        title: format!("{DEMO_MARKER} - {}", route.spec().title()),
        resolution: session.window_resolution(),
        present_mode: PresentMode::AutoNoVsync,
        canvas: cfg!(target_family = "wasm").then(|| "#bevy-demo".to_owned()),
        fit_canvas_to_parent: cfg!(target_family = "wasm"),
        ..default()
    };

    let mut app = App::new();
    session.add_capture_plugins(
        &mut app,
        DefaultPlugins.set(session.capture_window_plugin(window)),
    );
    session.add_route_state(&mut app);
    session.insert_mapped_scenario_resource(&mut app, ActiveScenario);
    app.add_systems(Startup, setup_scene);
    app.run();

    Ok(())
}

fn setup_scene(
    mut commands: Commands,
    route: Res<State<BevyExampleRoute>>,
    scenario: Option<Res<ActiveScenario>>,
) {
    commands.spawn(Camera2d);
    let alert = scenario
        .as_deref()
        .is_some_and(|scenario| matches!(scenario.0, Some(BevyExampleScenario::Alert)));

    let route = *route.get();
    let (background, accent, label, metric) = match route {
        BevyExampleRoute::Dashboard => (
            Color::srgb(0.08, 0.10, 0.12),
            Color::srgb(0.18, 0.72, 0.55),
            "Dashboard",
            "92%",
        ),
        BevyExampleRoute::Detail => (
            Color::srgb(0.11, 0.09, 0.14),
            Color::srgb(0.72, 0.42, 0.86),
            "Detail",
            "17 ms",
        ),
    };
    let accent = if alert {
        Color::srgb(0.94, 0.28, 0.24)
    } else {
        accent
    };
    let metric = if alert { "ALERT" } else { metric };

    commands.insert_resource(ClearColor(background));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.15, 0.17, 0.20), Vec2::new(720.0, 340.0)),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
    commands.spawn((
        Sprite::from_color(accent, Vec2::new(720.0, 12.0)),
        Transform::from_xyz(0.0, 176.0, 1.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.22, 0.24, 0.28), Vec2::new(260.0, 150.0)),
        Transform::from_xyz(-190.0, -18.0, 1.0),
    ));
    commands.spawn((
        Sprite::from_color(Color::srgb(0.18, 0.20, 0.24), Vec2::new(260.0, 150.0)),
        Transform::from_xyz(190.0, -18.0, 1.0),
    ));
    commands.spawn((
        Text2d::new(format!("frame-capture {label}")),
        TextFont {
            font_size: FontSize::Px(44.0),
            ..default()
        },
        TextLayout::justify(Justify::Center),
        Transform::from_xyz(0.0, 88.0, 2.0),
    ));
    commands.spawn((
        Text2d::new(metric),
        TextFont {
            font_size: FontSize::Px(58.0),
            ..default()
        },
        TextColor(accent),
        TextLayout::justify(Justify::Center),
        Transform::from_xyz(-190.0, -30.0, 2.0),
    ));
    commands.spawn((
        Text2d::new(route.id()),
        TextFont {
            font_size: FontSize::Px(34.0),
            ..default()
        },
        TextColor(Color::srgb(0.78, 0.82, 0.88)),
        TextLayout::justify(Justify::Center),
        Transform::from_xyz(190.0, -30.0, 2.0),
    ));
}
