use crate::math::{Vec2, Vec3, Vec4, Matrix4};
use crate::texture::Texture;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub color: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            uv,
            color: Vec3::new(1.0, 1.0, 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClipVertex {
    pub position: Vec4,
    pub world_pos: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub color: Vec3,
}

pub struct Triangle {
    pub v0: ClipVertex,
    pub v1: ClipVertex,
    pub v2: ClipVertex,
}

#[derive(Clone)]
pub struct Fragment {
    pub screen_x: i32,
    pub screen_y: i32,
    pub depth: f32,
    pub world_pos: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub color: Vec3,
}

pub struct Uniforms {
    pub model: Matrix4,
    pub view: Matrix4,
    pub projection: Matrix4,
    pub mvp: Matrix4,
    pub normal_matrix: Matrix4,
    pub light_pos: Vec3,
    pub light_color: Vec3,
    pub ambient_color: Vec3,
    pub camera_pos: Vec3,
    pub time: f32,
}

impl Uniforms {
    pub fn new() -> Self {
        Self {
            model: Matrix4::identity(),
            view: Matrix4::identity(),
            projection: Matrix4::identity(),
            mvp: Matrix4::identity(),
            normal_matrix: Matrix4::identity(),
            light_pos: Vec3::new(0.0, 100.0, 50.0),
            light_color: Vec3::new(1.0, 1.0, 1.0),
            ambient_color: Vec3::new(0.2, 0.2, 0.2),
            camera_pos: Vec3::zero(),
            time: 0.0,
        }
    }

    pub fn update_mvp(&mut self) {
        self.mvp = self.projection * self.view * self.model;
        let mv = self.view * self.model;
        self.normal_matrix = mv.inverse().transpose();
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ShaderType {
    Basic,
    Phong,
    Textured,
    Star,
}

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms, shader_type: &ShaderType) -> ClipVertex {
    let world_pos = (uniforms.model * vertex.position.to_vec4_point()).to_vec3();
    let clip_pos = uniforms.mvp * vertex.position.to_vec4_point();
    let normal = (uniforms.normal_matrix * vertex.normal.to_vec4_dir()).to_vec3().normalized();
    
    ClipVertex {
        position: clip_pos,
        world_pos,
        normal,
        uv: vertex.uv,
        color: vertex.color,
    }
}

pub fn fragment_shader(
    fragment: &Fragment,
    uniforms: &Uniforms,
    shader_type: ShaderType,
    texture: Option<&Texture>,
) -> u32 {
    let color = match shader_type {
        ShaderType::Basic => {
            fragment.color
        }
        ShaderType::Phong => {
            let ambient = uniforms.ambient_color;
            let light_dir = (uniforms.light_pos - fragment.world_pos).normalized();
            let diff = fragment.normal.dot(light_dir).max(0.0);
            let diffuse = uniforms.light_color * diff;
            let view_dir = (uniforms.camera_pos - fragment.world_pos).normalized();
            let reflect_dir = reflect(light_dir * -1.0, fragment.normal);
            let spec = view_dir.dot(reflect_dir).max(0.0).powf(32.0);
            let specular = uniforms.light_color * spec * 0.5;
            let final_color = fragment.color;
            Vec3::new(
                (ambient.x + diffuse.x + specular.x) * final_color.x,
                (ambient.y + diffuse.y + specular.y) * final_color.y,
                (ambient.z + diffuse.z + specular.z) * final_color.z,
            )
        }
        ShaderType::Textured => {
            let tex_color = if let Some(tex) = texture {
                let color_u32 = tex.sample_uv(fragment.uv.x, fragment.uv.y);
                Vec3::new(
                    ((color_u32 >> 16) & 0xFF) as f32 / 255.0,
                    ((color_u32 >> 8) & 0xFF) as f32 / 255.0,
                    (color_u32 & 0xFF) as f32 / 255.0,
                )
            } else {
                fragment.color
            };
            let light_dir = (uniforms.light_pos - fragment.world_pos).normalized();
            let diff = fragment.normal.dot(light_dir).max(0.0);
            let lighting = 0.3 + 0.7 * diff;
            tex_color * lighting
        }
        ShaderType::Star => {
            if let Some(tex) = texture {
                let color_u32 = tex.sample_uv(fragment.uv.x, fragment.uv.y);
                let tex_color = Vec3::new(
                    ((color_u32 >> 16) & 0xFF) as f32 / 255.0,
                    ((color_u32 >> 8) & 0xFF) as f32 / 255.0,
                    (color_u32 & 0xFF) as f32 / 255.0,
                );
                let glow = 1.0 + (uniforms.time * 2.0).sin() * 0.1;
                tex_color * glow
            } else {
                Vec3::new(1.0, 0.9, 0.7)
            }
        }
    };
    
    vec3_to_color(color)
}

pub struct Pipeline {
    pub width: usize,
    pub height: usize,
    pub depth_buffer: Vec<f32>,
    pub color_buffer: Vec<u32>,
    pub uniforms: Uniforms,
}

impl Pipeline {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            depth_buffer: vec![f32::INFINITY; width * height],
            color_buffer: vec![0x00000000; width * height],
            uniforms: Uniforms::new(),
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.color_buffer.fill(color);
        self.depth_buffer.fill(f32::INFINITY);
    }

    pub fn process_vertices(
        &self,
        vertices: &[Vertex],
        shader_type: ShaderType,
    ) -> Vec<ClipVertex> {
        vertices
            .iter()
            .map(|v| vertex_shader(v, &self.uniforms, &shader_type))
            .collect()
    }

    pub fn assemble_triangles(
        &self,
        vertices: &[ClipVertex],
        indices: &[u32],
    ) -> Vec<Triangle> {
        let mut triangles = Vec::new();
        
        for i in (0..indices.len()).step_by(3) {
            if i + 2 < indices.len() {
                let i0 = indices[i] as usize;
                let i1 = indices[i + 1] as usize;
                let i2 = indices[i + 2] as usize;
                
                if i0 < vertices.len() && i1 < vertices.len() && i2 < vertices.len() {
                    triangles.push(Triangle {
                        v0: vertices[i0].clone(),
                        v1: vertices[i1].clone(),
                        v2: vertices[i2].clone(),
                    });
                }
            }
        }
        
        triangles
    }

    pub fn rasterize_triangle(
        &mut self,
        triangle: &Triangle,
        shader_type: ShaderType,
        texture: Option<&Texture>,
    ) {
        let v0_ndc = triangle.v0.position.perspective_divide();
        let v1_ndc = triangle.v1.position.perspective_divide();
        let v2_ndc = triangle.v2.position.perspective_divide();
        
        let v0_screen = self.ndc_to_screen(v0_ndc);
        let v1_screen = self.ndc_to_screen(v1_ndc);
        let v2_screen = self.ndc_to_screen(v2_ndc);
        
        let min_x = v0_screen.x.min(v1_screen.x).min(v2_screen.x).max(0.0) as i32;
        let max_x = v0_screen.x.max(v1_screen.x).max(v2_screen.x).min(self.width as f32 - 1.0) as i32;
        let min_y = v0_screen.y.min(v1_screen.y).min(v2_screen.y).max(0.0) as i32;
        let max_y = v0_screen.y.max(v1_screen.y).max(v2_screen.y).min(self.height as f32 - 1.0) as i32;
        
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let p = Vec3::new(x as f32 + 0.5, y as f32 + 0.5, 0.0);
                
                let (alpha, beta, gamma) = barycentric(
                    Vec2::new(v0_screen.x, v0_screen.y),
                    Vec2::new(v1_screen.x, v1_screen.y),
                    Vec2::new(v2_screen.x, v2_screen.y),
                    Vec2::new(p.x, p.y),
                );
                
                if alpha >= 0.0 && beta >= 0.0 && gamma >= 0.0 {
                    let depth = alpha * v0_screen.z + beta * v1_screen.z + gamma * v2_screen.z;
                    let pixel_index = y as usize * self.width + x as usize;
                    
                    if depth < self.depth_buffer[pixel_index] {
                        let world_pos = triangle.v0.world_pos * alpha
                            + triangle.v1.world_pos * beta
                            + triangle.v2.world_pos * gamma;
                        
                        let normal = (triangle.v0.normal * alpha
                            + triangle.v1.normal * beta
                            + triangle.v2.normal * gamma)
                            .normalized();
                        
                        let uv = Vec2::new(
                            triangle.v0.uv.x * alpha + triangle.v1.uv.x * beta + triangle.v2.uv.x * gamma,
                            triangle.v0.uv.y * alpha + triangle.v1.uv.y * beta + triangle.v2.uv.y * gamma,
                        );
                        
                        let color = triangle.v0.color * alpha
                            + triangle.v1.color * beta
                            + triangle.v2.color * gamma;
                        
                        let fragment = Fragment {
                            screen_x: x,
                            screen_y: y,
                            depth,
                            world_pos,
                            normal,
                            uv,
                            color,
                        };
                        
                        let final_color = fragment_shader(&fragment, &self.uniforms, shader_type, texture);
                        
                        self.color_buffer[pixel_index] = final_color;
                        self.depth_buffer[pixel_index] = depth;
                    }
                }
            }
        }
    }

    fn ndc_to_screen(&self, ndc: Vec4) -> Vec3 {
        Vec3::new(
            (ndc.x + 1.0) * 0.5 * self.width as f32,
            (1.0 - ndc.y) * 0.5 * self.height as f32,
            ndc.z,
        )
    }

    pub fn render_mesh(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
        shader_type: ShaderType,
        texture: Option<&Texture>,
    ) {
        let clip_vertices = self.process_vertices(vertices, shader_type);
        let triangles = self.assemble_triangles(&clip_vertices, indices);
        
        for triangle in triangles {
            self.rasterize_triangle(&triangle, shader_type, texture);
        }
    }
}

