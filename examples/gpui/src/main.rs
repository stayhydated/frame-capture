use std::error::Error;

use frame_capture_example_gpui::{GpuiExampleRoute, GpuiExampleScenario};
use frame_capture_routes::{
    CaptureConfig, CaptureEnv, CaptureRoute as _, CaptureScenario as _, PixelSize,
};
use gpui::*;
use gpui_component::{Root, badge::Badge, h_flex, progress::Progress, v_flex};
use image::{RgbaImage, imageops::FilterType};

#[derive(Clone, Copy)]
struct RouteSummary {
    route: GpuiExampleRoute,
    id: &'static str,
    title: &'static str,
    size: PixelSize,
}

struct ExampleApp {
    capture: Option<CaptureConfig>,
    active_route: GpuiExampleRoute,
    scenario: Option<GpuiExampleScenario>,
    routes: Vec<RouteSummary>,
    size: PixelSize,
}

fn main() -> Result<(), Box<dyn Error>> {
    let env = CaptureEnv::frame_capture();
    let session = env.read_session::<GpuiExampleRoute>()?;
    let scenario = env.read_scenario::<GpuiExampleScenario>()?;
    let route = *session.route();
    let capture = session.capture().cloned();
    let size = session
        .capture()
        .map(|capture| capture.size())
        .unwrap_or_else(|| route.spec().default_size());

    if let Some(capture) = session.capture().cloned() {
        save_native_capture(route, scenario, capture.clone())?;
        println!(
            "saved native GPUI capture for frame {} to {}",
            capture.frame().get(),
            capture.path().display()
        );
        return Ok(());
    }

    let routes = route_summaries();
    let title = route.spec().title();

    let app = gpui_platform::application();
    app.run(move |cx: &mut App| {
        gpui_component::init(cx);
        cx.activate(true);

        let options = window_options(title, size, cx);
        cx.open_window(options, |window, cx| {
            let view = cx.new(|_| ExampleApp {
                capture,
                active_route: route,
                scenario,
                routes,
                size,
            });
            cx.new(|cx| Root::new(view, window, cx))
        })
        .expect("failed to open GPUI example window");
    });

    Ok(())
}

fn route_summaries() -> Vec<RouteSummary> {
    GpuiExampleRoute::ROUTES
        .iter()
        .copied()
        .map(RouteSummary::from)
        .collect()
}

fn window_options(title: &'static str, pixel_size: PixelSize, cx: &App) -> WindowOptions {
    let size = size(
        px(pixel_size.width() as f32),
        px(pixel_size.height() as f32),
    );

    WindowOptions {
        window_bounds: Some(WindowBounds::centered(size, cx)),
        titlebar: Some(TitlebarOptions {
            title: Some(format!("frame-capture GPUI - {title}").into()),
            ..Default::default()
        }),
        window_min_size: Some(size),
        ..Default::default()
    }
}

impl From<GpuiExampleRoute> for RouteSummary {
    fn from(route: GpuiExampleRoute) -> Self {
        let spec = route.spec();
        Self {
            route,
            id: spec.id(),
            title: spec.title(),
            size: spec.default_size(),
        }
    }
}

impl Render for ExampleApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let mut route_items = Vec::with_capacity(self.routes.len());
        for route in self.routes.iter().copied() {
            route_items.push(route_nav_item(route, self.active_route, cx).into_any_element());
        }

        div()
            .flex()
            .size_full()
            .bg(rgb(0xf6f4ef))
            .text_color(rgb(0x25282a))
            .child(
                v_flex()
                    .gap_3()
                    .w(px(260.0))
                    .p_4()
                    .bg(rgb(0x25302f))
                    .text_color(rgb(0xf7f3ea))
                    .child(
                        div()
                            .text_lg()
                            .font_weight(FontWeight::BOLD)
                            .child("Routes"),
                    )
                    .children(route_items)
                    .child(div().flex_1())
                    .child(capture_status(self.capture.as_ref())),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w(px(0.0))
                    .child(
                        h_flex()
                            .justify_between()
                            .px_5()
                            .py_4()
                            .border_b_1()
                            .border_color(rgb(0xd5d0c7))
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_2xl()
                                            .font_weight(FontWeight::BOLD)
                                            .child("frame-capture GPUI"),
                                    )
                                    .child(
                                        v_flex()
                                            .gap_1()
                                            .child(div().text_sm().text_color(rgb(0x68635c)).child(
                                                format!(
                                                    "{} at {}",
                                                    self.active_route.id(),
                                                    self.size,
                                                ),
                                            ))
                                            .child(
                                                div().text_xs().text_color(rgb(0x68635c)).child(
                                                    format!(
                                                        "scenario: {}",
                                                        scenario_name(self.scenario),
                                                    ),
                                                ),
                                            ),
                                    ),
                            )
                            .child(
                                Badge::new().dot().child(
                                    div()
                                        .rounded_md()
                                        .bg(rgb(0xffffff))
                                        .border_1()
                                        .border_color(rgb(0xd5d0c7))
                                        .px_3()
                                        .py_2()
                                        .text_sm()
                                        .child("route-only facade"),
                                ),
                            ),
                    )
                    .child(
                        div()
                            .flex_1()
                            .p_5()
                            .child(route_page(self.active_route, self.scenario)),
                    ),
            )
    }
}

