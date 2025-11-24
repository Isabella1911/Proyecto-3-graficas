use crate::math::Vec3;
use super::orbit::Orbit;

#[derive(Clone, Copy)]
pub enum BodyKind {
    Star,
    Planet,
    Moon,
}

pub struct Body {
    pub name: String,
    pub kind: BodyKind,
    pub radius: f32,
    pub color: u32,

    // Parámetros orbitales
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub angle: f32,
    pub parent: Option<usize>, // índice del cuerpo alrededor del que orbita
}

impl Body {
    /// Actualiza el ángulo orbital según el tiempo transcurrido
    pub fn update(&mut self, dt: f32) {
        match self.kind {
            BodyKind::Star => {
                // Las estrellas no orbitan en este modelo
            }
            BodyKind::Planet | BodyKind::Moon => {
                self.angle += self.orbit_speed * dt;
            }
        }
    }

    /// Indica si este cuerpo tiene órbita "dibujable"
    pub fn has_orbit(&self) -> bool {
        matches!(self.kind, BodyKind::Planet | BodyKind::Moon) && self.orbit_radius > 0.0
    }

    /// Construye la órbita asociada a este cuerpo, dada la posición del centro
    /// (normalmente el padre, o el origen si no tiene padre).
    ///
    /// Esto se usa como parte de "Primitive Assembly": a partir de parámetros
    /// (radio, centro) generamos una primitiva geométrica (`Orbit`) que luego
    /// se discretiza en líneas.
    pub fn build_orbit(&self, center_world: Vec3, segments: usize) -> Option<Orbit> {
        if !self.has_orbit() {
            return None;
        }

        Some(Orbit::new(center_world, self.orbit_radius, segments))
    }
}