fn reflect(incident: Vec3, normal: Vec3) -> Vec3 {
    incident - normal * (2.0 * incident.dot(normal))
}

fn barycentric(a: Vec2, b: Vec2, c: Vec2, p: Vec2) -> (f32, f32, f32) {
    let v0 = Vec2::new(c.x - a.x, c.y - a.y);
    let v1 = Vec2::new(b.x - a.x, b.y - a.y);
    let v2 = Vec2::new(p.x - a.x, p.y - a.y);
    
    let dot00 = v0.x * v0.x + v0.y * v0.y;
    let dot01 = v0.x * v1.x + v0.y * v1.y;
    let dot02 = v0.x * v2.x + v0.y * v2.y;
    let dot11 = v1.x * v1.x + v1.y * v1.y;
    let dot12 = v1.x * v2.x + v1.y * v2.y;
    
    let inv_denom = 1.0 / (dot00 * dot11 - dot01 * dot01);
    let u = (dot11 * dot02 - dot01 * dot12) * inv_denom;
    let v = (dot00 * dot12 - dot01 * dot02) * inv_denom;
    
    (1.0 - u - v, v, u)
}

fn vec3_to_color(color: Vec3) -> u32 {
    let r = (color.x.clamp(0.0, 1.0) * 255.0) as u32;
    let g = (color.y.clamp(0.0, 1.0) * 255.0) as u32;
    let b = (color.z.clamp(0.0, 1.0) * 255.0) as u32;
    0xFF000000 | (r << 16) | (g << 8) | b
}
