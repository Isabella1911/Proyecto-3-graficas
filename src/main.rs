mod math;
mod renderer;
mod world;
mod camera;
mod texture;
mod skybox;
mod app;
mod input;
mod collision;
mod warp;

fn main() {
    let width: usize = 960;
    let height: usize = 540;

    let mut app = app::App::new(width, height);
    app.run();
}
