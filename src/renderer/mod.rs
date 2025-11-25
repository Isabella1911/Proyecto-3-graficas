pub mod framebuffer;
pub mod draw2d;
pub mod pipeline;
pub mod mesh;

use framebuffer::FrameBuffer;
use draw2d::Draw2D;
use pipeline::{Pipeline, ShaderType, Uniforms};
use mesh::Mesh;

use crate::camera::Camera;
use crate::math::{Vec2, Vec3, Vec4, Matrix4};
use crate::texture::Texture;

pub struct Renderer {
    pub width: usize,
    pub height: usize,
    fb: FrameBuffer,
    pub pipeline: Pipeline,
}

impl Renderer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            fb: FrameBuffer::new(width, height),
            pipeline: Pipeline::new(width, height),
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.fb.clear(color);
        self.pipeline.clear(color);
    }

    pub fn buffer(&self) -> &[u32] {
        &self.pipeline.color_buffer
    }

    pub fn put_pixel(&mut self, x: i32, y: i32, color: u32) {
        self.fb.put_pixel(x, y, color);
    }

    
    pub fn setup_camera(&mut self, camera: &Camera) {
        let aspect = self.width as f32 / self.height as f32;
        
        
        let forward = camera.forward();
        let target = camera.position + forward;
        
        
        self.pipeline.uniforms.view = Matrix4::look_at(
            camera.position,
            target,
            Vec3::up(),
        );
        
        self.pipeline.uniforms.projection = Matrix4::perspective(
            camera.fov_y,
            aspect,
            0.1,
            1000.0,
        );
        
        self.pipeline.uniforms.camera_pos = camera.position;
        self.pipeline.uniforms.update_mvp();
    }

    
    pub fn render_mesh_pipeline(
        &mut self,
        mesh: &Mesh,
        model_matrix: Matrix4,
        shader_type: ShaderType,
        texture: Option<&Texture>,
    ) {
        
        self.pipeline.uniforms.model = model_matrix;
        self.pipeline.uniforms.update_mvp();
        
        
        self.pipeline.render_mesh(
            &mesh.vertices,
            &mesh.indices,
            shader_type,
            texture,
        );
    }

    
    pub fn render_solar_body(
        &mut self,
        position: Vec3,
        radius: f32,
        rotation: f32,
        shader_type: ShaderType,
        texture: Option<&Texture>,
        mesh: &Mesh,
    ) {
        
        let model = Matrix4::translation(position.x, position.y, position.z)
            * Matrix4::rotation_y(rotation)
            * Matrix4::scale(radius, radius, radius);
        
        self.render_mesh_pipeline(mesh, model, shader_type, texture);
    }

    /// Renderizar órbita
    pub fn render_orbit_ring(
        &mut self,
        center: Vec3,
        radius: f32,
        mesh: &Mesh,
    ) {
        let model = Matrix4::translation(center.x, center.y, center.z)
            * Matrix4::scale(1.0, 1.0, 1.0);
        
        self.pipeline.uniforms.model = model;
        self.pipeline.uniforms.update_mvp();
        
        
        self.pipeline.render_mesh(
            &mesh.vertices,
            &mesh.indices,
            ShaderType::Basic,
            None,
        );
    }

    
    pub fn present(&mut self) {
        // Copiar del pipeline al framebuffer
        for (i, &color) in self.pipeline.color_buffer.iter().enumerate() {
            self.fb.pixels[i] = color;
        }
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

    pub fn project_point(&self, world: Vec3, camera: &Camera) -> Option<(i32, i32)> {
        let aspect = self.width as f32 / self.height as f32;
        let (x_ndc, y_ndc, _z_cam) = camera.project_to_ndc(world, aspect)?;
        let sx = ((x_ndc + 1.0) * 0.5 * self.width as f32).round() as i32;
        let sy = ((1.0 - (y_ndc + 1.0) * 0.5) * self.height as f32).round() as i32;
        Some((sx, sy))
    }

    
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
                    continue;
                }

                let nx = x / r;
                let ny = y / r;

                let rx = nx * cos_a - ny * sin_a;
                let ry = nx * sin_a + ny * cos_a;

                let u = (rx + 1.0) * 0.5;
                let v = 1.0 - (ry + 1.0) * 0.5;

                let color = tex.sample_uv(u, v);
                self.put_pixel(sx, sy, color);
            }
        }
    }

    pub fn draw_lit_sphere(
        &mut self,
        center: (i32, i32),
        radius: i32,
        base_color: u32,
        light_dir: Vec3,
    ) {
        
        if radius <= 0 {
            return;
        }

        let (cx, cy) = center;
        let r = radius as f32;
        let r2 = r * r;
        let light = light_dir.normalized();

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
                    continue;
                }

                let nx = x / r;
                let ny = y / r;
                let nz2 = 1.0 - nx * nx - ny * ny;
                if nz2 <= 0.0 {
                    continue;
                }
                let nz = nz2.sqrt();

                let normal = Vec3::new(nx, ny, nz).normalized();
                let lambert = normal.dot(light).max(0.0);
                let k = 0.2 + 0.8 * lambert;

                let a = (base_color >> 24) & 0xFF;
                let r_c = (base_color >> 16) & 0xFF;
                let g_c = (base_color >> 8) & 0xFF;
                let b_c = base_color & 0xFF;

                let r_sh = ((r_c as f32) * k).clamp(0.0, 255.0) as u32;
                let g_sh = ((g_c as f32) * k).clamp(0.0, 255.0) as u32;
                let b_sh = ((b_c as f32) * k).clamp(0.0, 255.0) as u32;

                let final_color = (a << 24) | (r_sh << 16) | (g_sh << 8) | b_sh;
                self.put_pixel(sx, sy, final_color);
            }
        }
    }
}