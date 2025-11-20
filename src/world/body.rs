use crate::math::Vec3;

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
}

impl Body {
    pub fn update(&mut self, dt: f32) {
        match self.kind {
            BodyKind::Star => {}
            BodyKind::Planet | BodyKind::Moon => {
                self.angle += self.orbit_speed * dt;
            }
        }
    }
}
