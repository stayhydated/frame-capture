use bevy::app::AppExit;

#[path = "app.rs"]
mod app;

fn main() -> Result<AppExit, Box<dyn std::error::Error>> {
    app::run()
}
