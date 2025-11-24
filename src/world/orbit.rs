// src/world/orbit.rs
use crate::math::Vec3;

/// Representa una órbita circular en el plano X-Y.
/// Este módulo se encarga de la "Primitive Assembly":
/// genera los vértices y los segmentos de línea que
/// el renderer usará para dibujar la órbita.
pub struct Orbit {
    /// Centro de la órbita en espacio de mundo
    pub center: Vec3,
    /// Radio de la órbita
    pub radius: f32,
    /// Cantidad de segmentos en los que se discretiza el círculo
    pub segments: usize,
}

impl Orbit {
    pub fn new(center: Vec3, radius: f32, segments: usize) -> Self {
        Self {
            center,
            radius,
            segments: segments.max(3), // evitar cosas degeneradas
        }
    }

    /// Genera los vértices (en espacio de mundo) que forman el círculo.
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

    /// Convierte la lista de vértices en primitivas de línea (pares de puntos).
    /// Esto es literalmente "primitive assembly": de vértices sueltos → líneas.
    pub fn generate_line_primitives(&self) -> Vec<(Vec3, Vec3)> {
        let verts = self.generate_vertices();
        let mut lines = Vec::with_capacity(self.segments);

        for i in 0..verts.len() {
            let a = verts[i];
            let b = verts[(i + 1) % verts.len()]; // cerrar el círculo
            lines.push((a, b));
        }

        lines
    }

    /// Devuelve la posición en la órbita para un ángulo dado (en radianes).
    /// Esto lo podemos usar para posicionar un Body (planeta, luna, etc.)
    pub fn position_at(&self, angle: f32) -> Vec3 {
        let x = self.center.x + self.radius * angle.cos();
        let y = self.center.y + self.radius * angle.sin();
        let z = self.center.z;

        Vec3::new(x, y, z)
    }
}
