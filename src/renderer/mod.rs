pub mod framebuffer;
pub mod draw2d;

use framebuffer::FrameBuffer;
use draw2d::Draw2D;

use crate::camera::Camera;
use crate::math::{Vec2, Vec3};

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    fb: FrameBuffer,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            fb: FrameBuffer::new(width, height),
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.fb.clear(color);
    }

    pub fn buffer(&self) -> &[u32] {
        &self.fb.pixels
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.fb.put_pixel(x, y, color);
    }

    pub fn draw_filled_circle(&mut self, center: (i32, i32), radius: i32, color: u32) {
        let mut d = Draw2D::new(&mut self.fb);
        d.filled_circle(center, radius, color);
    }

    pub fn draw_circle(&mut self, center: (i32, i32), radius: i32, color: u32) {
        let mut d = Draw2D::new(&mut self.fb);
        d.circle(center, radius, color);
    }

    pub fn draw_line(&mut self, p0: (i32, i32), p1: (i32, i32), color: u32) {
        let mut d = Draw2D::new(&mut self.fb);
        d.line(p0, p1, color);
    }

    pub fn draw_triangle(&mut self, p0: (i32, i32), p1: (i32, i32), p2: (i32, i32), color: u32) {
        let mut d = Draw2D::new(&mut self.fb);
        d.triangle(p0, p1, p2, color);
    }

    // Proyección 3D: mundo -> pantalla
    pub fn project_point(&self, world: Vec3, camera: &Camera) -> Option<(i32, i32)> {
        let rel = world - camera.position;

        let forward = camera.forward();
        let right = forward.cross(Vec3::up()).normalized();
        let up = right.cross(forward).normalized();

        let x_cam = rel.dot(right);
        let y_cam = rel.dot(up);
        let z_cam = -rel.dot(forward);

        if z_cam <= 0.1 {
            return None;
        }

        let f = (self.height as f32 / 2.0) / (camera.fov_y * 0.5).tan();

        let sx = self.width as f32 / 2.0 + x_cam * f / z_cam;
        let sy = self.height as f32 / 2.0 - y_cam * f / z_cam;

        Some((sx as i32, sy as i32))
    }

    // (opcional) proyección 2D simple
    pub fn world_to_screen_2d(&self, world: Vec2, camera_pos: Vec2, zoom: f32) -> (i32, i32) {
        let sx = (world.x - camera_pos.x) * zoom + (self.width as f32 / 2.0);
        let sy = (world.y - camera_pos.y) * zoom + (self.height as f32 / 2.0);

        (sx as i32, sy as i32)
    }
}