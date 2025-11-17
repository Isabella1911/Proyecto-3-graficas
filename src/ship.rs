use crate::camera::Camera;
use crate::math::Vec3;
use crate::renderer::Renderer;

pub struct Ship {
    pub position: Vec3,
}

impl Ship {
    pub fn new() -> Self {
        Ship {
            position: Vec3::zero(),
        }
    }

    pub fn update(&mut self, _dt: f32, camera: &Camera) {
        let forward = camera.forward();
        self.position = camera.position + forward * 15.0;
    }

    pub fn render(&self, renderer: &mut Renderer, camera: &Camera) {
        if let Some((cx, cy)) = renderer.project_point(self.position, camera) {
            let size = 10;
            let p0 = (cx, cy - size);
            let p1 = (cx - size / 2, cy + size / 2);
            let p2 = (cx + size / 2, cy + size / 2);

            renderer.draw_triangle(p0, p1, p2, 0xFFFFFFFF);
        }
    }
}
