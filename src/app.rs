use std::time::Instant;

use minifb::{Key, Window, WindowOptions};

use crate::camera::Camera;
use crate::collision;
use crate::input::InputState;
use crate::math::Vec3;
use crate::renderer::Renderer;
use crate::ship::Ship;
use crate::skybox;
use crate::warp::WarpState;
use crate::world::SolarSystem;

pub struct App {
    window: Window,
    pub renderer: Renderer,
    system: SolarSystem,
    camera: Camera,
    ship: Ship,
    input: InputState,
    last_frame: Instant,
    running: bool,
    warp: WarpState,
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
        let ship = Ship::new();

        Self {
            window,
            renderer,
            system,
            camera,
            ship,
            input: InputState::new(),
            last_frame: Instant::now(),
            running: true,
            warp: WarpState::new(),
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
        self.ship.update(dt, &self.camera);

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

        skybox::draw_skybox(&mut self.renderer);
        self.system.render(&mut self.renderer, &self.camera);
        self.ship.render(&mut self.renderer, &self.camera);

        self.window
            .update_with_buffer(self.renderer.buffer(), self.renderer.width, self.renderer.height)
            .expect("Error al actualizar la ventana");
    }
}
