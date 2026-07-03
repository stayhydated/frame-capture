use bevy::prelude::States;

#[derive(frame_capture_bevy::CaptureRouteBevy, Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
#[capture_route(default = Dashboard)]
pub enum BevyExampleRoute {
    #[capture_route(id = "bevy/dashboard", title = "Bevy Dashboard")]
    Dashboard,
    #[capture_route(id = "bevy/detail", title = "Bevy Detail")]
    Detail,
}

#[derive(frame_capture_bevy::CaptureScenarioBevy, Clone, Copy, Debug, Eq, PartialEq)]
pub enum BevyExampleScenario {
    #[capture_scenario(id = "alert", title = "Alert State")]
    Alert,
}

#[cfg(test)]
mod tests {
    use frame_capture_bevy::CaptureRoute;

    #[derive(
        frame_capture_bevy::CaptureRouteBevy,
        Clone,
        Copy,
        Debug,
        Eq,
        Hash,
        PartialEq,
        bevy::prelude::States,
    )]
    #[capture_route(default = Dashboard)]
    enum BevyTomlRoute {
        #[capture_route(id = "bevy/toml-dashboard", title = "Toml Dashboard")]
        Dashboard,
        #[capture_route(id = "bevy/toml-detail", title = "Toml Detail")]
        Detail,
    }

    #[test]
    fn route_default_size_comes_from_bevy_toml_override() {
        assert_eq!(BevyTomlRoute::Dashboard.spec().default_size().width(), 1365);
        assert_eq!(BevyTomlRoute::Dashboard.spec().default_size().height(), 769);
        assert_eq!(BevyTomlRoute::Detail.spec().default_size().width(), 1365);
        assert_eq!(BevyTomlRoute::Detail.spec().default_size().height(), 769);
    }
}
