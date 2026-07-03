use std::ops::Deref;

use bevy::{
    app::{App, Plugin},
    prelude::*,
    state::{app::StatesPlugin, state::FreelyMutableState},
};
use frame_capture::{CaptureRoute, CaptureScenario, RouteSpec};

pub struct RoutePlugin<R, F> {
    route: R,
    install: F,
}

/// Bevy resource holding the selected capture route.
///
/// Use this when the route enum lives in a target-neutral crate and should not
/// derive Bevy traits.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Resource)]
pub struct SelectedCaptureRoute<R: CaptureRoute> {
    route: R,
}

/// Inserts the selected capture route as a Bevy resource.
pub struct RouteResourcePlugin<R> {
    route: R,
}

/// Inserts the selected capture route as a Bevy `State`.
///
/// Use this when a route enum also drives app schedules through `OnEnter` or
/// `in_state`.
pub struct RouteStatePlugin<R> {
    route: R,
}

pub struct ScenarioPlugin<S, F> {
    scenario: Option<S>,
    install: F,
}

/// Bevy resource holding the selected capture scenario, if any.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Resource)]
pub struct SelectedCaptureScenario<S: CaptureScenario> {
    scenario: Option<S>,
}

/// Inserts the selected capture scenario as a Bevy resource.
pub struct ScenarioResourcePlugin<S> {
    scenario: Option<S>,
}

/// Inserts the selected capture scenario as a Bevy `State`.
///
/// The `default_state` is used when no scenario was requested, which keeps live
/// mode and capture mode explicit.
pub struct ScenarioStatePlugin<S> {
    scenario: Option<S>,
    default_state: S,
}

/// Inserts an already selected value as a Bevy `State`.
///
/// Use this after mapping capture route, scenario, or application config
/// into the app's own state enum.
pub struct SelectedStatePlugin<S> {
    state: S,
}

impl<R, F> RoutePlugin<R, F> {
    pub fn new(route: R, install: F) -> Self {
        Self { route, install }
    }
}

impl<R> SelectedCaptureRoute<R>
where
    R: CaptureRoute,
{
    pub const fn new(route: R) -> Self {
        Self { route }
    }

    pub const fn route(self) -> R {
        self.route
    }

    pub fn id(self) -> &'static str {
        self.route.id()
    }

    pub fn spec(self) -> RouteSpec {
        self.route.spec()
    }
}

impl<R> Deref for SelectedCaptureRoute<R>
where
    R: CaptureRoute,
{
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.route
    }
}

impl<R> RouteResourcePlugin<R> {
    pub const fn new(route: R) -> Self {
        Self { route }
    }
}

impl<R> RouteStatePlugin<R> {
    pub fn new(route: R) -> Self {
        Self { route }
    }

    pub fn route(&self) -> R
    where
        R: Copy,
    {
        self.route
    }
}

impl<S, F> ScenarioPlugin<S, F> {
    pub fn new(scenario: Option<S>, install: F) -> Self {
        Self { scenario, install }
    }
}

impl<S> SelectedCaptureScenario<S>
where
    S: CaptureScenario,
{
    pub const fn new(scenario: Option<S>) -> Self {
        Self { scenario }
    }

    pub const fn scenario(self) -> Option<S> {
        self.scenario
    }

    pub const fn unwrap_or(self, default: S) -> S {
        match self.scenario {
            Some(scenario) => scenario,
            None => default,
        }
    }
}

impl<S> ScenarioResourcePlugin<S> {
    pub const fn new(scenario: Option<S>) -> Self {
        Self { scenario }
    }
}

impl<S> ScenarioStatePlugin<S> {
    pub fn new(scenario: Option<S>, default_state: S) -> Self {
        Self {
            scenario,
            default_state,
        }
    }

    pub fn selected_state(&self) -> S
    where
        S: Copy,
    {
        self.scenario.unwrap_or(self.default_state)
    }
}

impl<S> SelectedStatePlugin<S> {
    pub fn new(state: S) -> Self {
        Self { state }
    }
}

impl<R, F> Plugin for RoutePlugin<R, F>
where
    R: CaptureRoute + Send + Sync,
    F: Fn(R, &mut App) + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        (self.install)(self.route, app);
    }
}

impl<R> Plugin for RouteResourcePlugin<R>
where
    R: CaptureRoute + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedCaptureRoute::new(self.route));
    }
}

impl<R> Plugin for RouteStatePlugin<R>
where
    R: CaptureRoute + FreelyMutableState,
{
    fn build(&self, app: &mut App) {
        ensure_states_plugin(app);
        app.insert_state(self.route);
    }
}

impl<S, F> Plugin for ScenarioPlugin<S, F>
where
    S: CaptureScenario + Send + Sync,
    F: Fn(Option<S>, &mut App) + Send + Sync + 'static,
{
    fn build(&self, app: &mut App) {
        (self.install)(self.scenario, app);
    }
}

impl<S> Plugin for ScenarioResourcePlugin<S>
where
    S: CaptureScenario + Send + Sync,
{
    fn build(&self, app: &mut App) {
        app.insert_resource(SelectedCaptureScenario::new(self.scenario));
    }
}

impl<S> Plugin for ScenarioStatePlugin<S>
where
    S: CaptureScenario + FreelyMutableState,
{
    fn build(&self, app: &mut App) {
        ensure_states_plugin(app);
        app.insert_state(self.selected_state());
    }
}

impl<S> Plugin for SelectedStatePlugin<S>
where
    S: FreelyMutableState,
{
    fn build(&self, app: &mut App) {
        ensure_states_plugin(app);
        app.insert_state(self.state.clone());
    }
}

fn ensure_states_plugin(app: &mut App) {
    if !app.is_plugin_added::<StatesPlugin>() {
        app.add_plugins(StatesPlugin);
    }
}
