pub mod framebuffer;
pub mod draw2d;

use framebuffer::FrameBuffer;
use draw2d::Draw2D;

use crate::camera::Camera;
use crate::math::{Vec2, Vec3};
use crate::texture::Texture;

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

    /// Proyección 3D: mundo -> pantalla
    pub fn project_point(&self, world: Vec3, camera: &Camera) -> Option<(i32, i32)> {
        let rel = world - camera.position;

        let forward = camera.forward();
        // ⬇⬇⬇ AQUÍ EL CAMBIO IMPORTANTE ⬇⬇⬇
        let right = Vec3::cross(forward, Vec3::up()).normalized();
        let up    = Vec3::cross(right, forward).normalized();

        let x_cam = rel.dot(right);
        let y_cam = rel.dot(up);
        let z_cam = -rel.dot(forward);

        // Si está demasiado cerca o detrás, no se dibuja
        if z_cam <= 0.1 {
            return None;
        }

        let f = (self.height as f32 / 2.0) / (camera.fov_y * 0.5).tan();

        let sx = self.width as f32 / 2.0 + x_cam * f / z_cam;
        let sy = self.height as f32 / 2.0 - y_cam * f / z_cam;

        Some((sx as i32, sy as i32))
    }

    pub fn world_to_screen_2d(&self, world: Vec2, camera_pos: Vec2, zoom: f32) -> (i32, i32) {
        let sx = (world.x - camera_pos.x) * zoom + (self.width as f32 / 2.0);
        let sy = (world.y - camera_pos.y) * zoom + (self.height as f32 / 2.0);

        (sx as i32, sy as i32)
    }

    /// Dibuja un planeta como DISCO 2D texturizado.
    pub fn draw_textured_sphere(
        &mut self,
        tex: &Texture,
        center: (i32, i32),
        radius: i32,
        rotation: f32,
    ) {
        if radius <= 0 {
            return;
        }

        let (cx, cy) = center;
        let r = radius as f32;
        let r2 = r * r;

        let cos_a = rotation.cos();
        let sin_a = rotation.sin();

        for py in -radius..=radius {
            let sy = cy + py;
            if sy < 0 || sy >= self.height as i32 {
                continue;
            }

            for px in -radius..=radius {
                let sx = cx + px;
                if sx < 0 || sx >= self.width as i32 {
                    continue;
                }

                let x = px as f32;
                let y = py as f32;
                let dist2 = x * x + y * y;
                if dist2 > r2 {
                    continue; // fuera del círculo
                }

                // Coord. normalizadas [-1, 1]
                let nx = x / r;
                let ny = y / r;

                // Rotar en 2D para que la textura "gire" sobre el planeta
                let rx = nx * cos_a - ny * sin_a;
                let ry = nx * sin_a + ny * cos_a;

                // Mapear a [0,1]
                let u = (rx + 1.0) * 0.5;        // 0..1
                let v = 1.0 - (ry + 1.0) * 0.5;  // 0..1 (invertimos Y)

                if u < 0.0 || u > 1.0 || v < 0.0 || v > 1.0 {
                    continue;
                }

                let tx = (u * (tex.width as f32 - 1.0)) as usize;
                let ty = (v * (tex.height as f32 - 1.0)) as usize;

                if tx >= tex.width || ty >= tex.height {
                    continue;
                }

                let color = tex.pixels[ty * tex.width + tx];
                let a = (color >> 24) & 0xFF;
                if a < 10 {
                    continue;
                }

                self.put_pixel(sx, sy, color);
            }
        }
    }

    /// Blit cuadrado genérico (sprites 2D, HUD, etc.).
    pub fn blit_sprite(&mut self, tex: &Texture, center: (i32, i32), size: i32) {
        if size <= 0 {
            return;
        }

        let half = size / 2;
        let start_x = center.0 - half;
        let start_y = center.1 - half;

        for y in 0..size {
            let sy = start_y + y;
            if sy < 0 || sy >= self.height as i32 {
                continue;
            }

            let v = y as f32 / size as f32;
            let ty = (v * tex.height as f32) as usize;
            if ty >= tex.height {
                continue;
            }

            for x in 0..size {
                let sx = start_x + x;
                if sx < 0 || sx >= self.width as i32 {
                    continue;
                }

                let u = x as f32 / size as f32;
                let tx = (u * tex.width as f32) as usize;
                if tx >= tex.width {
                    continue;
                }

                let idx = ty * tex.width + tx;
                let color = tex.pixels[idx];
                let a = (color >> 24) & 0xFF;

                if a < 10 {
                    continue;
                }

                self.put_pixel(sx, sy, color);
            }
        }
    }
}