fn route_nav_item(
    route: RouteSummary,
    active_route: GpuiExampleRoute,
    cx: &mut Context<ExampleApp>,
) -> impl IntoElement + 'static {
    let mut item = div()
        .id(ElementId::from(format!("route-nav-{}", route.id)))
        .rounded_md()
        .px_3()
        .py_2();

    if route.route == active_route {
        item = item
            .bg(rgb(0xf4b860))
            .text_color(rgb(0x1f2526))
            .font_weight(FontWeight::BOLD);
    }

    item.on_click(cx.listener(move |this, _, _, cx| {
        this.active_route = route.route;
        cx.notify();
    }))
    .child(
        v_flex().gap_1().child(route.title).child(
            div()
                .text_xs()
                .text_color(rgb(0xc9d4cf))
                .child(format!("{}  {}", route.id, route.size)),
        ),
    )
}

fn route_page(route: GpuiExampleRoute, scenario: Option<GpuiExampleScenario>) -> AnyElement {
    match route {
        GpuiExampleRoute::Dashboard => dashboard_page(scenario).into_any_element(),
        GpuiExampleRoute::Review => review_page(scenario).into_any_element(),
    }
}

fn capture_status(capture: Option<&CaptureConfig>) -> impl IntoElement {
    let panel = div()
        .rounded_md()
        .border_1()
        .border_color(rgb(0x53615f))
        .bg(rgb(0x303c3a))
        .p_3()
        .text_sm();

    match capture {
        Some(capture) => panel
            .child(
                div()
                    .font_weight(FontWeight::BOLD)
                    .child(format!("capture frame {}", capture.frame().get())),
            )
            .child(
                div()
                    .mt_2()
                    .text_color(rgb(0xd6ddd9))
                    .child(capture.path().display().to_string()),
            ),
        None => panel
            .child(div().font_weight(FontWeight::BOLD).child("live launch"))
            .child(
                div()
                    .mt_2()
                    .text_color(rgb(0xd6ddd9))
                    .child("No capture path requested"),
            ),
    }
}

fn dashboard_page(scenario: Option<GpuiExampleScenario>) -> impl IntoElement {
    let (queue, latency, pass) = match scenario {
        Some(GpuiExampleScenario::Seeded) => ("42", "6 ms", "99%"),
        Some(GpuiExampleScenario::Default) | None => ("18", "17 ms", "92%"),
    };

    v_flex()
        .gap_4()
        .size_full()
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::BOLD)
                .child("Dashboard"),
        )
        .child(
            div()
                .text_sm()
                .text_color(rgb(0x68635c))
                .child(format!("Scenario: {}", scenario_name(scenario))),
        )
        .child(
            div()
                .grid()
                .grid_cols(3)
                .gap_4()
                .child(metric("Queue", queue, rgb(0x2f7d64)))
                .child(metric("Latency", latency, rgb(0x6547a5)))
                .child(metric("Pass", pass, rgb(0x2d5f9a))),
        )
        .child(
            div()
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd5d0c7))
                .bg(rgb(0xffffff))
                .p_4()
                .child(div().font_weight(FontWeight::BOLD).child("Route state"))
                .child(
                    div()
                        .mt_2()
                        .text_color(rgb(0x625d56))
                        .child("The selected capture route is the Dashboard enum variant."),
                )
                .child(
                    Progress::new("dashboard-capture-progress")
                        .mt_4()
                        .value(92.0),
                ),
        )
}

fn review_page(scenario: Option<GpuiExampleScenario>) -> impl IntoElement {
    let statuses = match scenario {
        Some(GpuiExampleScenario::Seeded) => [
            ("layout", "seeded"),
            ("textures", "ready"),
            ("routing", "stable"),
            ("screenshot", "native"),
        ],
        Some(GpuiExampleScenario::Default) | None => [
            ("layout", "stable"),
            ("textures", "loaded"),
            ("routing", "validated"),
            ("screenshot", "native"),
        ],
    };

    v_flex()
        .gap_4()
        .size_full()
        .child(
            div()
                .text_2xl()
                .font_weight(FontWeight::BOLD)
                .child("Review"),
        )
        .child(
            div()
                .rounded_md()
                .border_1()
                .border_color(rgb(0xd5d0c7))
                .bg(rgb(0xffffff))
                .p_4()
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(0x68635c))
                        .child(format!("Scenario: {}", scenario_name(scenario))),
                )
                .children(statuses.into_iter().map(|(name, value)| {
                    h_flex()
                        .justify_between()
                        .w_full()
                        .py_2()
                        .border_b_1()
                        .border_color(rgb(0xebe6dd))
                        .child(name)
                        .child(
                            div()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x2d5f9a))
                                .child(value),
                        )
                })),
        )
}

