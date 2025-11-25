use crate::math::{Vec2, Vec3};
use crate::renderer::pipeline::Vertex;
use std::f32::consts::PI;


pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
        }
    }

    /// esfera
    pub fn create_sphere(radius: f32, rings: usize, sectors: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let r_step = PI / rings as f32;
        let s_step = 2.0 * PI / sectors as f32;

        
        for r in 0..=rings {
            let lat = PI / 2.0 - r as f32 * r_step;
            let y = radius * lat.sin();
            let xz = radius * lat.cos();

            for s in 0..=sectors {
                let lon = s as f32 * s_step;
                let x = xz * lon.cos();
                let z = xz * lon.sin();

                let position = Vec3::new(x, y, z);
                let normal = position.normalized();
                let uv = Vec2::new(
                    s as f32 / sectors as f32,
                    r as f32 / rings as f32,
                );

                vertices.push(Vertex::new(position, normal, uv));
            }
        }

        
        for r in 0..rings {
            for s in 0..sectors {
                let curr = r * (sectors + 1) + s;
                let next = curr + sectors + 1;

                
                indices.push(curr as u32);
                indices.push(next as u32);
                indices.push((curr + 1) as u32);

                
                indices.push((curr + 1) as u32);
                indices.push(next as u32);
                indices.push((next + 1) as u32);
            }
        }

        Self { vertices, indices }
    }

    /// Cubo
    pub fn create_cube(size: f32) -> Self {
        let half = size * 0.5;
        
        let positions = [
            
            Vec3::new(-half, -half,  half),
            Vec3::new( half, -half,  half),
            Vec3::new( half,  half,  half),
            Vec3::new(-half,  half,  half),
            
            Vec3::new(-half, -half, -half),
            Vec3::new(-half,  half, -half),
            Vec3::new( half,  half, -half),
            Vec3::new( half, -half, -half),
            
            Vec3::new(-half,  half, -half),
            Vec3::new(-half,  half,  half),
            Vec3::new( half,  half,  half),
            Vec3::new( half,  half, -half),
            
            Vec3::new(-half, -half, -half),
            Vec3::new( half, -half, -half),
            Vec3::new( half, -half,  half),
            Vec3::new(-half, -half,  half),
            
            Vec3::new( half, -half, -half),
            Vec3::new( half,  half, -half),
            Vec3::new( half,  half,  half),
            Vec3::new( half, -half,  half),
            
            Vec3::new(-half, -half, -half),
            Vec3::new(-half, -half,  half),
            Vec3::new(-half,  half,  half),
            Vec3::new(-half,  half, -half),
        ];

        let normals = [
            Vec3::new( 0.0,  0.0,  1.0), // Front
            Vec3::new( 0.0,  0.0, -1.0), // Back
            Vec3::new( 0.0,  1.0,  0.0), // Top
            Vec3::new( 0.0, -1.0,  0.0), // Bottom
            Vec3::new( 1.0,  0.0,  0.0), // Right
            Vec3::new(-1.0,  0.0,  0.0), // Left
        ];

        let uvs = [
            Vec2::new(0.0, 1.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(1.0, 0.0),
            Vec2::new(0.0, 0.0),
        ];

        let mut vertices = Vec::new();
        
        for face in 0..6 {
            let normal = normals[face];
            for i in 0..4 {
                let pos_idx = face * 4 + i;
                vertices.push(Vertex::new(
                    positions[pos_idx],
                    normal,
                    uvs[i],
                ));
            }
        }

        let mut indices = Vec::new();
        for face in 0..6 {
            let base = (face * 4) as u32;
            
            indices.push(base);
            indices.push(base + 1);
            indices.push(base + 2);
            
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 3);
        }

        Self { vertices, indices }
    }

    
    pub fn create_orbit_ring(radius: f32, segments: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        let thickness = 0.2; // Grosor del anillo
        let angle_step = 2.0 * PI / segments as f32;
        
        
        for i in 0..=segments {
            let angle = i as f32 * angle_step;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            
            
            let inner_pos = Vec3::new(
                (radius - thickness) * cos_a,
                0.0,
                (radius - thickness) * sin_a,
            );
            vertices.push(Vertex::new(
                inner_pos,
                Vec3::up(),
                Vec2::new(i as f32 / segments as f32, 0.0),
            ));
            
        
            let outer_pos = Vec3::new(
                (radius + thickness) * cos_a,
                0.0,
                (radius + thickness) * sin_a,
            );
            vertices.push(Vertex::new(
                outer_pos,
                Vec3::up(),
                Vec2::new(i as f32 / segments as f32, 1.0),
            ));
        }
        
        
        for i in 0..segments {
            let base = (i * 2) as u32;
            
            
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 1);
            
            
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base + 3);
        }
        
        Self { vertices, indices }
    }

    /// plano
    pub fn create_plane(size: f32, subdivisions: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        let step = size / subdivisions as f32;
        let half = size * 0.5;
        
        // Generar vértices
        for z in 0..=subdivisions {
            for x in 0..=subdivisions {
                let pos = Vec3::new(
                    x as f32 * step - half,
                    0.0,
                    z as f32 * step - half,
                );
                let uv = Vec2::new(
                    x as f32 / subdivisions as f32,
                    z as f32 / subdivisions as f32,
                );
                
                vertices.push(Vertex::new(pos, Vec3::up(), uv));
            }
        }
        
        
        for z in 0..subdivisions {
            for x in 0..subdivisions {
                let base = (z * (subdivisions + 1) + x) as u32;
                let next_row = base + (subdivisions + 1) as u32;
                
                
                indices.push(base);
                indices.push(next_row);
                indices.push(base + 1);
                
                
                indices.push(base + 1);
                indices.push(next_row);
                indices.push(next_row + 1);
            }
        }
        
        Self { vertices, indices }
    }

    /// Cilindro
    pub fn create_cylinder(radius: f32, height: f32, segments: usize) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        
        let half_height = height * 0.5;
        let angle_step = 2.0 * PI / segments as f32;
        
        
        for i in 0..=segments {
            let angle = i as f32 * angle_step;
            let cos_a = angle.cos();
            let sin_a = angle.sin();
            
            
            vertices.push(Vertex::new(
                Vec3::new(radius * cos_a, half_height, radius * sin_a),
                Vec3::new(cos_a, 0.0, sin_a),
                Vec2::new(i as f32 / segments as f32, 0.0),
            ));
            
            
            vertices.push(Vertex::new(
                Vec3::new(radius * cos_a, -half_height, radius * sin_a),
                Vec3::new(cos_a, 0.0, sin_a),
                Vec2::new(i as f32 / segments as f32, 1.0),
            ));
        }
        
        
        for i in 0..segments {
            let base = (i * 2) as u32;
            
            indices.push(base);
            indices.push(base + 2);
            indices.push(base + 1);
            
            indices.push(base + 1);
            indices.push(base + 2);
            indices.push(base + 3);
        }
        
        
        let top_center_idx = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, half_height, 0.0),
            Vec3::up(),
            Vec2::new(0.5, 0.5),
        ));
        
        let bottom_center_idx = vertices.len() as u32;
        vertices.push(Vertex::new(
            Vec3::new(0.0, -half_height, 0.0),
            Vec3::up() * -1.0,
            Vec2::new(0.5, 0.5),
        ));
        
        Self { vertices, indices }
    }

    
    pub fn transform(&mut self, transform: impl Fn(Vec3) -> Vec3) {
        for vertex in &mut self.vertices {
            vertex.position = transform(vertex.position);
        }
        self.recalculate_normals();
    }

    
    pub fn recalculate_normals(&mut self) {
        
        for vertex in &mut self.vertices {
            vertex.normal = Vec3::zero();
        }
        
        
        for i in (0..self.indices.len()).step_by(3) {
            let i0 = self.indices[i] as usize;
            let i1 = self.indices[i + 1] as usize;
            let i2 = self.indices[i + 2] as usize;
            
            let v0 = self.vertices[i0].position;
            let v1 = self.vertices[i1].position;
            let v2 = self.vertices[i2].position;
            
            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let face_normal = Vec3::cross(edge1, edge2).normalized();
            
            self.vertices[i0].normal = self.vertices[i0].normal + face_normal;
            self.vertices[i1].normal = self.vertices[i1].normal + face_normal;
            self.vertices[i2].normal = self.vertices[i2].normal + face_normal;
        }
        
        
        for vertex in &mut self.vertices {
            vertex.normal = vertex.normal.normalized();
        }
    }

    
    pub fn append(&mut self, other: &Mesh) {
        let base_index = self.vertices.len() as u32;
        
        self.vertices.extend_from_slice(&other.vertices);
        
        for &idx in &other.indices {
            self.indices.push(base_index + idx);
        }
    }
}