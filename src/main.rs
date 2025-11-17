mod app;
mod renderer;
mod math;
mod world;
mod camera;
mod ship;
mod warp;
mod skybox;
mod collision;
mod input;

use app::App;

fn main() {
    let mut app = App::new(800, 600);
    app.run();
}