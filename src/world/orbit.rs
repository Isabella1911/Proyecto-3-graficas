
use crate::math::Vec3;


pub struct Orbit {
    pub center: Vec3,
    pub radius: f32,
    pub segments: usize,
}

impl Orbit {
    pub fn new(center: Vec3, radius: f32, segments: usize) -> Self {
        Self {
            center,
            radius,
            segments: segments.max(3), 
        }
    }

    
    pub fn generate_vertices(&self) -> Vec<Vec3> {
        let mut vertices = Vec::with_capacity(self.segments);

        let step = std::f32::consts::TAU / self.segments as f32;

        for i in 0..self.segments {
            let angle = i as f32 * step;
            let x = self.center.x + self.radius * angle.cos();
            let y = self.center.y + self.radius * angle.sin();
            let z = self.center.z; // plano eclíptico

            vertices.push(Vec3::new(x, y, z));
        }

        vertices
    }

    
    pub fn generate_line_primitives(&self) -> Vec<(Vec3, Vec3)> {
        let verts = self.generate_vertices();
        let mut lines = Vec::with_capacity(self.segments);

        for i in 0..verts.len() {
            let a = verts[i];
            let b = verts[(i + 1) % verts.len()]; 
            lines.push((a, b));
        }

        lines
    }

    
    pub fn position_at(&self, angle: f32) -> Vec3 {
        let x = self.center.x + self.radius * angle.cos();
        let y = self.center.y + self.radius * angle.sin();
        let z = self.center.z;

        Vec3::new(x, y, z)
    }
}