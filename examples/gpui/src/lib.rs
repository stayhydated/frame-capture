#[derive(frame_capture_routes::CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
#[capture_route(default = Dashboard)]
pub enum GpuiExampleRoute {
    #[capture_route(id = "gpui/dashboard", title = "GPUI Dashboard")]
    Dashboard,
    #[capture_route(id = "gpui/review", title = "GPUI Review")]
    Review,
}

impl GpuiExampleRoute {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Dashboard => "/dashboard",
            Self::Review => "/review",
        }
    }
}

#[derive(frame_capture_routes::CaptureScenarioRoutes, Clone, Copy, Debug, Eq, PartialEq)]
pub enum GpuiExampleScenario {
    #[capture_scenario(id = "default", title = "Default State")]
    Default,
    #[capture_scenario(id = "seeded", title = "Seeded State")]
    Seeded,
}

#[cfg(test)]
mod tests {
    use frame_capture_routes::CaptureRoute as _;

    #[derive(frame_capture_routes::CaptureRouteRoutes, Clone, Copy, Debug, Eq, PartialEq)]
    #[capture_route(default = Dashboard)]
    enum GpuiTomlRoute {
        #[capture_route(id = "gpui/toml-dashboard", title = "Toml Dashboard")]
        Dashboard,
        #[capture_route(id = "gpui/toml-review", title = "Toml Review")]
        Review,
    }

    #[test]
    fn route_default_size_comes_from_gpui_toml_override() {
        assert_eq!(
            super::GpuiExampleRoute::Dashboard
                .spec()
                .default_size()
                .width(),
            1024
        );
        assert_eq!(
            super::GpuiExampleRoute::Dashboard
                .spec()
                .default_size()
                .height(),
            576
        );
        assert_eq!(
            super::GpuiExampleRoute::Review
                .spec()
                .default_size()
                .width(),
            1024
        );
        assert_eq!(
            super::GpuiExampleRoute::Review
                .spec()
                .default_size()
                .height(),
            576
        );
        assert_eq!(GpuiTomlRoute::Dashboard.spec().default_size().width(), 1024);
        assert_eq!(GpuiTomlRoute::Dashboard.spec().default_size().height(), 576);
    }
}
