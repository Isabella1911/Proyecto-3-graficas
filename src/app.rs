use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::camera::Camera;
use crate::collision;
use crate::input::InputState;
use crate::math::Vec3;
use crate::renderer::Renderer;
use crate::skybox;
use crate::texture::Texture;
use crate::warp::WarpState;
use crate::world::SolarSystem;

pub struct App {
    window: Window,
    pub renderer: Renderer,
    system: SolarSystem,
    camera: Camera,
    input: InputState,
    last_frame: Instant,
    running: bool,
    warp: WarpState,

    // Texturas
    textura_sol: Texture,
    textura_planeta1: Texture,
    textura_planeta2: Texture,
    textura_planeta3: Texture,
    textura_planeta4: Texture,
    textura_luna: Texture,
    textura_cielo: Texture,
}

impl App {
    pub fn new(width: usize, height: usize) -> Self {
        let window = Window::new(
            "Sistema Solar - Rust Software Renderer",
            width,
            height,
            WindowOptions::default(),
        )
        .expect("No se pudo crear la ventana");

        let renderer = Renderer::new(width, height);
        let system = SolarSystem::new_demo();
        let camera = Camera::new();

        let textura_sol = Texture::from_file("assets/textures/sun.jpg");
        let textura_planeta1 = Texture::from_file("assets/textures/mercury.jpg");
        let textura_planeta2 = Texture::from_file("assets/textures/venus.jpg");
        let textura_planeta3 = Texture::from_file("assets/textures/earth.jpg");
        let textura_planeta4 = Texture::from_file("assets/textures/moon.jpg");
        let textura_luna = Texture::from_file("assets/textures/mars.jpg");
        let textura_cielo = Texture::from_file("assets/textures/stars.jpg");

        Self {
            window,
            renderer,
            system,
            camera,
            input: InputState::new(),
            last_frame: Instant::now(),
            running: true,
            warp: WarpState::new(),
            textura_sol,
            textura_planeta1,
            textura_planeta2,
            textura_planeta3,
            textura_planeta4,
            textura_luna,
            textura_cielo,
        }
    }

    pub fn run(&mut self) {
        while self.running && self.window.is_open() && !self.window.is_key_down(Key::Escape) {
            let now = Instant::now();
            let dt = (now - self.last_frame).as_secs_f32();
            self.last_frame = now;

            self.input.update(&self.window);
            self.update(dt);
            self.render();

            std::thread::sleep(std::time::Duration::from_millis(5));
        }
    }

    fn update(&mut self, dt: f32) {
        self.handle_warps();

        if self.warp.active {
            self.warp.update(dt, &mut self.camera);
        } else {
            self.camera.update(dt, &self.input);
        }

        self.system.update(dt);

        collision::resolve_collisions(&self.system, &mut self.camera);
    }

    fn handle_warps(&mut self) {
        if self.input.warp_1 {
            self.instant_warp_to_body(1);
        }
        if self.input.warp_2 {
            self.instant_warp_to_body(2);
        }
        if self.input.warp_3 {
            self.instant_warp_to_body(3);
        }

        if self.input.warp_animated && !self.warp.active {
            self.start_animated_warp(1);
        }
    }

    fn instant_warp_to_body(&mut self, index: usize) {
        if index >= self.system.bodies.len() {
            return;
        }
        let center = self.system.body_position(index);
        self.camera.position = center + Vec3::new(0.0, 20.0, 40.0);
    }

    fn start_animated_warp(&mut self, index: usize) {
        if index >= self.system.bodies.len() {
            return;
        }
        let center = self.system.body_position(index);
        let target = center + Vec3::new(0.0, 20.0, 40.0);
        let start = self.camera.position;

        self.warp.start_animated(start, target, 1.8);
    }

    fn render(&mut self) {
        self.renderer.clear(0x000000);

        skybox::draw_skybox(&mut self.renderer, &self.camera, &self.textura_cielo);  

        self.system.render(&mut self.renderer, &self.camera);

        let mut body_indices: Vec<(usize, f32)> = Vec::new();
        
        for i in 0..self.system.bodies.len() {
            let body_pos = self.system.body_position(i);
            let distance = (body_pos - self.camera.position).length();
            body_indices.push((i, distance));
        }
        
        body_indices.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        
        for (i, _) in body_indices {
            if let Some(((sx, sy), radius_px)) =
                self.system.project_body(i, &self.renderer, &self.camera)
            {
                let tex = match i {
                    0 => &self.textura_sol,
                    1 => &self.textura_planeta1,
                    2 => &self.textura_planeta2,
                    3 => &self.textura_planeta3,
                    4 => &self.textura_planeta4,
                    5 => &self.textura_luna,
                    _ => continue,
                };
                
                let rotation = self.system.bodies[i].angle;

                self.renderer
                    .draw_textured_sphere(tex, (sx, sy), radius_px, rotation);
            }
        }

        self.window
            .update_with_buffer(self.renderer.buffer(), self.renderer.width, self.renderer.height)
            .expect("Error al actualizar la ventana");
    }
}
