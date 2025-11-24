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
    pub orbit_radius: f32,
    pub orbit_speed: f32,
    pub angle: f32,
    pub parent: Option<usize>,
    pub use_procedural: bool,
}

impl Body {
    pub fn update(&mut self, dt: f32) {
        match self.kind {
            BodyKind::Star => {
            }
            BodyKind::Planet | BodyKind::Moon => {
                self.angle += self.orbit_speed * dt;
            }
        }
    }

    pub fn has_orbit(&self) -> bool {
        matches!(self.kind, BodyKind::Planet | BodyKind::Moon) && self.orbit_radius > 0.0
    }

    pub fn build_orbit(&self, center_world: Vec3, segments: usize) -> Option<Orbit> {
        if !self.has_orbit() {
            return None;
        }

        Some(Orbit::new(center_world, self.orbit_radius, segments))
    }
}
