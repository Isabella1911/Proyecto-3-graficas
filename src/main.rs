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
    let width: usize = 1280;  
    let height: usize = 720;

    println!("Iniciando Sistema Solar con Graphics Pipeline...");
    
    let mut app = app::App::new(width, height);
    app.run();
    
    println!("Aplicación finalizada.");
}