fn scenario_name(scenario: Option<GpuiExampleScenario>) -> &'static str {
    scenario
        .map(|scenario| scenario.spec().title())
        .unwrap_or("default")
}

fn metric(title: &'static str, value: &'static str, color: Rgba) -> impl IntoElement {
    v_flex()
        .rounded_md()
        .border_1()
        .border_color(rgb(0xd5d0c7))
        .bg(rgb(0xffffff))
        .p_4()
        .child(div().text_sm().text_color(rgb(0x68635c)).child(title))
        .child(
            div()
                .mt_2()
                .text_3xl()
                .font_weight(FontWeight::BOLD)
                .text_color(color)
                .child(value),
        )
}

// #[cfg(target_os = "macos")]
fn save_native_capture(
    route: GpuiExampleRoute,
    scenario: Option<GpuiExampleScenario>,
    capture: CaptureConfig,
) -> Result<(), Box<dyn Error>> {
    use std::{fs, sync::Arc, time::Duration};

    use gpui::HeadlessAppContext;

    if let Some(parent) = capture
        .path()
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)?;
    }

    let size = capture.size();
    let mut cx = HeadlessAppContext::with_platform(
        Arc::new(gpui_wgpu::CosmicTextSystem::new("Helvetica")),
        Arc::new(()),
        || gpui_platform::current_headless_renderer(),
    );

    cx.update(|cx| {
        gpui_component::init(cx);
    });

    let capture_for_view = capture.clone();
    let routes = route_summaries();
    let window = cx.open_window(
        gpui::size(px(size.width() as f32), px(size.height() as f32)),
        move |window, cx| {
            let view = cx.new(|_| ExampleApp {
                capture: Some(capture_for_view),
                active_route: route,
                scenario,
                routes,
                size,
            });
            cx.new(|cx| Root::new(view, window, cx))
        },
    )?;
    let window = window.into();

    for _ in 0..capture.frame().get() {
        cx.run_until_parked();
        cx.update_window(window, |_, window, _| window.refresh())?;
        cx.advance_clock(Duration::from_secs_f64(1.0 / 60.0));
    }
    cx.run_until_parked();

    let mut image = cx.capture_screenshot(window)?;
    image = normalize_capture_screenshot(image, capture.size());
    image.save(capture.path())?;
    Ok(())
}

fn normalize_capture_screenshot(mut image: RgbaImage, size: PixelSize) -> RgbaImage {
    if image.width() == size.width() && image.height() == size.height() {
        return image;
    }

    image::imageops::resize(
        &mut image,
        size.width(),
        size.height(),
        FilterType::Lanczos3,
    )
}

#[cfg(test)]
mod tests {
    use frame_capture_routes::CaptureEnv;
    use image::RgbaImage;

    #[test]
    fn capture_env_width_and_height_override_route_default() {
        let env = CaptureEnv::frame_capture();
        let previous_route = std::env::var_os(env.route_var());
        let previous_path = std::env::var_os(env.path_var());
        let previous_width = std::env::var_os(env.width_var());
        let previous_height = std::env::var_os(env.height_var());

        unsafe {
            std::env::set_var(env.route_var(), "gpui/dashboard");
            std::env::set_var(env.path_var(), "captures/inline-env.png");
            std::env::set_var(env.width_var(), "960");
            std::env::set_var(env.height_var(), "540");
        }

        let session = env
            .read_session::<super::GpuiExampleRoute>()
            .expect("capture env should parse");
        let size = session
            .capture()
            .expect("capture env should request a capture")
            .size();

        assert_eq!(size.width(), 960);
        assert_eq!(size.height(), 540);

        match previous_route {
            Some(value) => unsafe { std::env::set_var(env.route_var(), value) },
            None => unsafe {
                std::env::remove_var(env.route_var());
            },
        }
        match previous_path {
            Some(value) => unsafe { std::env::set_var(env.path_var(), value) },
            None => unsafe {
                std::env::remove_var(env.path_var());
            },
        }
        match previous_width {
            Some(value) => unsafe { std::env::set_var(env.width_var(), value) },
            None => unsafe {
                std::env::remove_var(env.width_var());
            },
        }
        match previous_height {
            Some(value) => unsafe { std::env::set_var(env.height_var(), value) },
            None => unsafe {
                std::env::remove_var(env.height_var());
            },
        }
    }

    #[test]
    fn normalize_capture_screenshot_matches_requested_size() {
        let image = RgbaImage::new(1920, 1080);
        let normalized =
            super::normalize_capture_screenshot(image, super::PixelSize::new(960, 540));

        assert_eq!(normalized.width(), 960);
        assert_eq!(normalized.height(), 540);
    }
}
