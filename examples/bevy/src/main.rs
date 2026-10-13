use bevy::app::AppExit;

mod app;

fn main() -> Result<AppExit, Box<dyn std::error::Error>> {
    app::run()
}
